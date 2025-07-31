use clap::Parser;
use fm_data::error::{FMDataError, Result};
use fm_data::{
    process_table_data, read_table, validate_data_size, validate_table_structure, 
    AppRunner, CLIArgumentValidator,
};
use log::{debug, info};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Upload Football Manager player data to Google Sheets",
    long_about = "A tool to extract player data from Football Manager HTML exports and upload them to Google Sheets.

Examples:
    # Basic usage with config file
    fm_google_up -c my_config.json
    
    # Override specific settings
    fm_google_up -i players.html -s 1BCD...xyz123 --credfile creds.json
    
    # Verbose mode for debugging
    fm_google_up -v -i players.html
    
    # Scripting mode (no progress bar)
    fm_google_up --no-progress -i players.html"
)]
struct CLIArguments {
    #[arg(
        short,
        long,
        env = "FM_SPREADSHEET_ID",
        help = "Google Sheets spreadsheet ID",
        long_help = "The Google Sheets spreadsheet ID where data will be uploaded.
Example: 1BCD...xyz123 (the long ID from the spreadsheet URL)
Can also be set via FM_SPREADSHEET_ID environment variable."
    )]
    spreadsheet: Option<String>,

    #[arg(
        long,
        env = "FM_CREDENTIALS_FILE",
        help = "Path to Google API credentials JSON file",
        long_help = "Path to the Google API service account credentials file.
Download this from Google Cloud Console under APIs & Services > Credentials.
Example: /path/to/service-account-key.json
Can also be set via FM_CREDENTIALS_FILE environment variable."
    )]
    credfile: Option<String>,

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
    input: Option<String>,

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
    config: String,

    #[arg(short, long, help = "Enable verbose logging for debugging")]
    verbose: bool,

    #[arg(
        long,
        help = "Disable progress bar (useful for scripting)",
        long_help = "Disable the progress bar display. Useful when running in scripts 
or CI/CD environments where progress bars may interfere with output parsing."
    )]
    no_progress: bool,
}

impl CLIArgumentValidator for CLIArguments {
    fn validate(&self) -> Result<()> {
        // Validate config file path if it's not the default and doesn't exist
        if self.config != "config.json" {
            let config_path = Path::new(&self.config);
            if !config_path.exists() {
                return Err(FMDataError::config(format!(
                    "Config file '{}' does not exist. Use --config to specify a valid config file or create '{}'",
                    self.config,
                    self.config
                )));
            }
        }

        // The rest of the validation is now handled by the validation module
        // when resolve_paths is called, so we just do basic existence checks here
        Ok(())
    }

    fn is_verbose(&self) -> bool {
        self.verbose
    }

    fn is_no_progress(&self) -> bool {
        self.no_progress
    }

    fn config_path(&self) -> &str {
        &self.config
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = CLIArguments::parse();

    // Use AppRunner for consolidated setup and authentication
    let mut app_runner = AppRunner::new_complete(&cli, "fm_google_up").await?;
    let (_spreadsheet, _credfile, input) = app_runner
        .setup_for_player_uploader(cli.spreadsheet, cli.credfile, cli.input)
        .await?;


    // Read table from HTML
    app_runner.progress().update(40, 100, "Reading HTML table data...");
    let table = read_table(&input)
        .await
        .map_err(|e| FMDataError::table(format!("Failed to extract table from '{input}': {e}")))?;

    // Validate table structure
    app_runner.progress().update(50, 100, "Validating table structure...");
    validate_table_structure(&table)
        .map_err(|e| FMDataError::table(format!("Invalid table structure: {e}")))?;

    let row_count = table.iter().count();
    info!("Got table with {} rows", row_count);

    // Validate data size
    validate_data_size(row_count)?;
    app_runner.progress().update(60, 100, &format!("Processing {row_count} rows of data..."));

    if let Some(first_row) = table.iter().next() {
        debug!("Table first row has {} columns", first_row.len());
    }

    // Clear and upload data
    app_runner.progress().update(70, 100, "Preparing data for upload...");
    let matrix = process_table_data(&table)?;

    let sheets_manager = app_runner.sheets_manager();
    sheets_manager
        .clear_range(&app_runner.config.google.team_sheet, Some(app_runner.progress()))
        .await?;
    sheets_manager
        .upload_data(&app_runner.config.google.team_sheet, matrix, Some(app_runner.progress()))
        .await?;

    app_runner.finish("Upload");

    Ok(())
}
