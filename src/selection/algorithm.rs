use super::categories::role_belongs_to_category;
use super::types::{
    Assignment, Footedness, Player, PlayerFilter, Role, Team, ABILITIES, VALID_ROLES,
};
use crate::constants::data_layout;
use crate::error::{FMDataError, Result};
use crate::error_helpers::validation_error;

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
            row[1].trim().parse::<u8>().unwrap_or(0)
        } else {
            0
        };

        // Parse footedness (column C)
        let footedness = if row.len() > 2 {
            row[2]
                .trim()
                .parse::<Footedness>()
                .unwrap_or(Footedness::Right)
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
        for i in 0..VALID_ROLES.len() {
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

/// Check if a player is eligible for a role based on player filters
/// Returns true if:
/// - The player has no filter (no restrictions)
/// - The player has a filter and the role belongs to one of their allowed categories
pub fn is_player_eligible_for_role(
    player_name: &str,
    role: &Role,
    filters: &[PlayerFilter],
) -> bool {
    // Find filter for this player
    if let Some(filter) = filters.iter().find(|f| f.player_name == player_name) {
        // Player has a filter - check if role belongs to any allowed category
        filter
            .allowed_categories
            .iter()
            .any(|category| role_belongs_to_category(&role.name, category))
    } else {
        // No filter for this player - eligible for all roles
        true
    }
}

/// Find optimal player-to-role assignments using a greedy algorithm with optional player filters
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

    let mut assignments = Vec::new();
    let mut available_players = players;

    // For each role, find the best available eligible player
    for role in roles {
        let mut best_player_index = None;
        let mut best_score = f32::NEG_INFINITY;

        // Find the eligible player with the highest rating for this role
        for (i, player) in available_players.iter().enumerate() {
            if is_player_eligible_for_role(&player.name, &role, filters) {
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
