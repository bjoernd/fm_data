use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

fn default_team_sheet() -> String {
    String::from("Squad")
}

fn default_team_perf_sheet() -> String {
    String::from("Stats_Team")
}

fn default_league_perf_sheet() -> String {
    String::from("Stats_Division")
}

fn default_token_file() -> String {
    dirs::config_dir()
        .unwrap_or_else(|| dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")))
        .join("fm_data")
        .join("tokencache.json")
        .to_string_lossy()
        .to_string()
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Config {
    #[serde(default)]
    pub google: GoogleConfig,
    #[serde(default)]
    pub input: InputConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GoogleConfig {
    #[serde(default)]
    pub creds_file: String,
    #[serde(default = "default_token_file")]
    pub token_file: String,
    #[serde(default)]
    pub spreadsheet_name: String,
    #[serde(default = "default_team_sheet")]
    pub team_sheet: String,
    #[serde(default = "default_team_perf_sheet")]
    pub team_perf_sheet: String,
    #[serde(default = "default_league_perf_sheet")]
    pub league_perf_sheet: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct InputConfig {
    #[serde(default)]
    pub data_html: String,
    #[serde(default)]
    pub league_perf_html: String,
    #[serde(default)]
    pub team_perf_html: String,
}

impl Default for GoogleConfig {
    fn default() -> Self {
        GoogleConfig {
            creds_file: String::new(),
            token_file: default_token_file(),
            spreadsheet_name: String::new(),
            team_sheet: default_team_sheet(),
            team_perf_sheet: default_team_perf_sheet(),
            league_perf_sheet: default_league_perf_sheet(),
        }
    }
}

impl Config {
    pub fn from_file(config_path: &Path) -> Result<Config> {
        let config_str = fs::read_to_string(config_path)
            .with_context(|| format!("Failed to read config file: {}", config_path.display()))?;

        let config: Config =
            serde_json::from_str(&config_str).with_context(|| "Failed to parse config JSON")?;

        Ok(config)
    }

    pub fn create_default() -> Config {
        Config::default()
    }

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

    pub fn resolve_paths(
        &self,
        spreadsheet: Option<String>,
        credfile: Option<String>,
        input: Option<String>,
    ) -> (String, String, String) {
        let (default_spreadsheet, default_creds, default_html) = Self::get_default_paths();

        let resolved_spreadsheet = spreadsheet
            .or_else(|| Some(self.google.spreadsheet_name.clone()))
            .filter(|s| !s.is_empty())
            .unwrap_or(default_spreadsheet);

        let resolved_credfile = credfile
            .or_else(|| Some(self.google.creds_file.clone()))
            .filter(|s| !s.is_empty())
            .unwrap_or(default_creds);

        let resolved_input = input
            .or_else(|| Some(self.input.data_html.clone()))
            .filter(|s| !s.is_empty())
            .unwrap_or(default_html);

        (resolved_spreadsheet, resolved_credfile, resolved_input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_config_default() {
        let config = Config::create_default();
        assert_eq!(config.google.team_sheet, "Squad");
        assert_eq!(config.google.team_perf_sheet, "Stats_Team");
        assert_eq!(config.google.league_perf_sheet, "Stats_Division");
        assert!(config.google.token_file.contains("fm_data"));
        assert!(config.google.token_file.contains("tokencache.json"));
        assert!(config.google.creds_file.is_empty());
        assert!(config.google.spreadsheet_name.is_empty());
    }

    #[test]
    fn test_config_from_file_valid() -> Result<()> {
        let config_json = r#"{
            "google": {
                "creds_file": "/path/to/creds.json",
                "token_file": "/path/to/token.json",
                "spreadsheet_name": "test-spreadsheet-id",
                "team_sheet": "MySquad",
                "team_perf_sheet": "MyStats",
                "league_perf_sheet": "MyLeague"
            },
            "input": {
                "data_html": "/path/to/data.html",
                "league_perf_html": "/path/to/league.html",
                "team_perf_html": "/path/to/team.html"
            }
        }"#;

        let mut temp_file = NamedTempFile::new()?;
        temp_file.write_all(config_json.as_bytes())?;

        let config = Config::from_file(temp_file.path())?;

        assert_eq!(config.google.creds_file, "/path/to/creds.json");
        assert_eq!(config.google.spreadsheet_name, "test-spreadsheet-id");
        assert_eq!(config.google.team_sheet, "MySquad");
        assert_eq!(config.input.data_html, "/path/to/data.html");

        Ok(())
    }

    #[test]
    fn test_config_from_file_invalid_json() {
        let invalid_json = r#"{ invalid json }"#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(invalid_json.as_bytes()).unwrap();

        let result = Config::from_file(temp_file.path());
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Failed to parse config JSON"));
    }

    #[test]
    fn test_config_from_file_nonexistent() {
        let result = Config::from_file(Path::new("/nonexistent/config.json"));
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Failed to read config file"));
    }

    #[test]
    fn test_resolve_paths_cli_overrides() {
        let config = Config {
            google: GoogleConfig {
                creds_file: "config_creds.json".to_string(),
                token_file: "tokencache.json".to_string(),
                spreadsheet_name: "config_spreadsheet".to_string(),
                team_sheet: "Squad".to_string(),
                team_perf_sheet: "Stats_Team".to_string(),
                league_perf_sheet: "Stats_Division".to_string(),
            },
            input: InputConfig {
                data_html: "config_data.html".to_string(),
                league_perf_html: "config_league.html".to_string(),
                team_perf_html: "config_team.html".to_string(),
            },
        };

        let (spreadsheet, credfile, input) = config.resolve_paths(
            Some("cli_spreadsheet".to_string()),
            Some("cli_creds.json".to_string()),
            Some("cli_data.html".to_string()),
        );

        assert_eq!(spreadsheet, "cli_spreadsheet");
        assert_eq!(credfile, "cli_creds.json");
        assert_eq!(input, "cli_data.html");
    }

    #[test]
    fn test_resolve_paths_config_fallback() {
        let config = Config {
            google: GoogleConfig {
                creds_file: "config_creds.json".to_string(),
                token_file: "tokencache.json".to_string(),
                spreadsheet_name: "config_spreadsheet".to_string(),
                team_sheet: "Squad".to_string(),
                team_perf_sheet: "Stats_Team".to_string(),
                league_perf_sheet: "Stats_Division".to_string(),
            },
            input: InputConfig {
                data_html: "config_data.html".to_string(),
                league_perf_html: "config_league.html".to_string(),
                team_perf_html: "config_team.html".to_string(),
            },
        };

        let (spreadsheet, credfile, input) = config.resolve_paths(None, None, None);

        assert_eq!(spreadsheet, "config_spreadsheet");
        assert_eq!(credfile, "config_creds.json");
        assert_eq!(input, "config_data.html");
    }

    #[test]
    fn test_resolve_paths_default_fallback() {
        let config = Config::create_default();
        let (spreadsheet, credfile, input) = config.resolve_paths(None, None, None);

        // Should fall back to defaults when config values are empty
        assert_eq!(spreadsheet, "1ZrBTdlMlGaLD6LhMs948YvZ41NE71mcy7jhmygJU2Bc");
        assert!(credfile.contains("fm_data"));
        assert!(credfile.contains("credentials.json"));
        assert!(input.contains("bd.html"));
    }

    #[test]
    fn test_get_default_paths() {
        let (spreadsheet, credfile, input) = Config::get_default_paths();

        assert_eq!(spreadsheet, "1ZrBTdlMlGaLD6LhMs948YvZ41NE71mcy7jhmygJU2Bc");
        assert!(credfile.contains("fm_data"));
        assert!(credfile.ends_with("credentials.json"));
        assert!(input.contains("Football Manager 2024"));
        assert!(input.ends_with("bd.html"));
    }

    #[test]
    fn test_config_from_file_partial_google() -> Result<()> {
        let config_json = r#"{
            "google": {
                "creds_file": "/path/to/creds.json",
                "spreadsheet_name": "test-spreadsheet-id"
            }
        }"#;

        let mut temp_file = NamedTempFile::new()?;
        temp_file.write_all(config_json.as_bytes())?;

        let config = Config::from_file(temp_file.path())?;

        assert_eq!(config.google.creds_file, "/path/to/creds.json");
        assert_eq!(config.google.spreadsheet_name, "test-spreadsheet-id");
        assert_eq!(config.google.team_sheet, "Squad");
        assert_eq!(config.google.team_perf_sheet, "Stats_Team");
        assert_eq!(config.google.league_perf_sheet, "Stats_Division");
        assert!(
            config.google.token_file.contains("fm_data")
                || config.google.token_file.contains("tokencache.json")
        );
        assert!(config.input.data_html.is_empty());

        Ok(())
    }

    #[test]
    fn test_config_from_file_partial_input() -> Result<()> {
        let config_json = r#"{
            "input": {
                "data_html": "/path/to/data.html"
            }
        }"#;

        let mut temp_file = NamedTempFile::new()?;
        temp_file.write_all(config_json.as_bytes())?;

        let config = Config::from_file(temp_file.path())?;

        assert_eq!(config.input.data_html, "/path/to/data.html");
        assert!(config.input.league_perf_html.is_empty());
        assert!(config.input.team_perf_html.is_empty());
        assert!(config.google.creds_file.is_empty());
        assert_eq!(config.google.team_sheet, "Squad");

        Ok(())
    }

    #[test]
    fn test_config_from_file_empty_object() -> Result<()> {
        let config_json = r#"{}"#;

        let mut temp_file = NamedTempFile::new()?;
        temp_file.write_all(config_json.as_bytes())?;

        let config = Config::from_file(temp_file.path())?;

        assert!(config.google.creds_file.is_empty());
        assert!(config.google.spreadsheet_name.is_empty());
        assert_eq!(config.google.team_sheet, "Squad");
        assert_eq!(config.google.team_perf_sheet, "Stats_Team");
        assert_eq!(config.google.league_perf_sheet, "Stats_Division");
        assert!(config.google.token_file.contains("fm_data"));
        assert!(config.input.data_html.is_empty());

        Ok(())
    }

    #[test]
    fn test_config_from_file_mixed_partial() -> Result<()> {
        let config_json = r#"{
            "google": {
                "creds_file": "/custom/creds.json",
                "team_sheet": "CustomSquad"
            },
            "input": {
                "league_perf_html": "/custom/league.html"
            }
        }"#;

        let mut temp_file = NamedTempFile::new()?;
        temp_file.write_all(config_json.as_bytes())?;

        let config = Config::from_file(temp_file.path())?;

        assert_eq!(config.google.creds_file, "/custom/creds.json");
        assert_eq!(config.google.team_sheet, "CustomSquad");
        assert_eq!(config.google.team_perf_sheet, "Stats_Team");
        assert!(config.google.spreadsheet_name.is_empty());
        assert_eq!(config.input.league_perf_html, "/custom/league.html");
        assert!(config.input.data_html.is_empty());
        assert!(config.input.team_perf_html.is_empty());

        Ok(())
    }
}
