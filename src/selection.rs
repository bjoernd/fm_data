use crate::error::{FMDataError, Result};
use serde::{Deserialize, Serialize};
use std::fmt;
use tokio::fs;

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

        // Validate no duplicate roles
        let mut role_names = std::collections::HashSet::new();
        for assignment in &assignments {
            if !role_names.insert(&assignment.role.name) {
                return Err(FMDataError::selection(format!(
                    "Role {} is assigned to multiple players",
                    assignment.role.name
                )));
            }
        }

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
            let col_index = i + 3; // Abilities start at column D (index 3)
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
            let col_index = i + 51; // Role ratings start at column AZ (index 51)
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

#[cfg(test)]
mod tests {
    use super::*;

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
        assert!(team.is_err());
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
}
