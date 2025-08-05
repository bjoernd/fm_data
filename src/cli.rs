use crate::error::{FMDataError, Result};
use crate::validators::ConfigValidator;
use clap::Parser;

pub struct CommonArgs {
    pub config_file: String,
    pub spreadsheet_id: Option<String>,
    pub creds_file: Option<String>,
    pub verbose: bool,
    pub no_progress: bool,
}

impl CommonArgs {
    pub fn new(
        config_file: String,
        spreadsheet_id: Option<String>,
        creds_file: Option<String>,
        verbose: bool,
        no_progress: bool,
    ) -> Self {
        Self {
            config_file,
            spreadsheet_id,
            creds_file,
            verbose,
            no_progress,
        }
    }
}

pub trait CommonCLIArgs {
    fn get_common_args(&self) -> CommonArgs;
    fn validate_common(&self) -> Result<()>;
}

pub fn validate_config_file(config_file: &str) -> Result<()> {
    // Validate config file path if it's not the default and doesn't exist
    if config_file != "config.json" {
        ConfigValidator::validate_config_file(config_file)?;
    }
    Ok(())
}

#[derive(Parser, Debug)]
pub struct UploaderCLI {
    #[arg(
        short,
        long,
        env = "FM_SPREADSHEET_ID",
        help = "Google Sheets spreadsheet ID",
        long_help = "The Google Sheets spreadsheet ID where data will be uploaded.
Example: 1BCD...xyz123 (the long ID from the spreadsheet URL)
Can also be set via FM_SPREADSHEET_ID environment variable."
    )]
    pub spreadsheet: Option<String>,

    #[arg(
        long,
        env = "FM_CREDENTIALS_FILE",
        help = "Path to Google API credentials JSON file",
        long_help = "Path to the Google API service account credentials file.
Download this from Google Cloud Console under APIs & Services > Credentials.
Example: /path/to/service-account-key.json
Can also be set via FM_CREDENTIALS_FILE environment variable."
    )]
    pub credfile: Option<String>,

    #[arg(
        short,
        long,
        env = "FM_INPUT_FILE",
        help = "Path to Football Manager HTML export file",
        long_help = "Path to the HTML file exported from Football Manager containing player data.
The file should contain a table with player statistics.
Example: /path/to/players_export.html
Can also be set via FM_INPUT_FILE environment variable."
    )]
    pub input: Option<String>,

    #[arg(
        short,
        long,
        default_value = "config.json",
        help = "Path to configuration file",
        long_help = "Path to JSON configuration file containing default settings.
If the file doesn't exist, default values will be used.
Example config.json structure:
{
  \"google\": {
    \"spreadsheet_id\": \"1BCD...xyz123\",
    \"credentials_file\": \"creds.json\",
    \"team_sheet\": \"Sheet1\",
    \"token_file\": \"tokencache.json\"
  },
  \"input_file\": \"players.html\"
}"
    )]
    pub config: String,

    #[arg(short, long, help = "Enable verbose logging for debugging")]
    pub verbose: bool,

    #[arg(
        long,
        help = "Disable progress bar (useful for scripting)",
        long_help = "Disable the progress bar display. Useful when running in scripts 
or CI/CD environments where progress bars may interfere with output parsing."
    )]
    pub no_progress: bool,
}

impl CommonCLIArgs for UploaderCLI {
    fn get_common_args(&self) -> CommonArgs {
        CommonArgs::new(
            self.config.clone(),
            self.spreadsheet.clone(),
            self.credfile.clone(),
            self.verbose,
            self.no_progress,
        )
    }

    fn validate_common(&self) -> Result<()> {
        validate_config_file(&self.config)
    }
}

#[derive(Parser, Debug)]
pub struct SelectorCLI {
    #[arg(
        short,
        long,
        env = "FM_ROLE_FILE",
        help = "Path to role file containing 11 roles and optional player filters (required)",
        long_help = "Path to a text file containing exactly 11 Football Manager roles and optional player filters.

Basic format (legacy, still supported):
GK
CD(d)
...

New sectioned format with player filters:
[roles]
GK
CD(d)
...

[filters]
Alisson: goal
Van Dijk: cd
...

Each role must be valid. Duplicate roles are allowed. Player filters restrict players to specific position categories (goal, cd, wb, dm, cm, wing, am, pm, str).
Can also be set via FM_ROLE_FILE environment variable."
    )]
    pub role_file: Option<String>,

    #[arg(
        short,
        long,
        env = "FM_SPREADSHEET_ID",
        help = "Google Sheets spreadsheet ID",
        long_help = "The Google Sheets spreadsheet ID containing player data.
Example: 1BCD...xyz123 (the long ID from the spreadsheet URL)
Can also be set via FM_SPREADSHEET_ID environment variable."
    )]
    pub spreadsheet: Option<String>,

    #[arg(
        long,
        env = "FM_CREDENTIALS_FILE",
        help = "Path to Google API credentials JSON file",
        long_help = "Path to the Google API service account credentials file.
Download this from Google Cloud Console under APIs & Services > Credentials.
Example: /path/to/service-account-key.json
Can also be set via FM_CREDENTIALS_FILE environment variable."
    )]
    pub credfile: Option<String>,

    #[arg(
        short,
        long,
        default_value = "config.json",
        help = "Path to configuration file",
        long_help = "Path to JSON configuration file containing default settings.
If the file doesn't exist, default values will be used.
Example config.json structure:
{
  \"google\": {
    \"spreadsheet_id\": \"1BCD...xyz123\",
    \"credentials_file\": \"creds.json\",
    \"team_sheet\": \"Squad\",
    \"token_file\": \"tokencache.json\"
  },
  \"input\": {
    \"role_file\": \"roles.txt\"
  }
}"
    )]
    pub config: String,

    #[arg(short, long, help = "Enable verbose logging for debugging")]
    pub verbose: bool,

    #[arg(
        long,
        help = "Disable progress bar (useful for scripting)",
        long_help = "Disable the progress bar display. Useful when running in scripts 
or CI/CD environments where progress bars may interfere with output parsing."
    )]
    pub no_progress: bool,
}

impl CommonCLIArgs for SelectorCLI {
    fn get_common_args(&self) -> CommonArgs {
        CommonArgs::new(
            self.config.clone(),
            self.spreadsheet.clone(),
            self.credfile.clone(),
            self.verbose,
            self.no_progress,
        )
    }

    fn validate_common(&self) -> Result<()> {
        validate_config_file(&self.config)?;

        // Role file is required for team selection
        if self.role_file.is_none() {
            return Err(FMDataError::config(
                "Role file is required. Use --role-file or -r to specify the path to your role file.".to_string()
            ));
        }

        Ok(())
    }
}
