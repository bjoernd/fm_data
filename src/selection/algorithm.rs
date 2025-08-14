use super::categories::role_belongs_to_category;
use super::types::{Assignment, Footedness, Player, PlayerFilter, Role, Team, ABILITIES};
use crate::constants::data_layout;
use crate::domain::{PlayerId, RoleId};
use crate::error::{FMDataError, Result};
use crate::error_helpers::validation_error;
use std::collections::HashMap;

/// Parse player data from Google Sheets raw data into Player structs
pub fn parse_player_data(sheet_data: Vec<Vec<String>>) -> Result<Vec<Player>> {
    let mut players = Vec::new();

    for (row_index, row) in sheet_data.iter().enumerate() {
        // Skip rows where column A (player name) is empty
        if row.is_empty() || row[0].trim().is_empty() {
            continue;
        }

        let player_name = row[0].trim().to_string();

        // Parse age (column B)
        let age = if row.len() > 1 {
            match row[1].trim().parse::<u8>() {
                Ok(age) => age,
                Err(_) => {
                    log::warn!(
                        "Invalid age '{}' for player '{}' on row {}, using 0",
                        row[1].trim(),
                        player_name,
                        row_index + 1
                    );
                    0
                }
            }
        } else {
            0
        };

        // Parse footedness (column C)
        let footedness = if row.len() > 2 {
            match row[2].trim().parse::<Footedness>() {
                Ok(footedness) => footedness,
                Err(_) => {
                    log::warn!(
                        "Invalid footedness '{}' for player '{}' on row {}, using Right",
                        row[2].trim(),
                        player_name,
                        row_index + 1
                    );
                    Footedness::Right
                }
            }
        } else {
            Footedness::Right
        };

        // Parse abilities (columns D-AX, indices 3-49)
        let mut abilities = Vec::new();
        for i in 0..ABILITIES.len() {
            let col_index = i + data_layout::ABILITIES_START_COL; // Abilities start at column D
            let value = if col_index < row.len() {
                row[col_index].trim().parse::<f32>().ok()
            } else {
                None
            };
            abilities.push(value);
        }

        // Parse DNA score (column AY, index 50)
        let dna_score = if row.len() > 50 {
            row[50].trim().parse::<f32>().ok()
        } else {
            None
        };

        // Parse role ratings (columns AZ-EQ, indices 51+)
        let mut role_ratings = Vec::new();
        for i in 0..RoleId::VALID_ROLES.len() {
            let col_index = i + data_layout::ROLE_RATINGS_START_COL; // Role ratings start at column AZ
            let value = if col_index < row.len() {
                row[col_index].trim().parse::<f32>().ok()
            } else {
                None
            };
            role_ratings.push(value);
        }

        match Player::new(
            player_name.clone(),
            age,
            footedness,
            abilities,
            dna_score,
            role_ratings,
        ) {
            Ok(player) => players.push(player),
            Err(e) => {
                return Err(validation_error(
                    "player",
                    &player_name,
                    &format!("creation failed on row {}: {}", row_index + 1, e),
                ));
            }
        }
    }

    Ok(players)
}

/// Calculate the assignment score for a player-role pair
pub fn calculate_assignment_score(player: &Player, role: &Role) -> f32 {
    player.get_role_rating(role)
}

/// Pre-computed eligibility matrix for efficient player-role matching
/// Caches eligibility results to reduce algorithm complexity from O(n×m×f) to O(n×m)
pub struct EligibilityMatrix {
    /// Map from (player_name, role_name) to eligibility boolean
    matrix: HashMap<(PlayerId, RoleId), bool>,
}

impl EligibilityMatrix {
    /// Create a new eligibility matrix by pre-computing all player-role combinations
    pub fn new(players: &[Player], roles: &[Role], filters: &[PlayerFilter]) -> Self {
        let mut matrix = HashMap::new();

        for player in players {
            for role in roles {
                let eligible = Self::compute_eligibility(&player.name, role, filters);
                matrix.insert((player.name.clone(), role.name.clone()), eligible);
            }
        }

        Self { matrix }
    }

    /// Check if a player is eligible for a role using the pre-computed matrix
    pub fn is_eligible(&self, player_name: &PlayerId, role_name: &RoleId) -> bool {
        self.matrix
            .get(&(player_name.clone(), role_name.clone()))
            .copied()
            .unwrap_or(true) // Default to eligible if not found
    }

    /// Compute eligibility for a specific player-role combination
    /// This logic is extracted from the original is_player_eligible_for_role function
    fn compute_eligibility(player_name: &PlayerId, role: &Role, filters: &[PlayerFilter]) -> bool {
        // Find filter for this player
        if let Some(filter) = filters.iter().find(|f| f.player_name == *player_name) {
            // Player has a filter - check if role belongs to any allowed category
            filter
                .allowed_categories
                .iter()
                .any(|category| role_belongs_to_category(role.name.as_str(), category))
        } else {
            // No filter for this player - eligible for all roles
            true
        }
    }
}

/// Check if a player is eligible for a role based on player filters
/// Returns true if:
/// - The player has no filter (no restrictions)
/// - The player has a filter and the role belongs to one of their allowed categories
///
/// Note: This is the original implementation kept for backward compatibility.
/// For performance-critical operations, use EligibilityMatrix instead.
pub fn is_player_eligible_for_role(
    player_name: &str,
    role: &Role,
    filters: &[PlayerFilter],
) -> bool {
    // Find filter for this player
    if let Some(filter) = filters
        .iter()
        .find(|f| f.player_name.as_str() == player_name)
    {
        // Player has a filter - check if role belongs to any allowed category
        filter
            .allowed_categories
            .iter()
            .any(|category| role_belongs_to_category(role.name.as_str(), category))
    } else {
        // No filter for this player - eligible for all roles
        true
    }
}

/// Find optimal player-to-role assignments using a greedy algorithm with pre-computed eligibility caching
/// This optimized version uses EligibilityMatrix to reduce complexity from O(n×m×f) to O(n×m)
pub fn find_optimal_assignments_with_filters(
    players: Vec<Player>,
    roles: Vec<Role>,
    filters: &[PlayerFilter],
) -> Result<Team> {
    use crate::constants::team::REQUIRED_ROLE_COUNT;

    if roles.len() != REQUIRED_ROLE_COUNT {
        return Err(FMDataError::selection(format!(
            "Must have exactly {} roles for team selection, got {}",
            REQUIRED_ROLE_COUNT,
            roles.len()
        )));
    }

    if players.len() < REQUIRED_ROLE_COUNT {
        return Err(FMDataError::selection(format!(
            "Need at least {} players for team selection, got {}",
            REQUIRED_ROLE_COUNT,
            players.len()
        )));
    }

    // Pre-compute eligibility matrix for all player-role combinations
    // This reduces algorithm complexity from O(n×m×f) to O(n×m)
    let eligibility_matrix = EligibilityMatrix::new(&players, &roles, filters);

    let mut assignments = Vec::new();
    let mut available_players = players;

    // For each role, find the best available eligible player
    for role in roles {
        let mut best_player_index = None;
        let mut best_score = f32::NEG_INFINITY;

        // Find the eligible player with the highest rating for this role
        // Using pre-computed eligibility matrix for O(1) lookup instead of O(f) filtering
        for (i, player) in available_players.iter().enumerate() {
            if eligibility_matrix.is_eligible(&player.name, &role.name) {
                let score = calculate_assignment_score(player, &role);
                if score > best_score {
                    best_score = score;
                    best_player_index = Some(i);
                }
            }
        }

        // Check if we found an eligible player
        if let Some(index) = best_player_index {
            // Remove the selected player from available players and create assignment
            let selected_player = available_players.remove(index);
            let assignment = Assignment::new(selected_player, role);
            assignments.push(assignment);
        } else {
            // No eligible player found for this role - log warning and continue
            log::warn!("No eligible players found for role '{}'", role.name);
        }
    }

    Team::new_unchecked(assignments)
}

/// Find optimal player-to-role assignments using a greedy algorithm (backward compatibility)
pub fn find_optimal_assignments(players: Vec<Player>, roles: Vec<Role>) -> Result<Team> {
    find_optimal_assignments_with_filters(players, roles, &[])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{PlayerId, RoleId};
    use crate::selection::types::{Footedness, Player, Role};
    use crate::selection::types::{PlayerCategory, PlayerFilter};

    fn create_test_player(name: &str) -> Player {
        Player::new(
            name.to_string(),
            25,
            Footedness::Right,
            vec![Some(10.0); 47], // All abilities at 10.0
            Some(50.0),
            vec![Some(15.0); 94], // All role ratings at 15.0
        )
        .unwrap()
    }

    fn create_test_role(name: &str) -> Role {
        Role::new(name).unwrap()
    }

    #[test]
    fn test_eligibility_matrix_creation() {
        let players = vec![create_test_player("Test Player")];
        let roles = vec![create_test_role("GK"), create_test_role("CD(d)")];
        let filters = vec![];

        let matrix = EligibilityMatrix::new(&players, &roles, &filters);

        // Without filters, all players should be eligible for all roles
        assert!(matrix.is_eligible(
            &PlayerId::new("Test Player").unwrap(),
            &RoleId::new("GK").unwrap()
        ));
        assert!(matrix.is_eligible(
            &PlayerId::new("Test Player").unwrap(),
            &RoleId::new("CD(d)").unwrap()
        ));
    }

    #[test]
    fn test_eligibility_matrix_with_filters() {
        let players = vec![create_test_player("Goalkeeper Only")];
        let roles = vec![create_test_role("GK"), create_test_role("CD(d)")];
        let filters = vec![PlayerFilter::new(
            PlayerId::new("Goalkeeper Only").unwrap(),
            vec![PlayerCategory::Goal],
        )];

        let matrix = EligibilityMatrix::new(&players, &roles, &filters);

        // Player should be eligible for GK (Goal category) but not CD(d) (CentralDefender category)
        assert!(matrix.is_eligible(
            &PlayerId::new("Goalkeeper Only").unwrap(),
            &RoleId::new("GK").unwrap()
        ));
        assert!(!matrix.is_eligible(
            &PlayerId::new("Goalkeeper Only").unwrap(),
            &RoleId::new("CD(d)").unwrap()
        ));
    }

    #[test]
    fn test_eligibility_matrix_multiple_categories() {
        let players = vec![create_test_player("Versatile Player")];
        let roles = vec![
            create_test_role("GK"),
            create_test_role("CD(d)"),
            create_test_role("CM(s)"),
        ];
        let filters = vec![PlayerFilter::new(
            PlayerId::new("Versatile Player").unwrap(),
            vec![PlayerCategory::Goal, PlayerCategory::CentralDefender],
        )];

        let matrix = EligibilityMatrix::new(&players, &roles, &filters);

        // Player should be eligible for GK and CD(d) but not CM(s)
        assert!(matrix.is_eligible(
            &PlayerId::new("Versatile Player").unwrap(),
            &RoleId::new("GK").unwrap()
        ));
        assert!(matrix.is_eligible(
            &PlayerId::new("Versatile Player").unwrap(),
            &RoleId::new("CD(d)").unwrap()
        ));
        assert!(!matrix.is_eligible(
            &PlayerId::new("Versatile Player").unwrap(),
            &RoleId::new("CM(s)").unwrap()
        ));
    }

    #[test]
    fn test_eligibility_matrix_nonexistent_player_role() {
        let players = vec![create_test_player("Test Player")];
        let roles = vec![create_test_role("GK")];
        let filters = vec![];

        let matrix = EligibilityMatrix::new(&players, &roles, &filters);

        // Non-existent combinations should default to eligible (true)
        assert!(matrix.is_eligible(
            &PlayerId::new("Non Existent Player").unwrap(),
            &RoleId::new("GK").unwrap()
        ));
        assert!(matrix.is_eligible(
            &PlayerId::new("Test Player").unwrap(),
            &RoleId::new("CD(d)").unwrap()
        ));
    }

    #[test]
    fn test_eligibility_matrix_performance_improvement() {
        // This test verifies that the matrix approach is functionally equivalent to the original approach
        let players = vec![
            create_test_player("Player 1"),
            create_test_player("Player 2"),
        ];
        let roles = vec![create_test_role("GK"), create_test_role("CD(d)")];
        let filters = vec![PlayerFilter::new(
            PlayerId::new("Player 1").unwrap(),
            vec![PlayerCategory::Goal],
        )];

        let matrix = EligibilityMatrix::new(&players, &roles, &filters);

        // Test that matrix results match the original function results
        for player in &players {
            for role in &roles {
                let matrix_result = matrix.is_eligible(&player.name, &role.name);
                let original_result =
                    is_player_eligible_for_role(player.name.as_str(), role, &filters);
                assert_eq!(
                    matrix_result, original_result,
                    "Matrix and original function should give same result for player {} and role {}",
                    player.name, role.name
                );
            }
        }
    }
}
