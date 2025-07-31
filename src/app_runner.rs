use crate::error::{FMDataError, Result};
use crate::{
    create_authenticator_and_token, get_secure_config_dir, Config, ProgressCallback,
    ProgressTracker, SheetsManager,
};
use log::{debug, error, info, warn};
use std::path::Path;
use std::time::Instant;

/// Common CLI argument validation trait
pub trait CLIArgumentValidator {
    fn validate(&self) -> Result<()>;
    fn is_verbose(&self) -> bool;
    fn is_no_progress(&self) -> bool;
    fn config_path(&self) -> &str;
}

/// Application runner that consolidates common binary functionality
pub struct AppRunner {
    pub config: Config,
    pub progress: ProgressTracker,
    pub sheets_manager: Option<SheetsManager>,
    pub start_time: Instant,
}

impl AppRunner {
    /// Initialize the application with common setup steps
    pub async fn new<T: CLIArgumentValidator>(cli: &T, binary_name: &str) -> Result<Self> {
        let start_time = Instant::now();

        // Validate CLI arguments early
        if let Err(e) = cli.validate() {
            error!("Invalid arguments: {}", e);
            std::process::exit(1);
        }

        // Set up logging level based on verbose flag
        Self::setup_logging(cli.is_verbose(), binary_name);

        info!("Starting {}", binary_name);

        // Create progress tracker
        let show_progress = !cli.is_no_progress() && !cli.is_verbose();
        let progress = ProgressTracker::new(100, show_progress);
        let progress_ref: &dyn ProgressCallback = &progress;

        progress_ref.update(0, 100, "Starting process...");

        // Read config file
        let config_path = Path::new(cli.config_path());
        let config = Self::load_config(config_path, progress_ref).await?;

        Ok(AppRunner {
            config,
            progress,
            sheets_manager: None,
            start_time,
        })
    }

    /// Setup logging configuration
    fn setup_logging(verbose: bool, binary_name: &str) {
        if verbose {
            // Set debug level only for our crate, info for others
            std::env::set_var("RUST_LOG", &format!("{}=debug,info", binary_name));
        } else {
            // Only show warnings and errors when not in verbose mode to avoid interfering with progress bar
            std::env::set_var("RUST_LOG", "warn");
        }

        // Initialize logging after setting the environment variable
        env_logger::init();
    }

    /// Load configuration with fallback to defaults
    async fn load_config(config_path: &Path, progress: &dyn ProgressCallback) -> Result<Config> {
        progress.update(5, 100, "Loading configuration...");

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

        Ok(config)
    }

    /// Complete authentication setup with resolved paths and create sheets manager
    pub async fn complete_authentication(
        &mut self,
        spreadsheet_id: String,
        credfile: String,
        auth_progress_start: u64,
    ) -> Result<()> {
        let progress: &dyn ProgressCallback = &self.progress;
        progress.update(auth_progress_start, 100, "Setting up authentication...");

        // Ensure secure config directory exists
        let _secure_dir = get_secure_config_dir().await.map_err(|e| {
            FMDataError::auth(format!(
                "Failed to setup secure configuration directory: {e}"
            ))
        })?;

        let token_cache = if self.config.google.token_file.is_empty() {
            get_secure_config_dir()
                .await?
                .join("tokencache.json")
                .to_string_lossy()
                .to_string()
        } else {
            self.config.google.token_file.clone()
        };

        let (secret, token) = create_authenticator_and_token(&credfile, &token_cache).await?;
        info!("Successfully obtained access token");
        progress.update(auth_progress_start + 15, 100, "Authentication completed");

        // Create sheets manager
        progress.update(auth_progress_start + 20, 100, "Creating sheets manager...");
        let sheets_manager = SheetsManager::new(secret, token, spreadsheet_id)?;

        sheets_manager
            .verify_spreadsheet_access(Some(progress))
            .await?;
        sheets_manager
            .verify_sheet_exists(&self.config.google.team_sheet, Some(progress))
            .await?;

        self.sheets_manager = Some(sheets_manager);
        Ok(())
    }

    /// Get progress callback reference
    pub fn progress(&self) -> &dyn ProgressCallback {
        &self.progress
    }

    /// Get sheets manager reference (panics if authentication not completed)
    pub fn sheets_manager(&self) -> &SheetsManager {
        self.sheets_manager
            .as_ref()
            .expect("Authentication not completed - call complete_authentication first")
    }

    /// Resolve paths for player uploader and setup authentication
    pub async fn setup_for_player_uploader(
        &mut self,
        spreadsheet: Option<String>,
        credfile: Option<String>,
        input: Option<String>,
    ) -> Result<(String, String, String)> {
        let progress = self.progress();
        progress.update(5, 100, "Resolving configuration paths...");

        let (spreadsheet_id, credfile_path, input_path) = self
            .config
            .resolve_paths(spreadsheet, credfile, input)
            .map_err(|e| {
                error!("Configuration validation failed: {}", e);
                e
            })?;

        debug!("Using spreadsheet: {}", spreadsheet_id);
        debug!("Using credentials file: {}", credfile_path);
        debug!("Using input HTML file: {}", input_path);

        // Complete authentication setup
        self.complete_authentication(spreadsheet_id.clone(), credfile_path.clone(), 10)
            .await?;

        Ok((spreadsheet_id, credfile_path, input_path))
    }

    /// Resolve paths for team selector and setup authentication
    pub async fn setup_for_team_selector(
        &mut self,
        spreadsheet: Option<String>,
        credfile: Option<String>,
        role_file: Option<String>,
    ) -> Result<(String, String, String)> {
        let progress = self.progress();
        progress.update(5, 100, "Resolving configuration paths...");

        let (spreadsheet_id, credfile_path, role_file_path) = self
            .config
            .resolve_team_selector_paths(spreadsheet, credfile, role_file)
            .map_err(|e| {
                error!("Configuration validation failed: {}", e);
                e
            })?;

        debug!("Using spreadsheet: {}", spreadsheet_id);
        debug!("Using credentials file: {}", credfile_path);
        debug!("Using role file: {}", role_file_path);

        // Complete authentication setup (delayed to allow for role file processing)
        Ok((spreadsheet_id, credfile_path, role_file_path))
    }

    /// Complete authentication for team selector (called after role file processing)
    pub async fn complete_team_selector_auth(
        &mut self,
        spreadsheet_id: String,
        credfile_path: String,
    ) -> Result<()> {
        self.complete_authentication(spreadsheet_id, credfile_path, 20)
            .await
    }

    /// Finish the application with timing information
    pub fn finish(&self, message: &str) {
        self.progress.finish(&format!(
            "{} completed in {} ms",
            message,
            self.start_time.elapsed().as_millis()
        ));
        info!(
            "Program finished in {} ms",
            self.start_time.elapsed().as_millis()
        );
    }

    /// Create AppRunner with minimal setup for consolidated authentication
    pub async fn new_complete<T: CLIArgumentValidator>(
        cli: &T,
        binary_name: &str,
    ) -> Result<AppRunner> {
        let start_time = Instant::now();

        // Validate CLI arguments early
        if let Err(e) = cli.validate() {
            error!("Invalid arguments: {}", e);
            std::process::exit(1);
        }

        // Set up logging level based on verbose flag
        Self::setup_logging(cli.is_verbose(), binary_name);

        info!("Starting {}", binary_name);

        // Create progress tracker
        let show_progress = !cli.is_no_progress() && !cli.is_verbose();
        let progress = ProgressTracker::new(100, show_progress);
        let progress_ref: &dyn ProgressCallback = &progress;

        progress_ref.update(0, 100, "Starting process...");

        // Read config file
        let config_path = Path::new(cli.config_path());
        let config = Self::load_config(config_path, progress_ref).await?;

        Ok(AppRunner {
            config,
            progress,
            sheets_manager: None,
            start_time,
        })
    }

    /// Create AppRunner with minimal setup for incremental refactoring (backward compatibility)
    pub async fn new_minimal<T: CLIArgumentValidator>(
        cli: &T,
        binary_name: &str,
    ) -> Result<(Config, ProgressTracker, Instant)> {
        let start_time = Instant::now();

        // Validate CLI arguments early
        if let Err(e) = cli.validate() {
            error!("Invalid arguments: {}", e);
            std::process::exit(1);
        }

        // Set up logging level based on verbose flag
        Self::setup_logging(cli.is_verbose(), binary_name);

        info!("Starting {}", binary_name);

        // Create progress tracker
        let show_progress = !cli.is_no_progress() && !cli.is_verbose();
        let progress = ProgressTracker::new(100, show_progress);
        let progress_ref: &dyn ProgressCallback = &progress;

        progress_ref.update(0, 100, "Starting process...");

        // Read config file
        let config_path = Path::new(cli.config_path());
        let config = Self::load_config(config_path, progress_ref).await?;

        Ok((config, progress, start_time))
    }
}
