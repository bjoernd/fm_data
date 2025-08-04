use clap::Parser;
use fm_data::error::{FMDataError, Result};
use fm_data::{
    process_table_data, read_table, validate_data_size, validate_table_structure, AppRunnerBuilder,
    CLIArgumentValidator, CommonCLIArgs, UploaderCLI,
};
use log::{debug, info};

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
    #[command(flatten)]
    common: UploaderCLI,
}

impl CLIArgumentValidator for CLIArguments {
    fn validate(&self) -> Result<()> {
        self.common.validate_common()
    }

    fn is_verbose(&self) -> bool {
        self.common.verbose
    }

    fn is_no_progress(&self) -> bool {
        self.common.no_progress
    }

    fn config_path(&self) -> &str {
        &self.common.config
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = CLIArguments::parse();

    // Use AppRunnerBuilder for consolidated setup and authentication
    let mut app_runner = AppRunnerBuilder::from_cli(&cli, "fm_google_up")
        .build_basic()
        .await?;

    // Setup for player uploader and get resolved paths
    let (_spreadsheet_id, _credfile_path, input_path) = app_runner
        .setup_for_player_uploader(
            cli.common.spreadsheet,
            cli.common.credfile,
            cli.common.input,
        )
        .await?;

    // Read table from HTML
    app_runner
        .progress()
        .update(40, 100, "Reading HTML table data...");
    let table = read_table(&input_path)
        .await
        .map_err(|e| FMDataError::table(format!("Failed to extract table from '{input_path}': {e}")))?;

    // Validate table structure
    app_runner
        .progress()
        .update(50, 100, "Validating table structure...");
    validate_table_structure(&table)
        .map_err(|e| FMDataError::table(format!("Invalid table structure: {e}")))?;

    let row_count = table.iter().count();
    info!("Got table with {} rows", row_count);

    // Validate data size
    validate_data_size(row_count)?;
    app_runner
        .progress()
        .update(60, 100, &format!("Processing {row_count} rows of data..."));

    if let Some(first_row) = table.iter().next() {
        debug!("Table first row has {} columns", first_row.len());
    }

    // Clear and upload data
    app_runner
        .progress()
        .update(70, 100, "Preparing data for upload...");
    let matrix = process_table_data(&table)?;

    let sheets_manager = app_runner.sheets_manager();
    sheets_manager
        .clear_range(
            &app_runner.config.google.team_sheet,
            Some(app_runner.progress()),
        )
        .await?;
    sheets_manager
        .upload_data(
            &app_runner.config.google.team_sheet,
            matrix,
            Some(app_runner.progress()),
        )
        .await?;

    app_runner.finish("Upload");

    Ok(())
}
