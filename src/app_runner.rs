use crate::error::{FMDataError, Result};
use crate::setup_commands::SetupCommand;
use crate::{
    create_authenticator_and_token, get_secure_config_dir, Config, ProgressCallback,
    ProgressReporter, ProgressTracker, SheetsManager,
};
use log::info;
use std::marker::PhantomData;
use std::time::Instant;

/// Common CLI argument validation trait
pub trait CLIArgumentValidator {
    fn validate(&self) -> impl std::future::Future<Output = Result<()>> + Send;
    fn is_verbose(&self) -> bool;
    fn is_no_progress(&self) -> bool;
    fn config_path(&self) -> &str;
}

// ============================================================================
// Type-State Pattern Implementation for AppRunner
// ============================================================================

/// Uninitialized state - AppRunner has been created but not configured
pub struct Uninitialized;

/// Configured state - Config has been loaded and validated
pub struct Configured;

/// Authenticated state - Authentication completed, SheetsManager available
pub struct Authenticated;

/// Application runner that consolidates common binary functionality with compile-time state safety
pub struct AppRunner<S = Uninitialized> {
    pub config: Option<Config>,
    pub progress: Option<ProgressTracker>,
    pub sheets_manager: Option<SheetsManager>,
    pub start_time: Instant,
    pub state: PhantomData<S>,
}

/// Legacy type alias for backward compatibility
pub type LegacyAppRunner = AppRunner<Authenticated>;

// ============================================================================
// Uninitialized State Implementation
// ============================================================================

impl Default for AppRunner<Uninitialized> {
    fn default() -> Self {
        Self::new()
    }
}

impl AppRunner<Uninitialized> {
    /// Create a new uninitialized AppRunner
    pub fn new() -> Self {
        Self {
            config: None,
            progress: None,
            sheets_manager: None,
            start_time: Instant::now(),
            state: PhantomData,
        }
    }

    /// Configure the AppRunner with config and progress tracking
    pub fn configure(self, config: Config, show_progress: bool) -> Result<AppRunner<Configured>> {
        let progress = ProgressTracker::new(100, show_progress);

        Ok(AppRunner {
            config: Some(config),
            progress: Some(progress),
            sheets_manager: None,
            start_time: self.start_time,
            state: PhantomData,
        })
    }
}

// ============================================================================
// Configured State Implementation
// ============================================================================

impl AppRunner<Configured> {
    /// Complete authentication and transition to Authenticated state
    pub async fn authenticate(
        self,
        spreadsheet_id: String,
        credfile: String,
        auth_progress_start: u64,
    ) -> Result<AppRunner<Authenticated>> {
        let config = self.config.as_ref().unwrap(); // Safe: guaranteed by type state
        let progress = self.progress.as_ref().unwrap(); // Safe: guaranteed by type state

        let progress_reporter: &dyn ProgressReporter = progress;
        progress_reporter.update(auth_progress_start, 100, "Setting up authentication...");

        // Ensure secure config directory exists
        let _secure_dir = get_secure_config_dir().await.map_err(|e| {
            FMDataError::auth(format!(
                "Failed to setup secure configuration directory: {e}"
            ))
        })?;

        let token_cache = if config.google.token_file.is_empty() {
            get_secure_config_dir()
                .await?
                .join(crate::constants::config::TOKEN_CACHE_FILE)
                .to_string_lossy()
                .to_string()
        } else {
            config.google.token_file.clone()
        };

        let (secret, token) = create_authenticator_and_token(&credfile, &token_cache).await?;
        info!("Successfully obtained access token");
        progress_reporter.update(auth_progress_start + 15, 100, "Authentication completed");

        // Create sheets manager
        progress_reporter.update(auth_progress_start + 20, 100, "Creating sheets manager...");
        let sheets_manager = SheetsManager::new(secret, token, spreadsheet_id)?;

        sheets_manager
            .verify_spreadsheet_access(progress_reporter)
            .await?;
        sheets_manager
            .verify_sheet_exists(&config.google.team_sheet, progress_reporter)
            .await?;

        Ok(AppRunner {
            config: self.config,
            progress: self.progress,
            sheets_manager: Some(sheets_manager),
            start_time: self.start_time,
            state: PhantomData,
        })
    }

    /// Get config reference (compile-time guaranteed to exist)
    pub fn config(&self) -> &Config {
        self.config.as_ref().unwrap() // Safe: guaranteed by type state
    }

    /// Get progress tracker reference (compile-time guaranteed to exist)
    pub fn progress_tracker(&self) -> &ProgressTracker {
        self.progress.as_ref().unwrap() // Safe: guaranteed by type state
    }

    /// Get progress callback reference
    pub fn progress(&self) -> &dyn ProgressCallback {
        self.progress_tracker()
    }

    /// Get progress reporter reference
    pub fn progress_reporter(&self) -> &dyn ProgressReporter {
        self.progress_tracker()
    }

    /// Finish the application with timing information (available in Configured state)
    pub fn finish(&self, message: &str) {
        ProgressCallback::finish(
            self.progress_tracker(),
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

// ============================================================================
// Authenticated State Implementation
// ============================================================================

impl AppRunner<Authenticated> {
    /// Get config reference (compile-time guaranteed to exist)
    pub fn config(&self) -> &Config {
        self.config.as_ref().unwrap() // Safe: guaranteed by type state
    }

    /// Get progress tracker reference (compile-time guaranteed to exist)
    pub fn progress_tracker(&self) -> &ProgressTracker {
        self.progress.as_ref().unwrap() // Safe: guaranteed by type state
    }

    /// Get progress callback reference
    pub fn progress(&self) -> &dyn ProgressCallback {
        self.progress_tracker()
    }

    /// Get progress reporter reference
    pub fn progress_reporter(&self) -> &dyn ProgressReporter {
        self.progress_tracker()
    }

    /// Get sheets manager reference (compile-time guaranteed to exist)
    pub fn sheets_manager(&self) -> &SheetsManager {
        self.sheets_manager.as_ref().unwrap() // Safe: guaranteed by type state
    }

    /// Execute a setup command using the Command pattern
    pub async fn execute_setup<T: SetupCommand>(&mut self, command: T) -> Result<T::Output> {
        command.execute(self).await
    }

    /// Finish the application with timing information
    pub fn finish(&self, message: &str) {
        ProgressCallback::finish(
            self.progress_tracker(),
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

// ============================================================================
// Legacy Implementation for Backward Compatibility
// ============================================================================

impl AppRunner<Authenticated> {}
