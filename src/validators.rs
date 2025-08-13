use crate::error::{FMDataError, Result};
use crate::error_messages::{ErrorBuilder, ErrorCode};
use crate::selection::Role;
use std::collections::HashMap;
use std::path::Path;

// ==============================================================================
// Config Validator - Consolidates configuration-related validation
// ==============================================================================

pub struct ConfigValidator;

impl ConfigValidator {
    /// Validate Google-related configuration parameters  
    pub fn validate_google_config(spreadsheet_id: &str, sheet_name: &str) -> Result<()> {
        Self::validate_spreadsheet_id(spreadsheet_id)?;
        Self::validate_sheet_name(sheet_name)?;
        Ok(())
    }

    /// Validate spreadsheet ID format and content
    pub fn validate_spreadsheet_id(id: &str) -> Result<()> {
        if id.is_empty() {
            return Err(ErrorBuilder::new(ErrorCode::E105)
                .with_context("spreadsheet ID")
                .build());
        }

        if !id
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        {
            return Err(ErrorBuilder::new(ErrorCode::E106)
                .with_context(format!("invalid spreadsheet ID format: '{id}'"))
                .build());
        }

        if id.len() < 20 {
            log::warn!("Spreadsheet ID '{}' seems unusually short", id);
        }

        Ok(())
    }

    /// Validate Google Sheets sheet name
    pub fn validate_sheet_name(name: &str) -> Result<()> {
        if name.is_empty() {
            return Err(ErrorBuilder::new(ErrorCode::E105)
                .with_context("sheet name")
                .build());
        }

        // Google Sheets has some character restrictions
        let invalid_chars = ['[', ']', '*', '?', ':', '\\', '/', '\''];
        if name.chars().any(|c| invalid_chars.contains(&c)) {
            return Err(ErrorBuilder::new(ErrorCode::E106)
                .with_context(format!("sheet name '{name}' contains invalid characters"))
                .build());
        }

        if name.len() > 100 {
            return Err(FMDataError::config(format!(
                "Sheet name '{name}' is too long (max 100 characters)"
            )));
        }

        Ok(())
    }

    /// Validate configuration file structure and content
    pub fn validate_config_file(config_file: &str) -> Result<()> {
        use crate::constants::FileExtension;

        FileValidator::validate_file_exists(config_file, "Configuration")?;
        FileValidator::validate_file_extension_typed(config_file, FileExtension::Json)?;
        Ok(())
    }
}

// ==============================================================================
// Role Validator - Consolidates role-related validation
// ==============================================================================

pub struct RoleValidator;

impl RoleValidator {
    /// Validate that a role name is a valid Football Manager role
    pub fn validate_role_name(role: &str) -> Result<()> {
        if !Role::is_valid_role(role) {
            return Err(FMDataError::selection(format!("Invalid role: '{role}'")));
        }
        Ok(())
    }

    /// Validate that all roles in a collection are valid
    pub fn validate_roles(roles: &[String]) -> Result<()> {
        for role_name in roles {
            Self::validate_role_name(role_name)?;
        }
        Ok(())
    }

    /// Validate role file format and content
    pub fn validate_role_file_format(roles: &[String]) -> Result<()> {
        use crate::constants::team::REQUIRED_ROLE_COUNT;

        if roles.len() != REQUIRED_ROLE_COUNT {
            return Err(FMDataError::selection(format!(
                "Role file must contain exactly {} roles, found {}",
                REQUIRED_ROLE_COUNT,
                roles.len()
            )));
        }

        Self::validate_roles(roles)?;
        Ok(())
    }
}

// ==============================================================================
// Player Validator - Consolidates player-related validation
// ==============================================================================

pub struct PlayerValidator;

impl PlayerValidator {
    /// Validate player filter categories are valid
    pub fn validate_filter_categories(filters: &HashMap<String, Vec<String>>) -> Result<()> {
        use crate::selection::categories::is_valid_category;

        for (player_name, categories) in filters {
            for category in categories {
                if !is_valid_category(category) {
                    return Err(FMDataError::selection(format!(
                        "Invalid category '{category}' for player '{player_name}'"
                    )));
                }
            }
        }
        Ok(())
    }

    /// Validate that player data structure is complete and correct
    pub fn validate_player_data_structure(data: &[Vec<String>]) -> Result<()> {
        if data.is_empty() {
            return Err(FMDataError::selection("No player data provided"));
        }

        // Check that all rows have the same number of columns
        let expected_columns = data[0].len();
        for (index, row) in data.iter().enumerate() {
            if row.len() != expected_columns {
                return Err(FMDataError::selection(format!(
                    "Inconsistent row length: row {} has {} columns, expected {}",
                    index,
                    row.len(),
                    expected_columns
                )));
            }
        }

        Ok(())
    }
}

// ==============================================================================
// File Validator - Consolidates file system validation
// ==============================================================================

pub struct FileValidator;

impl FileValidator {
    /// Validate that a file exists
    pub fn validate_file_exists(path: &str, file_type: &str) -> Result<()> {
        let path_obj = Path::new(path);
        if !path_obj.exists() {
            return Err(FMDataError::config(format!(
                "{file_type} file does not exist: {path}"
            )));
        }
        Ok(())
    }

    /// Validate file extension (with warning for mismatches)
    pub fn validate_file_extension(path: &str, expected_ext: &str) -> Result<()> {
        let path_obj = Path::new(path);
        if let Some(extension) = path_obj.extension() {
            if extension.to_string_lossy().to_lowercase() != expected_ext {
                log::warn!(
                    "File '{}' does not have .{} extension. This may not be valid.",
                    path,
                    expected_ext
                );
            }
        }
        Ok(())
    }

    /// Validate file extension using type-safe enum (preferred method)
    pub fn validate_file_extension_typed(
        path: &str,
        expected: crate::constants::FileExtension,
    ) -> Result<()> {
        use crate::constants::FileExtension;

        let path_obj = Path::new(path);
        if let Some(actual_ext) = FileExtension::from_path(path) {
            if actual_ext != expected {
                log::warn!(
                    "File '{}' has .{} extension but expected .{}",
                    path,
                    actual_ext,
                    expected
                );
            }
        } else if path_obj.extension().is_some() {
            log::warn!(
                "File '{}' has an unrecognized extension, expected .{}",
                path,
                expected
            );
        }
        Ok(())
    }

    /// Validate that path exists and is a file
    pub fn validate_path_is_file(path: &str) -> Result<()> {
        let path_obj = Path::new(path);
        if path_obj.exists() && !path_obj.is_file() {
            return Err(FMDataError::config(format!(
                "Path '{path}' exists but is not a file"
            )));
        }
        Ok(())
    }

    /// Validate that parent directory exists for a given path
    pub fn validate_parent_directory_exists(path: &str) -> Result<()> {
        let path_obj = Path::new(path);
        if let Some(parent) = path_obj.parent() {
            if !parent.exists() {
                return Err(FMDataError::config(format!(
                    "Parent directory does not exist for path: {path}"
                )));
            }
        }
        Ok(())
    }

    /// Validate file permissions are readable
    pub fn validate_file_permissions(path: &str) -> Result<()> {
        let path_obj = Path::new(path);
        if path_obj.exists() {
            if let Ok(metadata) = path_obj.metadata() {
                if metadata.permissions().readonly() {
                    log::warn!("File '{}' is read-only", path);
                }
            }
        }
        Ok(())
    }
}

// ==============================================================================
// Data Validator - Consolidates data structure validation
// ==============================================================================

pub struct DataValidator;

impl DataValidator {
    /// Validate table/data size constraints
    pub fn validate_table_size(rows: usize, max_rows: usize) -> Result<()> {
        if rows > max_rows {
            return Err(FMDataError::table(format!(
                "Data has {rows} rows but maximum allowed is {max_rows} rows (hardcoded range limit)"
            )));
        }
        Ok(())
    }

    /// Validate row consistency across tabular data
    pub fn validate_row_consistency(rows: &[Vec<String>]) -> Result<()> {
        if rows.is_empty() {
            return Ok(());
        }

        let expected_columns = rows[0].len();
        for (index, row) in rows.iter().enumerate() {
            if row.len() != expected_columns {
                return Err(FMDataError::table(format!(
                    "Inconsistent row length: row {index} has {} columns, expected {expected_columns}",
                    row.len()
                )));
            }
        }
        Ok(())
    }

    /// Validate that data is not empty
    pub fn validate_non_empty_data(data: &[Vec<String>]) -> Result<()> {
        if data.is_empty() {
            return Err(FMDataError::table("No data to upload"));
        }

        let has_non_empty_row = data
            .iter()
            .any(|row| row.iter().any(|cell| !cell.trim().is_empty()));

        if !has_non_empty_row {
            return Err(FMDataError::table("All data rows are empty"));
        }
        Ok(())
    }
}

// ==============================================================================
// Auth Validator - Consolidates authentication validation
// ==============================================================================

pub struct AuthValidator;

impl AuthValidator {
    /// Validate Google OAuth credentials file structure and content
    pub fn validate_credentials_content(content: &str) -> Result<()> {
        // Parse JSON to validate structure
        let json: serde_json::Value = serde_json::from_str(content)
            .map_err(|e| FMDataError::auth(format!("Invalid JSON in credentials file: {e}")))?;

        // Check for required OAuth2 fields
        let installed = json
            .get("installed")
            .or_else(|| json.get("web"))
            .ok_or_else(|| {
                FMDataError::auth("Credentials file must contain 'installed' or 'web' section")
            })?;

        let required_fields = ["client_id", "client_secret", "auth_uri", "token_uri"];
        for field in &required_fields {
            if installed.get(field).is_none() {
                return Err(FMDataError::auth(format!(
                    "Missing required field '{field}' in credentials"
                )));
            }
        }

        // Validate URLs are HTTPS
        let auth_uri = installed
            .get("auth_uri")
            .and_then(|v| v.as_str())
            .ok_or_else(|| FMDataError::auth("auth_uri must be a string"))?;

        let token_uri = installed
            .get("token_uri")
            .and_then(|v| v.as_str())
            .ok_or_else(|| FMDataError::auth("token_uri must be a string"))?;

        if !auth_uri.starts_with("https://") {
            return Err(FMDataError::auth(format!(
                "auth_uri must use HTTPS: {auth_uri}"
            )));
        }

        if !token_uri.starts_with("https://") {
            return Err(FMDataError::auth(format!(
                "token_uri must use HTTPS: {token_uri}"
            )));
        }

        Ok(())
    }

    /// Validate credentials file and its content
    pub async fn validate_credentials_file(path: &str) -> Result<()> {
        use crate::constants::FileExtension;

        FileValidator::validate_file_exists(path, "Credentials")?;
        FileValidator::validate_file_extension_typed(path, FileExtension::Json)?;

        // Read and validate content
        let content = tokio::fs::read_to_string(path)
            .await
            .map_err(|e| FMDataError::config(format!("Failed to read credentials file: {e}")))?;

        Self::validate_credentials_content(&content)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    // Config Validator Tests
    #[test]
    fn test_config_validator_spreadsheet_id() {
        assert!(ConfigValidator::validate_spreadsheet_id("valid-spreadsheet_id123").is_ok());
        assert!(ConfigValidator::validate_spreadsheet_id("").is_err());
        assert!(ConfigValidator::validate_spreadsheet_id("invalid@id").is_err());
    }

    #[test]
    fn test_config_validator_sheet_name() {
        assert!(ConfigValidator::validate_sheet_name("ValidSheet").is_ok());
        assert!(ConfigValidator::validate_sheet_name("").is_err());
        assert!(ConfigValidator::validate_sheet_name("Invalid[Sheet]").is_err());
        assert!(ConfigValidator::validate_sheet_name(&"x".repeat(101)).is_err());
    }

    #[test]
    fn test_config_validator_google_config() {
        assert!(ConfigValidator::validate_google_config("valid-id", "ValidSheet").is_ok());
        assert!(ConfigValidator::validate_google_config("", "ValidSheet").is_err());
        assert!(ConfigValidator::validate_google_config("valid-id", "").is_err());
    }

    // Role Validator Tests
    #[test]
    fn test_role_validator_role_name() {
        assert!(RoleValidator::validate_role_name("GK").is_ok());
        assert!(RoleValidator::validate_role_name("InvalidRole").is_err());
    }

    #[test]
    fn test_role_validator_roles() {
        let valid_roles = vec!["GK".to_string(), "CD(d)".to_string()];
        assert!(RoleValidator::validate_roles(&valid_roles).is_ok());

        let invalid_roles = vec!["GK".to_string(), "InvalidRole".to_string()];
        assert!(RoleValidator::validate_roles(&invalid_roles).is_err());
    }

    // File Validator Tests
    #[test]
    fn test_file_validator_file_exists() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_string_lossy();

        assert!(FileValidator::validate_file_exists(&path, "test").is_ok());
        assert!(FileValidator::validate_file_exists("nonexistent.txt", "test").is_err());
    }

    #[test]
    fn test_file_validator_file_extension() {
        use crate::constants::FileExtension;

        assert!(FileValidator::validate_file_extension("test.json", "json").is_ok());
        // Should not error for mismatched extensions, just warn
        assert!(FileValidator::validate_file_extension("test.txt", "json").is_ok());

        // Test typed version
        assert!(
            FileValidator::validate_file_extension_typed("test.json", FileExtension::Json).is_ok()
        );
        assert!(
            FileValidator::validate_file_extension_typed("test.html", FileExtension::Html).is_ok()
        );
        assert!(
            FileValidator::validate_file_extension_typed("test.txt", FileExtension::Txt).is_ok()
        );
    }

    // Data Validator Tests
    #[test]
    fn test_data_validator_table_size() {
        assert!(DataValidator::validate_table_size(10, 50).is_ok());
        assert!(DataValidator::validate_table_size(0, 50).is_ok()); // Allow 0 rows
        assert!(DataValidator::validate_table_size(60, 50).is_err());
    }

    #[test]
    fn test_data_validator_row_consistency() {
        let consistent_rows = vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["c".to_string(), "d".to_string()],
        ];
        assert!(DataValidator::validate_row_consistency(&consistent_rows).is_ok());

        let inconsistent_rows = vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["c".to_string()],
        ];
        assert!(DataValidator::validate_row_consistency(&inconsistent_rows).is_err());
    }

    #[test]
    fn test_data_validator_non_empty_data() {
        let valid_data = vec![vec!["data".to_string(), "more".to_string()]];
        assert!(DataValidator::validate_non_empty_data(&valid_data).is_ok());

        let empty_data: Vec<Vec<String>> = vec![];
        assert!(DataValidator::validate_non_empty_data(&empty_data).is_err());

        let all_empty_data = vec![vec!["".to_string(), "  ".to_string()]];
        assert!(DataValidator::validate_non_empty_data(&all_empty_data).is_err());
    }

    // Player Validator Tests
    #[test]
    fn test_player_validator_filter_categories() {
        let mut valid_filters = HashMap::new();
        valid_filters.insert(
            "Player1".to_string(),
            vec!["goal".to_string(), "cd".to_string()],
        );
        assert!(PlayerValidator::validate_filter_categories(&valid_filters).is_ok());

        let mut invalid_filters = HashMap::new();
        invalid_filters.insert("Player1".to_string(), vec!["invalid_category".to_string()]);
        assert!(PlayerValidator::validate_filter_categories(&invalid_filters).is_err());
    }

    // Auth Validator Tests
    #[test]
    fn test_auth_validator_credentials_content_valid() {
        let credentials_json = r#"{
            "installed": {
                "client_id": "test_client_id",
                "client_secret": "test_client_secret",
                "auth_uri": "https://accounts.google.com/o/oauth2/auth",
                "token_uri": "https://oauth2.googleapis.com/token"
            }
        }"#;

        assert!(AuthValidator::validate_credentials_content(credentials_json).is_ok());
    }

    #[test]
    fn test_auth_validator_credentials_content_missing_fields() {
        let credentials_json = r#"{
            "installed": {
                "client_id": "test_client_id"
            }
        }"#;

        assert!(AuthValidator::validate_credentials_content(credentials_json).is_err());
    }

    #[test]
    fn test_auth_validator_credentials_content_insecure_urls() {
        let credentials_json = r#"{
            "installed": {
                "client_id": "test_client_id",
                "client_secret": "test_client_secret", 
                "auth_uri": "http://accounts.google.com/o/oauth2/auth",
                "token_uri": "https://oauth2.googleapis.com/token"
            }
        }"#;

        assert!(AuthValidator::validate_credentials_content(credentials_json).is_err());
    }
}
