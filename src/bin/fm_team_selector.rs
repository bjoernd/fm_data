use clap::Parser;
use fm_data::constants::ranges;
use fm_data::error::{FMDataError, Result};
use fm_data::{
    find_optimal_assignments_with_filters, format_team_output, parse_player_data,
    parse_role_file_content, AppRunnerBuilder, CLIArgumentValidator, CommonCLIArgs, SelectorCLI,
};
use log::{debug, error, info};

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Find optimal player-to-role assignments for Football Manager teams",
    long_about = "A tool to analyze player data from Google Sheets and find the optimal assignment 
of players to roles that maximizes the total team score using a greedy algorithm.

The tool reads a list of 11 required roles and optional player filters from a local file, 
downloads player data from Google Sheets, then assigns each role to the eligible player 
with the highest rating for that specific role. Player filters can restrict players to 
specific position categories (goal, cd, wb, dm, cm, wing, am, pm, str).

Examples:
    # Basic usage with config file and role file
    fm_team_selector -r roles.txt -c my_config.json
    
    # Override specific settings  
    fm_team_selector -r roles.txt -s 1BCD...xyz123 --credfile creds.json
    
    # Verbose mode for debugging
    fm_team_selector -r roles.txt -v
    
    # Scripting mode (no progress bar)
    fm_team_selector -r roles.txt --no-progress"
)]
struct CLIArguments {
    #[command(flatten)]
    common: SelectorCLI,
}

impl CLIArgumentValidator for CLIArguments {
    async fn validate(&self) -> Result<()> {
        self.common.validate_common().await
    }

    fn is_verbose(&self) -> bool {
        self.common.common.verbose
    }

    fn is_no_progress(&self) -> bool {
        self.common.common.no_progress
    }

    fn config_path(&self) -> &str {
        &self.common.common.config
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = CLIArguments::parse();

    // Use new type-state pattern for setup
    let configured_runner = AppRunnerBuilder::from_cli(&cli, "fm_team_selector")
        .build_new()
        .await?;

    // First resolve paths using config
    let (spreadsheet_id, credfile_path, role_file_path) = configured_runner
        .config()
        .resolve_team_selector_paths(
            cli.common.common.spreadsheet,
            cli.common.common.credfile,
            cli.common.role_file,
        )
        .map_err(|e| FMDataError::config(format!("Configuration validation failed: {e}")))?;

    // Parse role file before authentication
    configured_runner
        .progress()
        .update(10, 100, "Loading and validating roles...");
    let role_file_content = parse_role_file_content(&role_file_path)
        .await
        .map_err(|e| {
            error!("Failed to parse role file: {}", e);
            e
        })?;

    info!(
        "Successfully loaded {} roles and {} player filters from {}",
        role_file_content.roles.len(),
        role_file_content.filters.len(),
        role_file_path
    );
    debug!("Roles: {:?}", role_file_content.roles);
    debug!("Filters: {:?}", role_file_content.filters);

    // Now authenticate with resolved paths
    let app_runner = configured_runner
        .authenticate(spreadsheet_id, credfile_path, 20)
        .await?;

    // Download player data from Google Sheets
    app_runner
        .progress()
        .update(50, 100, "Downloading player data from Google Sheets...");
    let sheets_manager = app_runner.sheets_manager();
    let sheet_data = sheets_manager
        .read_data(
            &app_runner.config().google.team_sheet,
            ranges::DOWNLOAD_RANGE,
            app_runner.progress_reporter(),
        )
        .await?;

    info!("Downloaded {} rows of player data", sheet_data.len());

    // Parse player data
    app_runner
        .progress()
        .update(65, 100, "Parsing player data...");
    let players = parse_player_data(sheet_data).map_err(|e| {
        error!("Failed to parse player data: {}", e);
        e
    })?;

    info!("Successfully parsed {} players", players.len());

    // Find optimal assignments
    app_runner
        .progress()
        .update(80, 100, "Finding optimal player assignments...");
    let team = find_optimal_assignments_with_filters(
        players,
        role_file_content.roles,
        &role_file_content.filters,
    )
    .map_err(|e| {
        error!("Failed to find optimal assignments: {}", e);
        e
    })?;

    info!(
        "Found optimal team with total score: {:.1}",
        team.total_score()
    );

    // Generate and output results
    app_runner
        .progress()
        .update(95, 100, "Generating output...");
    let output = format_team_output(&team);

    app_runner.finish("Team selection");

    // Print the final team assignments to stdout
    print!("{output}");

    Ok(())
}
