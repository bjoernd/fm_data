use anyhow::{Context, Result};
use clap::Parser;
use fm_data::{
    Config,
    read_table, validate_table_structure, process_table_data, validate_data_size,
    create_authenticator_and_token,
    SheetsManager,
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
        std::env::set_var("RUST_LOG", "info");
    }

    // Initialize logging after setting the environment variable
    env_logger::init();

    info!("Starting FM player data uploader");

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
    let (spreadsheet, credfile, input) = config.resolve_paths(cli.spreadsheet, cli.credfile, cli.input);

    debug!("Using spreadsheet: {}", spreadsheet);
    debug!("Using credentials file: {}", credfile);
    debug!("Using input HTML file: {}", input);

    // Validate input files exist
    if !Path::new(&input).exists() {
        error!("Input file does not exist: {}", input);
        return Err(anyhow::anyhow!("Input file does not exist: {}", input));
    }

    // Authentication setup
    let token_cache = if config.google.token_file.is_empty() {
        "tokencache.json"
    } else {
        &config.google.token_file
    };

    let (secret, token) = create_authenticator_and_token(&credfile, token_cache).await?;
    info!("Successfully obtained access token");

    // Create sheets manager
    let sheets_manager = SheetsManager::new(secret, token, spreadsheet)?;
    sheets_manager.verify_spreadsheet_access().await?;
    sheets_manager.verify_sheet_exists(&config.google.team_sheet).await?;

    // Read table from HTML
    let table =
        read_table(&input).with_context(|| format!("Failed to extract table from {}", input))?;

    // Validate table structure
    validate_table_structure(&table).with_context(|| "Invalid table structure")?;

    let row_count = table.iter().count();
    info!("Got table with {} rows", row_count);

    // Validate data size
    validate_data_size(row_count)?;

    if let Some(first_row) = table.iter().next() {
        debug!("Table first row has {} columns", first_row.len());
    }

    // Clear and upload data
    sheets_manager.clear_range(&config.google.team_sheet).await?;
    
    let matrix = process_table_data(&table);
    sheets_manager.upload_data(&config.google.team_sheet, matrix).await?;
    info!(
        "Program finished in {} ms",
        start_time.elapsed().as_millis()
    );

    Ok(())
}
