use crate::error::{FMDataError, Result};
use crate::{
    create_authenticator_and_token, get_secure_config_dir, Config, ProgressCallback,
    ProgressReporter, ProgressTracker, SheetsManager,
};
use log::{debug, error, info};
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
    /// Initialize the application with common setup steps (deprecated - use AppRunnerBuilder)
    #[deprecated(note = "Use AppRunnerBuilder::from_cli().build_basic() instead")]
    pub async fn new<T: CLIArgumentValidator>(cli: &T, binary_name: &str) -> Result<Self> {
        use crate::AppRunnerBuilder;
        AppRunnerBuilder::from_cli(cli, binary_name)
            .build_basic()
            .await
    }

    /// Complete authentication setup with resolved paths and create sheets manager
    pub async fn complete_authentication(
        &mut self,
        spreadsheet_id: String,
        credfile: String,
        auth_progress_start: u64,
    ) -> Result<()> {
        let progress: &dyn ProgressReporter = &self.progress;
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
                .join(crate::constants::config::TOKEN_CACHE_FILE)
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

        sheets_manager.verify_spreadsheet_access(progress).await?;
        sheets_manager
            .verify_sheet_exists(&self.config.google.team_sheet, progress)
            .await?;

        self.sheets_manager = Some(sheets_manager);
        Ok(())
    }

    /// Get progress callback reference (legacy interface)
    pub fn progress(&self) -> &dyn ProgressCallback {
        &self.progress
    }

    /// Get progress reporter reference (new interface)
    pub fn progress_reporter(&self) -> &dyn ProgressReporter {
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
        ProgressCallback::finish(
            &self.progress,
            &format!(
                "{} completed in {} ms",
                message,
                self.start_time.elapsed().as_millis()
            ),
        );
        info!(
            "Program finished in {} ms",
            self.start_time.elapsed().as_millis()
        );
    }
}
