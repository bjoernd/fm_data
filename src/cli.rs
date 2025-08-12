use crate::error::{FMDataError, Result};
use crate::error_messages::{ErrorBuilder, ErrorCode};
use crate::validators::ConfigValidator;
use clap::Parser;

pub struct CommonArgs {
    pub config_file: String,
    pub spreadsheet_id: Option<String>,
    pub creds_file: Option<String>,
    pub verbose: bool,
    pub no_progress: bool,
}

impl CommonArgs {
    pub fn new(
        config_file: String,
        spreadsheet_id: Option<String>,
        creds_file: Option<String>,
        verbose: bool,
        no_progress: bool,
    ) -> Self {
        Self {
            config_file,
            spreadsheet_id,
            creds_file,
            verbose,
            no_progress,
        }
    }
}

pub trait CommonCLIArgs {
    fn get_common_args(&self) -> CommonArgs;
    fn validate_common(&self) -> impl std::future::Future<Output = Result<()>> + Send;
}

/// Common CLI arguments shared across all binaries (for clap flattening)
#[derive(Parser, Debug, Clone)]
pub struct CommonCLI {
    #[arg(
        short,
        long,
        env = "FM_SPREADSHEET_ID",
        help = "Google Sheets spreadsheet ID",
        long_help = "The Google Sheets spreadsheet ID where data will be stored or retrieved.
Example: 1BCD...xyz123 (the long ID from the spreadsheet URL)
Can also be set via FM_SPREADSHEET_ID environment variable."
    )]
    pub spreadsheet: Option<String>,

    #[arg(
        long,
        env = "FM_CREDENTIALS_FILE",
        help = "Path to Google API credentials JSON file",
        long_help = "Path to the Google API service account credentials file.
Download this from Google Cloud Console under APIs & Services > Credentials.
Example: /path/to/service-account-key.json
Can also be set via FM_CREDENTIALS_FILE environment variable."
    )]
    pub credfile: Option<String>,

    #[arg(
        short,
        long,
        default_value = crate::constants::config::DEFAULT_CONFIG_FILE,
        help = "Path to configuration file",
        long_help = "Path to JSON configuration file containing default settings.
If the file doesn't exist, default values will be used.
See the documentation for example config.json structure."
    )]
    pub config: String,

    #[arg(short, long, help = "Enable verbose logging for debugging")]
    pub verbose: bool,

    #[arg(
        long,
        help = "Disable progress bar (useful for scripting)",
        long_help = "Disable the progress bar display. Useful when running in scripts 
or CI/CD environments where progress bars may interfere with output parsing."
    )]
    pub no_progress: bool,
}

impl CommonCLI {
    /// Convert to CommonArgs for existing trait compatibility
    pub fn to_common_args(&self) -> CommonArgs {
        CommonArgs::new(
            self.config.clone(),
            self.spreadsheet.clone(),
            self.credfile.clone(),
            self.verbose,
            self.no_progress,
        )
    }

    /// Basic validation common to all CLI tools
    pub fn validate_common(&self) -> Result<()> {
        validate_config_file(&self.config)
    }
}

pub fn validate_config_file(config_file: &str) -> Result<()> {
    use crate::constants::config::DEFAULT_CONFIG_FILE;

    // Validate config file path if it's not the default and doesn't exist
    if config_file != DEFAULT_CONFIG_FILE {
        ConfigValidator::validate_config_file(config_file)?;
    }
    Ok(())
}

pub async fn validate_image_file(image_path: &str) -> Result<()> {
    use std::path::Path;
    use tokio::fs;
    use tokio::io::AsyncReadExt;

    let path = Path::new(image_path);
    if !path.exists() {
        return Err(ErrorBuilder::new(ErrorCode::E600)
            .with_context(image_path)
            .build());
    }

    if !path.is_file() {
        return Err(ErrorBuilder::new(ErrorCode::E104)
            .with_context(format!("path is not a file: {image_path}"))
            .build());
    }

    // Basic PNG file validation (check magic bytes)
    let mut file = fs::File::open(path).await.map_err(|_| {
        ErrorBuilder::new(ErrorCode::E104)
            .with_context(image_path)
            .build()
    })?;
    let mut png_header = [0u8; 8];
    if file.read_exact(&mut png_header).await.is_ok() {
        let png_signature = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        if png_header != png_signature {
            return Err(ErrorBuilder::new(ErrorCode::E601)
                .with_context(image_path)
                .build());
        }
    } else {
        return Err(ErrorBuilder::new(ErrorCode::E104)
            .with_context(format!("unable to read PNG header: {image_path}"))
            .build());
    }

    Ok(())
}

#[derive(Parser, Debug)]
pub struct UploaderCLI {
    #[command(flatten)]
    pub common: CommonCLI,

    #[arg(
        short,
        long,
        env = "FM_INPUT_FILE",
        help = "Path to Football Manager HTML export file",
        long_help = "Path to the HTML file exported from Football Manager containing player data.
The file should contain a table with player statistics.
Example: /path/to/players_export.html
Can also be set via FM_INPUT_FILE environment variable."
    )]
    pub input: Option<String>,
}

impl CommonCLIArgs for UploaderCLI {
    fn get_common_args(&self) -> CommonArgs {
        self.common.to_common_args()
    }

    async fn validate_common(&self) -> Result<()> {
        self.common.validate_common()
    }
}

#[derive(Parser, Debug)]
pub struct SelectorCLI {
    #[command(flatten)]
    pub common: CommonCLI,

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
    pub role_file: Option<String>,
}

impl CommonCLIArgs for SelectorCLI {
    fn get_common_args(&self) -> CommonArgs {
        self.common.to_common_args()
    }

    async fn validate_common(&self) -> Result<()> {
        self.common.validate_common()?;

        // Role file is required for team selection
        if self.role_file.is_none() {
            return Err(FMDataError::config(
                "Role file is required. Use --role-file or -r to specify the path to your role file.".to_string()
            ));
        }

        Ok(())
    }
}

#[derive(Parser, Debug)]
pub struct ImageCLI {
    #[command(flatten)]
    pub common: CommonCLI,

    #[arg(
        short,
        long,
        env = "FM_IMAGE_FILE",
        help = "Path to Football Manager PNG screenshot file (optional - if not provided, will read from clipboard)",
        long_help = "Path to a PNG screenshot file exported from Football Manager containing player data.
The screenshot should show a player's attributes page with all technical, mental, physical, and (optionally) goalkeeping attributes visible.

If not provided, the tool will wait for an image to be pasted from the clipboard using Cmd+V on macOS.

Example: /path/to/player_screenshot.png
Can also be set via FM_IMAGE_FILE environment variable."
    )]
    pub image_file: Option<String>,

    #[arg(
        long,
        default_value = "Scouting",
        help = "Name of the Google Sheets sheet for scouting data",
        long_help = "Name of the sheet in the Google Sheets spreadsheet where player scouting data will be uploaded.
The sheet must exist in the spreadsheet before uploading.
Default: \"Scouting\""
    )]
    pub sheet: String,
}

impl CommonCLIArgs for ImageCLI {
    fn get_common_args(&self) -> CommonArgs {
        self.common.to_common_args()
    }

    async fn validate_common(&self) -> Result<()> {
        self.common.validate_common()?;

        // Validate that the image file exists and is readable (if provided)
        if let Some(ref image_path) = self.image_file {
            validate_image_file(image_path).await?;
        }
        // If no image file is provided, we'll use clipboard mode

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_test_png() -> NamedTempFile {
        let mut temp_file = NamedTempFile::new().unwrap();
        // Write PNG magic bytes to create a valid PNG file
        let png_signature = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        temp_file.write_all(&png_signature).unwrap();
        temp_file.flush().unwrap();
        temp_file
    }

    fn create_test_config() -> NamedTempFile {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"{}").unwrap();
        temp_file.flush().unwrap();
        temp_file
    }

    #[test]
    fn test_image_cli_common_args() {
        let _temp_config = create_test_config();
        let temp_png = create_test_png();

        let cli = ImageCLI {
            image_file: Some(temp_png.path().to_string_lossy().to_string()),
            sheet: "Scouting".to_string(),
            common: CommonCLI {
                spreadsheet: Some("test_spreadsheet_id".to_string()),
                credfile: Some("test_creds.json".to_string()),
                config: "test_config.json".to_string(),
                verbose: true,
                no_progress: false,
            },
        };

        let common_args = cli.get_common_args();
        assert_eq!(common_args.config_file, "test_config.json");
        assert_eq!(
            common_args.spreadsheet_id,
            Some("test_spreadsheet_id".to_string())
        );
        assert_eq!(common_args.creds_file, Some("test_creds.json".to_string()));
        assert!(common_args.verbose);
        assert!(!common_args.no_progress);
    }

    #[tokio::test]
    async fn test_image_cli_validate_missing_image_file_clipboard_mode() {
        // Missing image file is now valid (clipboard mode)
        let cli = ImageCLI {
            image_file: None,
            sheet: "Scouting".to_string(),
            common: CommonCLI {
                spreadsheet: None,
                credfile: None,
                config: crate::constants::config::DEFAULT_CONFIG_FILE.to_string(),
                verbose: false,
                no_progress: false,
            },
        };

        let result = cli.validate_common().await;
        assert!(result.is_ok()); // Should now succeed (clipboard mode)
    }

    #[tokio::test]
    async fn test_image_cli_validate_nonexistent_image_file() {
        let cli = ImageCLI {
            image_file: Some("/nonexistent/path/image.png".to_string()),
            sheet: "Scouting".to_string(),
            common: CommonCLI {
                spreadsheet: None,
                credfile: None,
                config: crate::constants::config::DEFAULT_CONFIG_FILE.to_string(),
                verbose: false,
                no_progress: false,
            },
        };

        let result = cli.validate_common().await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Image file not found"));
    }

    #[tokio::test]
    async fn test_image_cli_validate_invalid_png() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"not a png file").unwrap();
        temp_file.flush().unwrap();

        let cli = ImageCLI {
            image_file: Some(temp_file.path().to_string_lossy().to_string()),
            sheet: "Scouting".to_string(),
            common: CommonCLI {
                spreadsheet: None,
                credfile: None,
                config: crate::constants::config::DEFAULT_CONFIG_FILE.to_string(),
                verbose: false,
                no_progress: false,
            },
        };

        let result = cli.validate_common().await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Invalid image format"));
    }

    #[tokio::test]
    async fn test_image_cli_validate_valid_png() {
        let temp_png = create_test_png();

        let cli = ImageCLI {
            image_file: Some(temp_png.path().to_string_lossy().to_string()),
            sheet: "Scouting".to_string(),
            common: CommonCLI {
                spreadsheet: None,
                credfile: None,
                config: crate::constants::config::DEFAULT_CONFIG_FILE.to_string(),
                verbose: false,
                no_progress: false,
            },
        };

        let result = cli.validate_common().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_image_cli_validate_directory_instead_of_file() {
        let temp_dir = tempfile::TempDir::new().unwrap();

        let cli = ImageCLI {
            image_file: Some(temp_dir.path().to_string_lossy().to_string()),
            sheet: "Scouting".to_string(),
            common: CommonCLI {
                spreadsheet: None,
                credfile: None,
                config: crate::constants::config::DEFAULT_CONFIG_FILE.to_string(),
                verbose: false,
                no_progress: false,
            },
        };

        let result = cli.validate_common().await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Cannot read file"));
    }

    #[tokio::test]
    async fn test_image_cli_validate_unreadable_file() {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            let temp_png = create_test_png();

            // Remove read permissions
            let mut perms = temp_png.path().metadata().unwrap().permissions();
            perms.set_mode(0o000);
            std::fs::set_permissions(temp_png.path(), perms).unwrap();

            let cli = ImageCLI {
                image_file: Some(temp_png.path().to_string_lossy().to_string()),
                sheet: "Scouting".to_string(),
                common: CommonCLI {
                    spreadsheet: None,
                    credfile: None,
                    config: crate::constants::config::DEFAULT_CONFIG_FILE.to_string(),
                    verbose: false,
                    no_progress: false,
                },
            };

            let result = cli.validate_common().await;
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Cannot read file"));

            // Restore permissions for cleanup
            let mut perms = temp_png.path().metadata().unwrap().permissions();
            perms.set_mode(0o644);
            std::fs::set_permissions(temp_png.path(), perms).unwrap();
        }
    }
}
