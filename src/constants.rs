/// Constants for Google Sheets ranges and data layout
pub mod ranges {
    /// Range for uploading player data to Google Sheets
    pub const UPLOAD_RANGE: &str = "A2:AX58";

    /// Range for downloading player data from Google Sheets  
    pub const DOWNLOAD_RANGE: &str = "A2:EQ58";

    /// Maximum number of data rows allowed for upload
    pub const MAX_DATA_ROWS: usize = 57;
}

/// Default sheet names used in the application
pub mod defaults {
    /// Default name for the team/squad sheet
    pub const TEAM_SHEET: &str = "Squad";

    /// Default name for the team performance statistics sheet
    pub const TEAM_PERF_SHEET: &str = "Stats_Team";

    /// Default name for the league/division performance statistics sheet
    pub const LEAGUE_PERF_SHEET: &str = "Stats_Division";
}

/// Data layout constants for column positions in spreadsheets
pub mod data_layout {
    /// Starting column index for player abilities (Column D)
    pub const ABILITIES_START_COL: usize = 3;

    /// Starting column index for role ratings (Column AZ)
    pub const ROLE_RATINGS_START_COL: usize = 51;
}

/// Application configuration constants
pub mod config {
    /// Default configuration file name
    pub const DEFAULT_CONFIG_FILE: &str = "config.json";

    /// Default OAuth token cache file name
    pub const TOKEN_CACHE_FILE: &str = "tokencache.json";
}

/// Team and role constants
pub mod team {
    /// Required number of roles for a complete team
    pub const REQUIRED_ROLE_COUNT: usize = 11;
}

/// File extension constants for validation
pub mod file_extensions {
    /// JSON file extension
    pub const JSON: &str = "json";

    /// HTML file extension
    pub const HTML: &str = "html";

    /// Text file extension
    pub const TXT: &str = "txt";
}

/// File extension enum for type-safe file extension handling
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileExtension {
    Json,
    Html,
    Txt,
}

impl FileExtension {
    /// Get the string representation of the file extension
    pub fn as_str(&self) -> &'static str {
        match self {
            FileExtension::Json => file_extensions::JSON,
            FileExtension::Html => file_extensions::HTML,
            FileExtension::Txt => file_extensions::TXT,
        }
    }

    /// Get file extension from a file path
    pub fn from_path(path: &str) -> Option<Self> {
        std::path::Path::new(path)
            .extension()
            .and_then(|ext| ext.to_str())
            .and_then(|s| s.parse().ok())
    }
}

impl std::str::FromStr for FileExtension {
    type Err = String;

    fn from_str(ext: &str) -> Result<Self, Self::Err> {
        match ext.to_lowercase().as_str() {
            "json" => Ok(FileExtension::Json),
            "html" => Ok(FileExtension::Html),
            "txt" => Ok(FileExtension::Txt),
            _ => Err(format!("Unknown file extension: {ext}")),
        }
    }
}

impl std::fmt::Display for FileExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_extension_as_str() {
        assert_eq!(FileExtension::Json.as_str(), "json");
        assert_eq!(FileExtension::Html.as_str(), "html");
        assert_eq!(FileExtension::Txt.as_str(), "txt");
    }

    #[test]
    fn test_file_extension_from_str() {
        use std::str::FromStr;

        assert_eq!(FileExtension::from_str("json"), Ok(FileExtension::Json));
        assert_eq!(FileExtension::from_str("JSON"), Ok(FileExtension::Json));
        assert_eq!(FileExtension::from_str("html"), Ok(FileExtension::Html));
        assert_eq!(FileExtension::from_str("txt"), Ok(FileExtension::Txt));
        assert!(FileExtension::from_str("unknown").is_err());

        // Test with parse method
        assert_eq!("json".parse::<FileExtension>(), Ok(FileExtension::Json));
        assert_eq!("HTML".parse::<FileExtension>(), Ok(FileExtension::Html));
        assert!("unknown".parse::<FileExtension>().is_err());
    }

    #[test]
    fn test_file_extension_from_path() {
        assert_eq!(
            FileExtension::from_path("test.json"),
            Some(FileExtension::Json)
        );
        assert_eq!(
            FileExtension::from_path("path/to/file.html"),
            Some(FileExtension::Html)
        );
        assert_eq!(
            FileExtension::from_path("config.txt"),
            Some(FileExtension::Txt)
        );
        assert_eq!(FileExtension::from_path("no_extension"), None);
        assert_eq!(FileExtension::from_path("unknown.xyz"), None);
    }

    #[test]
    fn test_file_extension_display() {
        assert_eq!(format!("{}", FileExtension::Json), "json");
        assert_eq!(format!("{}", FileExtension::Html), "html");
        assert_eq!(format!("{}", FileExtension::Txt), "txt");
    }
}

/// Player data transformation constants
pub mod player_data {
    /// Valid footedness values from Football Manager
    pub const VALID_FOOT_VALUES: &[&str] = &["Left", "Right", "Either", "Left Only", "Right Only"];

    /// Footedness value mappings for data transformation
    pub const FOOT_MAPPINGS: &[(&str, &str)] = &[
        ("Left", "l"),
        ("Left Only", "l"),
        ("Right", "r"),
        ("Right Only", "r"),
        ("Either", "rl"),
    ];
}
