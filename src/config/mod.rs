/*!
Configuration management with hierarchical priority and path resolution.

This module provides configuration loading from JSON files with fallback to defaults,
and path resolution logic that follows the priority: CLI > config file > defaults.

# Module Organization

- [`types`]: Core configuration data structures
- [`defaults`]: Default value functions for configuration fields  
- [`paths`]: Path resolution logic with validation

# Example

```rust
use fm_data::config::Config;
use std::path::Path;

# async fn example() -> anyhow::Result<()> {
// Load configuration from file
let config = Config::from_file(Path::new("config.json")).await?;

// Resolve paths with CLI overrides
let (spreadsheet, creds, input) = config.resolve_paths(
    Some("spreadsheet_id".to_string()),
    None, // Use config file value
    None, // Use config file value
)?;
# Ok(())
# }
```
*/

pub mod defaults;
pub mod paths;
pub mod types;

use crate::error::Result;
use crate::error_helpers::ErrorContext;
use std::path::Path;
use tokio::fs;

// Re-export main types for convenience
pub use paths::{PathResolver, resolve_with_fallback};
pub use types::{Config, GoogleConfig, InputConfig};

impl Config {
    /// Load configuration from a JSON file with comprehensive error handling
    /// 
    /// This method reads the configuration file asynchronously and deserializes it
    /// from JSON. Missing fields are automatically filled with default values
    /// through serde's `#[serde(default)]` attributes.
    /// 
    /// # Arguments
    /// 
    /// * `config_path` - Path to the JSON configuration file
    /// 
    /// # Returns
    /// 
    /// A `Result` containing the loaded configuration or an error with context
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// # use fm_data::config::Config;
    /// # use std::path::Path;
    /// # tokio_test::block_on(async {
    /// let config = Config::from_file(Path::new("config.json")).await?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn from_file(config_path: &Path) -> Result<Config> {
        let config_str = fs::read_to_string(config_path)
            .await
            .with_file_context(&config_path.display().to_string(), "read")?;

        let config: Config =
            serde_json::from_str(&config_str).with_config_context("JSON parsing")?;

        Ok(config)
    }

    /// Create a default configuration with all fields set to their default values
    /// 
    /// This is equivalent to `Config::default()` but provides a more explicit
    /// method name that clearly indicates the intent to create a configuration
    /// with default values.
    /// 
    /// # Returns
    /// 
    /// A new `Config` instance with default values for all fields
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// # use fm_data::config::Config;
    /// let config = Config::create_default();
    /// assert_eq!(config.google.team_sheet, "Squad");
    /// ```
    pub fn create_default() -> Config {
        Config::default()
    }

    /// Get the default paths used when no configuration is provided
    /// 
    /// This method returns the hardcoded default values for the three most
    /// commonly needed paths: spreadsheet ID, credentials file, and HTML input file.
    /// These defaults are used as the final fallback in the path resolution hierarchy.
    /// 
    /// # Returns
    /// 
    /// A tuple containing `(spreadsheet_id, credentials_path, html_input_path)`
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// # use fm_data::config::Config;
    /// let (spreadsheet, creds, html) = Config::get_default_paths();
    /// assert!(spreadsheet.len() > 0);
    /// assert!(creds.contains("credentials.json"));
    /// assert!(html.contains("bd.html"));
    /// ```
    pub fn get_default_paths() -> (String, String, String) {
        defaults::get_default_paths()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::SpreadsheetId;
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
    fn test_resolve_paths_cli_overrides() -> Result<()> {
        let config = Config {
            google: types::GoogleConfig {
                creds_file: "config_creds.json".to_string(),
                token_file: "tokencache.json".to_string(),
                spreadsheet_name: Some(SpreadsheetId::new("config_spreadsheet").unwrap()),
                team_sheet: "Squad".to_string(),
                team_perf_sheet: "Stats_Team".to_string(),
                league_perf_sheet: "Stats_Division".to_string(),
                scouting_sheet: "Scouting".to_string(),
            },
            input: types::InputConfig {
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
}