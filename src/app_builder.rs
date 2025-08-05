use crate::error::Result;
use crate::{AppRunner, CLIArgumentValidator, Config, ProgressCallback, ProgressTracker};
use log::{info, warn};
use std::path::Path;
use std::time::Instant;

/// Builder pattern for AppRunner to consolidate initialization logic
pub struct AppRunnerBuilder {
    config_file: Option<String>,
    spreadsheet_id: Option<String>,
    creds_file: Option<String>,
    sheet_name: Option<String>,
    input_file: Option<String>,
    role_file: Option<String>,
    verbose: bool,
    no_progress: bool,
    binary_name: String,
}

impl AppRunnerBuilder {
    /// Create a new builder instance
    pub fn new(binary_name: &str) -> Self {
        Self {
            config_file: None,
            spreadsheet_id: None,
            creds_file: None,
            sheet_name: None,
            input_file: None,
            role_file: None,
            verbose: false,
            no_progress: false,
            binary_name: binary_name.to_string(),
        }
    }

    /// Create builder from CLI arguments (convenient method)
    pub fn from_cli<T: CLIArgumentValidator>(cli: &T, binary_name: &str) -> Self {
        Self {
            config_file: Some(cli.config_path().to_string()),
            spreadsheet_id: None,
            creds_file: None,
            sheet_name: None,
            input_file: None,
            role_file: None,
            verbose: cli.is_verbose(),
            no_progress: cli.is_no_progress(),
            binary_name: binary_name.to_string(),
        }
    }

    /// Set config file path
    pub fn config_file(mut self, path: Option<String>) -> Self {
        self.config_file = path;
        self
    }

    /// Set spreadsheet ID
    pub fn spreadsheet_id(mut self, id: Option<String>) -> Self {
        self.spreadsheet_id = id;
        self
    }

    /// Set credentials file path
    pub fn creds_file(mut self, path: Option<String>) -> Self {
        self.creds_file = path;
        self
    }

    /// Set sheet name
    pub fn sheet_name(mut self, name: Option<String>) -> Self {
        self.sheet_name = name;
        self
    }

    /// Set input file path (for uploader)
    pub fn input_file(mut self, path: Option<String>) -> Self {
        self.input_file = path;
        self
    }

    /// Set role file path (for team selector)
    pub fn role_file(mut self, path: Option<String>) -> Self {
        self.role_file = path;
        self
    }

    /// Set verbose mode
    pub fn verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    /// Set no progress mode
    pub fn no_progress(mut self, no_progress: bool) -> Self {
        self.no_progress = no_progress;
        self
    }

    /// Build AppRunner with full authentication setup for uploader
    pub async fn build_uploader(self) -> Result<AppRunner> {
        // Store values before self is moved
        let spreadsheet_id = self.spreadsheet_id.clone();
        let creds_file = self.creds_file.clone();
        let input_file = self.input_file.clone();

        let mut app_runner = self.build_basic().await?;

        // Resolve paths and setup authentication for uploader
        let (_spreadsheet_id, _credfile_path, _input_path) = app_runner
            .setup_for_player_uploader(spreadsheet_id, creds_file, input_file)
            .await?;

        Ok(app_runner)
    }

    /// Build AppRunner with authentication setup for team selector
    pub async fn build_team_selector(self) -> Result<AppRunner> {
        // Store values before self is moved
        let spreadsheet_id = self.spreadsheet_id.clone();
        let creds_file = self.creds_file.clone();
        let role_file = self.role_file.clone();

        let mut app_runner = self.build_basic().await?;

        // Resolve paths for team selector (authentication will be completed later)
        let (_spreadsheet_id, _credfile_path, _role_file_path) = app_runner
            .setup_for_team_selector(spreadsheet_id, creds_file, role_file)
            .await?;

        Ok(app_runner)
    }

    /// Build basic AppRunner without authentication
    pub async fn build_basic(self) -> Result<AppRunner> {
        let start_time = Instant::now();

        // Set up logging level based on verbose flag
        Self::setup_logging(self.verbose, &self.binary_name);

        info!("Starting {}", self.binary_name);

        // Create progress tracker
        let show_progress = !self.no_progress && !self.verbose;
        let progress = ProgressTracker::new(100, show_progress);
        let progress_ref: &dyn ProgressCallback = &progress;

        progress_ref.update(0, 100, "Starting process...");

        // Read config file
        let config_path = Path::new(self.config_file.as_deref().unwrap_or(crate::constants::config::DEFAULT_CONFIG_FILE));
        let config = Self::load_config(config_path, progress_ref).await?;

        Ok(AppRunner {
            config,
            progress,
            sheets_manager: None,
            start_time,
        })
    }

    /// Build minimal components for backward compatibility
    pub async fn build_minimal(self) -> Result<(Config, ProgressTracker, Instant)> {
        let start_time = Instant::now();

        // Set up logging level based on verbose flag
        Self::setup_logging(self.verbose, &self.binary_name);

        info!("Starting {}", self.binary_name);

        // Create progress tracker
        let show_progress = !self.no_progress && !self.verbose;
        let progress = ProgressTracker::new(100, show_progress);
        let progress_ref: &dyn ProgressCallback = &progress;

        progress_ref.update(0, 100, "Starting process...");

        // Read config file
        let config_path = Path::new(self.config_file.as_deref().unwrap_or(crate::constants::config::DEFAULT_CONFIG_FILE));
        let config = Self::load_config(config_path, progress_ref).await?;

        Ok((config, progress, start_time))
    }

    /// Setup logging configuration (consolidated from AppRunner)
    fn setup_logging(verbose: bool, binary_name: &str) {
        if verbose {
            // Set debug level only for our crate, info for others
            std::env::set_var("RUST_LOG", format!("{binary_name}=debug,info"));
        } else {
            // Only show warnings and errors when not in verbose mode to avoid interfering with progress bar
            std::env::set_var("RUST_LOG", "warn");
        }

        // Initialize logging after setting the environment variable
        env_logger::init();
    }

    /// Load configuration with fallback to defaults (consolidated from AppRunner)
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
}
