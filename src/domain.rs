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
                "Spreadsheet ID appears too short (minimum 10 characters)",
            ));
        }

        if !id
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        {
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
                "Player name is too long (maximum 100 characters)",
            ));
        }

        // Ensure name contains at least one letter (not just numbers/spaces)
        if !name.chars().any(|c| c.is_alphabetic()) {
            return Err(FMDataError::selection(
                "Player name must contain at least one letter",
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
        "W(s) R", "W(s) L", "W(a) R", "W(a) L", "IF(s)", "IF(a)", "AP(s)", "AP(a)", "WTM(s)",
        "WTM(a)", "TQ(a)", "RD(A)", "IW(s)", "IW(a)", "DW(d)", "DW(s)", "WM(d)", "WM(s)", "WM(a)",
        "WP(s)", "WP(a)", "MEZ(s)", "MEZ(a)", "BWM(d)", "BWM(s)", "BBM", "CAR", "CM(d)", "CM(s)",
        "CM(a)", "DLP(d)", "DLP(s)", "RPM", "HB", "DM(d)", "DM(s)", "A", "SV(s)", "SV(a)", "RGA",
        "CD(d)", "CD(s)", "CD(c)", "NCB(d)", "WCB(d)", "WCB(s)", "WCB(a)", "BPD(d)", "BPD(s)",
        "BPD(c)", "L(s)", "L(a)", "FB(d) R", "FB(s) R", "FB(a) R", "FB(d) L", "FB(s) L", "FB(a) L",
        "IFB(d) R", "IFB(d) L", "WB(d) R", "WB(s) R", "WB(a) R", "WB(d) L", "WB(s) L", "WB(a) L",
        "IWB(d) R", "IWB(s) R", "IWB(a) R", "IWB(d) L", "IWB(s) L", "IWB(a) L", "CWB(s) R",
        "CWB(a) R", "CWB(s) L", "CWB(a) L", "PF(d)", "PF(s)", "PF(a)", "TM(s)", "TM(a)", "AF", "P",
        "DLF(s)", "DLF(a)", "CF(s)", "CF(a)", "F9", "SS", "EG", "SK(d)", "SK(s)", "SK(a)", "GK",
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

/// A typed file path with validation for specific file types
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct FilePath(String);

impl FilePath {
    /// Create a new file path with basic validation
    pub fn new(path: impl AsRef<str>) -> Result<Self> {
        let path = path.as_ref().trim();

        if path.is_empty() {
            return Err(FMDataError::config("File path cannot be empty"));
        }

        if path.len() > 4096 {
            return Err(FMDataError::config(
                "File path is too long (maximum 4096 characters)",
            ));
        }

        // Check for invalid characters (basic validation)
        if path.contains('\0') {
            return Err(FMDataError::config(
                "File path contains invalid null character",
            ));
        }

        Ok(Self(path.to_string()))
    }

    /// Get the underlying string value
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Convert to String
    pub fn into_string(self) -> String {
        self.0
    }

    /// Get the file extension if present
    pub fn extension(&self) -> Option<crate::constants::FileExtension> {
        crate::constants::FileExtension::from_path(&self.0)
    }

    /// Get the file name without directory path
    pub fn file_name(&self) -> Option<&str> {
        std::path::Path::new(&self.0)
            .file_name()
            .and_then(|s| s.to_str())
    }

    /// Check if the file path is absolute
    pub fn is_absolute(&self) -> bool {
        std::path::Path::new(&self.0).is_absolute()
    }
}

impl fmt::Display for FilePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for FilePath {
    type Err = FMDataError;

    fn from_str(s: &str) -> Result<Self> {
        Self::new(s)
    }
}

impl AsRef<str> for FilePath {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// A credentials file path with additional validation
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CredentialsFile(FilePath);

impl CredentialsFile {
    /// Create a new credentials file path with validation
    pub fn new(path: impl AsRef<str>) -> Result<Self> {
        let file_path = FilePath::new(path)?;

        // Validate extension is JSON
        match file_path.extension() {
            Some(crate::constants::FileExtension::Json) => {}
            Some(ext) => {
                return Err(FMDataError::config(format!(
                    "Credentials file must be a JSON file, found .{ext}"
                )));
            }
            None => {
                return Err(FMDataError::config(
                    "Credentials file must have a .json extension",
                ));
            }
        }

        Ok(Self(file_path))
    }

    /// Get the underlying file path
    pub fn path(&self) -> &FilePath {
        &self.0
    }

    /// Get the underlying string value
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Convert to String
    pub fn into_string(self) -> String {
        self.0.into_string()
    }
}

impl fmt::Display for CredentialsFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for CredentialsFile {
    type Err = FMDataError;

    fn from_str(s: &str) -> Result<Self> {
        Self::new(s)
    }
}

impl AsRef<str> for CredentialsFile {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

/// A role file path with additional validation
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct RoleFile(FilePath);

impl RoleFile {
    /// Create a new role file path with validation
    pub fn new(path: impl AsRef<str>) -> Result<Self> {
        let file_path = FilePath::new(path)?;

        // Validate extension is TXT
        match file_path.extension() {
            Some(crate::constants::FileExtension::Txt) => {}
            Some(ext) => {
                return Err(FMDataError::config(format!(
                    "Role file must be a text file, found .{ext}"
                )));
            }
            None => {
                return Err(FMDataError::config("Role file must have a .txt extension"));
            }
        }

        Ok(Self(file_path))
    }

    /// Get the underlying file path
    pub fn path(&self) -> &FilePath {
        &self.0
    }

    /// Get the underlying string value
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Convert to String
    pub fn into_string(self) -> String {
        self.0.into_string()
    }
}

impl fmt::Display for RoleFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for RoleFile {
    type Err = FMDataError;

    fn from_str(s: &str) -> Result<Self> {
        Self::new(s)
    }
}

impl AsRef<str> for RoleFile {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

/// An image file path with additional validation
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ImageFile(FilePath);

impl ImageFile {
    /// Create a new image file path with validation
    pub fn new(path: impl AsRef<str>) -> Result<Self> {
        let file_path = FilePath::new(path)?;

        // Validate extension is PNG (the only supported format for FM screenshots)
        if let Some(ext_str) = std::path::Path::new(file_path.as_str())
            .extension()
            .and_then(|s| s.to_str())
        {
            let ext_lower = ext_str.to_lowercase();
            if ext_lower != "png" {
                return Err(FMDataError::config(format!(
                    "Image file must be a PNG file, found .{ext_str}"
                )));
            }
        } else {
            // For now, allow files without extensions (clipboard mode may not have extension)
            // The actual PNG validation happens later in the image processing pipeline
        }

        Ok(Self(file_path))
    }

    /// Get the underlying file path
    pub fn path(&self) -> &FilePath {
        &self.0
    }

    /// Get the underlying string value
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Convert to String
    pub fn into_string(self) -> String {
        self.0.into_string()
    }
}

impl fmt::Display for ImageFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for ImageFile {
    type Err = FMDataError;

    fn from_str(s: &str) -> Result<Self> {
        Self::new(s)
    }
}

impl AsRef<str> for ImageFile {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

/// An HTML file path with additional validation
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct HtmlFile(FilePath);

impl HtmlFile {
    /// Create a new HTML file path with validation
    pub fn new(path: impl AsRef<str>) -> Result<Self> {
        let file_path = FilePath::new(path)?;

        // Validate extension is HTML
        match file_path.extension() {
            Some(crate::constants::FileExtension::Html) => {}
            Some(ext) => {
                return Err(FMDataError::config(format!(
                    "HTML file must have an HTML extension, found .{ext}"
                )));
            }
            None => {
                return Err(FMDataError::config(
                    "HTML file must have an .html extension",
                ));
            }
        }

        Ok(Self(file_path))
    }

    /// Get the underlying file path
    pub fn path(&self) -> &FilePath {
        &self.0
    }

    /// Get the underlying string value
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Convert to String
    pub fn into_string(self) -> String {
        self.0.into_string()
    }
}

impl fmt::Display for HtmlFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for HtmlFile {
    type Err = FMDataError;

    fn from_str(s: &str) -> Result<Self> {
        Self::new(s)
    }
}

impl AsRef<str> for HtmlFile {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

/// A Google Sheets sheet name with validation
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SheetName(String);

impl SheetName {
    /// Create a new sheet name with validation
    pub fn new(name: impl AsRef<str>) -> Result<Self> {
        let name = name.as_ref().trim();

        if name.is_empty() {
            return Err(FMDataError::config("Sheet name cannot be empty"));
        }

        if name.len() > 100 {
            return Err(FMDataError::config(
                "Sheet name is too long (maximum 100 characters)",
            ));
        }

        // Google Sheets sheet names have certain restrictions
        if name.contains('[')
            || name.contains(']')
            || name.contains('*')
            || name.contains('?')
            || name.contains(':')
            || name.contains('/')
            || name.contains('\\')
        {
            return Err(FMDataError::config(
                "Sheet name contains invalid characters ([]?:/*\\ are not allowed)",
            ));
        }

        // Sheet name cannot be solely apostrophes
        if name.chars().all(|c| c == '\'') {
            return Err(FMDataError::config(
                "Sheet name cannot consist only of apostrophes",
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

impl fmt::Display for SheetName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for SheetName {
    type Err = FMDataError;

    fn from_str(s: &str) -> Result<Self> {
        Self::new(s)
    }
}

impl AsRef<str> for SheetName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// A Google Sheets cell range with validation
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CellRange(String);

impl CellRange {
    /// Create a new cell range with validation
    pub fn new(range: impl AsRef<str>) -> Result<Self> {
        let range = range.as_ref().trim();

        if range.is_empty() {
            return Err(FMDataError::config("Cell range cannot be empty"));
        }

        if range.len() > 255 {
            return Err(FMDataError::config(
                "Cell range is too long (maximum 255 characters)",
            ));
        }

        // Basic validation of range format (A1 notation)
        // Examples: A1, A1:B2, A:A, 1:1, Sheet1!A1:B2
        if !Self::is_valid_range_format(range) {
            return Err(FMDataError::config(
                "Invalid cell range format (expected A1 notation like 'A1:B2' or 'A1')",
            ));
        }

        Ok(Self(range.to_string()))
    }

    /// Check if the range format is valid (basic validation)
    fn is_valid_range_format(range: &str) -> bool {
        // Split on sheet separator if present
        let range_part = if let Some(pos) = range.find('!') {
            &range[pos + 1..]
        } else {
            range
        };

        // Handle single cell or range
        if range_part.contains(':') {
            let parts: Vec<&str> = range_part.split(':').collect();
            if parts.len() != 2 {
                return false;
            }
            Self::is_valid_cell_or_column_row(parts[0])
                && Self::is_valid_cell_or_column_row(parts[1])
        } else {
            Self::is_valid_cell_or_column_row(range_part)
        }
    }

    /// Check if a cell reference, column, or row is valid
    fn is_valid_cell_or_column_row(cell: &str) -> bool {
        if cell.is_empty() {
            return false;
        }

        // Column range (e.g., A:A, A:Z)
        if cell.chars().all(|c| c.is_ascii_uppercase()) {
            return !cell.is_empty() && cell.len() <= 3; // Max 3 letters for column
        }

        // Row range (e.g., 1:1, 1:100)
        if cell.chars().all(|c| c.is_ascii_digit()) {
            return cell.parse::<u32>().is_ok()
                && !cell.starts_with('0')
                && cell != "0"
                && cell.len() <= 7;
        }

        // Cell reference (e.g., A1, Z99, AA100)
        let mut letter_part = String::new();
        let mut digit_part = String::new();
        let mut parsing_digits = false;

        for ch in cell.chars() {
            if !parsing_digits && ch.is_ascii_uppercase() {
                letter_part.push(ch);
            } else if ch.is_ascii_digit() {
                parsing_digits = true;
                digit_part.push(ch);
            } else {
                return false;
            }
        }

        // Must have both letters and digits
        if letter_part.is_empty() || digit_part.is_empty() {
            return false;
        }

        // Column part cannot be more than 3 letters (Google Sheets max: AAA = 703 columns)
        if letter_part.len() > 3 {
            return false;
        }

        // Row numbers must not be 0 or start with 0 (except single 0, but we don't allow row 0)
        if digit_part == "0" || (digit_part.len() > 1 && digit_part.starts_with('0')) {
            return false;
        }

        // Row number cannot be too long (Google Sheets max is 1,048,576 = 7 digits)
        if digit_part.len() > 7 {
            return false;
        }

        true
    }

    /// Get the underlying string value
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Convert to String
    pub fn into_string(self) -> String {
        self.0
    }

    /// Create a range with sheet name
    pub fn with_sheet(sheet: &SheetName, range: impl AsRef<str>) -> Result<Self> {
        let full_range = format!("{}!{}", sheet.as_str(), range.as_ref());
        Self::new(full_range)
    }

    /// Get the sheet name if this range includes one
    pub fn sheet_name(&self) -> Option<&str> {
        self.0.find('!').map(|pos| &self.0[..pos])
    }

    /// Get the range part without sheet name
    pub fn range_part(&self) -> &str {
        if let Some(pos) = self.0.find('!') {
            &self.0[pos + 1..]
        } else {
            &self.0
        }
    }
}

impl fmt::Display for CellRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for CellRange {
    type Err = FMDataError;

    fn from_str(s: &str) -> Result<Self> {
        Self::new(s)
    }
}

impl AsRef<str> for CellRange {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// A Football Manager attribute name with validation
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AttributeName(String);

impl AttributeName {
    /// List of all valid Football Manager attributes
    pub const VALID_ATTRIBUTES: &'static [&'static str] = &[
        // Technical attributes
        "Corners",
        "Crossing",
        "Dribbling",
        "Finishing",
        "First Touch",
        "Free Kick Taking",
        "Heading",
        "Long Shots",
        "Long Throws",
        "Marking",
        "Passing",
        "Penalty Taking",
        "Tackling",
        "Technique",
        // Mental attributes
        "Aggression",
        "Anticipation",
        "Bravery",
        "Composure",
        "Concentration",
        "Decisions",
        "Determination",
        "Flair",
        "Leadership",
        "Off the Ball",
        "Positioning",
        "Teamwork",
        "Vision",
        "Work Rate",
        // Physical attributes
        "Acceleration",
        "Agility",
        "Balance",
        "Jumping Reach",
        "Natural Fitness",
        "Pace",
        "Stamina",
        "Strength",
        // Goalkeeping attributes
        "Aerial Reach",
        "Command of Area",
        "Communication",
        "Eccentricity",
        "Handling",
        "Kicking",
        "One on Ones",
        "Reflexes",
        "Rushing Out",
        "Tendency to Punch",
        "Throwing",
    ];

    /// Create a new attribute name with validation
    pub fn new(name: impl AsRef<str>) -> Result<Self> {
        let name = name.as_ref().trim();

        if name.is_empty() {
            return Err(FMDataError::selection("Attribute name cannot be empty"));
        }

        if !Self::is_valid_attribute(name) {
            return Err(FMDataError::selection(format!("Invalid attribute: {name}")));
        }

        Ok(Self(name.to_string()))
    }

    /// Check if an attribute name is valid
    pub fn is_valid_attribute(name: &str) -> bool {
        Self::VALID_ATTRIBUTES.contains(&name.trim())
    }

    /// Get all valid attribute names
    pub fn get_valid_attributes() -> &'static [&'static str] {
        Self::VALID_ATTRIBUTES
    }

    /// Check if this is a technical attribute
    pub fn is_technical(&self) -> bool {
        matches!(
            self.0.as_str(),
            "Corners"
                | "Crossing"
                | "Dribbling"
                | "Finishing"
                | "First Touch"
                | "Free Kick Taking"
                | "Heading"
                | "Long Shots"
                | "Long Throws"
                | "Marking"
                | "Passing"
                | "Penalty Taking"
                | "Tackling"
                | "Technique"
        )
    }

    /// Check if this is a mental attribute
    pub fn is_mental(&self) -> bool {
        matches!(
            self.0.as_str(),
            "Aggression"
                | "Anticipation"
                | "Bravery"
                | "Composure"
                | "Concentration"
                | "Decisions"
                | "Determination"
                | "Flair"
                | "Leadership"
                | "Off the Ball"
                | "Positioning"
                | "Teamwork"
                | "Vision"
                | "Work Rate"
        )
    }

    /// Check if this is a physical attribute
    pub fn is_physical(&self) -> bool {
        matches!(
            self.0.as_str(),
            "Acceleration"
                | "Agility"
                | "Balance"
                | "Jumping Reach"
                | "Natural Fitness"
                | "Pace"
                | "Stamina"
                | "Strength"
        )
    }

    /// Check if this is a goalkeeping attribute
    pub fn is_goalkeeping(&self) -> bool {
        matches!(
            self.0.as_str(),
            "Aerial Reach"
                | "Command of Area"
                | "Communication"
                | "Eccentricity"
                | "Handling"
                | "Kicking"
                | "One on Ones"
                | "Reflexes"
                | "Rushing Out"
                | "Tendency to Punch"
                | "Throwing"
        )
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

impl fmt::Display for AttributeName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for AttributeName {
    type Err = FMDataError;

    fn from_str(s: &str) -> Result<Self> {
        Self::new(s)
    }
}

impl AsRef<str> for AttributeName {
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
            assert!(result
                .unwrap_err()
                .to_string()
                .contains("invalid characters"));
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
            assert!(result
                .unwrap_err()
                .to_string()
                .contains("must contain at least one letter"));
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

    mod file_path_tests {
        use super::*;

        #[test]
        fn test_valid_file_path() {
            let path = FilePath::new("/path/to/file.txt").unwrap();
            assert_eq!(path.as_str(), "/path/to/file.txt");
        }

        #[test]
        fn test_file_path_with_spaces() {
            let path = FilePath::new("/path with spaces/file.txt").unwrap();
            assert_eq!(path.as_str(), "/path with spaces/file.txt");
        }

        #[test]
        fn test_empty_file_path() {
            let result = FilePath::new("");
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("cannot be empty"));
        }

        #[test]
        fn test_whitespace_only_file_path() {
            let result = FilePath::new("   ");
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("cannot be empty"));
        }

        #[test]
        fn test_too_long_file_path() {
            let long_path = "a".repeat(4097);
            let result = FilePath::new(&long_path);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("too long"));
        }

        #[test]
        fn test_null_character_file_path() {
            let result = FilePath::new("path\0with\0null");
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("null character"));
        }

        #[test]
        fn test_file_path_extension() {
            let path = FilePath::new("test.json").unwrap();
            assert_eq!(
                path.extension(),
                Some(crate::constants::FileExtension::Json)
            );

            let path = FilePath::new("test.html").unwrap();
            assert_eq!(
                path.extension(),
                Some(crate::constants::FileExtension::Html)
            );

            let path = FilePath::new("test.txt").unwrap();
            assert_eq!(path.extension(), Some(crate::constants::FileExtension::Txt));

            let path = FilePath::new("test.png").unwrap();
            assert_eq!(path.extension(), Some(crate::constants::FileExtension::Png));

            let path = FilePath::new("no_extension").unwrap();
            assert_eq!(path.extension(), None);
        }

        #[test]
        fn test_file_path_file_name() {
            let path = FilePath::new("/path/to/file.txt").unwrap();
            assert_eq!(path.file_name(), Some("file.txt"));

            let path = FilePath::new("file.txt").unwrap();
            assert_eq!(path.file_name(), Some("file.txt"));

            let path = FilePath::new("/path/to/").unwrap();
            assert_eq!(path.file_name(), Some("to"));
        }

        #[test]
        fn test_file_path_is_absolute() {
            let path = FilePath::new("/absolute/path").unwrap();
            assert!(path.is_absolute());

            let path = FilePath::new("relative/path").unwrap();
            assert!(!path.is_absolute());
        }

        #[test]
        fn test_file_path_display() {
            let path = FilePath::new("/test/path").unwrap();
            assert_eq!(format!("{path}"), "/test/path");
        }

        #[test]
        fn test_file_path_from_str() {
            let path: FilePath = "/test/path".parse().unwrap();
            assert_eq!(path.as_str(), "/test/path");
        }

        #[test]
        fn test_file_path_trim() {
            let path = FilePath::new("  /test/path  ").unwrap();
            assert_eq!(path.as_str(), "/test/path");
        }
    }

    mod credentials_file_tests {
        use super::*;

        #[test]
        fn test_valid_credentials_file() {
            let file = CredentialsFile::new("credentials.json").unwrap();
            assert_eq!(file.as_str(), "credentials.json");
        }

        #[test]
        fn test_credentials_file_with_path() {
            let file = CredentialsFile::new("/path/to/credentials.json").unwrap();
            assert_eq!(file.as_str(), "/path/to/credentials.json");
        }

        #[test]
        fn test_credentials_file_non_json_extension() {
            let result = CredentialsFile::new("credentials.txt");
            assert!(result.is_err());
            assert!(result
                .unwrap_err()
                .to_string()
                .contains("must be a JSON file"));
        }

        #[test]
        fn test_credentials_file_no_extension() {
            let result = CredentialsFile::new("credentials");
            assert!(result.is_err());
            assert!(result
                .unwrap_err()
                .to_string()
                .contains("must have a .json extension"));
        }

        #[test]
        fn test_credentials_file_empty() {
            let result = CredentialsFile::new("");
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("cannot be empty"));
        }

        #[test]
        fn test_credentials_file_display() {
            let file = CredentialsFile::new("test.json").unwrap();
            assert_eq!(format!("{file}"), "test.json");
        }

        #[test]
        fn test_credentials_file_from_str() {
            let file: CredentialsFile = "test.json".parse().unwrap();
            assert_eq!(file.as_str(), "test.json");
        }

        #[test]
        fn test_credentials_file_path_access() {
            let file = CredentialsFile::new("test.json").unwrap();
            assert_eq!(file.path().as_str(), "test.json");
        }
    }

    mod role_file_tests {
        use super::*;

        #[test]
        fn test_valid_role_file() {
            let file = RoleFile::new("roles.txt").unwrap();
            assert_eq!(file.as_str(), "roles.txt");
        }

        #[test]
        fn test_role_file_with_path() {
            let file = RoleFile::new("/path/to/roles.txt").unwrap();
            assert_eq!(file.as_str(), "/path/to/roles.txt");
        }

        #[test]
        fn test_role_file_non_txt_extension() {
            let result = RoleFile::new("roles.json");
            assert!(result.is_err());
            assert!(result
                .unwrap_err()
                .to_string()
                .contains("must be a text file"));
        }

        #[test]
        fn test_role_file_no_extension() {
            let result = RoleFile::new("roles");
            assert!(result.is_err());
            assert!(result
                .unwrap_err()
                .to_string()
                .contains("must have a .txt extension"));
        }

        #[test]
        fn test_role_file_empty() {
            let result = RoleFile::new("");
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("cannot be empty"));
        }

        #[test]
        fn test_role_file_display() {
            let file = RoleFile::new("test.txt").unwrap();
            assert_eq!(format!("{file}"), "test.txt");
        }

        #[test]
        fn test_role_file_from_str() {
            let file: RoleFile = "test.txt".parse().unwrap();
            assert_eq!(file.as_str(), "test.txt");
        }

        #[test]
        fn test_role_file_path_access() {
            let file = RoleFile::new("test.txt").unwrap();
            assert_eq!(file.path().as_str(), "test.txt");
        }
    }

    mod image_file_tests {
        use super::*;

        #[test]
        fn test_valid_image_file() {
            let file = ImageFile::new("screenshot.png").unwrap();
            assert_eq!(file.as_str(), "screenshot.png");
        }

        #[test]
        fn test_image_file_with_path() {
            let file = ImageFile::new("/path/to/screenshot.png").unwrap();
            assert_eq!(file.as_str(), "/path/to/screenshot.png");
        }

        #[test]
        fn test_image_file_non_png_extension() {
            let result = ImageFile::new("image.jpg");
            assert!(result.is_err());
            assert!(result
                .unwrap_err()
                .to_string()
                .contains("must be a PNG file"));
        }

        #[test]
        fn test_image_file_no_extension() {
            // Should be allowed for clipboard mode
            let file = ImageFile::new("temp_image").unwrap();
            assert_eq!(file.as_str(), "temp_image");
        }

        #[test]
        fn test_image_file_empty() {
            let result = ImageFile::new("");
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("cannot be empty"));
        }

        #[test]
        fn test_image_file_display() {
            let file = ImageFile::new("test.png").unwrap();
            assert_eq!(format!("{file}"), "test.png");
        }

        #[test]
        fn test_image_file_from_str() {
            let file: ImageFile = "test.png".parse().unwrap();
            assert_eq!(file.as_str(), "test.png");
        }

        #[test]
        fn test_image_file_path_access() {
            let file = ImageFile::new("test.png").unwrap();
            assert_eq!(file.path().as_str(), "test.png");
        }
    }

    mod html_file_tests {
        use super::*;

        #[test]
        fn test_valid_html_file() {
            let file = HtmlFile::new("data.html").unwrap();
            assert_eq!(file.as_str(), "data.html");
        }

        #[test]
        fn test_html_file_with_path() {
            let file = HtmlFile::new("/path/to/data.html").unwrap();
            assert_eq!(file.as_str(), "/path/to/data.html");
        }

        #[test]
        fn test_html_file_non_html_extension() {
            let result = HtmlFile::new("data.txt");
            assert!(result.is_err());
            assert!(result
                .unwrap_err()
                .to_string()
                .contains("must have an HTML extension"));
        }

        #[test]
        fn test_html_file_no_extension() {
            let result = HtmlFile::new("data");
            assert!(result.is_err());
            assert!(result
                .unwrap_err()
                .to_string()
                .contains("must have an .html extension"));
        }

        #[test]
        fn test_html_file_empty() {
            let result = HtmlFile::new("");
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("cannot be empty"));
        }

        #[test]
        fn test_html_file_display() {
            let file = HtmlFile::new("test.html").unwrap();
            assert_eq!(format!("{file}"), "test.html");
        }

        #[test]
        fn test_html_file_from_str() {
            let file: HtmlFile = "test.html".parse().unwrap();
            assert_eq!(file.as_str(), "test.html");
        }

        #[test]
        fn test_html_file_path_access() {
            let file = HtmlFile::new("test.html").unwrap();
            assert_eq!(file.path().as_str(), "test.html");
        }
    }

    mod sheet_name_tests {
        use super::*;

        #[test]
        fn test_valid_sheet_name() {
            let name = SheetName::new("Squad").unwrap();
            assert_eq!(name.as_str(), "Squad");
        }

        #[test]
        fn test_sheet_name_with_spaces() {
            let name = SheetName::new("Player Stats").unwrap();
            assert_eq!(name.as_str(), "Player Stats");
        }

        #[test]
        fn test_empty_sheet_name() {
            let result = SheetName::new("");
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("cannot be empty"));
        }

        #[test]
        fn test_whitespace_only_sheet_name() {
            let result = SheetName::new("   ");
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("cannot be empty"));
        }

        #[test]
        fn test_too_long_sheet_name() {
            let long_name = "a".repeat(101);
            let result = SheetName::new(&long_name);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("too long"));
        }

        #[test]
        fn test_sheet_name_invalid_characters() {
            let invalid_chars = ["[", "]", "*", "?", ":", "/", "\\"];
            for char in invalid_chars {
                let result = SheetName::new(format!("Sheet{char}Name"));
                assert!(result.is_err());
                assert!(result
                    .unwrap_err()
                    .to_string()
                    .contains("invalid characters"));
            }
        }

        #[test]
        fn test_sheet_name_only_apostrophes() {
            let result = SheetName::new("'''");
            assert!(result.is_err());
            assert!(result
                .unwrap_err()
                .to_string()
                .contains("only of apostrophes"));
        }

        #[test]
        fn test_sheet_name_display() {
            let name = SheetName::new("TestSheet").unwrap();
            assert_eq!(format!("{name}"), "TestSheet");
        }

        #[test]
        fn test_sheet_name_from_str() {
            let name: SheetName = "TestSheet".parse().unwrap();
            assert_eq!(name.as_str(), "TestSheet");
        }

        #[test]
        fn test_sheet_name_trim() {
            let name = SheetName::new("  TestSheet  ").unwrap();
            assert_eq!(name.as_str(), "TestSheet");
        }
    }

    mod cell_range_tests {
        use super::*;

        #[test]
        fn test_valid_single_cell_range() {
            let range = CellRange::new("A1").unwrap();
            assert_eq!(range.as_str(), "A1");
        }

        #[test]
        fn test_valid_multi_cell_range() {
            let range = CellRange::new("A1:B2").unwrap();
            assert_eq!(range.as_str(), "A1:B2");
        }

        #[test]
        fn test_valid_column_range() {
            let range = CellRange::new("A:A").unwrap();
            assert_eq!(range.as_str(), "A:A");
        }

        #[test]
        fn test_valid_row_range() {
            let range = CellRange::new("1:1").unwrap();
            assert_eq!(range.as_str(), "1:1");
        }

        #[test]
        fn test_valid_single_column() {
            let range = CellRange::new("A").unwrap();
            assert_eq!(range.as_str(), "A");
        }

        #[test]
        fn test_valid_single_row() {
            let range = CellRange::new("1").unwrap();
            assert_eq!(range.as_str(), "1");
        }

        #[test]
        fn test_valid_range_with_sheet() {
            let range = CellRange::new("Sheet1!A1:B2").unwrap();
            assert_eq!(range.as_str(), "Sheet1!A1:B2");
        }

        #[test]
        fn test_empty_cell_range() {
            let result = CellRange::new("");
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("cannot be empty"));
        }

        #[test]
        fn test_too_long_cell_range() {
            let long_range = "a".repeat(256);
            let result = CellRange::new(&long_range);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("too long"));
        }

        #[test]
        fn test_invalid_cell_range_format() {
            let invalid_ranges = ["A1:B2:C3", "A1:", ":B2", "A1B2", "123A"];
            for range in invalid_ranges {
                let result = CellRange::new(range);
                assert!(result.is_err(), "Expected error for range: {range}");
                assert!(result
                    .unwrap_err()
                    .to_string()
                    .contains("Invalid cell range format"));
            }
        }

        #[test]
        fn test_cell_range_with_sheet() {
            let sheet = SheetName::new("TestSheet").unwrap();
            let range = CellRange::with_sheet(&sheet, "A1:B2").unwrap();
            assert_eq!(range.as_str(), "TestSheet!A1:B2");
        }

        #[test]
        fn test_cell_range_sheet_name() {
            let range = CellRange::new("TestSheet!A1:B2").unwrap();
            assert_eq!(range.sheet_name(), Some("TestSheet"));

            let range = CellRange::new("A1:B2").unwrap();
            assert_eq!(range.sheet_name(), None);
        }

        #[test]
        fn test_cell_range_range_part() {
            let range = CellRange::new("TestSheet!A1:B2").unwrap();
            assert_eq!(range.range_part(), "A1:B2");

            let range = CellRange::new("A1:B2").unwrap();
            assert_eq!(range.range_part(), "A1:B2");
        }

        #[test]
        fn test_cell_range_display() {
            let range = CellRange::new("A1:B2").unwrap();
            assert_eq!(format!("{range}"), "A1:B2");
        }

        #[test]
        fn test_cell_range_from_str() {
            let range: CellRange = "A1:B2".parse().unwrap();
            assert_eq!(range.as_str(), "A1:B2");
        }

        #[test]
        fn test_cell_range_trim() {
            let range = CellRange::new("  A1:B2  ").unwrap();
            assert_eq!(range.as_str(), "A1:B2");
        }

        #[test]
        fn test_cell_range_validation_edge_cases() {
            // Valid complex ranges
            assert!(CellRange::new("AA100").is_ok());
            assert!(CellRange::new("Z999").is_ok());
            assert!(CellRange::new("A:Z").is_ok());
            assert!(CellRange::new("1:999").is_ok());

            // Invalid ranges
            assert!(CellRange::new("A0").is_err()); // No row 0
            assert!(CellRange::new("01:01").is_err()); // Leading zeros
            assert!(CellRange::new("AAAA1").is_err()); // Too many letters
            assert!(CellRange::new("A12345678").is_err()); // Row number too long
        }
    }

    mod attribute_name_tests {
        use super::*;

        #[test]
        fn test_valid_technical_attribute() {
            let attr = AttributeName::new("Passing").unwrap();
            assert_eq!(attr.as_str(), "Passing");
            assert!(attr.is_technical());
            assert!(!attr.is_mental());
            assert!(!attr.is_physical());
            assert!(!attr.is_goalkeeping());
        }

        #[test]
        fn test_valid_mental_attribute() {
            let attr = AttributeName::new("Determination").unwrap();
            assert_eq!(attr.as_str(), "Determination");
            assert!(!attr.is_technical());
            assert!(attr.is_mental());
            assert!(!attr.is_physical());
            assert!(!attr.is_goalkeeping());
        }

        #[test]
        fn test_valid_physical_attribute() {
            let attr = AttributeName::new("Pace").unwrap();
            assert_eq!(attr.as_str(), "Pace");
            assert!(!attr.is_technical());
            assert!(!attr.is_mental());
            assert!(attr.is_physical());
            assert!(!attr.is_goalkeeping());
        }

        #[test]
        fn test_valid_goalkeeping_attribute() {
            let attr = AttributeName::new("Reflexes").unwrap();
            assert_eq!(attr.as_str(), "Reflexes");
            assert!(!attr.is_technical());
            assert!(!attr.is_mental());
            assert!(!attr.is_physical());
            assert!(attr.is_goalkeeping());
        }

        #[test]
        fn test_empty_attribute_name() {
            let result = AttributeName::new("");
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("cannot be empty"));
        }

        #[test]
        fn test_invalid_attribute_name() {
            let result = AttributeName::new("Invalid Attribute");
            assert!(result.is_err());
            assert!(result
                .unwrap_err()
                .to_string()
                .contains("Invalid attribute"));
        }

        #[test]
        fn test_attribute_name_is_valid() {
            assert!(AttributeName::is_valid_attribute("Passing"));
            assert!(AttributeName::is_valid_attribute("Determination"));
            assert!(AttributeName::is_valid_attribute("Pace"));
            assert!(AttributeName::is_valid_attribute("Reflexes"));
            assert!(!AttributeName::is_valid_attribute("Invalid"));
        }

        #[test]
        fn test_attribute_name_get_valid_attributes() {
            let attributes = AttributeName::get_valid_attributes();
            assert!(attributes.contains(&"Passing"));
            assert!(attributes.contains(&"Determination"));
            assert!(attributes.contains(&"Pace"));
            assert!(attributes.contains(&"Reflexes"));
            assert_eq!(attributes.len(), 47); // Total number of attributes
        }

        #[test]
        fn test_attribute_name_display() {
            let attr = AttributeName::new("Passing").unwrap();
            assert_eq!(format!("{attr}"), "Passing");
        }

        #[test]
        fn test_attribute_name_from_str() {
            let attr: AttributeName = "Passing".parse().unwrap();
            assert_eq!(attr.as_str(), "Passing");
        }

        #[test]
        fn test_attribute_name_trim() {
            let attr = AttributeName::new("  Passing  ").unwrap();
            assert_eq!(attr.as_str(), "Passing");
        }
    }
}
