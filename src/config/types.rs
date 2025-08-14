use crate::domain::SpreadsheetId;
use serde::{Deserialize, Serialize};

/// Serialization helper for SpreadsheetId
pub(crate) mod spreadsheet_id_serde {
    use super::SpreadsheetId;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(value: &Option<SpreadsheetId>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(id) => id.as_str().serialize(serializer),
            None => "".serialize(serializer),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<SpreadsheetId>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.trim().is_empty() {
            Ok(None)
        } else {
            SpreadsheetId::new(s)
                .map(Some)
                .map_err(serde::de::Error::custom)
        }
    }
}

/// Main configuration structure with Google and input settings
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Config {
    /// Google Sheets and authentication configuration
    #[serde(default)]
    pub google: GoogleConfig,
    /// Input file path configuration
    #[serde(default)]
    pub input: InputConfig,
}

/// Google Sheets API and authentication configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GoogleConfig {
    /// Path to Google service account credentials JSON file
    #[serde(default)]
    pub creds_file: String,
    /// Path to OAuth token cache file
    #[serde(default = "super::defaults::default_token_file")]
    pub token_file: String,
    /// Google Sheets spreadsheet ID (validated)
    #[serde(default = "super::defaults::default_spreadsheet_id", with = "spreadsheet_id_serde")]
    pub spreadsheet_name: Option<SpreadsheetId>,
    /// Sheet name for team data
    #[serde(default = "super::defaults::default_team_sheet")]
    pub team_sheet: String,
    /// Sheet name for team performance data
    #[serde(default = "super::defaults::default_team_perf_sheet")]
    pub team_perf_sheet: String,
    /// Sheet name for league performance data
    #[serde(default = "super::defaults::default_league_perf_sheet")]
    pub league_perf_sheet: String,
    /// Sheet name for scouting data
    #[serde(default = "super::defaults::default_scouting_sheet")]
    pub scouting_sheet: String,
}

/// Input file path configuration for various data sources
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct InputConfig {
    /// Path to HTML file containing player data
    #[serde(default)]
    pub data_html: String,
    /// Path to HTML file containing league performance data
    #[serde(default)]
    pub league_perf_html: String,
    /// Path to HTML file containing team performance data
    #[serde(default)]
    pub team_perf_html: String,
    /// Path to text file containing team role definitions
    #[serde(default)]
    pub role_file: String,
    /// Path to PNG image file containing player attributes
    #[serde(default)]
    pub image_file: String,
}

impl Default for GoogleConfig {
    fn default() -> Self {
        GoogleConfig {
            creds_file: String::new(),
            token_file: super::defaults::default_token_file(),
            spreadsheet_name: None,
            team_sheet: super::defaults::default_team_sheet(),
            team_perf_sheet: super::defaults::default_team_perf_sheet(),
            league_perf_sheet: super::defaults::default_league_perf_sheet(),
            scouting_sheet: super::defaults::default_scouting_sheet(),
        }
    }
}