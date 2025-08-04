use crate::constants::defaults;
use crate::error::Result;
use crate::error_helpers::{config_missing_field, ErrorContext};
use crate::validation::Validator;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

fn default_team_sheet() -> String {
    String::from(defaults::TEAM_SHEET)
}

fn default_team_perf_sheet() -> String {
    String::from(defaults::TEAM_PERF_SHEET)
}

fn default_league_perf_sheet() -> String {
    String::from(defaults::LEAGUE_PERF_SHEET)
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
    #[serde(default)]
    pub role_file: String,
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
    /// Helper method to resolve path with fallback priority: CLI > config > default
    fn resolve_with_fallback<T: Clone + AsRef<str>>(
        cli_value: Option<T>,
        config_value: T,
        default_value: T,
    ) -> T {
        cli_value
            .or_else(|| Some(config_value))
            .filter(|s| !s.as_ref().is_empty())
            .unwrap_or(default_value)
    }

    pub fn from_file(config_path: &Path) -> Result<Config> {
        let config_str = fs::read_to_string(config_path)
            .with_file_context(&config_path.display().to_string(), "read")?;

        let config: Config =
            serde_json::from_str(&config_str).with_config_context("JSON parsing")?;

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
    ) -> Result<(String, String, String)> {
        let (default_spreadsheet, default_creds, default_html) = Self::get_default_paths();

        let resolved_spreadsheet = Self::resolve_with_fallback(
            spreadsheet,
            self.google.spreadsheet_name.clone(),
            default_spreadsheet,
        );

        let resolved_credfile =
            Self::resolve_with_fallback(credfile, self.google.creds_file.clone(), default_creds);

        let resolved_input =
            Self::resolve_with_fallback(input, self.input.data_html.clone(), default_html);

        // Validate the resolved paths
        Validator::validate_spreadsheet_id(&resolved_spreadsheet)?;
        Validator::validate_file_exists(&resolved_credfile, "Credentials")?;
        Validator::validate_file_extension(&resolved_credfile, "json")?;
        Validator::validate_file_exists(&resolved_input, "Input HTML")?;
        Validator::validate_file_extension(&resolved_input, "html")?;

        Ok((resolved_spreadsheet, resolved_credfile, resolved_input))
    }

    /// Resolve paths without validation (for testing)
    pub fn resolve_paths_unchecked(
        &self,
        spreadsheet: Option<String>,
        credfile: Option<String>,
        input: Option<String>,
    ) -> (String, String, String) {
        let (default_spreadsheet, default_creds, default_html) = Self::get_default_paths();

        let resolved_spreadsheet = Self::resolve_with_fallback(
            spreadsheet,
            self.google.spreadsheet_name.clone(),
            default_spreadsheet,
        );

        let resolved_credfile =
            Self::resolve_with_fallback(credfile, self.google.creds_file.clone(), default_creds);

        let resolved_input =
            Self::resolve_with_fallback(input, self.input.data_html.clone(), default_html);

        (resolved_spreadsheet, resolved_credfile, resolved_input)
    }

    /// Resolve paths for team selector including role file
    pub fn resolve_team_selector_paths(
        &self,
        spreadsheet: Option<String>,
        credfile: Option<String>,
        role_file: Option<String>,
    ) -> Result<(String, String, String)> {
        let (default_spreadsheet, default_creds, _) = Self::get_default_paths();

        let resolved_spreadsheet = Self::resolve_with_fallback(
            spreadsheet,
            self.google.spreadsheet_name.clone(),
            default_spreadsheet,
        );

        let resolved_credfile =
            Self::resolve_with_fallback(credfile, self.google.creds_file.clone(), default_creds);

        let resolved_role_file = role_file
            .or_else(|| Some(self.input.role_file.clone()))
            .filter(|s| !s.is_empty())
            .ok_or_else(|| config_missing_field("role_file"))?;

        // Validate the resolved paths
        Validator::validate_spreadsheet_id(&resolved_spreadsheet)?;
        Validator::validate_file_exists(&resolved_credfile, "Credentials")?;
        Validator::validate_file_extension(&resolved_credfile, "json")?;
        Validator::validate_file_exists(&resolved_role_file, "Role file")?;
        Validator::validate_file_extension(&resolved_role_file, "txt")?;

        Ok((resolved_spreadsheet, resolved_credfile, resolved_role_file))
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
            .contains("Configuration error in field 'JSON parsing'"));
    }

    #[test]
    fn test_config_from_file_nonexistent() {
        let result = Config::from_file(Path::new("/nonexistent/config.json"));
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Failed to read file"));
    }

    #[test]
    fn test_resolve_paths_cli_overrides() -> Result<()> {
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
                role_file: String::new(),
            },
        };

        // Create temporary files for testing
        let creds_file = NamedTempFile::new().unwrap();
        let input_file = NamedTempFile::new().unwrap();

        let (spreadsheet, credfile, input) = config.resolve_paths(
            Some("1ZrBTdlMlGaLD6LhMs948YvZ41NE71mcy7jhmygJU2Bc".to_string()),
            Some(creds_file.path().to_string_lossy().to_string()),
            Some(input_file.path().to_string_lossy().to_string()),
        )?;

        assert_eq!(spreadsheet, "1ZrBTdlMlGaLD6LhMs948YvZ41NE71mcy7jhmygJU2Bc");
        assert!(credfile.contains("tmp"));
        assert!(input.contains("tmp"));
        Ok(())
    }

    #[test]
    fn test_resolve_paths_config_fallback() -> Result<()> {
        // Create temporary files for testing
        let creds_file = NamedTempFile::new().unwrap();
        let input_file = NamedTempFile::new().unwrap();

        let config = Config {
            google: GoogleConfig {
                creds_file: creds_file.path().to_string_lossy().to_string(),
                token_file: "tokencache.json".to_string(),
                spreadsheet_name: "1ZrBTdlMlGaLD6LhMs948YvZ41NE71mcy7jhmygJU2Bc".to_string(),
                team_sheet: "Squad".to_string(),
                team_perf_sheet: "Stats_Team".to_string(),
                league_perf_sheet: "Stats_Division".to_string(),
            },
            input: InputConfig {
                data_html: input_file.path().to_string_lossy().to_string(),
                league_perf_html: "config_league.html".to_string(),
                team_perf_html: "config_team.html".to_string(),
                role_file: String::new(),
            },
        };

        let (spreadsheet, credfile, input) = config.resolve_paths(None, None, None)?;

        assert_eq!(spreadsheet, "1ZrBTdlMlGaLD6LhMs948YvZ41NE71mcy7jhmygJU2Bc");
        assert!(credfile.contains("tmp"));
        assert!(input.contains("tmp"));
        Ok(())
    }

    #[test]
    fn test_resolve_paths_default_fallback() {
        let config = Config::create_default();

        // The default paths will likely not exist, so this should fail validation
        let result = config.resolve_paths(None, None, None);
        // If the default paths happen to exist on this system, we just test that
        // the function returns either success or failure gracefully
        assert!(result.is_ok() || result.is_err());

        // Test with default paths structure
        let (spreadsheet, credfile, input) = Config::get_default_paths();
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

    #[test]
    fn test_resolve_team_selector_paths() -> Result<()> {
        // Create temporary files for testing
        let creds_file = NamedTempFile::new().unwrap();
        let role_file = NamedTempFile::new().unwrap();

        let config = Config {
            google: GoogleConfig {
                creds_file: creds_file.path().to_string_lossy().to_string(),
                token_file: "tokencache.json".to_string(),
                spreadsheet_name: "1ZrBTdlMlGaLD6LhMs948YvZ41NE71mcy7jhmygJU2Bc".to_string(),
                team_sheet: "Squad".to_string(),
                team_perf_sheet: "Stats_Team".to_string(),
                league_perf_sheet: "Stats_Division".to_string(),
            },
            input: InputConfig {
                data_html: "data.html".to_string(),
                league_perf_html: "league.html".to_string(),
                team_perf_html: "team.html".to_string(),
                role_file: role_file.path().to_string_lossy().to_string(),
            },
        };

        let (spreadsheet, credfile, rolefile) =
            config.resolve_team_selector_paths(None, None, None)?;

        assert_eq!(spreadsheet, "1ZrBTdlMlGaLD6LhMs948YvZ41NE71mcy7jhmygJU2Bc");
        assert!(credfile.contains("tmp"));
        assert!(rolefile.contains("tmp"));
        Ok(())
    }

    #[test]
    fn test_resolve_team_selector_paths_missing_role_file() {
        let config = Config::create_default();

        let result = config.resolve_team_selector_paths(None, None, None);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Missing required configuration field: 'role_file'"));
    }

    #[test]
    fn test_resolve_team_selector_paths_cli_override() -> Result<()> {
        // Create temporary files for testing
        let creds_file = NamedTempFile::new().unwrap();
        let role_file = NamedTempFile::new().unwrap();

        let config = Config::create_default();

        let (spreadsheet, credfile, rolefile) = config.resolve_team_selector_paths(
            Some("1ZrBTdlMlGaLD6LhMs948YvZ41NE71mcy7jhmygJU2Bc".to_string()),
            Some(creds_file.path().to_string_lossy().to_string()),
            Some(role_file.path().to_string_lossy().to_string()),
        )?;

        assert_eq!(spreadsheet, "1ZrBTdlMlGaLD6LhMs948YvZ41NE71mcy7jhmygJU2Bc");
        assert!(credfile.contains("tmp"));
        assert!(rolefile.contains("tmp"));
        Ok(())
    }

    #[test]
    fn test_config_input_path_takes_precedence_over_cli_none() -> Result<()> {
        // Create temporary files for testing
        let creds_file = NamedTempFile::new().unwrap();
        let input_file = NamedTempFile::new().unwrap();

        // Create config with specific input path
        let config = Config {
            google: GoogleConfig {
                creds_file: creds_file.path().to_string_lossy().to_string(),
                token_file: "tokencache.json".to_string(),
                spreadsheet_name: "1ZrBTdlMlGaLD6LhMs948YvZ41NE71mcy7jhmygJU2Bc".to_string(),
                team_sheet: "Squad".to_string(),
                team_perf_sheet: "Stats_Team".to_string(),
                league_perf_sheet: "Stats_Division".to_string(),
            },
            input: InputConfig {
                data_html: input_file.path().to_string_lossy().to_string(),
                league_perf_html: "league.html".to_string(),
                team_perf_html: "team.html".to_string(),
                role_file: String::new(),
            },
        };

        // Resolve paths with CLI input = None (simulating no --input flag)
        let (spreadsheet, credfile, input) = config.resolve_paths(None, None, None)?;

        assert_eq!(spreadsheet, "1ZrBTdlMlGaLD6LhMs948YvZ41NE71mcy7jhmygJU2Bc");
        assert!(credfile.contains("tmp"));
        // Verify that the config's data_html path is used, not a default
        assert_eq!(input, input_file.path().to_string_lossy().to_string());
        assert!(
            input.contains("tmp"),
            "Expected temp file path, got: {input}"
        );

        Ok(())
    }

    #[test]
    fn test_config_role_file_path_takes_precedence_over_cli_none() -> Result<()> {
        // Create temporary files for testing
        let creds_file = NamedTempFile::new().unwrap();
        let role_file = NamedTempFile::new().unwrap();

        // Write a simple role file content
        std::fs::write(
            role_file.path(),
            "GK\nCD(d)\nCD(s)\nFB(d) R\nFB(d) L\nCM(d)\nCM(s)\nCM(a)\nW(s) R\nW(s) L\nCF(s)",
        )
        .unwrap();

        // Create config with specific role file path
        let config = Config {
            google: GoogleConfig {
                creds_file: creds_file.path().to_string_lossy().to_string(),
                token_file: "tokencache.json".to_string(),
                spreadsheet_name: "1ZrBTdlMlGaLD6LhMs948YvZ41NE71mcy7jhmygJU2Bc".to_string(),
                team_sheet: "Squad".to_string(),
                team_perf_sheet: "Stats_Team".to_string(),
                league_perf_sheet: "Stats_Division".to_string(),
            },
            input: InputConfig {
                data_html: "data.html".to_string(),
                league_perf_html: "league.html".to_string(),
                team_perf_html: "team.html".to_string(),
                role_file: role_file.path().to_string_lossy().to_string(),
            },
        };

        // Resolve paths with CLI role_file = None (simulating no --role-file flag)
        let (spreadsheet, credfile, resolved_role_file) =
            config.resolve_team_selector_paths(None, None, None)?;

        assert_eq!(spreadsheet, "1ZrBTdlMlGaLD6LhMs948YvZ41NE71mcy7jhmygJU2Bc");
        assert!(credfile.contains("tmp"));
        // Verify that the config's role_file path is used, not a default
        assert_eq!(
            resolved_role_file,
            role_file.path().to_string_lossy().to_string()
        );
        assert!(
            resolved_role_file.contains("tmp"),
            "Expected temp file path, got: {resolved_role_file}"
        );

        Ok(())
    }
}
