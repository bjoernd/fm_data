use clap::Parser;
use fm_data::error::{FMDataError, Result};
use fm_data::{
    create_authenticator_and_token, get_secure_config_dir, process_table_data, read_table,
    validate_data_size, validate_table_structure, Config, ProgressCallback, ProgressTracker,
    SheetsManager,
};
use log::{debug, error, info, warn};
use std::path::Path;
use std::time::Instant;

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

impl CLIArguments {
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
}

#[tokio::main]
async fn main() -> Result<()> {
    let start_time = Instant::now();

    let cli = CLIArguments::parse();

    // Validate CLI arguments early
    if let Err(e) = cli.validate() {
        error!("Invalid arguments: {}", e);
        std::process::exit(1);
    }

    // Set up logging level based on verbose flag
    if cli.verbose {
        // Set debug level only for our crate, info for others
        std::env::set_var("RUST_LOG", "fm_google_up=debug,info");
    } else {
        // Only show warnings and errors when not in verbose mode to avoid interfering with progress bar
        std::env::set_var("RUST_LOG", "warn");
    }

    // Initialize logging after setting the environment variable
    env_logger::init();

    info!("Starting FM player data uploader");

    // Create progress tracker
    let show_progress = !cli.no_progress && !cli.verbose; // Don't show progress if verbose logging is on
    let progress_tracker = ProgressTracker::new(100, show_progress);
    let progress: &dyn ProgressCallback = &progress_tracker;

    progress.update(0, 100, "Starting upload process...");

    // Read config file
    let config_path = Path::new(&cli.config);
    let config = match Config::from_file(config_path) {
        Ok(cfg) => {
            info!("Successfully loaded config from {}", config_path.display());
            cfg
        }
        Err(e) => {
            warn!("Failed to load config: {}. Using default values.", e);
            Config::create_default()
        }
    };

    // Resolve configuration paths
    progress.update(5, 100, "Resolving configuration paths...");
    let (spreadsheet, credfile, input) = config
        .resolve_paths(cli.spreadsheet, cli.credfile, cli.input)
        .map_err(|e| {
            error!("Configuration validation failed: {}", e);
            e
        })?;

    debug!("Using spreadsheet: {}", spreadsheet);
    debug!("Using credentials file: {}", credfile);
    debug!("Using input HTML file: {}", input);

    // Input file validation is now handled by resolve_paths

    // Authentication setup
    progress.update(10, 100, "Setting up authentication...");

    // Ensure secure config directory exists
    let _secure_dir = get_secure_config_dir().await.map_err(|e| {
        FMDataError::auth(format!(
            "Failed to setup secure configuration directory: {e}"
        ))
    })?;

    let token_cache = if config.google.token_file.is_empty() {
        get_secure_config_dir()
            .await?
            .join("tokencache.json")
            .to_string_lossy()
            .to_string()
    } else {
        config.google.token_file.clone()
    };

    let (secret, token) = create_authenticator_and_token(&credfile, &token_cache).await?;
    info!("Successfully obtained access token");
    progress.update(25, 100, "Authentication completed");

    // Create sheets manager
    progress.update(30, 100, "Creating sheets manager...");
    let sheets_manager = SheetsManager::new(secret, token, spreadsheet)?;
    sheets_manager
        .verify_spreadsheet_access(Some(progress))
        .await?;
    sheets_manager
        .verify_sheet_exists(&config.google.team_sheet, Some(progress))
        .await?;

    // Read table from HTML
    progress.update(40, 100, "Reading HTML table data...");
    let table = read_table(&input)
        .await
        .map_err(|e| FMDataError::table(format!("Failed to extract table from '{input}': {e}")))?;

    // Validate table structure
    progress.update(50, 100, "Validating table structure...");
    validate_table_structure(&table)
        .map_err(|e| FMDataError::table(format!("Invalid table structure: {e}")))?;

    let row_count = table.iter().count();
    info!("Got table with {} rows", row_count);

    // Validate data size
    validate_data_size(row_count)?;
    progress.update(60, 100, &format!("Processing {row_count} rows of data..."));

    if let Some(first_row) = table.iter().next() {
        debug!("Table first row has {} columns", first_row.len());
    }

    // Clear and upload data
    progress.update(70, 100, "Preparing data for upload...");
    let matrix = process_table_data(&table)?;

    sheets_manager
        .clear_range(&config.google.team_sheet, Some(progress))
        .await?;
    sheets_manager
        .upload_data(&config.google.team_sheet, matrix, Some(progress))
        .await?;

    progress.finish(&format!(
        "Upload completed in {} ms",
        start_time.elapsed().as_millis()
    ));
    info!(
        "Program finished in {} ms",
        start_time.elapsed().as_millis()
    );

    Ok(())
}
