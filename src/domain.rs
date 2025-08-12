//! Domain value objects for compile-time safety and validation
//!
//! This module provides strongly-typed wrappers for common string-based identifiers
//! to prevent invalid states at compile time and improve API clarity.

use crate::error::{FMDataError, Result};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// A Google Sheets spreadsheet identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SpreadsheetId(String);

impl SpreadsheetId {
    /// Create a new spreadsheet ID with validation
    pub fn new(id: impl AsRef<str>) -> Result<Self> {
        let id = id.as_ref().trim();
        
        if id.is_empty() {
            return Err(FMDataError::config("Spreadsheet ID cannot be empty"));
        }
        
        // Google Sheets IDs are typically 44 characters long and contain alphanumeric 
        // characters, hyphens, and underscores
        if id.len() < 10 {
            return Err(FMDataError::config(
                "Spreadsheet ID appears too short (minimum 10 characters)"
            ));
        }
        
        if !id.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
            return Err(FMDataError::config(
                "Spreadsheet ID contains invalid characters (only alphanumeric, hyphens, and underscores allowed)"
            ));
        }
        
        Ok(Self(id.to_string()))
    }
    
    /// Get the underlying string value
    pub fn as_str(&self) -> &str {
        &self.0
    }
    
    /// Convert to String
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for SpreadsheetId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for SpreadsheetId {
    type Err = FMDataError;
    
    fn from_str(s: &str) -> Result<Self> {
        Self::new(s)
    }
}

impl AsRef<str> for SpreadsheetId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// A football player identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PlayerId(String);

impl PlayerId {
    /// Create a new player ID with validation
    pub fn new(name: impl AsRef<str>) -> Result<Self> {
        let name = name.as_ref().trim();
        
        if name.is_empty() {
            return Err(FMDataError::selection("Player name cannot be empty"));
        }
        
        if name.len() > 100 {
            return Err(FMDataError::selection(
                "Player name is too long (maximum 100 characters)"
            ));
        }
        
        // Ensure name contains at least one letter (not just numbers/spaces)
        if !name.chars().any(|c| c.is_alphabetic()) {
            return Err(FMDataError::selection(
                "Player name must contain at least one letter"
            ));
        }
        
        Ok(Self(name.to_string()))
    }
    
    /// Get the underlying string value
    pub fn as_str(&self) -> &str {
        &self.0
    }
    
    /// Convert to String
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for PlayerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for PlayerId {
    type Err = FMDataError;
    
    fn from_str(s: &str) -> Result<Self> {
        Self::new(s)
    }
}

impl AsRef<str> for PlayerId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// A Football Manager role identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct RoleId(String);

impl RoleId {
    /// List of all valid Football Manager roles
    pub const VALID_ROLES: &'static [&'static str] = &[
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
    
    /// Create a new role ID with validation
    pub fn new(role: impl AsRef<str>) -> Result<Self> {
        let role = role.as_ref().trim();
        
        if role.is_empty() {
            return Err(FMDataError::selection("Role name cannot be empty"));
        }
        
        if !Self::is_valid_role(role) {
            return Err(FMDataError::selection(format!("Invalid role: {role}")));
        }
        
        Ok(Self(role.to_string()))
    }
    
    /// Check if a role name is valid
    pub fn is_valid_role(role: &str) -> bool {
        Self::VALID_ROLES.contains(&role.trim())
    }
    
    /// Get all valid role names
    pub fn get_valid_roles() -> &'static [&'static str] {
        Self::VALID_ROLES
    }
    
    /// Get the underlying string value
    pub fn as_str(&self) -> &str {
        &self.0
    }
    
    /// Convert to String
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for RoleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for RoleId {
    type Err = FMDataError;
    
    fn from_str(s: &str) -> Result<Self> {
        Self::new(s)
    }
}

impl AsRef<str> for RoleId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod spreadsheet_id_tests {
        use super::*;

        #[test]
        fn test_valid_spreadsheet_id() {
            let id = SpreadsheetId::new("1ZrBTdlMlGaLD6LhMs948YvZ41NE71mcy7jhmygJU2Bc").unwrap();
            assert_eq!(id.as_str(), "1ZrBTdlMlGaLD6LhMs948YvZ41NE71mcy7jhmygJU2Bc");
        }

        #[test]
        fn test_spreadsheet_id_with_hyphens() {
            let id = SpreadsheetId::new("test-spreadsheet-id-123").unwrap();
            assert_eq!(id.as_str(), "test-spreadsheet-id-123");
        }

        #[test]
        fn test_spreadsheet_id_with_underscores() {
            let id = SpreadsheetId::new("test_spreadsheet_id_123").unwrap();
            assert_eq!(id.as_str(), "test_spreadsheet_id_123");
        }

        #[test]
        fn test_empty_spreadsheet_id() {
            let result = SpreadsheetId::new("");
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("cannot be empty"));
        }

        #[test]
        fn test_whitespace_only_spreadsheet_id() {
            let result = SpreadsheetId::new("   ");
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("cannot be empty"));
        }

        #[test]
        fn test_too_short_spreadsheet_id() {
            let result = SpreadsheetId::new("abc123");
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("too short"));
        }

        #[test]
        fn test_invalid_characters_spreadsheet_id() {
            let result = SpreadsheetId::new("invalid@spreadsheet#id");
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("invalid characters"));
        }

        #[test]
        fn test_spreadsheet_id_display() {
            let id = SpreadsheetId::new("test-spreadsheet-123").unwrap();
            assert_eq!(format!("{id}"), "test-spreadsheet-123");
        }

        #[test]
        fn test_spreadsheet_id_from_str() {
            let id: SpreadsheetId = "test-spreadsheet-123".parse().unwrap();
            assert_eq!(id.as_str(), "test-spreadsheet-123");
        }

        #[test]
        fn test_spreadsheet_id_trim() {
            let id = SpreadsheetId::new("  test-spreadsheet-123  ").unwrap();
            assert_eq!(id.as_str(), "test-spreadsheet-123");
        }
    }

    mod player_id_tests {
        use super::*;

        #[test]
        fn test_valid_player_id() {
            let id = PlayerId::new("Lionel Messi").unwrap();
            assert_eq!(id.as_str(), "Lionel Messi");
        }

        #[test]
        fn test_player_id_with_numbers() {
            let id = PlayerId::new("Player 1").unwrap();
            assert_eq!(id.as_str(), "Player 1");
        }

        #[test]
        fn test_empty_player_id() {
            let result = PlayerId::new("");
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("cannot be empty"));
        }

        #[test]
        fn test_whitespace_only_player_id() {
            let result = PlayerId::new("   ");
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("cannot be empty"));
        }

        #[test]
        fn test_too_long_player_id() {
            let long_name = "a".repeat(101);
            let result = PlayerId::new(&long_name);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("too long"));
        }

        #[test]
        fn test_numbers_only_player_id() {
            let result = PlayerId::new("123456");
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("must contain at least one letter"));
        }

        #[test]
        fn test_player_id_display() {
            let id = PlayerId::new("Test Player").unwrap();
            assert_eq!(format!("{id}"), "Test Player");
        }

        #[test]
        fn test_player_id_from_str() {
            let id: PlayerId = "Test Player".parse().unwrap();
            assert_eq!(id.as_str(), "Test Player");
        }

        #[test]
        fn test_player_id_trim() {
            let id = PlayerId::new("  Test Player  ").unwrap();
            assert_eq!(id.as_str(), "Test Player");
        }
    }

    mod role_id_tests {
        use super::*;

        #[test]
        fn test_valid_role_id() {
            let id = RoleId::new("GK").unwrap();
            assert_eq!(id.as_str(), "GK");
        }

        #[test]
        fn test_complex_role_id() {
            let id = RoleId::new("W(s) R").unwrap();
            assert_eq!(id.as_str(), "W(s) R");
        }

        #[test]
        fn test_empty_role_id() {
            let result = RoleId::new("");
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("cannot be empty"));
        }

        #[test]
        fn test_invalid_role_id() {
            let result = RoleId::new("INVALID_ROLE");
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Invalid role"));
        }

        #[test]
        fn test_role_id_is_valid_role() {
            assert!(RoleId::is_valid_role("GK"));
            assert!(RoleId::is_valid_role("W(s) R"));
            assert!(!RoleId::is_valid_role("INVALID"));
        }

        #[test]
        fn test_role_id_get_valid_roles() {
            let roles = RoleId::get_valid_roles();
            assert!(roles.contains(&"GK"));
            assert!(roles.contains(&"W(s) R"));
            assert_eq!(roles.len(), 94);
        }

        #[test]
        fn test_role_id_display() {
            let id = RoleId::new("GK").unwrap();
            assert_eq!(format!("{id}"), "GK");
        }

        #[test]
        fn test_role_id_from_str() {
            let id: RoleId = "GK".parse().unwrap();
            assert_eq!(id.as_str(), "GK");
        }

        #[test]
        fn test_role_id_trim() {
            let id = RoleId::new("  GK  ").unwrap();
            assert_eq!(id.as_str(), "GK");
        }
    }
}