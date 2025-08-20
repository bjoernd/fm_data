use clap::Parser;
use fm_data::{
    browser_ui::BrowserApp, restore_terminal, setup_terminal, AppRunnerBuilder, BrowserCLI,
    CLIArgumentValidator, CommonCLIArgs, FMDataError, Result,
};
use log::{debug, error, info};
use ratatui::Terminal;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Interactive terminal UI for browsing Football Manager player data from Google Sheets",
    long_about = "A terminal-based data browser that loads player data from Google Sheets 
and displays it in an interactive table with navigation using arrow keys.

The tool reads all player data from the configured Google Sheets spreadsheet
and presents it in a scrollable table format for easy browsing and analysis.

Examples:
    # Basic usage with config file
    fm_player_browser -c my_config.json
    
    # Override specific settings  
    fm_player_browser -s 1BCD...xyz123 --credfile creds.json
    
    # Verbose mode for debugging
    fm_player_browser -v
    
    # Scripting mode (no progress bar)
    fm_player_browser --no-progress"
)]
struct CLIArguments {
    #[command(flatten)]
    common: BrowserCLI,
}

impl CLIArgumentValidator for CLIArguments {
    async fn validate(&self) -> Result<()> {
        self.common.validate_common().await
    }

    fn is_verbose(&self) -> bool {
        self.common.common.verbose
    }

    fn is_no_progress(&self) -> bool {
        // Always disable progress bar for browser - it interferes with TUI
        true
    }

    fn config_path(&self) -> &str {
        &self.common.common.config
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = CLIArguments::parse();

    debug!("Starting FM Player Browser");

    // Use new type-state pattern for setup
    let configured_runner = AppRunnerBuilder::from_cli(&cli, "fm_player_browser")
        .build_new()
        .await?;

    // First resolve paths using config
    let (spreadsheet_id, credfile_path, _) = configured_runner
        .config()
        .resolve_paths(
            cli.common.common.spreadsheet,
            cli.common.common.credfile,
            None, // Browser doesn't need input file
        )
        .map_err(|e| FMDataError::config(format!("Configuration validation failed: {e}")))?;

    // Now authenticate with resolved paths
    let app_runner = configured_runner
        .authenticate(spreadsheet_id, credfile_path, 20)
        .await?;

    info!("Authentication successful");

    // Set up terminal for TUI
    let mut terminal = setup_terminal()?;

    // Run the application and ensure terminal cleanup happens regardless of outcome
    let result = run_app(&mut terminal, &app_runner).await;

    // Clean up terminal
    restore_terminal(&mut terminal)?;

    match result {
        Ok(_) => {
            info!("FM Player Browser completed successfully");
            Ok(())
        }
        Err(e) => {
            error!("FM Player Browser failed: {}", e);
            Err(e)
        }
    }
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app_runner: &fm_data::app_runner::AppRunner<fm_data::app_runner::Authenticated>,
) -> Result<()> {
    info!("Initializing application components");

    let config = app_runner.config();
    debug!(
        "Configuration loaded: spreadsheet_id={:?}",
        config.google.spreadsheet_name
    );

    info!("Fetching player data from Google Sheets");
    debug!("Using sheet: {}", config.google.browser_sheet);

    // Fetch player data directly using sheets manager
    let sheets_manager = app_runner.sheets_manager();
    let sheet_data = sheets_manager
        .read_data(
            &config.google.browser_sheet,
            "A2:EQ58", // 145 columns, up to 57 players
            app_runner.progress_reporter(),
        )
        .await?;

    info!("Downloaded {} rows of player data", sheet_data.len());

    // Convert raw data to BrowserPlayer objects
    let players: Vec<_> = sheet_data
        .into_iter()
        .filter_map(|row| fm_data::BrowserPlayer::from_row(&row))
        .filter(|player| player.is_valid())
        .collect();

    info!("Loaded {} players", players.len());

    if players.is_empty() {
        println!("No players found in the spreadsheet. Please check that:");
        println!("1. The spreadsheet contains player data in range A2:EQ58");
        println!("2. Player names are not empty in column A");
        println!("3. The sheet name '{}' exists", config.google.browser_sheet);
        return Ok(());
    }

    debug!("Starting terminal UI with {} players", players.len());

    // Create and run the browser UI
    let mut app = BrowserApp::new(players);
    app.run(terminal)?;

    info!("User exited the browser");
    Ok(())
}
