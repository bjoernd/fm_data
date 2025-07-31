use clap::Parser;
use fm_data::constants::ranges;
use fm_data::error::{FMDataError, Result};
use fm_data::{
    find_optimal_assignments_with_filters, format_team_output, parse_player_data, 
    parse_role_file_content, AppRunner, CLIArgumentValidator,
};
use log::{debug, error, info};
use std::path::Path;

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
    role_file: Option<String>,

    #[arg(
        short,
        long,
        env = "FM_SPREADSHEET_ID",
        help = "Google Sheets spreadsheet ID",
        long_help = "The Google Sheets spreadsheet ID containing player data.
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

        // Role file is required for team selection
        if self.role_file.is_none() {
            return Err(FMDataError::config(
                "Role file is required. Use --role-file or -r to specify the path to your role file.".to_string()
            ));
        }

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

    // Use AppRunner for consolidated setup
    let mut app_runner = AppRunner::new_complete(&cli, "fm_team_selector").await?;
    let (spreadsheet, credfile, role_file_path) = app_runner
        .setup_for_team_selector(cli.spreadsheet, cli.credfile, cli.role_file)
        .await?;

    // Parse role file
    app_runner
        .progress()
        .update(10, 100, "Loading and validating roles...");
    let role_file_content = parse_role_file_content(&role_file_path).await.map_err(|e| {
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

    // Complete authentication after role file processing
    app_runner
        .complete_team_selector_auth(spreadsheet, credfile)
        .await?;

    // Download player data from Google Sheets
    app_runner
        .progress()
        .update(50, 100, "Downloading player data from Google Sheets...");
    let sheets_manager = app_runner.sheets_manager();
    let sheet_data = sheets_manager
        .read_data(
            &app_runner.config.google.team_sheet,
            ranges::DOWNLOAD_RANGE,
            Some(app_runner.progress()),
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
        &role_file_content.filters
    ).map_err(|e| {
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
