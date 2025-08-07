use crate::error::FMDataError;

/// Extension trait for Result types to add contextual error information
pub trait ErrorContext<T> {
    /// Add file operation context to any error
    fn with_file_context(self, file_path: &str, operation: &str) -> Result<T, FMDataError>;

    /// Add configuration validation context to any error
    fn with_config_context(self, field_name: &str) -> Result<T, FMDataError>;

    /// Add Google Sheets operation context to any error
    fn with_sheets_context(self, operation: &str) -> Result<T, FMDataError>;

    /// Add table processing context to any error
    fn with_table_context(self, context: &str) -> Result<T, FMDataError>;

    /// Add authentication context to any error
    fn with_auth_context(self, context: &str) -> Result<T, FMDataError>;

    /// Add selection/team assignment context to any error
    fn with_selection_context(self, context: &str) -> Result<T, FMDataError>;
}

impl<T, E> ErrorContext<T> for Result<T, E>
where
    E: std::fmt::Display,
{
    fn with_file_context(self, file_path: &str, operation: &str) -> Result<T, FMDataError> {
        self.map_err(|e| {
            FMDataError::config(format!("Failed to {operation} file '{file_path}': {e}"))
        })
    }

    fn with_config_context(self, field_name: &str) -> Result<T, FMDataError> {
        self.map_err(|e| {
            FMDataError::config(format!("Configuration error in field '{field_name}': {e}"))
        })
    }

    fn with_sheets_context(self, operation: &str) -> Result<T, FMDataError> {
        self.map_err(|e| {
            FMDataError::sheets_api(format!("Google Sheets {operation} operation failed: {e}"))
        })
    }

    fn with_table_context(self, context: &str) -> Result<T, FMDataError> {
        self.map_err(|e| FMDataError::table(format!("Table processing error ({context}): {e}")))
    }

    fn with_auth_context(self, context: &str) -> Result<T, FMDataError> {
        self.map_err(|e| FMDataError::auth(format!("Authentication error ({context}): {e}")))
    }

    fn with_selection_context(self, context: &str) -> Result<T, FMDataError> {
        self.map_err(|e| FMDataError::selection(format!("Selection error ({context}): {e}")))
    }
}

// Helper functions for common error patterns
pub fn file_not_found(path: &str) -> FMDataError {
    FMDataError::config(format!("File not found: '{path}'"))
}

pub fn invalid_role(role_name: &str) -> FMDataError {
    FMDataError::selection(format!("Invalid role: '{role_name}'"))
}

pub fn invalid_category(category_name: &str) -> FMDataError {
    FMDataError::selection(format!("Invalid category: '{category_name}'"))
}

pub fn config_missing_field(field: &str) -> FMDataError {
    FMDataError::config(format!("Missing required configuration field: '{field}'"))
}

pub fn role_file_format_error(line_num: usize, message: &str) -> FMDataError {
    FMDataError::selection(format!(
        "Role file format error on line {line_num}: {message}"
    ))
}

pub fn player_filter_error(player_name: &str, line_num: usize, message: &str) -> FMDataError {
    FMDataError::selection(format!(
        "Player filter error for '{player_name}' on line {line_num}: {message}"
    ))
}

pub fn insufficient_players(needed: usize, available: usize) -> FMDataError {
    FMDataError::selection(format!(
        "Insufficient players: need {needed} but only {available} available"
    ))
}

pub fn duplicate_role_info(role_name: &str) -> FMDataError {
    FMDataError::selection(format!("Duplicate role found: '{role_name}'"))
}

pub fn assignment_blocked(player_name: &str, reason: &str) -> FMDataError {
    FMDataError::selection(format!(
        "Player '{player_name}' could not be assigned: {reason}"
    ))
}

/// Helper for creating validation errors with specific context
pub fn validation_error(item_type: &str, item_name: &str, message: &str) -> FMDataError {
    FMDataError::selection(format!(
        "Validation error for {item_type} '{item_name}': {message}"
    ))
}

/// Helper for creating table processing errors with row/column context
pub fn table_processing_error(
    row: Option<usize>,
    column: Option<&str>,
    message: &str,
) -> FMDataError {
    let location = match (row, column) {
        (Some(r), Some(c)) => format!(" at row {r}, column '{c}'"),
        (Some(r), None) => format!(" at row {r}"),
        (None, Some(c)) => format!(" at column '{c}'"),
        (None, None) => String::new(),
    };

    FMDataError::table(format!("Table processing error{location}: {message}"))
}
