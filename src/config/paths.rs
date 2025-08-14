use super::defaults;
use super::types::Config;
use crate::error::Result;
use crate::error_helpers::config_missing_field;
use crate::validators::{ConfigValidator, FileValidator};

/// PathResolver handles common path resolution patterns across different applications
pub struct PathResolver<'a> {
    config: &'a Config,
}

impl<'a> PathResolver<'a> {
    /// Create a new PathResolver for the given configuration
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    /// Resolve common paths (spreadsheet and credentials) used by all applications
    /// 
    /// This method handles the standard priority: CLI > config file > defaults
    /// and validates the resolved paths for correctness.
    pub fn resolve_common_paths(
        &self,
        spreadsheet: Option<String>,
        credfile: Option<String>,
    ) -> Result<(String, String)> {
        let (default_spreadsheet, default_creds, _) = defaults::get_default_paths();

        let resolved_spreadsheet = if let Some(cli_id) = spreadsheet {
            cli_id
        } else if let Some(config_id) = &self.config.google.spreadsheet_name {
            config_id.as_str().to_string()
        } else {
            default_spreadsheet
        };

        let resolved_credfile = resolve_with_fallback(
            credfile,
            self.config.google.creds_file.clone(),
            default_creds,
        );

        // Validate common paths
        ConfigValidator::validate_spreadsheet_id(&resolved_spreadsheet)?;
        FileValidator::validate_file_exists(&resolved_credfile, "Credentials")?;
        FileValidator::validate_file_extension_typed(
            &resolved_credfile,
            crate::constants::FileExtension::Json,
        )?;

        Ok((resolved_spreadsheet, resolved_credfile))
    }

    /// Template method for resolving paths with specific validation logic
    /// 
    /// This method first resolves common paths and then applies custom logic
    /// through the provided resolver function. This pattern eliminates duplication
    /// while allowing each application to add its specific path requirements.
    pub fn resolve_with_specific<T>(
        &self,
        spreadsheet: Option<String>,
        credfile: Option<String>,
        resolver_fn: impl FnOnce(&Config, String, String) -> Result<T>,
    ) -> Result<T> {
        let (resolved_spreadsheet, resolved_credfile) =
            self.resolve_common_paths(spreadsheet, credfile)?;
        resolver_fn(self.config, resolved_spreadsheet, resolved_credfile)
    }
}

/// Helper method to resolve path with fallback priority: CLI > config > default
/// 
/// This function implements the standard resolution logic used throughout the application.
/// It filters out empty values and applies the hierarchical priority system.
pub fn resolve_with_fallback<T: Clone + AsRef<str>>(
    cli_value: Option<T>,
    config_value: T,
    default_value: T,
) -> T {
    cli_value
        .or_else(|| Some(config_value))
        .filter(|s| !s.as_ref().is_empty())
        .unwrap_or(default_value)
}

impl Config {
    /// Resolve paths for the data uploader including input HTML file
    pub fn resolve_paths(
        &self,
        spreadsheet: Option<String>,
        credfile: Option<String>,
        input: Option<String>,
    ) -> Result<(String, String, String)> {
        let resolver = PathResolver::new(self);
        resolver.resolve_with_specific(spreadsheet, credfile, |config, spreadsheet, credfile| {
            let (_, _, default_html) = defaults::get_default_paths();

            let resolved_input =
                resolve_with_fallback(input, config.input.data_html.clone(), default_html);

            // Validate input-specific path
            FileValidator::validate_file_exists(&resolved_input, "Input HTML")?;
            FileValidator::validate_file_extension_typed(
                &resolved_input,
                crate::constants::FileExtension::Html,
            )?;

            Ok((spreadsheet, credfile, resolved_input))
        })
    }

    /// Resolve paths without validation (for testing)
    pub fn resolve_paths_unchecked(
        &self,
        spreadsheet: Option<String>,
        credfile: Option<String>,
        input: Option<String>,
    ) -> (String, String, String) {
        let (default_spreadsheet, default_creds, default_html) = defaults::get_default_paths();

        let resolved_spreadsheet = if let Some(cli_id) = spreadsheet {
            cli_id
        } else if let Some(config_id) = &self.google.spreadsheet_name {
            config_id.as_str().to_string()
        } else {
            default_spreadsheet
        };

        let resolved_credfile =
            resolve_with_fallback(credfile, self.google.creds_file.clone(), default_creds);

        let resolved_input =
            resolve_with_fallback(input, self.input.data_html.clone(), default_html);

        (resolved_spreadsheet, resolved_credfile, resolved_input)
    }

    /// Resolve paths for team selector including role file
    pub fn resolve_team_selector_paths(
        &self,
        spreadsheet: Option<String>,
        credfile: Option<String>,
        role_file: Option<String>,
    ) -> Result<(String, String, String)> {
        let resolver = PathResolver::new(self);
        resolver.resolve_with_specific(spreadsheet, credfile, |config, spreadsheet, credfile| {
            let resolved_role_file = role_file
                .or_else(|| Some(config.input.role_file.clone()))
                .filter(|s| !s.is_empty())
                .ok_or_else(|| config_missing_field("role_file"))?;

            // Validate role file specific path
            FileValidator::validate_file_exists(&resolved_role_file, "Role file")?;
            FileValidator::validate_file_extension_typed(
                &resolved_role_file,
                crate::constants::FileExtension::Txt,
            )?;

            Ok((spreadsheet, credfile, resolved_role_file))
        })
    }

    /// Resolve paths for image processor including image file and sheet name
    pub async fn resolve_image_paths(
        &self,
        spreadsheet: Option<String>,
        credfile: Option<String>,
        image_file: Option<String>,
        sheet: Option<String>,
    ) -> Result<(String, String, String, String)> {
        let resolver = PathResolver::new(self);
        let (resolved_spreadsheet, resolved_credfile) =
            resolver.resolve_common_paths(spreadsheet, credfile)?;

        let resolved_image_file = image_file
            .or_else(|| Some(self.input.image_file.clone()))
            .filter(|s| !s.is_empty())
            .ok_or_else(|| config_missing_field("image_file"))?;

        let resolved_sheet = resolve_with_fallback(
            sheet,
            self.google.scouting_sheet.clone(),
            defaults::default_scouting_sheet(),
        );

        // Validate image file specific paths
        FileValidator::validate_file_exists(&resolved_image_file, "Image")?;
        crate::cli::validate_image_file(&resolved_image_file).await?;

        Ok((
            resolved_spreadsheet,
            resolved_credfile,
            resolved_image_file,
            resolved_sheet,
        ))
    }
}