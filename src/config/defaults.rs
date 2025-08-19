use crate::constants::defaults;
use crate::domain::SpreadsheetId;
use std::path::PathBuf;

/// Default team sheet name
pub fn default_team_sheet() -> String {
    String::from(defaults::TEAM_SHEET)
}

/// Default team performance sheet name
pub fn default_team_perf_sheet() -> String {
    String::from(defaults::TEAM_PERF_SHEET)
}

/// Default league performance sheet name
pub fn default_league_perf_sheet() -> String {
    String::from(defaults::LEAGUE_PERF_SHEET)
}

/// Default scouting sheet name
pub fn default_scouting_sheet() -> String {
    String::from(defaults::SCOUTING_SHEET)
}

/// Default browser sheet name
pub fn default_browser_sheet() -> String {
    String::from(defaults::BROWSER_SHEET)
}

/// Default OAuth token cache file path
pub fn default_token_file() -> String {
    dirs::config_dir()
        .unwrap_or_else(|| dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")))
        .join("fm_data")
        .join(crate::constants::config::TOKEN_CACHE_FILE)
        .to_string_lossy()
        .to_string()
}

/// Default spreadsheet ID (None for no default)
pub fn default_spreadsheet_id() -> Option<SpreadsheetId> {
    None
}

/// Get default paths for spreadsheet ID, credentials file, and HTML input
///
/// Returns (spreadsheet_id, credentials_file_path, html_input_path)
pub fn get_default_paths() -> (String, String, String) {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));

    let default_spreadsheet = String::from("1ZrBTdlMlGaLD6LhMs948YvZ41NE71mcy7jhmygJU2Bc");

    // Use secure config directory for credentials
    let default_creds = dirs::config_dir()
        .unwrap_or_else(|| home.join(".config"))
        .join("fm_data")
        .join("credentials.json")
        .to_string_lossy()
        .to_string();

    let default_html = home
        .join("Library")
        .join("Application Support")
        .join("Sports Interactive")
        .join("Football Manager 2024")
        .join("bd.html")
        .to_string_lossy()
        .to_string();

    (default_spreadsheet, default_creds, default_html)
}
