use crate::constants::data_layout;
use crate::error::{FMDataError, Result};
use serde::{Deserialize, Serialize};
use std::fmt;
use tokio::fs;

/// Player position categories for filtering
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PlayerCategory {
    Goal,
    CentralDefender,
    WingBack,
    DefensiveMidfielder,
    CentralMidfielder,
    Winger,
    AttackingMidfielder,
    Playmaker,
    Striker,
}

impl PlayerCategory {
    /// Parse category from short name
    pub fn from_short_name(short: &str) -> Result<Self> {
        match short.trim().to_lowercase().as_str() {
            "goal" => Ok(PlayerCategory::Goal),
            "cd" => Ok(PlayerCategory::CentralDefender),
            "wb" => Ok(PlayerCategory::WingBack),
            "dm" => Ok(PlayerCategory::DefensiveMidfielder),
            "cm" => Ok(PlayerCategory::CentralMidfielder),
            "wing" => Ok(PlayerCategory::Winger),
            "am" => Ok(PlayerCategory::AttackingMidfielder),
            "pm" => Ok(PlayerCategory::Playmaker),
            "str" => Ok(PlayerCategory::Striker),
            _ => Err(FMDataError::selection(format!("Invalid category: {short}"))),
        }
    }

    /// Get short name for category
    pub fn to_short_name(&self) -> &'static str {
        match self {
            PlayerCategory::Goal => "goal",
            PlayerCategory::CentralDefender => "cd",
            PlayerCategory::WingBack => "wb",
            PlayerCategory::DefensiveMidfielder => "dm",
            PlayerCategory::CentralMidfielder => "cm",
            PlayerCategory::Winger => "wing",
            PlayerCategory::AttackingMidfielder => "am",
            PlayerCategory::Playmaker => "pm",
            PlayerCategory::Striker => "str",
        }
    }
}

impl fmt::Display for PlayerCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_short_name())
    }
}

/// Player filter restricting a player to specific categories
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerFilter {
    pub player_name: String,
    pub allowed_categories: Vec<PlayerCategory>,
}

impl PlayerFilter {
    pub fn new(player_name: String, allowed_categories: Vec<PlayerCategory>) -> Self {
        Self {
            player_name,
            allowed_categories,
        }
    }
}

/// Role file content with roles and optional filters
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoleFileContent {
    pub roles: Vec<Role>,
    pub filters: Vec<PlayerFilter>,
}

impl RoleFileContent {
    pub fn new(roles: Vec<Role>, filters: Vec<PlayerFilter>) -> Self {
        Self { roles, filters }
    }
}

/// Valid roles that can be assigned to players
const VALID_ROLES: &[&str] = &[
    "W(s) R", "W(s) L", "W(a) R", "W(a) L", "IF(s)", "IF(a)", "AP(s)", "AP(a)", "WTM(s)", "WTM(a)",
    "TQ(a)", "RD(A)", "IW(s)", "IW(a)", "DW(d)", "DW(s)", "WM(d)", "WM(s)", "WM(a)", "WP(s)",
    "WP(a)", "MEZ(s)", "MEZ(a)", "BWM(d)", "BWM(s)", "BBM", "CAR", "CM(d)", "CM(s)", "CM(a)",
    "DLP(d)", "DLP(s)", "RPM", "HB", "DM(d)", "DM(s)", "A", "SV(s)", "SV(a)", "RGA", "CD(d)",
    "CD(s)", "CD(c)", "NCB(d)", "WCB(d)", "WCB(s)", "WCB(a)", "BPD(d)", "BPD(s)", "BPD(c)", "L(s)",
    "L(a)", "FB(d) R", "FB(s) R", "FB(a) R", "FB(d) L", "FB(s) L", "FB(a) L", "IFB(d) R",
    "IFB(d) L", "WB(d) R", "WB(s) R", "WB(a) R", "WB(d) L", "WB(s) L", "WB(a) L", "IWB(d) R",
    "IWB(s) R", "IWB(a) R", "IWB(d) L", "IWB(s) L", "IWB(a) L", "CWB(s) R", "CWB(a) R", "CWB(s) L",
    "CWB(a) L", "PF(d)", "PF(s)", "PF(a)", "TM(s)", "TM(a)", "AF", "P", "DLF(s)", "DLF(a)",
    "CF(s)", "CF(a)", "F9", "SS", "EG", "AM(s)", "AM(a)", "SK(d)", "SK(s)", "SK(a)", "GK",
];

/// Get all roles that belong to a specific category
pub fn get_roles_for_category(category: &PlayerCategory) -> Vec<&'static str> {
    match category {
        PlayerCategory::Goal => vec!["GK", "SK(d)", "SK(s)", "SK(a)"],
        PlayerCategory::CentralDefender => vec![
            "CD(d)", "CD(s)", "CD(c)", "BPD(d)", "BPD(s)", "BPD(c)", "NCB(d)", "WCB(d)", "WCB(s)",
            "WCB(a)", "L(s)", "L(a)",
        ],
        PlayerCategory::WingBack => vec![
            "FB(d) R", "FB(s) R", "FB(a) R", "FB(d) L", "FB(s) L", "FB(a) L", "WB(d) R", "WB(s) R",
            "WB(a) R", "WB(d) L", "WB(s) L", "WB(a) L", "IFB(d) R", "IFB(d) L", "IWB(d) R",
            "IWB(s) R", "IWB(a) R", "IWB(d) L", "IWB(s) L", "IWB(a) L", "CWB(s) R", "CWB(a) R",
            "CWB(s) L", "CWB(a) L",
        ],
        PlayerCategory::DefensiveMidfielder => vec![
            "DM(d)", "DM(s)", "HB", "BWM(d)", "BWM(s)", "A", "CM(d)", "DLP(d)", "BBM", "SV(s)",
            "SV(a)",
        ],
        PlayerCategory::CentralMidfielder => vec![
            "CM(d)", "CM(s)", "CM(a)", "DLP(d)", "DLP(s)", "RPM", "BBM", "CAR", "MEZ(s)", "MEZ(a)",
        ],
        PlayerCategory::Winger => vec![
            "WM(d)", "WM(s)", "WM(a)", "WP(s)", "WP(a)", "W(s) R", "W(s) L", "W(a) R", "W(a) L",
            "IF(s)", "IF(a)", "IW(s)", "IW(a)", "WTM(s)", "WTM(a)", "TQ(a)", "RD(A)", "DW(d)",
            "DW(s)",
        ],
        PlayerCategory::AttackingMidfielder => vec![
            "SS", "EG", "AM(s)", "AM(a)", "AP(s)", "AP(a)", "CM(a)", "MEZ(a)", "IW(a)", "IW(s)",
        ],
        PlayerCategory::Playmaker => vec![
            "DLP(d)", "DLP(s)", "AP(s)", "AP(a)", "WP(s)", "WP(a)", "RGA", "RPM",
        ],
        PlayerCategory::Striker => vec![
            "AF", "P", "DLF(s)", "DLF(a)", "CF(s)", "CF(a)", "F9", "TM(s)", "TM(a)", "PF(d)",
            "PF(s)", "PF(a)", "IF(a)", "IF(s)",
        ],
    }
}

/// Check if a role belongs to a specific category
pub fn role_belongs_to_category(role_name: &str, category: &PlayerCategory) -> bool {
    get_roles_for_category(category).contains(&role_name)
}

/// Player abilities in the order they appear in the spreadsheet (columns D-AX)
const ABILITIES: &[&str] = &[
    "Cor", "Cro", "Dri", "Fin", "Fir", "Fre", "Hea", "Lon", "L Th", "Mar", "Pas", "Pen", "Tck",
    "Tec", "Agg", "Ant", "Bra", "Cmp", "Cnt", "Dec", "Det", "Fla", "Ldr", "OtB", "Pos", "Tea",
    "Vis", "Wor", "Acc", "Agi", "Bal", "Jum", "Nat", "Pac", "Sta", "Str", "Aer", "Cmd", "Com",
    "Ecc", "Han", "Kic", "1v1", "Pun", "Ref", "Rus", "Thr",
];

/// Footedness options for players
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Footedness {
    Right,
    Left,
    Both,
}

impl fmt::Display for Footedness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Footedness::Right => write!(f, "R"),
            Footedness::Left => write!(f, "L"),
            Footedness::Both => write!(f, "RL"),
        }
    }
}

impl std::str::FromStr for Footedness {
    type Err = FMDataError;

    fn from_str(s: &str) -> Result<Self> {
        match s.trim() {
            "R" => Ok(Footedness::Right),
            "L" => Ok(Footedness::Left),
            "RL" => Ok(Footedness::Both),
            _ => Err(FMDataError::selection(format!("Invalid footedness: {s}"))),
        }
    }
}

/// A role that can be assigned to a player
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Role {
    pub name: String,
}

impl Role {
    /// Create a new role, validating it against the list of valid roles
    pub fn new(name: &str) -> Result<Self> {
        let name = name.trim();
        if Self::is_valid_role(name) {
            Ok(Role {
                name: name.to_string(),
            })
        } else {
            Err(FMDataError::selection(format!("Invalid role: {name}")))
        }
    }

    /// Check if a role name is valid
    pub fn is_valid_role(name: &str) -> bool {
        VALID_ROLES.contains(&name.trim())
    }

    /// Get all valid role names
    pub fn get_valid_roles() -> Vec<&'static str> {
        VALID_ROLES.to_vec()
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// A football player with their attributes and role ratings
#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    pub name: String,
    pub age: u8,
    pub footedness: Footedness,
    pub abilities: Vec<Option<f32>>,
    pub dna_score: Option<f32>,
    pub role_ratings: Vec<Option<f32>>,
}

impl Player {
    /// Create a new player
    pub fn new(
        name: String,
        age: u8,
        footedness: Footedness,
        abilities: Vec<Option<f32>>,
        dna_score: Option<f32>,
        role_ratings: Vec<Option<f32>>,
    ) -> Result<Self> {
        if abilities.len() != ABILITIES.len() {
            return Err(FMDataError::selection(format!(
                "Player {} has {} abilities, expected {}",
                name,
                abilities.len(),
                ABILITIES.len()
            )));
        }

        let expected_roles = VALID_ROLES.len();
        if role_ratings.len() != expected_roles {
            return Err(FMDataError::selection(format!(
                "Player {} has {} role ratings, expected {}",
                name,
                role_ratings.len(),
                expected_roles
            )));
        }

        Ok(Player {
            name,
            age,
            footedness,
            abilities,
            dna_score,
            role_ratings,
        })
    }

    /// Get the player's rating for a specific role
    pub fn get_role_rating(&self, role: &Role) -> f32 {
        let role_index = VALID_ROLES
            .iter()
            .position(|&r| r == role.name)
            .unwrap_or(0);

        self.role_ratings
            .get(role_index)
            .copied()
            .flatten()
            .unwrap_or(0.0)
    }

    /// Get the player's ability score for a specific ability
    pub fn get_ability(&self, ability_name: &str) -> f32 {
        let ability_index = ABILITIES.iter().position(|&a| a == ability_name);

        match ability_index {
            Some(idx) => self.abilities.get(idx).copied().flatten().unwrap_or(0.0),
            None => 0.0,
        }
    }
}

/// An assignment of a player to a role with the calculated score
#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    pub player: Player,
    pub role: Role,
    pub score: f32,
}

impl Assignment {
    /// Create a new assignment and calculate the score
    pub fn new(player: Player, role: Role) -> Self {
        let score = player.get_role_rating(&role);
        Assignment {
            player,
            role,
            score,
        }
    }

    /// Calculate the score for this assignment
    pub fn calculate_score(&self) -> f32 {
        self.player.get_role_rating(&self.role)
    }
}

impl fmt::Display for Assignment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.role, self.player.name)
    }
}

/// A team consisting of 11 player-role assignments
#[derive(Debug, Clone, PartialEq)]
pub struct Team {
    pub assignments: Vec<Assignment>,
}

impl Team {
    /// Create a new team from assignments
    pub fn new(assignments: Vec<Assignment>) -> Result<Self> {
        if assignments.len() != 11 {
            return Err(FMDataError::selection(format!(
                "Team must have exactly 11 assignments, got {}",
                assignments.len()
            )));
        }

        // Validate no duplicate players
        let mut player_names = std::collections::HashSet::new();
        for assignment in &assignments {
            if !player_names.insert(&assignment.player.name) {
                return Err(FMDataError::selection(format!(
                    "Player {} is assigned to multiple roles",
                    assignment.player.name
                )));
            }
        }

        // Note: Duplicate roles are allowed (e.g., multiple goalkeepers)
        // No validation needed for duplicate roles

        Ok(Team { assignments })
    }

    /// Calculate the total score for this team
    pub fn total_score(&self) -> f32 {
        self.assignments.iter().map(|a| a.score).sum()
    }

    /// Get assignments sorted by role name
    pub fn sorted_by_role(&self) -> Vec<&Assignment> {
        let mut assignments: Vec<&Assignment> = self.assignments.iter().collect();
        assignments.sort_by(|a, b| a.role.name.cmp(&b.role.name));
        assignments
    }

    /// Get assignments sorted by score (descending)
    pub fn sorted_by_score(&self) -> Vec<&Assignment> {
        let mut assignments: Vec<&Assignment> = self.assignments.iter().collect();
        assignments.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        assignments
    }
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Team Assignments:")?;
        for assignment in self.sorted_by_role() {
            writeln!(f, "{assignment}")?;
        }
        writeln!(f, "Total Score: {:.1}", self.total_score())?;
        Ok(())
    }
}

/// Parse a role file containing 11 roles (one per line)
pub async fn parse_role_file(file_path: &str) -> Result<Vec<Role>> {
    let content = fs::read_to_string(file_path).await.map_err(|e| {
        FMDataError::selection(format!("Failed to read role file '{file_path}': {e}"))
    })?;

    let lines: Vec<String> = content
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();

    if lines.len() != 11 {
        return Err(FMDataError::selection(format!(
            "Role file must contain exactly 11 roles, found {}",
            lines.len()
        )));
    }

    let mut roles = Vec::new();

    for (line_num, line) in lines.iter().enumerate() {
        let role = Role::new(line).map_err(|e| {
            FMDataError::selection(format!("Invalid role on line {}: {}", line_num + 1, e))
        })?;

        roles.push(role);
    }

    Ok(roles)
}

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
                return Err(FMDataError::selection(format!(
                    "Failed to create player '{}' on row {}: {}",
                    player_name,
                    row_index + 1,
                    e
                )));
            }
        }
    }

    Ok(players)
}

/// Calculate the assignment score for a player-role pair
pub fn calculate_assignment_score(player: &Player, role: &Role) -> f32 {
    player.get_role_rating(role)
}

/// Find optimal player-to-role assignments using a greedy algorithm
pub fn find_optimal_assignments(players: Vec<Player>, roles: Vec<Role>) -> Result<Team> {
    if roles.len() != 11 {
        return Err(FMDataError::selection(format!(
            "Must have exactly 11 roles for team selection, got {}",
            roles.len()
        )));
    }

    if players.len() < 11 {
        return Err(FMDataError::selection(format!(
            "Need at least 11 players for team selection, got {}",
            players.len()
        )));
    }

    let mut assignments = Vec::new();
    let mut available_players = players;

    // For each role, find the best available player
    for role in roles {
        let mut best_player_index = 0;
        let mut best_score = calculate_assignment_score(&available_players[0], &role);

        // Find the player with the highest rating for this role
        for (i, player) in available_players.iter().enumerate() {
            let score = calculate_assignment_score(player, &role);
            if score > best_score {
                best_score = score;
                best_player_index = i;
            }
        }

        // Remove the selected player from available players and create assignment
        let selected_player = available_players.remove(best_player_index);
        let assignment = Assignment::new(selected_player, role);
        assignments.push(assignment);
    }

    Team::new(assignments)
}

/// Format team output for clean stdout display
pub fn format_team_output(team: &Team) -> String {
    let mut output = String::new();

    // Sort assignments by role name for consistent output
    let sorted_assignments = team.sorted_by_role();

    // Format each assignment as "$ROLE -> $PLAYER_NAME"
    for assignment in sorted_assignments {
        output.push_str(&format!(
            "{} -> {}\n",
            assignment.role.name, assignment.player.name
        ));
    }

    // Include total team score
    output.push_str(&format!("Total Score: {:.1}\n", team.total_score()));

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_category_from_short_name() {
        assert_eq!(
            PlayerCategory::from_short_name("goal").unwrap(),
            PlayerCategory::Goal
        );
        assert_eq!(
            PlayerCategory::from_short_name("cd").unwrap(),
            PlayerCategory::CentralDefender
        );
        assert_eq!(
            PlayerCategory::from_short_name("wb").unwrap(),
            PlayerCategory::WingBack
        );
        assert_eq!(
            PlayerCategory::from_short_name("dm").unwrap(),
            PlayerCategory::DefensiveMidfielder
        );
        assert_eq!(
            PlayerCategory::from_short_name("cm").unwrap(),
            PlayerCategory::CentralMidfielder
        );
        assert_eq!(
            PlayerCategory::from_short_name("wing").unwrap(),
            PlayerCategory::Winger
        );
        assert_eq!(
            PlayerCategory::from_short_name("am").unwrap(),
            PlayerCategory::AttackingMidfielder
        );
        assert_eq!(
            PlayerCategory::from_short_name("pm").unwrap(),
            PlayerCategory::Playmaker
        );
        assert_eq!(
            PlayerCategory::from_short_name("str").unwrap(),
            PlayerCategory::Striker
        );

        // Test case insensitivity
        assert_eq!(
            PlayerCategory::from_short_name("GOAL").unwrap(),
            PlayerCategory::Goal
        );
        assert_eq!(
            PlayerCategory::from_short_name("Cd").unwrap(),
            PlayerCategory::CentralDefender
        );
        assert_eq!(
            PlayerCategory::from_short_name(" wb ").unwrap(),
            PlayerCategory::WingBack
        );

        // Test invalid category
        assert!(PlayerCategory::from_short_name("invalid").is_err());
        assert!(PlayerCategory::from_short_name("").is_err());
    }

    #[test]
    fn test_player_category_to_short_name() {
        assert_eq!(PlayerCategory::Goal.to_short_name(), "goal");
        assert_eq!(PlayerCategory::CentralDefender.to_short_name(), "cd");
        assert_eq!(PlayerCategory::WingBack.to_short_name(), "wb");
        assert_eq!(PlayerCategory::DefensiveMidfielder.to_short_name(), "dm");
        assert_eq!(PlayerCategory::CentralMidfielder.to_short_name(), "cm");
        assert_eq!(PlayerCategory::Winger.to_short_name(), "wing");
        assert_eq!(PlayerCategory::AttackingMidfielder.to_short_name(), "am");
        assert_eq!(PlayerCategory::Playmaker.to_short_name(), "pm");
        assert_eq!(PlayerCategory::Striker.to_short_name(), "str");
    }

    #[test]
    fn test_player_category_display() {
        assert_eq!(format!("{}", PlayerCategory::Goal), "goal");
        assert_eq!(format!("{}", PlayerCategory::CentralDefender), "cd");
        assert_eq!(format!("{}", PlayerCategory::Striker), "str");
    }

    #[test]
    fn test_player_filter_creation() {
        let categories = vec![PlayerCategory::Goal, PlayerCategory::CentralDefender];
        let filter = PlayerFilter::new("Test Player".to_string(), categories.clone());

        assert_eq!(filter.player_name, "Test Player");
        assert_eq!(filter.allowed_categories, categories);
    }

    #[test]
    fn test_role_file_content_creation() {
        let roles = vec![Role::new("GK").unwrap(), Role::new("CD(d)").unwrap()];
        let filters = vec![PlayerFilter::new(
            "Player 1".to_string(),
            vec![PlayerCategory::Goal],
        )];
        let content = RoleFileContent::new(roles.clone(), filters.clone());

        assert_eq!(content.roles, roles);
        assert_eq!(content.filters, filters);
    }

    #[test]
    fn test_get_roles_for_category_goal() {
        let roles = get_roles_for_category(&PlayerCategory::Goal);
        assert_eq!(roles.len(), 4);
        assert!(roles.contains(&"GK"));
        assert!(roles.contains(&"SK(d)"));
        assert!(roles.contains(&"SK(s)"));
        assert!(roles.contains(&"SK(a)"));
    }

    #[test]
    fn test_get_roles_for_category_central_defender() {
        let roles = get_roles_for_category(&PlayerCategory::CentralDefender);
        assert_eq!(roles.len(), 12);
        assert!(roles.contains(&"CD(d)"));
        assert!(roles.contains(&"CD(s)"));
        assert!(roles.contains(&"CD(c)"));
        assert!(roles.contains(&"BPD(d)"));
        assert!(roles.contains(&"BPD(s)"));
        assert!(roles.contains(&"BPD(c)"));
        assert!(roles.contains(&"NCB(d)"));
        assert!(roles.contains(&"WCB(d)"));
        assert!(roles.contains(&"WCB(s)"));
        assert!(roles.contains(&"WCB(a)"));
        assert!(roles.contains(&"L(s)"));
        assert!(roles.contains(&"L(a)"));
    }

    #[test]
    fn test_get_roles_for_category_wing_back() {
        let roles = get_roles_for_category(&PlayerCategory::WingBack);
        assert_eq!(roles.len(), 24);
        assert!(roles.contains(&"FB(d) R"));
        assert!(roles.contains(&"FB(s) R"));
        assert!(roles.contains(&"CWB(a) L"));
        assert!(roles.contains(&"IWB(s) R"));
    }

    #[test]
    fn test_get_roles_for_category_defensive_midfielder() {
        let roles = get_roles_for_category(&PlayerCategory::DefensiveMidfielder);
        assert_eq!(roles.len(), 11);
        assert!(roles.contains(&"DM(d)"));
        assert!(roles.contains(&"DM(s)"));
        assert!(roles.contains(&"HB"));
        assert!(roles.contains(&"BWM(d)"));
        assert!(roles.contains(&"BWM(s)"));
        assert!(roles.contains(&"A"));
        assert!(roles.contains(&"CM(d)"));
        assert!(roles.contains(&"DLP(d)"));
        assert!(roles.contains(&"BBM"));
        assert!(roles.contains(&"SV(s)"));
        assert!(roles.contains(&"SV(a)"));
    }

    #[test]
    fn test_get_roles_for_category_central_midfielder() {
        let roles = get_roles_for_category(&PlayerCategory::CentralMidfielder);
        assert_eq!(roles.len(), 10);
        assert!(roles.contains(&"CM(d)"));
        assert!(roles.contains(&"CM(s)"));
        assert!(roles.contains(&"CM(a)"));
        assert!(roles.contains(&"DLP(d)"));
        assert!(roles.contains(&"DLP(s)"));
        assert!(roles.contains(&"RPM"));
        assert!(roles.contains(&"BBM"));
        assert!(roles.contains(&"CAR"));
        assert!(roles.contains(&"MEZ(s)"));
        assert!(roles.contains(&"MEZ(a)"));
    }

    #[test]
    fn test_get_roles_for_category_winger() {
        let roles = get_roles_for_category(&PlayerCategory::Winger);
        assert_eq!(roles.len(), 19);
        assert!(roles.contains(&"WM(d)"));
        assert!(roles.contains(&"W(s) R"));
        assert!(roles.contains(&"W(a) L"));
        assert!(roles.contains(&"IF(s)"));
        assert!(roles.contains(&"IW(a)"));
        assert!(roles.contains(&"TQ(a)"));
        assert!(roles.contains(&"DW(s)"));
    }

    #[test]
    fn test_get_roles_for_category_attacking_midfielder() {
        let roles = get_roles_for_category(&PlayerCategory::AttackingMidfielder);
        assert_eq!(roles.len(), 10);
        assert!(roles.contains(&"SS"));
        assert!(roles.contains(&"EG"));
        assert!(roles.contains(&"AM(s)"));
        assert!(roles.contains(&"AM(a)"));
        assert!(roles.contains(&"AP(s)"));
        assert!(roles.contains(&"AP(a)"));
        assert!(roles.contains(&"CM(a)"));
        assert!(roles.contains(&"MEZ(a)"));
        assert!(roles.contains(&"IW(a)"));
        assert!(roles.contains(&"IW(s)"));
    }

    #[test]
    fn test_get_roles_for_category_playmaker() {
        let roles = get_roles_for_category(&PlayerCategory::Playmaker);
        assert_eq!(roles.len(), 8);
        assert!(roles.contains(&"DLP(d)"));
        assert!(roles.contains(&"DLP(s)"));
        assert!(roles.contains(&"AP(s)"));
        assert!(roles.contains(&"AP(a)"));
        assert!(roles.contains(&"WP(s)"));
        assert!(roles.contains(&"WP(a)"));
        assert!(roles.contains(&"RGA"));
        assert!(roles.contains(&"RPM"));
    }

    #[test]
    fn test_get_roles_for_category_striker() {
        let roles = get_roles_for_category(&PlayerCategory::Striker);
        assert_eq!(roles.len(), 14);
        assert!(roles.contains(&"AF"));
        assert!(roles.contains(&"P"));
        assert!(roles.contains(&"DLF(s)"));
        assert!(roles.contains(&"DLF(a)"));
        assert!(roles.contains(&"CF(s)"));
        assert!(roles.contains(&"CF(a)"));
        assert!(roles.contains(&"F9"));
        assert!(roles.contains(&"TM(s)"));
        assert!(roles.contains(&"TM(a)"));
        assert!(roles.contains(&"PF(d)"));
        assert!(roles.contains(&"PF(s)"));
        assert!(roles.contains(&"PF(a)"));
        assert!(roles.contains(&"IF(a)"));
        assert!(roles.contains(&"IF(s)"));
    }

    #[test]
    fn test_role_belongs_to_category() {
        // Test goal category
        assert!(role_belongs_to_category("GK", &PlayerCategory::Goal));
        assert!(role_belongs_to_category("SK(d)", &PlayerCategory::Goal));
        assert!(!role_belongs_to_category("CD(d)", &PlayerCategory::Goal));

        // Test central defender category
        assert!(role_belongs_to_category(
            "CD(d)",
            &PlayerCategory::CentralDefender
        ));
        assert!(role_belongs_to_category(
            "BPD(s)",
            &PlayerCategory::CentralDefender
        ));
        assert!(!role_belongs_to_category(
            "GK",
            &PlayerCategory::CentralDefender
        ));

        // Test overlapping roles
        assert!(role_belongs_to_category(
            "CM(d)",
            &PlayerCategory::DefensiveMidfielder
        ));
        assert!(role_belongs_to_category(
            "CM(d)",
            &PlayerCategory::CentralMidfielder
        ));
        assert!(role_belongs_to_category(
            "DLP(d)",
            &PlayerCategory::DefensiveMidfielder
        ));
        assert!(role_belongs_to_category(
            "DLP(d)",
            &PlayerCategory::CentralMidfielder
        ));
        assert!(role_belongs_to_category(
            "DLP(d)",
            &PlayerCategory::Playmaker
        ));

        // Test winger overlaps
        assert!(role_belongs_to_category("IF(s)", &PlayerCategory::Winger));
        assert!(role_belongs_to_category("IF(s)", &PlayerCategory::Striker));
        assert!(role_belongs_to_category("IW(a)", &PlayerCategory::Winger));
        assert!(role_belongs_to_category(
            "IW(a)",
            &PlayerCategory::AttackingMidfielder
        ));
    }

    #[test]
    fn test_all_valid_roles_covered_by_categories() {
        // Check that every valid role belongs to at least one category
        let all_categories = vec![
            PlayerCategory::Goal,
            PlayerCategory::CentralDefender,
            PlayerCategory::WingBack,
            PlayerCategory::DefensiveMidfielder,
            PlayerCategory::CentralMidfielder,
            PlayerCategory::Winger,
            PlayerCategory::AttackingMidfielder,
            PlayerCategory::Playmaker,
            PlayerCategory::Striker,
        ];

        for &role in VALID_ROLES {
            let found_in_category = all_categories
                .iter()
                .any(|cat| role_belongs_to_category(role, cat));
            assert!(
                found_in_category,
                "Role '{}' is not covered by any category",
                role
            );
        }
    }

    #[test]
    fn test_category_roles_are_valid() {
        // Check that all roles in categories are actually valid roles
        let all_categories = vec![
            PlayerCategory::Goal,
            PlayerCategory::CentralDefender,
            PlayerCategory::WingBack,
            PlayerCategory::DefensiveMidfielder,
            PlayerCategory::CentralMidfielder,
            PlayerCategory::Winger,
            PlayerCategory::AttackingMidfielder,
            PlayerCategory::Playmaker,
            PlayerCategory::Striker,
        ];

        for category in all_categories {
            let category_roles = get_roles_for_category(&category);
            for &role in &category_roles {
                assert!(
                    VALID_ROLES.contains(&role),
                    "Role '{}' in category '{}' is not a valid role",
                    role,
                    category
                );
            }
        }
    }

    #[test]
    fn test_category_role_counts() {
        // Verify expected role counts for each category
        assert_eq!(get_roles_for_category(&PlayerCategory::Goal).len(), 4);
        assert_eq!(
            get_roles_for_category(&PlayerCategory::CentralDefender).len(),
            12
        );
        assert_eq!(get_roles_for_category(&PlayerCategory::WingBack).len(), 24);
        assert_eq!(
            get_roles_for_category(&PlayerCategory::DefensiveMidfielder).len(),
            11
        );
        assert_eq!(
            get_roles_for_category(&PlayerCategory::CentralMidfielder).len(),
            10
        );
        assert_eq!(get_roles_for_category(&PlayerCategory::Winger).len(), 19);
        assert_eq!(
            get_roles_for_category(&PlayerCategory::AttackingMidfielder).len(),
            10
        );
        assert_eq!(get_roles_for_category(&PlayerCategory::Playmaker).len(), 8);
        assert_eq!(get_roles_for_category(&PlayerCategory::Striker).len(), 14);

        // Total should be more than 96 due to overlaps
        let total_category_roles: usize = vec![
            PlayerCategory::Goal,
            PlayerCategory::CentralDefender,
            PlayerCategory::WingBack,
            PlayerCategory::DefensiveMidfielder,
            PlayerCategory::CentralMidfielder,
            PlayerCategory::Winger,
            PlayerCategory::AttackingMidfielder,
            PlayerCategory::Playmaker,
            PlayerCategory::Striker,
        ]
        .iter()
        .map(|cat| get_roles_for_category(cat).len())
        .sum();

        assert!(
            total_category_roles > VALID_ROLES.len(),
            "Total category roles ({}) should be greater than unique roles ({}) due to overlaps",
            total_category_roles,
            VALID_ROLES.len()
        );
    }

    #[test]
    fn test_footedness_from_str() {
        assert_eq!("R".parse::<Footedness>().unwrap(), Footedness::Right);
        assert_eq!("L".parse::<Footedness>().unwrap(), Footedness::Left);
        assert_eq!("RL".parse::<Footedness>().unwrap(), Footedness::Both);
        assert!("X".parse::<Footedness>().is_err());
    }

    #[test]
    fn test_footedness_display() {
        assert_eq!(format!("{}", Footedness::Right), "R");
        assert_eq!(format!("{}", Footedness::Left), "L");
        assert_eq!(format!("{}", Footedness::Both), "RL");
    }

    #[test]
    fn test_role_validation() {
        assert!(Role::new("GK").is_ok());
        assert!(Role::new("W(s) R").is_ok());
        assert!(Role::new("InvalidRole").is_err());
        assert!(Role::new("").is_err());
    }

    #[test]
    fn test_role_valid_roles() {
        let roles = Role::get_valid_roles();
        assert_eq!(roles.len(), 96); // Should have 96 roles
        assert!(roles.contains(&"GK"));
        assert!(roles.contains(&"W(s) R"));
    }

    #[test]
    fn test_player_creation() {
        let abilities = vec![Some(10.0); ABILITIES.len()];
        let role_ratings = vec![Some(8.0); VALID_ROLES.len()];

        let player = Player::new(
            "Test Player".to_string(),
            25,
            Footedness::Right,
            abilities,
            Some(85.0),
            role_ratings,
        );

        assert!(player.is_ok());
        let player = player.unwrap();
        assert_eq!(player.name, "Test Player");
        assert_eq!(player.age, 25);
        assert_eq!(player.footedness, Footedness::Right);
    }

    #[test]
    fn test_player_wrong_abilities_length() {
        let abilities = vec![Some(10.0); 5]; // Wrong length
        let role_ratings = vec![Some(8.0); VALID_ROLES.len()];

        let player = Player::new(
            "Test Player".to_string(),
            25,
            Footedness::Right,
            abilities,
            Some(85.0),
            role_ratings,
        );

        assert!(player.is_err());
    }

    #[test]
    fn test_player_role_rating() {
        let abilities = vec![Some(10.0); ABILITIES.len()];
        let mut role_ratings = vec![Some(0.0); VALID_ROLES.len()];
        role_ratings[0] = Some(15.0); // Set first role rating

        let player = Player::new(
            "Test Player".to_string(),
            25,
            Footedness::Right,
            abilities,
            Some(85.0),
            role_ratings,
        )
        .unwrap();

        let role = Role::new(VALID_ROLES[0]).unwrap();
        assert_eq!(player.get_role_rating(&role), 15.0);

        let other_role = Role::new(VALID_ROLES[1]).unwrap();
        assert_eq!(player.get_role_rating(&other_role), 0.0);
    }

    #[test]
    fn test_player_ability() {
        let mut abilities = vec![Some(0.0); ABILITIES.len()];
        abilities[0] = Some(12.0); // Set first ability
        let role_ratings = vec![Some(8.0); VALID_ROLES.len()];

        let player = Player::new(
            "Test Player".to_string(),
            25,
            Footedness::Right,
            abilities,
            Some(85.0),
            role_ratings,
        )
        .unwrap();

        assert_eq!(player.get_ability(ABILITIES[0]), 12.0);
        assert_eq!(player.get_ability(ABILITIES[1]), 0.0);
        assert_eq!(player.get_ability("NonExistent"), 0.0);
    }

    #[test]
    fn test_assignment_creation() {
        let abilities = vec![Some(10.0); ABILITIES.len()];
        let mut role_ratings = vec![Some(0.0); VALID_ROLES.len()];
        role_ratings[0] = Some(15.0);

        let player = Player::new(
            "Test Player".to_string(),
            25,
            Footedness::Right,
            abilities,
            Some(85.0),
            role_ratings,
        )
        .unwrap();

        let role = Role::new(VALID_ROLES[0]).unwrap();
        let assignment = Assignment::new(player, role);

        assert_eq!(assignment.score, 15.0);
        assert_eq!(assignment.calculate_score(), 15.0);
    }

    #[test]
    fn test_team_creation() {
        let mut assignments = Vec::new();

        for i in 0..11 {
            let abilities = vec![Some(10.0); ABILITIES.len()];
            let role_ratings = vec![Some(8.0); VALID_ROLES.len()];

            let player = Player::new(
                format!("Player {i}"),
                25,
                Footedness::Right,
                abilities,
                Some(85.0),
                role_ratings,
            )
            .unwrap();

            let role = Role::new(VALID_ROLES[i]).unwrap();
            assignments.push(Assignment::new(player, role));
        }

        let team = Team::new(assignments);
        assert!(team.is_ok());
        let team = team.unwrap();
        assert_eq!(team.total_score(), 88.0); // 11 players * 8.0 each
    }

    #[test]
    fn test_team_wrong_size() {
        let team = Team::new(vec![]);
        assert!(team.is_err());

        // Test with 10 assignments (should fail)
        let mut assignments = Vec::new();

        for i in 0..10 {
            let abilities = vec![Some(10.0); ABILITIES.len()];
            let role_ratings = vec![Some(8.0); VALID_ROLES.len()];

            let player = Player::new(
                format!("Player {i}"),
                25,
                Footedness::Right,
                abilities,
                Some(85.0),
                role_ratings,
            )
            .unwrap();

            let role = Role::new(VALID_ROLES[i]).unwrap();
            assignments.push(Assignment::new(player, role));
        }

        let team = Team::new(assignments);
        assert!(team.is_err());
    }

    #[test]
    fn test_team_duplicate_players() {
        let mut assignments = Vec::new();

        let abilities = vec![Some(10.0); ABILITIES.len()];
        let role_ratings = vec![Some(8.0); VALID_ROLES.len()];

        let player = Player::new(
            "Duplicate Player".to_string(),
            25,
            Footedness::Right,
            abilities.clone(),
            Some(85.0),
            role_ratings.clone(),
        )
        .unwrap();

        // Add same player to two different roles
        assignments.push(Assignment::new(
            player.clone(),
            Role::new(VALID_ROLES[0]).unwrap(),
        ));
        assignments.push(Assignment::new(player, Role::new(VALID_ROLES[1]).unwrap()));

        // Fill rest with unique players
        for i in 2..11 {
            let unique_player = Player::new(
                format!("Player {i}"),
                25,
                Footedness::Right,
                abilities.clone(),
                Some(85.0),
                role_ratings.clone(),
            )
            .unwrap();

            assignments.push(Assignment::new(
                unique_player,
                Role::new(VALID_ROLES[i]).unwrap(),
            ));
        }

        let team = Team::new(assignments);
        assert!(team.is_err());
    }

    #[test]
    fn test_team_duplicate_roles() {
        let mut assignments = Vec::new();

        let abilities = vec![Some(10.0); ABILITIES.len()];
        let role_ratings = vec![Some(8.0); VALID_ROLES.len()];

        // Add two different players to same role
        for i in 0..2 {
            let player = Player::new(
                format!("Player {i}"),
                25,
                Footedness::Right,
                abilities.clone(),
                Some(85.0),
                role_ratings.clone(),
            )
            .unwrap();

            assignments.push(Assignment::new(player, Role::new(VALID_ROLES[0]).unwrap()));
        }

        // Fill rest with unique players and roles
        for i in 2..11 {
            let player = Player::new(
                format!("Player {i}"),
                25,
                Footedness::Right,
                abilities.clone(),
                Some(85.0),
                role_ratings.clone(),
            )
            .unwrap();

            assignments.push(Assignment::new(
                player,
                Role::new(VALID_ROLES[i - 1]).unwrap(),
            ));
        }

        let team = Team::new(assignments);
        assert!(team.is_ok()); // Duplicate roles are now allowed
        let team = team.unwrap();

        // Should have 11 assignments with 2 players in the same role
        assert_eq!(team.assignments.len(), 11);

        // First two assignments should have the same role
        assert_eq!(team.assignments[0].role.name, VALID_ROLES[0]);
        assert_eq!(team.assignments[1].role.name, VALID_ROLES[0]);
    }

    #[test]
    fn test_team_sorting() {
        let mut assignments = Vec::new();

        for i in 0..11 {
            let abilities = vec![Some(10.0); ABILITIES.len()];
            let role_ratings = vec![Some((i as f32) * 2.0); VALID_ROLES.len()];

            let player = Player::new(
                format!("Player {i}"),
                25,
                Footedness::Right,
                abilities,
                Some(85.0),
                role_ratings,
            )
            .unwrap();

            let role = Role::new(VALID_ROLES[i]).unwrap();
            assignments.push(Assignment::new(player, role));
        }

        let team = Team::new(assignments).unwrap();

        // Test sorting by role
        let sorted_by_role = team.sorted_by_role();
        assert_eq!(sorted_by_role.len(), 11);

        // Test sorting by score
        let sorted_by_score = team.sorted_by_score();
        assert_eq!(sorted_by_score.len(), 11);
        assert!(sorted_by_score[0].score >= sorted_by_score[1].score);
    }

    #[tokio::test]
    async fn test_parse_role_file_valid() {
        use tempfile::NamedTempFile;
        use tokio::io::AsyncWriteExt;

        let valid_roles =
            "GK\nW(s) R\nW(s) L\nIF(s)\nCM(d)\nCM(s)\nCM(a)\nCD(d)\nCD(s)\nFB(d) R\nFB(d) L";

        let temp_file = NamedTempFile::new().unwrap();
        let mut async_file = tokio::fs::File::create(temp_file.path()).await.unwrap();
        async_file.write_all(valid_roles.as_bytes()).await.unwrap();
        async_file.flush().await.unwrap();

        let result = parse_role_file(temp_file.path().to_str().unwrap()).await;
        assert!(result.is_ok());
        let roles = result.unwrap();
        assert_eq!(roles.len(), 11);
        assert_eq!(roles[0].name, "GK");
        assert_eq!(roles[1].name, "W(s) R");
    }

    #[tokio::test]
    async fn test_parse_role_file_nonexistent() {
        let result = parse_role_file("/nonexistent/file.txt").await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Failed to read role file"));
    }

    #[tokio::test]
    async fn test_parse_role_file_wrong_number_roles() {
        use tempfile::NamedTempFile;
        use tokio::io::AsyncWriteExt;

        // Test with too few roles (only 5)
        let few_roles = "GK\nW(s) R\nIF(s)\nCM(d)\nCD(d)";

        let temp_file = NamedTempFile::new().unwrap();
        let mut async_file = tokio::fs::File::create(temp_file.path()).await.unwrap();
        async_file.write_all(few_roles.as_bytes()).await.unwrap();
        async_file.flush().await.unwrap();

        let result = parse_role_file(temp_file.path().to_str().unwrap()).await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Role file must contain exactly 11 roles, found 5"));

        // Test with too many roles (15)
        let many_roles = "GK\nW(s) R\nW(s) L\nIF(s)\nCM(d)\nCM(s)\nCM(a)\nCD(d)\nCD(s)\nFB(d) R\nFB(d) L\nExtra1\nExtra2\nExtra3\nExtra4";

        let temp_file2 = NamedTempFile::new().unwrap();
        let mut async_file2 = tokio::fs::File::create(temp_file2.path()).await.unwrap();
        async_file2.write_all(many_roles.as_bytes()).await.unwrap();
        async_file2.flush().await.unwrap();

        let result2 = parse_role_file(temp_file2.path().to_str().unwrap()).await;
        assert!(result2.is_err());
        assert!(result2
            .unwrap_err()
            .to_string()
            .contains("Role file must contain exactly 11 roles, found 15"));
    }

    #[tokio::test]
    async fn test_parse_role_file_invalid_role() {
        use tempfile::NamedTempFile;
        use tokio::io::AsyncWriteExt;

        let invalid_roles =
            "GK\nW(s) R\nW(s) L\nIF(s)\nCM(d)\nCM(s)\nCM(a)\nCD(d)\nCD(s)\nFB(d) R\nInvalidRole";

        let temp_file = NamedTempFile::new().unwrap();
        let mut async_file = tokio::fs::File::create(temp_file.path()).await.unwrap();
        async_file
            .write_all(invalid_roles.as_bytes())
            .await
            .unwrap();
        async_file.flush().await.unwrap();

        let result = parse_role_file(temp_file.path().to_str().unwrap()).await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Invalid role on line 11"));
    }

    #[tokio::test]
    async fn test_parse_role_file_duplicate_roles_allowed() {
        use tempfile::NamedTempFile;
        use tokio::io::AsyncWriteExt;

        let duplicate_roles =
            "GK\nW(s) R\nW(s) L\nIF(s)\nCM(d)\nCM(s)\nCM(a)\nCD(d)\nCD(s)\nFB(d) R\nGK";

        let temp_file = NamedTempFile::new().unwrap();
        let mut async_file = tokio::fs::File::create(temp_file.path()).await.unwrap();
        async_file
            .write_all(duplicate_roles.as_bytes())
            .await
            .unwrap();
        async_file.flush().await.unwrap();

        let result = parse_role_file(temp_file.path().to_str().unwrap()).await;
        assert!(result.is_ok());
        let roles = result.unwrap();
        assert_eq!(roles.len(), 11);
        assert_eq!(roles[0].name, "GK");
        assert_eq!(roles[10].name, "GK"); // Last role is also GK
    }

    #[tokio::test]
    async fn test_parse_role_file_whitespace_handling() {
        use tempfile::NamedTempFile;
        use tokio::io::AsyncWriteExt;

        let roles_with_whitespace = "  GK  \n\n  W(s) R\t\nW(s) L\n   IF(s)   \nCM(d)\nCM(s)\nCM(a)\nCD(d)\nCD(s)\nFB(d) R\nFB(d) L\n\n";

        let temp_file = NamedTempFile::new().unwrap();
        let mut async_file = tokio::fs::File::create(temp_file.path()).await.unwrap();
        async_file
            .write_all(roles_with_whitespace.as_bytes())
            .await
            .unwrap();
        async_file.flush().await.unwrap();

        let result = parse_role_file(temp_file.path().to_str().unwrap()).await;
        assert!(result.is_ok());
        let roles = result.unwrap();
        assert_eq!(roles.len(), 11);
        assert_eq!(roles[0].name, "GK");
        assert_eq!(roles[1].name, "W(s) R");
        assert_eq!(roles[3].name, "IF(s)");
    }

    #[tokio::test]
    async fn test_parse_role_file_empty_file() {
        use tempfile::NamedTempFile;
        use tokio::io::AsyncWriteExt;

        let empty_content = "";

        let temp_file = NamedTempFile::new().unwrap();
        let mut async_file = tokio::fs::File::create(temp_file.path()).await.unwrap();
        async_file
            .write_all(empty_content.as_bytes())
            .await
            .unwrap();
        async_file.flush().await.unwrap();

        let result = parse_role_file(temp_file.path().to_str().unwrap()).await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Role file must contain exactly 11 roles, found 0"));
    }

    #[test]
    fn test_parse_player_data_complete() {
        // Create test data with all fields populated
        let mut row = vec!["Test Player".to_string(), "25".to_string(), "R".to_string()];

        // Add abilities (47 values)
        for i in 0..ABILITIES.len() {
            row.push((i as f32 + 10.0).to_string());
        }

        // Add DNA score
        row.push("85.5".to_string());

        // Add role ratings (96 values)
        for i in 0..VALID_ROLES.len() {
            row.push((i as f32 + 5.0).to_string());
        }

        let sheet_data = vec![row];
        let result = parse_player_data(sheet_data);

        assert!(result.is_ok());
        let players = result.unwrap();
        assert_eq!(players.len(), 1);

        let player = &players[0];
        assert_eq!(player.name, "Test Player");
        assert_eq!(player.age, 25);
        assert_eq!(player.footedness, Footedness::Right);
        assert_eq!(player.dna_score, Some(85.5));
        assert_eq!(player.abilities.len(), ABILITIES.len());
        assert_eq!(player.role_ratings.len(), VALID_ROLES.len());

        // Check first ability and role rating
        assert_eq!(player.abilities[0], Some(10.0));
        assert_eq!(player.role_ratings[0], Some(5.0));
    }

    #[test]
    fn test_parse_player_data_missing_abilities() {
        // Create minimal data - only name, age, footedness
        let row = vec![
            "Minimal Player".to_string(),
            "30".to_string(),
            "L".to_string(),
        ];
        let sheet_data = vec![row];

        let result = parse_player_data(sheet_data);
        assert!(result.is_ok());

        let players = result.unwrap();
        assert_eq!(players.len(), 1);

        let player = &players[0];
        assert_eq!(player.name, "Minimal Player");
        assert_eq!(player.age, 30);
        assert_eq!(player.footedness, Footedness::Left);
        assert_eq!(player.dna_score, None);

        // All abilities should be None due to missing data
        for ability in &player.abilities {
            assert_eq!(*ability, None);
        }

        // All role ratings should be None due to missing data
        for rating in &player.role_ratings {
            assert_eq!(*rating, None);
        }
    }

    #[test]
    fn test_parse_player_data_empty_cells() {
        // Create data with empty cells that should become 0.0
        let mut row = vec![
            "Empty Cells Player".to_string(),
            "22".to_string(),
            "RL".to_string(),
        ];

        // Add some abilities with empty values mixed in
        for i in 0..ABILITIES.len() {
            if i % 3 == 0 {
                row.push("".to_string()); // Empty cell
            } else {
                row.push((i as f32 + 1.0).to_string());
            }
        }

        row.push("".to_string()); // Empty DNA score

        // Add role ratings with some empty
        for i in 0..VALID_ROLES.len() {
            if i % 4 == 0 {
                row.push("".to_string()); // Empty cell
            } else {
                row.push((i as f32 + 2.0).to_string());
            }
        }

        let sheet_data = vec![row];
        let result = parse_player_data(sheet_data);

        assert!(result.is_ok());
        let players = result.unwrap();
        let player = &players[0];

        assert_eq!(player.name, "Empty Cells Player");
        assert_eq!(player.footedness, Footedness::Both);
        assert_eq!(player.dna_score, None);

        // Check that empty cells become None
        for (i, ability) in player.abilities.iter().enumerate() {
            if i % 3 == 0 {
                assert_eq!(*ability, None);
            } else {
                assert_eq!(*ability, Some(i as f32 + 1.0));
            }
        }
    }

    #[test]
    fn test_parse_player_data_skip_empty_names() {
        let sheet_data = vec![
            vec!["".to_string(), "25".to_string(), "R".to_string()], // Empty name
            vec!["  ".to_string(), "30".to_string(), "L".to_string()], // Whitespace only
            vec![
                "Valid Player".to_string(),
                "28".to_string(),
                "RL".to_string(),
            ],
            vec![], // Empty row
        ];

        let result = parse_player_data(sheet_data);
        assert!(result.is_ok());

        let players = result.unwrap();
        assert_eq!(players.len(), 1); // Only the valid player
        assert_eq!(players[0].name, "Valid Player");
    }

    #[test]
    fn test_parse_player_data_invalid_footedness() {
        let row = vec![
            "Invalid Foot Player".to_string(),
            "25".to_string(),
            "INVALID".to_string(),
        ];
        let sheet_data = vec![row];

        let result = parse_player_data(sheet_data);
        assert!(result.is_ok());

        let players = result.unwrap();
        assert_eq!(players.len(), 1);

        // Invalid footedness should default to Right
        assert_eq!(players[0].footedness, Footedness::Right);
    }

    #[test]
    fn test_parse_player_data_malformed_numeric() {
        let mut row = vec![
            "Bad Numbers Player".to_string(),
            "not_a_number".to_string(),
            "R".to_string(),
        ];

        // Add some malformed numeric data
        for i in 0..ABILITIES.len() {
            if i < 5 {
                row.push("not_a_number".to_string());
            } else {
                row.push("10.5".to_string());
            }
        }

        row.push("invalid_dna".to_string());

        for _i in 0..VALID_ROLES.len() {
            row.push("7.5".to_string());
        }

        let sheet_data = vec![row];
        let result = parse_player_data(sheet_data);

        assert!(result.is_ok());
        let players = result.unwrap();
        let player = &players[0];

        // Bad age should become 0
        assert_eq!(player.age, 0);

        // Bad DNA should become None
        assert_eq!(player.dna_score, None);

        // Bad abilities should become None, good ones should parse
        for (i, ability) in player.abilities.iter().enumerate() {
            if i < 5 {
                assert_eq!(*ability, None);
            } else {
                assert_eq!(*ability, Some(10.5));
            }
        }
    }

    #[test]
    fn test_parse_player_data_different_row_sizes() {
        let sheet_data = vec![
            // Short row - missing most data
            vec!["Short Player".to_string()],
            // Medium row - has some abilities
            vec![
                "Medium Player".to_string(),
                "25".to_string(),
                "L".to_string(),
                "10.0".to_string(),
                "11.0".to_string(),
            ],
            // Very long row with extra columns (should be ignored)
            {
                let mut row = vec![
                    "Long Player".to_string(),
                    "30".to_string(),
                    "RL".to_string(),
                ];
                // Add all expected data plus extra
                for _i in 0..ABILITIES.len() {
                    row.push("8.0".to_string());
                }
                row.push("90.0".to_string()); // DNA
                for _i in 0..VALID_ROLES.len() {
                    row.push("6.0".to_string());
                }
                row.push("extra1".to_string()); // Extra columns
                row.push("extra2".to_string());
                row
            },
        ];

        let result = parse_player_data(sheet_data);
        assert!(result.is_ok());

        let players = result.unwrap();
        assert_eq!(players.len(), 3);

        // Short player - most data should be None/default
        assert_eq!(players[0].name, "Short Player");
        assert_eq!(players[0].age, 0);
        assert_eq!(players[0].footedness, Footedness::Right);
        assert_eq!(players[0].dna_score, None);

        // Medium player - partial data
        assert_eq!(players[1].name, "Medium Player");
        assert_eq!(players[1].age, 25);
        assert_eq!(players[1].footedness, Footedness::Left);
        assert_eq!(players[1].abilities[0], Some(10.0)); // First ability parsed
        assert_eq!(players[1].abilities[1], Some(11.0)); // Second ability parsed
        assert_eq!(players[1].abilities[2], None); // Third ability missing

        // Long player - all data should be parsed correctly
        assert_eq!(players[2].name, "Long Player");
        assert_eq!(players[2].age, 30);
        assert_eq!(players[2].footedness, Footedness::Both);
        assert_eq!(players[2].dna_score, Some(90.0));
        assert_eq!(players[2].abilities[0], Some(8.0));
        assert_eq!(players[2].role_ratings[0], Some(6.0));
    }

    #[test]
    fn test_parse_player_data_column_mapping() {
        // Test that column indices map correctly to abilities and roles
        let mut row = vec![
            "Mapping Test".to_string(),
            "25".to_string(),
            "R".to_string(),
        ];

        // Add abilities with specific test values
        for i in 0..ABILITIES.len() {
            row.push(format!("ability_{i}"));
        }

        row.push("99.9".to_string()); // DNA

        // Add role ratings with specific test values
        for i in 0..VALID_ROLES.len() {
            row.push(format!("role_{i}"));
        }

        let sheet_data = vec![row];
        let result = parse_player_data(sheet_data);

        assert!(result.is_ok());
        let players = result.unwrap();
        let player = &players[0];

        // Verify abilities map to correct indices
        assert_eq!(player.abilities.len(), ABILITIES.len());

        // Verify role ratings map to correct indices
        assert_eq!(player.role_ratings.len(), VALID_ROLES.len());

        // Test specific ability retrieval
        assert_eq!(player.get_ability("Cor"), 0.0); // "ability_0" can't parse as f32, becomes 0.0

        // Test specific role rating retrieval
        let first_role = Role::new(VALID_ROLES[0]).unwrap();
        assert_eq!(player.get_role_rating(&first_role), 0.0); // "role_0" can't parse as f32, becomes 0.0
    }

    #[test]
    fn test_calculate_assignment_score() {
        let abilities = vec![Some(10.0); ABILITIES.len()];
        let mut role_ratings = vec![Some(0.0); VALID_ROLES.len()];
        role_ratings[0] = Some(15.5); // Set first role rating

        let player = Player::new(
            "Test Player".to_string(),
            25,
            Footedness::Right,
            abilities,
            Some(85.0),
            role_ratings,
        )
        .unwrap();

        let role = Role::new(VALID_ROLES[0]).unwrap();
        let score = calculate_assignment_score(&player, &role);
        assert_eq!(score, 15.5);

        // Test with a role that has no rating (should be 0.0)
        let role2 = Role::new(VALID_ROLES[1]).unwrap();
        let score2 = calculate_assignment_score(&player, &role2);
        assert_eq!(score2, 0.0);
    }

    #[test]
    fn test_find_optimal_assignments_exactly_11_players() {
        let mut players = Vec::new();
        let mut roles = Vec::new();

        // Create 11 players with different ratings
        for i in 0..11 {
            let abilities = vec![Some(10.0); ABILITIES.len()];
            let mut role_ratings = vec![Some(0.0); VALID_ROLES.len()];

            // Give each player a high rating for the role they should be assigned to
            role_ratings[i] = Some((i as f32 + 1.0) * 10.0);

            let player = Player::new(
                format!("Player {i}"),
                25,
                Footedness::Right,
                abilities,
                Some(85.0),
                role_ratings,
            )
            .unwrap();

            players.push(player);
            roles.push(Role::new(VALID_ROLES[i]).unwrap());
        }

        let result = find_optimal_assignments(players, roles);
        assert!(result.is_ok());

        let team = result.unwrap();
        assert_eq!(team.assignments.len(), 11);

        // Each player should be assigned to their optimal role
        for (i, assignment) in team.assignments.iter().enumerate() {
            assert!(assignment.player.name.contains(&i.to_string()));
            assert_eq!(assignment.score, (i as f32 + 1.0) * 10.0);
        }
    }

    #[test]
    fn test_find_optimal_assignments_more_than_11_players() {
        let mut players = Vec::new();
        let mut roles = Vec::new();

        // Create 15 players, but only 11 roles
        for i in 0..15 {
            let abilities = vec![Some(10.0); ABILITIES.len()];
            let mut role_ratings = vec![Some(5.0); VALID_ROLES.len()]; // Base rating

            // Give better players higher ratings for first role
            if i < 11 {
                role_ratings[0] = Some((i as f32 + 10.0) * 2.0); // Better ratings
            } else {
                role_ratings[0] = Some(i as f32); // Lower ratings
            }

            let player = Player::new(
                format!("Player {i}"),
                25,
                Footedness::Right,
                abilities,
                Some(85.0),
                role_ratings,
            )
            .unwrap();

            players.push(player);
        }

        // Create 11 roles, all the same (first role)
        for _ in 0..11 {
            roles.push(Role::new(VALID_ROLES[0]).unwrap());
        }

        let result = find_optimal_assignments(players, roles);
        assert!(result.is_ok());

        let team = result.unwrap();
        assert_eq!(team.assignments.len(), 11);

        // The 11 best players should be selected (Player 0-10, not 11-14)
        for assignment in &team.assignments {
            let player_num: usize = assignment
                .player
                .name
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse()
                .unwrap();
            assert!(
                player_num < 11,
                "Player {player_num} was selected but should not have been"
            );
        }
    }

    #[test]
    fn test_find_optimal_assignments_fewer_than_11_players() {
        let mut players = Vec::new();
        let mut roles = Vec::new();

        // Create only 5 players
        for i in 0..5 {
            let abilities = vec![Some(10.0); ABILITIES.len()];
            let role_ratings = vec![Some(8.0); VALID_ROLES.len()];

            let player = Player::new(
                format!("Player {i}"),
                25,
                Footedness::Right,
                abilities,
                Some(85.0),
                role_ratings,
            )
            .unwrap();

            players.push(player);
        }

        // Create 11 roles
        for i in 0..11 {
            roles.push(Role::new(VALID_ROLES[i]).unwrap());
        }

        let result = find_optimal_assignments(players, roles);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Need at least 11 players"));
    }

    #[test]
    fn test_find_optimal_assignments_wrong_number_roles() {
        let mut players = Vec::new();
        let mut roles = Vec::new();

        // Create 11 players
        for i in 0..11 {
            let abilities = vec![Some(10.0); ABILITIES.len()];
            let role_ratings = vec![Some(8.0); VALID_ROLES.len()];

            let player = Player::new(
                format!("Player {i}"),
                25,
                Footedness::Right,
                abilities,
                Some(85.0),
                role_ratings,
            )
            .unwrap();

            players.push(player);
        }

        // Create only 10 roles
        for i in 0..10 {
            roles.push(Role::new(VALID_ROLES[i]).unwrap());
        }

        let result = find_optimal_assignments(players, roles);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Must have exactly 11 roles"));
    }

    #[test]
    fn test_find_optimal_assignments_tied_scores() {
        let mut players = Vec::new();
        let mut roles = Vec::new();

        // Create 11 players with identical ratings (tied scores)
        for i in 0..11 {
            let abilities = vec![Some(10.0); ABILITIES.len()];
            let role_ratings = vec![Some(8.0); VALID_ROLES.len()]; // All same rating

            let player = Player::new(
                format!("Player {i}"),
                25,
                Footedness::Right,
                abilities,
                Some(85.0),
                role_ratings,
            )
            .unwrap();

            players.push(player);
        }

        // Create 11 different roles
        for i in 0..11 {
            roles.push(Role::new(VALID_ROLES[i]).unwrap());
        }

        let result = find_optimal_assignments(players, roles);
        assert!(result.is_ok());

        let team = result.unwrap();
        assert_eq!(team.assignments.len(), 11);

        // All assignments should have the same score
        for assignment in &team.assignments {
            assert_eq!(assignment.score, 8.0);
        }

        // Should have deterministic behavior (first player wins ties)
        // The exact order depends on the order players are processed
        assert_eq!(team.assignments[0].player.name, "Player 0");
    }

    #[test]
    fn test_find_optimal_assignments_zero_ratings() {
        let mut players = Vec::new();
        let mut roles = Vec::new();

        // Create 11 players with zero ratings for all roles
        for i in 0..11 {
            let abilities = vec![Some(10.0); ABILITIES.len()];
            let role_ratings = vec![Some(0.0); VALID_ROLES.len()]; // All zero ratings

            let player = Player::new(
                format!("Player {i}"),
                25,
                Footedness::Right,
                abilities,
                Some(85.0),
                role_ratings,
            )
            .unwrap();

            players.push(player);
        }

        // Create 11 roles
        for i in 0..11 {
            roles.push(Role::new(VALID_ROLES[i]).unwrap());
        }

        let result = find_optimal_assignments(players, roles);
        assert!(result.is_ok());

        let team = result.unwrap();
        assert_eq!(team.assignments.len(), 11);
        assert_eq!(team.total_score(), 0.0);
    }

    #[test]
    fn test_find_optimal_assignments_known_optimal_solution() {
        // Create a scenario with a known optimal solution
        let mut players = Vec::new();
        let mut roles = Vec::new();

        // Player 0: Best at role 0 (score 20)
        // Player 1: Best at role 1 (score 19)
        // Player 2: Best at role 2 (score 18)
        // etc.
        for i in 0..11 {
            let abilities = vec![Some(10.0); ABILITIES.len()];
            let mut role_ratings = vec![Some(5.0); VALID_ROLES.len()]; // Base rating

            // Each player is best at their corresponding role
            role_ratings[i] = Some(20.0 - i as f32);

            let player = Player::new(
                format!("Player {i}"),
                25,
                Footedness::Right,
                abilities,
                Some(85.0),
                role_ratings,
            )
            .unwrap();

            players.push(player);
        }

        // Create roles in order
        for i in 0..11 {
            roles.push(Role::new(VALID_ROLES[i]).unwrap());
        }

        let result = find_optimal_assignments(players, roles);
        assert!(result.is_ok());

        let team = result.unwrap();
        assert_eq!(team.assignments.len(), 11);

        // Expected total score: 20 + 19 + 18 + ... + 10 = 165
        let expected_score: f32 = (10..=20).sum::<i32>() as f32;
        assert_eq!(team.total_score(), expected_score);

        // Each player should be optimally assigned
        for (i, assignment) in team.assignments.iter().enumerate() {
            assert_eq!(assignment.score, 20.0 - i as f32);
        }
    }

    #[test]
    fn test_find_optimal_assignments_large_dataset() {
        let mut players = Vec::new();
        let mut roles = Vec::new();

        // Create 50 players with varying ratings
        for i in 0..50 {
            let abilities = vec![Some(10.0); ABILITIES.len()];
            let mut role_ratings = vec![Some(1.0); VALID_ROLES.len()]; // Low base rating

            // Give some players higher ratings for first few roles
            if i < 11 {
                role_ratings[i % 11] = Some(15.0 + i as f32); // High ratings for best players
            } else {
                role_ratings[i % 11] = Some(5.0 + (i % 10) as f32); // Medium ratings for others
            }

            let player = Player::new(
                format!("Player {i}"),
                25,
                Footedness::Right,
                abilities,
                Some(85.0),
                role_ratings,
            )
            .unwrap();

            players.push(player);
        }

        // Create 11 different roles
        for i in 0..11 {
            roles.push(Role::new(VALID_ROLES[i]).unwrap());
        }

        let result = find_optimal_assignments(players, roles);
        assert!(result.is_ok());

        let team = result.unwrap();
        assert_eq!(team.assignments.len(), 11);

        // The total score should be reasonably high (selecting best players)
        assert!(team.total_score() > 100.0);

        // All assignments should have positive scores
        for assignment in &team.assignments {
            assert!(assignment.score > 0.0);
        }
    }

    #[test]
    fn test_format_team_output_basic() {
        let mut assignments = Vec::new();

        // Create a simple team with 11 players
        for i in 0..11 {
            let abilities = vec![Some(10.0); ABILITIES.len()];
            let role_ratings = vec![Some(8.0 + i as f32); VALID_ROLES.len()];

            let player = Player::new(
                format!("Player {i}"),
                25,
                Footedness::Right,
                abilities,
                Some(85.0),
                role_ratings,
            )
            .unwrap();

            let role = Role::new(VALID_ROLES[i]).unwrap();
            assignments.push(Assignment::new(player, role));
        }

        let team = Team::new(assignments).unwrap();
        let output = format_team_output(&team);

        // Check that output contains the correct format "$ROLE -> $PLAYER_NAME"
        assert!(output.contains(" -> "));
        assert!(output.contains("Player 0"));
        assert!(output.contains("Total Score:"));

        // Check that each line follows the expected format
        let lines: Vec<&str> = output.trim().split('\n').collect();
        assert_eq!(lines.len(), 12); // 11 assignments + 1 total score line

        // Verify total score line
        let total_line = lines[11];
        assert!(total_line.starts_with("Total Score:"));

        // Calculate expected score: each player has role rating of (8.0 + i)
        // Player 0 gets role 0 with rating 8.0, Player 1 gets role 1 with rating 9.0, etc.
        let expected_total: f32 = (0..11).map(|i| 8.0 + i as f32).sum();
        assert!(total_line.contains(&format!("{:.1}", expected_total)));
    }

    #[test]
    fn test_format_team_output_sorted_by_role() {
        let mut assignments = Vec::new();

        // Create assignments with roles in reverse order to test sorting
        let role_indices = vec![10, 5, 0, 8, 3, 7, 2, 9, 4, 6, 1];

        for (i, &role_idx) in role_indices.iter().enumerate() {
            let abilities = vec![Some(10.0); ABILITIES.len()];
            let role_ratings = vec![Some(15.0); VALID_ROLES.len()];

            let player = Player::new(
                format!("Player {i}"),
                25,
                Footedness::Right,
                abilities,
                Some(85.0),
                role_ratings,
            )
            .unwrap();

            let role = Role::new(VALID_ROLES[role_idx]).unwrap();
            assignments.push(Assignment::new(player, role));
        }

        let team = Team::new(assignments).unwrap();
        let output = format_team_output(&team);

        let lines: Vec<&str> = output.trim().split('\n').collect();

        // Extract role names from output lines (exclude total score line)
        let mut output_roles = Vec::new();
        for line in &lines[0..11] {
            let parts: Vec<&str> = line.split(" -> ").collect();
            assert_eq!(parts.len(), 2);
            output_roles.push(parts[0]);
        }

        // Check that roles are sorted alphabetically
        let mut expected_roles = role_indices
            .iter()
            .map(|&idx| VALID_ROLES[idx])
            .collect::<Vec<_>>();
        expected_roles.sort();

        assert_eq!(output_roles, expected_roles);
    }

    #[test]
    fn test_format_team_output_long_names() {
        let mut assignments = Vec::new();

        // Create players with long names
        for i in 0..11 {
            let abilities = vec![Some(10.0); ABILITIES.len()];
            let role_ratings = vec![Some(12.0); VALID_ROLES.len()];

            let player = Player::new(
                format!("Very Long Player Name With Many Words {i}"),
                25,
                Footedness::Right,
                abilities,
                Some(85.0),
                role_ratings,
            )
            .unwrap();

            let role = Role::new(VALID_ROLES[i]).unwrap();
            assignments.push(Assignment::new(player, role));
        }

        let team = Team::new(assignments).unwrap();
        let output = format_team_output(&team);

        // Check that long names are handled correctly
        assert!(output.contains("Very Long Player Name With Many Words"));
        assert!(output.contains(" -> "));
        assert!(output.contains("Total Score: 132.0")); // 11 * 12.0

        // Each line should still follow the format
        let lines: Vec<&str> = output.trim().split('\n').collect();
        assert_eq!(lines.len(), 12);

        for line in &lines[0..11] {
            assert!(line.contains(" -> "));
            let parts: Vec<&str> = line.split(" -> ").collect();
            assert_eq!(parts.len(), 2);
            assert!(!parts[0].is_empty());
            assert!(!parts[1].is_empty());
        }
    }

    #[test]
    fn test_format_team_output_edge_case_scores() {
        let mut assignments = Vec::new();

        // Create team with minimum and maximum scores
        for i in 0..11 {
            let abilities = vec![Some(10.0); ABILITIES.len()];
            let mut role_ratings = vec![Some(0.0); VALID_ROLES.len()];

            // Alternate between very low and very high scores
            if i % 2 == 0 {
                role_ratings[i] = Some(0.0); // Minimum score
            } else {
                role_ratings[i] = Some(100.0); // High score
            }

            let player = Player::new(
                format!("Player {i}"),
                25,
                Footedness::Right,
                abilities,
                Some(85.0),
                role_ratings,
            )
            .unwrap();

            let role = Role::new(VALID_ROLES[i]).unwrap();
            assignments.push(Assignment::new(player, role));
        }

        let team = Team::new(assignments).unwrap();
        let output = format_team_output(&team);

        // Check that scores are formatted correctly
        assert!(output.contains("Total Score:"));

        let lines: Vec<&str> = output.trim().split('\n').collect();
        let total_line = lines[11];

        // Should show total score with 1 decimal place
        assert!(total_line.contains("Total Score: 500.0")); // 5 * 100.0 = 500.0

        // Check that all assignments are present
        assert_eq!(lines.len(), 12);
        for line in &lines[0..11] {
            assert!(line.contains(" -> Player "));
        }
    }

    #[test]
    fn test_format_team_output_duplicate_roles() {
        let mut assignments = Vec::new();

        let abilities = vec![Some(10.0); ABILITIES.len()];
        let role_ratings = vec![Some(20.0); VALID_ROLES.len()];

        // Create a team with duplicate roles (e.g., multiple goalkeepers)
        for i in 0..11 {
            let player = Player::new(
                format!("Player {i}"),
                25,
                Footedness::Right,
                abilities.clone(),
                Some(85.0),
                role_ratings.clone(),
            )
            .unwrap();

            // Use same role for first 3 players, different roles for others
            let role_name = if i < 3 { "GK" } else { VALID_ROLES[i] };
            let role = Role::new(role_name).unwrap();
            assignments.push(Assignment::new(player, role));
        }

        let team = Team::new(assignments).unwrap();
        let output = format_team_output(&team);

        // Check that duplicate roles are handled correctly
        let lines: Vec<&str> = output.trim().split('\n').collect();
        assert_eq!(lines.len(), 12);

        // Should have multiple lines with "GK -> "
        let gk_lines: Vec<&str> = lines
            .iter()
            .filter(|line| line.starts_with("GK -> "))
            .copied()
            .collect();
        assert_eq!(gk_lines.len(), 3);

        // Check total score
        assert!(output.contains("Total Score: 220.0")); // 11 * 20.0
    }

    #[test]
    fn test_format_team_output_consistent_ordering() {
        // Test that output ordering is consistent across multiple runs
        let mut assignments = Vec::new();

        for i in 0..11 {
            let abilities = vec![Some(10.0); ABILITIES.len()];
            let role_ratings = vec![Some(10.0 + i as f32); VALID_ROLES.len()];

            let player = Player::new(
                format!("Player {i}"),
                25,
                Footedness::Right,
                abilities,
                Some(85.0),
                role_ratings,
            )
            .unwrap();

            // Use roles in random order to test sorting consistency
            let role_idx = (i * 7) % 11; // Semi-random but deterministic ordering
            let role = Role::new(VALID_ROLES[role_idx]).unwrap();
            assignments.push(Assignment::new(player, role));
        }

        let team = Team::new(assignments).unwrap();

        // Generate output multiple times
        let output1 = format_team_output(&team);
        let output2 = format_team_output(&team);
        let output3 = format_team_output(&team);

        // All outputs should be identical
        assert_eq!(output1, output2);
        assert_eq!(output2, output3);

        // Output should be sorted by role name
        let lines: Vec<&str> = output1.trim().split('\n').collect();
        let roles: Vec<&str> = lines[0..11]
            .iter()
            .map(|line| line.split(" -> ").next().unwrap())
            .collect();

        // Check that roles are in sorted order
        for i in 1..roles.len() {
            assert!(
                roles[i - 1] <= roles[i],
                "Roles not in sorted order: {} should come before {}",
                roles[i - 1],
                roles[i]
            );
        }
    }

    #[test]
    fn test_format_team_output_decimal_precision() {
        let mut assignments = Vec::new();

        // Create scores with various decimal values
        for i in 0..11 {
            let abilities = vec![Some(10.0); ABILITIES.len()];
            let mut role_ratings = vec![Some(0.0); VALID_ROLES.len()];

            // Create scores with specific decimal values
            role_ratings[i] = Some(10.123456 + i as f32 * 0.789);

            let player = Player::new(
                format!("Player {i}"),
                25,
                Footedness::Right,
                abilities,
                Some(85.0),
                role_ratings,
            )
            .unwrap();

            let role = Role::new(VALID_ROLES[i]).unwrap();
            assignments.push(Assignment::new(player, role));
        }

        let team = Team::new(assignments).unwrap();
        let output = format_team_output(&team);

        // Check that total score is formatted to 1 decimal place
        let lines: Vec<&str> = output.trim().split('\n').collect();
        let total_line = lines[11];

        assert!(total_line.starts_with("Total Score:"));

        // Extract the numeric part and verify it has exactly 1 decimal place
        let score_part = total_line.split(": ").nth(1).unwrap();
        assert!(score_part.contains('.'));

        let decimal_places = score_part.split('.').nth(1).unwrap().len();
        assert_eq!(
            decimal_places, 1,
            "Total score should have exactly 1 decimal place, got: {}",
            score_part
        );
    }
}
