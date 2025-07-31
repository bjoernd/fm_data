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
