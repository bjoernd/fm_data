use crate::app_runner::{AppRunner, Authenticated};
use crate::error::Result;
use async_trait::async_trait;
use log::{debug, error};

/// Trait for setup commands that can be executed on an AppRunner
#[async_trait]
pub trait SetupCommand {
    type Output;

    async fn execute(&self, app_runner: &mut AppRunner<Authenticated>) -> Result<Self::Output>;
}

/// Setup command for player uploader binary
pub struct PlayerUploaderSetup {
    pub spreadsheet: Option<String>,
    pub credfile: Option<String>,
    pub input: Option<String>,
}

impl PlayerUploaderSetup {
    pub fn new(
        spreadsheet: Option<String>,
        credfile: Option<String>,
        input: Option<String>,
    ) -> Self {
        Self {
            spreadsheet,
            credfile,
            input,
        }
    }
}

#[async_trait]
impl SetupCommand for PlayerUploaderSetup {
    type Output = (String, String, String); // (spreadsheet_id, credfile_path, input_path)

    async fn execute(&self, app_runner: &mut AppRunner<Authenticated>) -> Result<Self::Output> {
        let progress = app_runner.progress();
        progress.update(5, 100, "Resolving configuration paths...");

        let (spreadsheet_id, credfile_path, input_path) = app_runner
            .config()
            .resolve_paths(
                self.spreadsheet.clone(),
                self.credfile.clone(),
                self.input.clone(),
            )
            .map_err(|e| {
                error!("Configuration validation failed: {}", e);
                e
            })?;

        debug!("Using spreadsheet: {}", spreadsheet_id);
        debug!("Using credentials file: {}", credfile_path);
        debug!("Using input HTML file: {}", input_path);

        // AppRunner is already authenticated, no need to call complete_authentication
        // The type system guarantees this

        Ok((spreadsheet_id, credfile_path, input_path))
    }
}

/// Setup command for team selector binary
pub struct TeamSelectorSetup {
    pub spreadsheet: Option<String>,
    pub credfile: Option<String>,
    pub role_file: Option<String>,
}

impl TeamSelectorSetup {
    pub fn new(
        spreadsheet: Option<String>,
        credfile: Option<String>,
        role_file: Option<String>,
    ) -> Self {
        Self {
            spreadsheet,
            credfile,
            role_file,
        }
    }
}

#[async_trait]
impl SetupCommand for TeamSelectorSetup {
    type Output = (String, String, String); // (spreadsheet_id, credfile_path, role_file_path)

    async fn execute(&self, app_runner: &mut AppRunner<Authenticated>) -> Result<Self::Output> {
        let progress = app_runner.progress();
        progress.update(5, 100, "Resolving configuration paths...");

        let (spreadsheet_id, credfile_path, role_file_path) = app_runner
            .config()
            .resolve_team_selector_paths(
                self.spreadsheet.clone(),
                self.credfile.clone(),
                self.role_file.clone(),
            )
            .map_err(|e| {
                error!("Configuration validation failed: {}", e);
                e
            })?;

        debug!("Using spreadsheet: {}", spreadsheet_id);
        debug!("Using credentials file: {}", credfile_path);
        debug!("Using role file: {}", role_file_path);

        // AppRunner is already authenticated, no need for delayed authentication
        // The type system guarantees this
        Ok((spreadsheet_id, credfile_path, role_file_path))
    }
}

/// Setup command for image processor binary
pub struct ImageProcessorSetup {
    pub spreadsheet: Option<String>,
    pub credfile: Option<String>,
    pub image_file: Option<String>,
    pub sheet: Option<String>,
}

impl ImageProcessorSetup {
    pub fn new(
        spreadsheet: Option<String>,
        credfile: Option<String>,
        image_file: Option<String>,
        sheet: Option<String>,
    ) -> Self {
        Self {
            spreadsheet,
            credfile,
            image_file,
            sheet,
        }
    }
}

#[async_trait]
impl SetupCommand for ImageProcessorSetup {
    type Output = (String, String, String, String); // (spreadsheet_id, credfile_path, image_file_path, sheet_name)

    async fn execute(&self, app_runner: &mut AppRunner<Authenticated>) -> Result<Self::Output> {
        let progress = app_runner.progress();
        progress.update(5, 100, "Resolving configuration paths...");

        let (spreadsheet_id, credfile_path, image_file_path, sheet_name) = app_runner
            .config()
            .resolve_image_paths(
                self.spreadsheet.clone(),
                self.credfile.clone(),
                self.image_file.clone(),
                self.sheet.clone(),
            )
            .await
            .map_err(|e| {
                error!("Configuration validation failed: {}", e);
                e
            })?;

        debug!("Using spreadsheet: {}", spreadsheet_id);
        debug!("Using credentials file: {}", credfile_path);
        debug!("Using image file: {}", image_file_path);
        debug!("Using sheet: {}", sheet_name);

        // AppRunner is already authenticated, no need to call complete_authentication
        // The type system guarantees this

        Ok((spreadsheet_id, credfile_path, image_file_path, sheet_name))
    }
}

/// Authentication setup command that can be used independently
pub struct AuthenticationSetup {
    pub spreadsheet_id: String,
    pub credfile_path: String,
    pub auth_progress_start: u64,
}

impl AuthenticationSetup {
    pub fn new(spreadsheet_id: String, credfile_path: String, auth_progress_start: u64) -> Self {
        Self {
            spreadsheet_id,
            credfile_path,
            auth_progress_start,
        }
    }
}

#[async_trait]
impl SetupCommand for AuthenticationSetup {
    type Output = ();

    async fn execute(&self, _app_runner: &mut AppRunner<Authenticated>) -> Result<Self::Output> {
        // AppRunner is already authenticated in the Authenticated state
        // This command is now redundant but kept for compatibility
        debug!("Authentication already completed - AppRunner is in Authenticated state");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_uploader_setup_command_creation() {
        let setup_command = PlayerUploaderSetup::new(
            Some("test_spreadsheet".to_string()),
            Some("test_creds.json".to_string()),
            Some("test_input.html".to_string()),
        );

        assert_eq!(
            setup_command.spreadsheet,
            Some("test_spreadsheet".to_string())
        );
        assert_eq!(setup_command.credfile, Some("test_creds.json".to_string()));
        assert_eq!(setup_command.input, Some("test_input.html".to_string()));
    }

    #[test]
    fn test_team_selector_setup_command_creation() {
        let setup_command = TeamSelectorSetup::new(
            Some("test_spreadsheet".to_string()),
            Some("test_creds.json".to_string()),
            Some("test_roles.txt".to_string()),
        );

        assert_eq!(
            setup_command.spreadsheet,
            Some("test_spreadsheet".to_string())
        );
        assert_eq!(setup_command.credfile, Some("test_creds.json".to_string()));
        assert_eq!(setup_command.role_file, Some("test_roles.txt".to_string()));
    }

    #[test]
    fn test_image_processor_setup_command_creation() {
        let setup_command = ImageProcessorSetup::new(
            Some("test_spreadsheet".to_string()),
            Some("test_creds.json".to_string()),
            Some("test_image.png".to_string()),
            Some("TestSheet".to_string()),
        );

        assert_eq!(
            setup_command.spreadsheet,
            Some("test_spreadsheet".to_string())
        );
        assert_eq!(setup_command.credfile, Some("test_creds.json".to_string()));
        assert_eq!(setup_command.image_file, Some("test_image.png".to_string()));
        assert_eq!(setup_command.sheet, Some("TestSheet".to_string()));
    }

    #[test]
    fn test_authentication_setup_command_creation() {
        let setup_command = AuthenticationSetup::new(
            "test_spreadsheet".to_string(),
            "test_creds.json".to_string(),
            20,
        );

        assert_eq!(setup_command.spreadsheet_id, "test_spreadsheet");
        assert_eq!(setup_command.credfile_path, "test_creds.json");
        assert_eq!(setup_command.auth_progress_start, 20);
    }
}
