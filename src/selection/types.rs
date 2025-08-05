use crate::error::{FMDataError, Result};
use crate::error_helpers::{invalid_category, validation_error};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;

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
            _ => Err(invalid_category(short)),
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
pub const VALID_ROLES: &[&str] = &[
    "W(s) R", "W(s) L", "W(a) R", "W(a) L", "IF(s)", "IF(a)", "AP(s)", "AP(a)", "WTM(s)", "WTM(a)",
    "TQ(a)", "RD(A)", "IW(s)", "IW(a)", "DW(d)", "DW(s)", "WM(d)", "WM(s)", "WM(a)", "WP(s)",
    "WP(a)", "MEZ(s)", "MEZ(a)", "BWM(d)", "BWM(s)", "BBM", "CAR", "CM(d)", "CM(s)", "CM(a)",
    "DLP(d)", "DLP(s)", "RPM", "HB", "DM(d)", "DM(s)", "A", "SV(s)", "SV(a)", "RGA", "CD(d)",
    "CD(s)", "CD(c)", "NCB(d)", "WCB(d)", "WCB(s)", "WCB(a)", "BPD(d)", "BPD(s)", "BPD(c)", "L(s)",
    "L(a)", "FB(d) R", "FB(s) R", "FB(a) R", "FB(d) L", "FB(s) L", "FB(a) L", "IFB(d) R",
    "IFB(d) L", "WB(d) R", "WB(s) R", "WB(a) R", "WB(d) L", "WB(s) L", "WB(a) L", "IWB(d) R",
    "IWB(s) R", "IWB(a) R", "IWB(d) L", "IWB(s) L", "IWB(a) L", "CWB(s) R", "CWB(a) R", "CWB(s) L",
    "CWB(a) L", "PF(d)", "PF(s)", "PF(a)", "TM(s)", "TM(a)", "AF", "P", "DLF(s)", "DLF(a)",
    "CF(s)", "CF(a)", "F9", "SS", "EG", "SK(d)", "SK(s)", "SK(a)", "GK",
];

/// Player abilities tracked in the system
pub const ABILITIES: &[&str] = &[
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
            _ => Err(validation_error("footedness", s, "must be R, L, or RL")),
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
        use crate::constants::team::REQUIRED_ROLE_COUNT;
        
        if assignments.len() != REQUIRED_ROLE_COUNT {
            return Err(FMDataError::selection(format!(
                "Team must have exactly {} assignments, got {}",
                REQUIRED_ROLE_COUNT, assignments.len()
            )));
        }

        Self::new_unchecked(assignments)
    }

    /// Create a new team from assignments without size validation (allows partial teams)
    pub fn new_unchecked(assignments: Vec<Assignment>) -> Result<Self> {
        // Validate no duplicate players
        let mut player_names = HashSet::new();
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
