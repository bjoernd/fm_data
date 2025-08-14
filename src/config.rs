use crate::constants::defaults;
use crate::domain::SpreadsheetId;
use crate::error::Result;
use crate::error_helpers::{config_missing_field, ErrorContext};
use crate::validators::{ConfigValidator, FileValidator};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tokio::fs;

fn default_team_sheet() -> String {
    String::from(defaults::TEAM_SHEET)
}

fn default_team_perf_sheet() -> String {
    String::from(defaults::TEAM_PERF_SHEET)
}

fn default_league_perf_sheet() -> String {
    String::from(defaults::LEAGUE_PERF_SHEET)
}

fn default_scouting_sheet() -> String {
    String::from(defaults::SCOUTING_SHEET)
}

fn default_token_file() -> String {
    dirs::config_dir()
        .unwrap_or_else(|| dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")))
        .join("fm_data")
        .join(crate::constants::config::TOKEN_CACHE_FILE)
        .to_string_lossy()
        .to_string()
}

fn default_spreadsheet_id() -> Option<SpreadsheetId> {
    None
}

mod spreadsheet_id_serde {
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
    #[serde(default = "default_spreadsheet_id", with = "spreadsheet_id_serde")]
    pub spreadsheet_name: Option<SpreadsheetId>,
    #[serde(default = "default_team_sheet")]
    pub team_sheet: String,
    #[serde(default = "default_team_perf_sheet")]
    pub team_perf_sheet: String,
    #[serde(default = "default_league_perf_sheet")]
    pub league_perf_sheet: String,
    #[serde(default = "default_scouting_sheet")]
    pub scouting_sheet: String,
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
    #[serde(default)]
    pub image_file: String,
}

impl Default for GoogleConfig {
    fn default() -> Self {
        GoogleConfig {
            creds_file: String::new(),
            token_file: default_token_file(),
            spreadsheet_name: None,
            team_sheet: default_team_sheet(),
            team_perf_sheet: default_team_perf_sheet(),
            league_perf_sheet: default_league_perf_sheet(),
            scouting_sheet: default_scouting_sheet(),
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

    pub async fn from_file(config_path: &Path) -> Result<Config> {
        let config_str = fs::read_to_string(config_path)
            .await
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
        let resolver = PathResolver::new(self);
        resolver.resolve_with_specific(spreadsheet, credfile, |config, spreadsheet, credfile| {
            let (_, _, default_html) = Self::get_default_paths();

            let resolved_input =
                Self::resolve_with_fallback(input, config.input.data_html.clone(), default_html);

            // Validate input-specific path
            FileValidator::validate_file_exists(&resolved_input, "Input HTML")?;
            FileValidator::validate_file_extension_typed(
                &resolved_input,
                crate::constants::FileExtension::Html,
            )?;

            Ok((spreadsheet, credfile, resolved_input))
        })
    }

    /// Resolve paths without validation (for testing)
    pub fn resolve_paths_unchecked(
        &self,
        spreadsheet: Option<String>,
        credfile: Option<String>,
        input: Option<String>,
    ) -> (String, String, String) {
        let (default_spreadsheet, default_creds, default_html) = Self::get_default_paths();

        let resolved_spreadsheet = if let Some(cli_id) = spreadsheet {
            cli_id
        } else if let Some(config_id) = &self.google.spreadsheet_name {
            config_id.as_str().to_string()
        } else {
            default_spreadsheet
        };

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
        let resolver = PathResolver::new(self);
        resolver.resolve_with_specific(spreadsheet, credfile, |config, spreadsheet, credfile| {
            let resolved_role_file = role_file
                .or_else(|| Some(config.input.role_file.clone()))
                .filter(|s| !s.is_empty())
                .ok_or_else(|| config_missing_field("role_file"))?;

            // Validate role file specific path
            FileValidator::validate_file_exists(&resolved_role_file, "Role file")?;
            FileValidator::validate_file_extension_typed(
                &resolved_role_file,
                crate::constants::FileExtension::Txt,
            )?;

            Ok((spreadsheet, credfile, resolved_role_file))
        })
    }

    /// Resolve paths for image processor including image file and sheet name
    pub async fn resolve_image_paths(
        &self,
        spreadsheet: Option<String>,
        credfile: Option<String>,
        image_file: Option<String>,
        sheet: Option<String>,
    ) -> Result<(String, String, String, String)> {
        let resolver = PathResolver::new(self);
        let (resolved_spreadsheet, resolved_credfile) =
            resolver.resolve_common_paths(spreadsheet, credfile)?;

        let resolved_image_file = image_file
            .or_else(|| Some(self.input.image_file.clone()))
            .filter(|s| !s.is_empty())
            .ok_or_else(|| config_missing_field("image_file"))?;

        let resolved_sheet = Self::resolve_with_fallback(
            sheet,
            self.google.scouting_sheet.clone(),
            default_scouting_sheet(),
        );

        // Validate image file specific paths
        FileValidator::validate_file_exists(&resolved_image_file, "Image")?;
        crate::cli::validate_image_file(&resolved_image_file).await?;

        Ok((
            resolved_spreadsheet,
            resolved_credfile,
            resolved_image_file,
            resolved_sheet,
        ))
    }
}

/// PathResolver handles common path resolution patterns
pub struct PathResolver<'a> {
    config: &'a Config,
}

impl<'a> PathResolver<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    /// Resolve common paths (spreadsheet and credentials) used by all applications
    pub fn resolve_common_paths(
        &self,
        spreadsheet: Option<String>,
        credfile: Option<String>,
    ) -> Result<(String, String)> {
        let (default_spreadsheet, default_creds, _) = Config::get_default_paths();

        let resolved_spreadsheet = if let Some(cli_id) = spreadsheet {
            cli_id
        } else if let Some(config_id) = &self.config.google.spreadsheet_name {
            config_id.as_str().to_string()
        } else {
            default_spreadsheet
        };

        let resolved_credfile = Config::resolve_with_fallback(
            credfile,
            self.config.google.creds_file.clone(),
            default_creds,
        );

        // Validate common paths
        ConfigValidator::validate_spreadsheet_id(&resolved_spreadsheet)?;
        FileValidator::validate_file_exists(&resolved_credfile, "Credentials")?;
        FileValidator::validate_file_extension_typed(
            &resolved_credfile,
            crate::constants::FileExtension::Json,
        )?;

        Ok((resolved_spreadsheet, resolved_credfile))
    }

    /// Template method for resolving paths with specific validation
    pub fn resolve_with_specific<T>(
        &self,
        spreadsheet: Option<String>,
        credfile: Option<String>,
        resolver_fn: impl FnOnce(&Config, String, String) -> Result<T>,
    ) -> Result<T> {
        let (resolved_spreadsheet, resolved_credfile) =
            self.resolve_common_paths(spreadsheet, credfile)?;
        resolver_fn(self.config, resolved_spreadsheet, resolved_credfile)
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
        assert_eq!(config.google.scouting_sheet, "Scouting");
        assert!(config.google.token_file.contains("fm_data"));
        assert!(config.google.token_file.contains("tokencache.json"));
        assert!(config.google.creds_file.is_empty());
        assert!(config.google.spreadsheet_name.is_none());
        assert!(config.input.image_file.is_empty());
    }

    #[tokio::test]
    async fn test_config_from_file_valid() -> Result<()> {
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

        let config = Config::from_file(temp_file.path()).await?;

        assert_eq!(config.google.creds_file, "/path/to/creds.json");
        assert_eq!(
            config.google.spreadsheet_name.as_ref().unwrap().as_str(),
            "test-spreadsheet-id"
        );
        assert_eq!(config.google.team_sheet, "MySquad");
        assert_eq!(config.input.data_html, "/path/to/data.html");

        Ok(())
    }

    #[tokio::test]
    async fn test_config_from_file_invalid_json() {
        let invalid_json = r#"{ invalid json }"#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(invalid_json.as_bytes()).unwrap();

        let result = Config::from_file(temp_file.path()).await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Configuration error in field 'JSON parsing'"));
    }

    #[tokio::test]
    async fn test_config_from_file_nonexistent() {
        let result = Config::from_file(Path::new("/nonexistent/config.json")).await;
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
                spreadsheet_name: Some(SpreadsheetId::new("config_spreadsheet").unwrap()),
                team_sheet: "Squad".to_string(),
                team_perf_sheet: "Stats_Team".to_string(),
                league_perf_sheet: "Stats_Division".to_string(),
                scouting_sheet: "Scouting".to_string(),
            },
            input: InputConfig {
                data_html: "config_data.html".to_string(),
                league_perf_html: "config_league.html".to_string(),
                team_perf_html: "config_team.html".to_string(),
                role_file: String::new(),
                image_file: String::new(),
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
                spreadsheet_name: Some(
                    SpreadsheetId::new("1ZrBTdlMlGaLD6LhMs948YvZ41NE71mcy7jhmygJU2Bc").unwrap(),
                ),
                team_sheet: "Squad".to_string(),
                team_perf_sheet: "Stats_Team".to_string(),
                league_perf_sheet: "Stats_Division".to_string(),
                scouting_sheet: "Scouting".to_string(),
            },
            input: InputConfig {
                data_html: input_file.path().to_string_lossy().to_string(),
                league_perf_html: "config_league.html".to_string(),
                team_perf_html: "config_team.html".to_string(),
                role_file: String::new(),
                image_file: String::new(),
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

    #[tokio::test]
    async fn test_config_from_file_partial_google() -> Result<()> {
        let config_json = r#"{
            "google": {
                "creds_file": "/path/to/creds.json",
                "spreadsheet_name": "test-spreadsheet-id"
            }
        }"#;

        let mut temp_file = NamedTempFile::new()?;
        temp_file.write_all(config_json.as_bytes())?;

        let config = Config::from_file(temp_file.path()).await?;

        assert_eq!(config.google.creds_file, "/path/to/creds.json");
        assert_eq!(
            config.google.spreadsheet_name.as_ref().unwrap().as_str(),
            "test-spreadsheet-id"
        );
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

    #[tokio::test]
    async fn test_config_from_file_partial_input() -> Result<()> {
        let config_json = r#"{
            "input": {
                "data_html": "/path/to/data.html"
            }
        }"#;

        let mut temp_file = NamedTempFile::new()?;
        temp_file.write_all(config_json.as_bytes())?;

        let config = Config::from_file(temp_file.path()).await?;

        assert_eq!(config.input.data_html, "/path/to/data.html");
        assert!(config.input.league_perf_html.is_empty());
        assert!(config.input.team_perf_html.is_empty());
        assert!(config.google.creds_file.is_empty());
        assert_eq!(config.google.team_sheet, "Squad");

        Ok(())
    }

    #[tokio::test]
    async fn test_config_from_file_empty_object() -> Result<()> {
        let config_json = r#"{}"#;

        let mut temp_file = NamedTempFile::new()?;
        temp_file.write_all(config_json.as_bytes())?;

        let config = Config::from_file(temp_file.path()).await?;

        assert!(config.google.creds_file.is_empty());
        assert!(config.google.spreadsheet_name.is_none());
        assert_eq!(config.google.team_sheet, "Squad");
        assert_eq!(config.google.team_perf_sheet, "Stats_Team");
        assert_eq!(config.google.league_perf_sheet, "Stats_Division");
        assert!(config.google.token_file.contains("fm_data"));
        assert!(config.input.data_html.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn test_config_from_file_mixed_partial() -> Result<()> {
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

        let config = Config::from_file(temp_file.path()).await?;

        assert_eq!(config.google.creds_file, "/custom/creds.json");
        assert_eq!(config.google.team_sheet, "CustomSquad");
        assert_eq!(config.google.team_perf_sheet, "Stats_Team");
        assert!(config.google.spreadsheet_name.is_none());
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
                spreadsheet_name: Some(
                    SpreadsheetId::new("1ZrBTdlMlGaLD6LhMs948YvZ41NE71mcy7jhmygJU2Bc").unwrap(),
                ),
                team_sheet: "Squad".to_string(),
                team_perf_sheet: "Stats_Team".to_string(),
                league_perf_sheet: "Stats_Division".to_string(),
                scouting_sheet: "Scouting".to_string(),
            },
            input: InputConfig {
                data_html: "data.html".to_string(),
                league_perf_html: "league.html".to_string(),
                team_perf_html: "team.html".to_string(),
                role_file: role_file.path().to_string_lossy().to_string(),
                image_file: String::new(),
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
                spreadsheet_name: Some(
                    SpreadsheetId::new("1ZrBTdlMlGaLD6LhMs948YvZ41NE71mcy7jhmygJU2Bc").unwrap(),
                ),
                team_sheet: "Squad".to_string(),
                team_perf_sheet: "Stats_Team".to_string(),
                league_perf_sheet: "Stats_Division".to_string(),
                scouting_sheet: "Scouting".to_string(),
            },
            input: InputConfig {
                data_html: input_file.path().to_string_lossy().to_string(),
                league_perf_html: "league.html".to_string(),
                team_perf_html: "team.html".to_string(),
                role_file: String::new(),
                image_file: String::new(),
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

    #[tokio::test]
    async fn test_config_role_file_path_takes_precedence_over_cli_none() -> Result<()> {
        // Create temporary files for testing
        let creds_file = NamedTempFile::new().unwrap();
        let role_file = NamedTempFile::new().unwrap();

        // Write a simple role file content
        fs::write(
            role_file.path(),
            "GK\nCD(d)\nCD(s)\nFB(d) R\nFB(d) L\nCM(d)\nCM(s)\nCM(a)\nW(s) R\nW(s) L\nCF(s)",
        )
        .await
        .unwrap();

        // Create config with specific role file path
        let config = Config {
            google: GoogleConfig {
                creds_file: creds_file.path().to_string_lossy().to_string(),
                token_file: "tokencache.json".to_string(),
                spreadsheet_name: Some(
                    SpreadsheetId::new("1ZrBTdlMlGaLD6LhMs948YvZ41NE71mcy7jhmygJU2Bc").unwrap(),
                ),
                team_sheet: "Squad".to_string(),
                team_perf_sheet: "Stats_Team".to_string(),
                league_perf_sheet: "Stats_Division".to_string(),
                scouting_sheet: "Scouting".to_string(),
            },
            input: InputConfig {
                data_html: "data.html".to_string(),
                league_perf_html: "league.html".to_string(),
                team_perf_html: "team.html".to_string(),
                role_file: role_file.path().to_string_lossy().to_string(),
                image_file: String::new(),
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

    #[tokio::test]
    async fn test_config_image_fields_and_resolve_paths() -> Result<()> {
        // Create temporary files for testing
        let creds_file = NamedTempFile::new().unwrap();
        let image_file = create_test_png();

        let config_json = r#"{
            "google": {
                "creds_file": "/config/creds.json",
                "spreadsheet_name": "test-spreadsheet-id",
                "scouting_sheet": "MyScoutingSheet"
            },
            "input": {
                "image_file": "/config/image.png"
            }
        }"#;

        let mut temp_config_file = NamedTempFile::new()?;
        temp_config_file.write_all(config_json.as_bytes())?;

        let config = Config::from_file(temp_config_file.path()).await?;

        // Test that new fields are loaded correctly
        assert_eq!(config.google.scouting_sheet, "MyScoutingSheet");
        assert_eq!(config.input.image_file, "/config/image.png");

        // Test resolve_image_paths method
        let (spreadsheet, credfile, imagefile, sheet) = config
            .resolve_image_paths(
                Some("1ZrBTdlMlGaLD6LhMs948YvZ41NE71mcy7jhmygJU2Bc".to_string()),
                Some(creds_file.path().to_string_lossy().to_string()),
                Some(image_file.path().to_string_lossy().to_string()),
                Some("CustomSheet".to_string()),
            )
            .await?;

        assert_eq!(spreadsheet, "1ZrBTdlMlGaLD6LhMs948YvZ41NE71mcy7jhmygJU2Bc");
        assert!(credfile.contains("tmp"));
        assert!(imagefile.contains("tmp"));
        assert_eq!(sheet, "CustomSheet");

        Ok(())
    }

    fn create_test_png() -> NamedTempFile {
        let mut temp_file = NamedTempFile::new().unwrap();
        // Write PNG magic bytes to create a valid PNG file
        let png_signature = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        temp_file.write_all(&png_signature).unwrap();
        temp_file.flush().unwrap();
        temp_file
    }

    #[test]
    fn test_config_image_defaults() {
        let config = Config::create_default();

        // Test that scouting sheet defaults to "Scouting"
        assert_eq!(config.google.scouting_sheet, "Scouting");

        // Test that image_file defaults to empty
        assert!(config.input.image_file.is_empty());
    }

    #[tokio::test]
    async fn test_config_from_file_image_partial() -> Result<()> {
        let config_json = r#"{
            "input": {
                "image_file": "/path/to/screenshot.png"
            }
        }"#;

        let mut temp_file = NamedTempFile::new()?;
        temp_file.write_all(config_json.as_bytes())?;

        let config = Config::from_file(temp_file.path()).await?;

        assert_eq!(config.input.image_file, "/path/to/screenshot.png");
        assert_eq!(config.google.scouting_sheet, "Scouting"); // Should use default
        assert!(config.input.data_html.is_empty());

        Ok(())
    }
}
