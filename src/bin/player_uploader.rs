use anyhow::{Context, Result};
use clap::Parser;
use fm_data::{
    create_authenticator_and_token, process_table_data, read_table, validate_data_size,
    validate_table_structure, Config, ProgressCallback, ProgressTracker, SheetsManager,
};
use log::{debug, error, info, warn};
use std::path::Path;
use std::time::Instant;
use tokio;

#[derive(Parser, Debug)]
#[command(version, about="Upload FM Player data to Google sheets", long_about = None)]
struct CLIArguments {
    #[arg(short, long)]
    spreadsheet: Option<String>,
    #[arg(long)]
    credfile: Option<String>,
    #[arg(short, long)]
    input: Option<String>,
    #[arg(short, long, default_value = "config.json")]
    config: String,
    #[arg(short, long)]
    verbose: bool,
    #[arg(long, help = "Disable progress bar (useful for scripting)")]
    no_progress: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let start_time = Instant::now();

    let cli = CLIArguments::parse();

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
            Config::default()
        }
    };

    // Resolve configuration paths
    progress.update(5, 100, "Resolving configuration paths...");
    let (spreadsheet, credfile, input) =
        config.resolve_paths(cli.spreadsheet, cli.credfile, cli.input);

    debug!("Using spreadsheet: {}", spreadsheet);
    debug!("Using credentials file: {}", credfile);
    debug!("Using input HTML file: {}", input);

    // Validate input files exist
    if !Path::new(&input).exists() {
        error!("Input file does not exist: {}", input);
        return Err(anyhow::anyhow!("Input file does not exist: {}", input));
    }

    // Authentication setup
    progress.update(10, 100, "Setting up authentication...");
    let token_cache = if config.google.token_file.is_empty() {
        "tokencache.json"
    } else {
        &config.google.token_file
    };

    let (secret, token) = create_authenticator_and_token(&credfile, token_cache).await?;
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
    let table =
        read_table(&input).with_context(|| format!("Failed to extract table from {}", input))?;

    // Validate table structure
    progress.update(50, 100, "Validating table structure...");
    validate_table_structure(&table).with_context(|| "Invalid table structure")?;

    let row_count = table.iter().count();
    info!("Got table with {} rows", row_count);

    // Validate data size
    validate_data_size(row_count)?;
    progress.update(
        60,
        100,
        &format!("Processing {} rows of data...", row_count),
    );

    if let Some(first_row) = table.iter().next() {
        debug!("Table first row has {} columns", first_row.len());
    }

    // Clear and upload data
    progress.update(70, 100, "Preparing data for upload...");
    let matrix = process_table_data(&table);

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
