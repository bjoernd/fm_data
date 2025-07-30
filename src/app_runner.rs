use crate::error::{FMDataError, Result};
use crate::{
    create_authenticator_and_token, get_secure_config_dir, Config, ProgressCallback,
    ProgressTracker, SheetsManager,
};
use log::{error, info, warn};
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
    pub sheets_manager: SheetsManager,
    pub start_time: Instant,
}

impl AppRunner {
    /// Initialize the application with common setup steps
    pub async fn new<T: CLIArgumentValidator>(
        cli: &T,
        binary_name: &str,
    ) -> Result<Self> {
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

        // Setup authentication and sheets manager
        let sheets_manager = Self::setup_authentication_and_sheets(&config, progress_ref).await?;

        Ok(AppRunner {
            config,
            progress,
            sheets_manager,
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
    async fn load_config(
        config_path: &Path,
        progress: &dyn ProgressCallback,
    ) -> Result<Config> {
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

    /// Setup authentication and create sheets manager
    async fn setup_authentication_and_sheets(
        config: &Config,
        progress: &dyn ProgressCallback,
    ) -> Result<SheetsManager> {
        progress.update(10, 100, "Setting up authentication...");

        // Ensure secure config directory exists
        let _secure_dir = get_secure_config_dir().await.map_err(|e| {
            FMDataError::auth(format!(
                "Failed to setup secure configuration directory: {e}"
            ))
        })?;

        let _token_cache = if config.google.token_file.is_empty() {
            get_secure_config_dir()
                .await?
                .join("tokencache.json")
                .to_string_lossy()
                .to_string()
        } else {
            config.google.token_file.clone()
        };

        // This method will be called by the specific binary implementations
        // with their resolved paths
        progress.update(20, 100, "Authentication setup ready");
        
        // Return a placeholder - actual authentication will be done by specific implementations
        // This is a temporary approach until we can fully consolidate authentication
        Err(FMDataError::config("Authentication setup needs to be completed by specific binary".to_string()))
    }

    /// Complete authentication setup with resolved paths
    pub async fn complete_authentication(
        &mut self,
        spreadsheet_id: String,
        credfile: String,
    ) -> Result<()> {
        let progress: &dyn ProgressCallback = &self.progress;
        progress.update(20, 100, "Completing authentication...");

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
        progress.update(25, 100, "Authentication completed");

        // Create sheets manager
        progress.update(30, 100, "Creating sheets manager...");
        let sheets_manager = SheetsManager::new(secret, token, spreadsheet_id)?;
        
        sheets_manager
            .verify_spreadsheet_access(Some(progress))
            .await?;
        sheets_manager
            .verify_sheet_exists(&self.config.google.team_sheet, Some(progress))
            .await?;

        self.sheets_manager = sheets_manager;
        Ok(())
    }

    /// Get progress callback reference
    pub fn progress(&self) -> &dyn ProgressCallback {
        &self.progress
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
}

/// Partial implementation to create AppRunner without full authentication
/// This allows us to incrementally refactor the binaries
impl AppRunner {
    /// Create AppRunner with minimal setup for incremental refactoring
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