use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub google: GoogleConfig,
    pub input: InputConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GoogleConfig {
    pub creds_file: String,
    pub token_file: String,
    pub spreadsheet_name: String,
    pub team_sheet: String,
    pub team_perf_sheet: String,
    pub league_perf_sheet: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InputConfig {
    pub data_html: String,
    pub league_perf_html: String,
    pub team_perf_html: String,
}

impl Config {
    pub fn from_file(config_path: &Path) -> Result<Config> {
        let config_str = fs::read_to_string(config_path)
            .with_context(|| format!("Failed to read config file: {}", config_path.display()))?;

        let config: Config =
            serde_json::from_str(&config_str).with_context(|| "Failed to parse config JSON")?;

        Ok(config)
    }

    pub fn default() -> Config {
        Config {
            google: GoogleConfig {
                creds_file: String::new(),
                token_file: String::from("tokencache.json"),
                spreadsheet_name: String::new(),
                team_sheet: String::from("Squad"),
                team_perf_sheet: String::from("Stats_Team"),
                league_perf_sheet: String::from("Stats_Division"),
            },
            input: InputConfig {
                data_html: String::new(),
                league_perf_html: String::new(),
                team_perf_html: String::new(),
            },
        }
    }

    pub fn get_default_paths() -> (String, String, String) {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));

        let default_spreadsheet = String::from("1ZrBTdlMlGaLD6LhMs948YvZ41NE71mcy7jhmygJU2Bc");

        let default_creds = home
            .join("Downloads")
            .join("client_secret.json")
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

    pub fn resolve_paths(&self, spreadsheet: Option<String>, credfile: Option<String>, input: Option<String>) -> (String, String, String) {
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