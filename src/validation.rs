use crate::error::{FMDataError, Result};
use std::path::Path;

pub struct PathValidator;

impl PathValidator {
    pub fn validate_file_exists(path: &str, file_type: &str) -> Result<()> {
        let path_obj = Path::new(path);
        if !path_obj.exists() {
            return Err(FMDataError::config(format!(
                "{file_type} file does not exist: {path}"
            )));
        }
        Ok(())
    }

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

    pub fn validate_path_is_file(path: &str) -> Result<()> {
        let path_obj = Path::new(path);
        if path_obj.exists() && !path_obj.is_file() {
            return Err(FMDataError::config(format!(
                "Path '{path}' exists but is not a file"
            )));
        }
        Ok(())
    }

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
}

pub struct IdValidator;

impl IdValidator {
    pub fn validate_spreadsheet_id(id: &str) -> Result<()> {
        if id.is_empty() {
            return Err(FMDataError::config("Spreadsheet ID cannot be empty"));
        }

        if !id
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        {
            return Err(FMDataError::config(format!(
                "Invalid spreadsheet ID format: '{id}'. Must contain only letters, numbers, hyphens, and underscores"
            )));
        }

        if id.len() < 20 {
            log::warn!("Spreadsheet ID '{}' seems unusually short", id);
        }

        Ok(())
    }

    pub fn validate_sheet_name(name: &str) -> Result<()> {
        if name.is_empty() {
            return Err(FMDataError::config("Sheet name cannot be empty"));
        }

        // Google Sheets has some character restrictions
        let invalid_chars = ['[', ']', '*', '?', ':', '\\', '/', '\''];
        if name.chars().any(|c| invalid_chars.contains(&c)) {
            return Err(FMDataError::config(format!(
                "Sheet name '{name}' contains invalid characters. Avoid: [ ] * ? : \\ / '"
            )));
        }

        if name.len() > 100 {
            return Err(FMDataError::config(format!(
                "Sheet name '{name}' is too long (max 100 characters)"
            )));
        }

        Ok(())
    }
}

pub struct DataValidator;

impl DataValidator {
    pub fn validate_table_size(rows: usize, max_rows: usize) -> Result<()> {
        if rows > max_rows {
            return Err(FMDataError::table(format!(
                "Data has {rows} rows but maximum allowed is {max_rows} rows (hardcoded range limit)"
            )));
        }

        Ok(())
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_validate_file_exists() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_string_lossy();

        assert!(PathValidator::validate_file_exists(&path, "test").is_ok());
        assert!(PathValidator::validate_file_exists("nonexistent.txt", "test").is_err());
    }

    #[test]
    fn test_validate_file_extension() {
        assert!(PathValidator::validate_file_extension("test.json", "json").is_ok());
        // Should not error for mismatched extensions, just warn
        assert!(PathValidator::validate_file_extension("test.txt", "json").is_ok());
    }

    #[test]
    fn test_validate_spreadsheet_id() {
        assert!(IdValidator::validate_spreadsheet_id("valid-spreadsheet_id123").is_ok());
        assert!(IdValidator::validate_spreadsheet_id("").is_err());
        assert!(IdValidator::validate_spreadsheet_id("invalid@id").is_err());
    }

    #[test]
    fn test_validate_sheet_name() {
        assert!(IdValidator::validate_sheet_name("ValidSheet").is_ok());
        assert!(IdValidator::validate_sheet_name("").is_err());
        assert!(IdValidator::validate_sheet_name("Invalid[Sheet]").is_err());
        assert!(IdValidator::validate_sheet_name(&"x".repeat(101)).is_err());
    }

    #[test]
    fn test_validate_table_size() {
        assert!(DataValidator::validate_table_size(10, 50).is_ok());
        assert!(DataValidator::validate_table_size(0, 50).is_ok()); // Allow 0 rows
        assert!(DataValidator::validate_table_size(60, 50).is_err());
    }

    #[test]
    fn test_validate_row_consistency() {
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
    fn test_validate_non_empty_data() {
        let valid_data = vec![vec!["data".to_string(), "more".to_string()]];
        assert!(DataValidator::validate_non_empty_data(&valid_data).is_ok());

        let empty_data: Vec<Vec<String>> = vec![];
        assert!(DataValidator::validate_non_empty_data(&empty_data).is_err());

        let all_empty_data = vec![vec!["".to_string(), "  ".to_string()]];
        assert!(DataValidator::validate_non_empty_data(&all_empty_data).is_err());
    }
}
