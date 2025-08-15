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

/// Domain-specific extension trait for configuration-related Results
pub trait ConfigResult<T> {
    /// Add configuration operation context
    fn config_context(self, operation: &str) -> Result<T, FMDataError>;

    /// Add file path context for configuration files
    fn file_context(self, path: &str) -> Result<T, FMDataError>;

    /// Add field validation context
    fn field_context(self, field_name: &str) -> Result<T, FMDataError>;

    /// Add JSON parsing context
    fn json_context(self, description: &str) -> Result<T, FMDataError>;
}

impl<T, E> ConfigResult<T> for Result<T, E>
where
    E: std::fmt::Display,
{
    fn config_context(self, operation: &str) -> Result<T, FMDataError> {
        self.map_err(|e| FMDataError::config(format!("Configuration {operation}: {e}")))
    }

    fn file_context(self, path: &str) -> Result<T, FMDataError> {
        self.map_err(|e| FMDataError::config(format!("Config file '{path}': {e}")))
    }

    fn field_context(self, field_name: &str) -> Result<T, FMDataError> {
        self.map_err(|e| FMDataError::config(format!("Config field '{field_name}': {e}")))
    }

    fn json_context(self, description: &str) -> Result<T, FMDataError> {
        self.map_err(|e| FMDataError::config(format!("JSON {description}: {e}")))
    }
}

/// Domain-specific extension trait for Google Sheets-related Results  
pub trait SheetsResult<T> {
    /// Add Google Sheets operation context
    fn sheets_context(self, operation: &str) -> Result<T, FMDataError>;

    /// Add range-specific context  
    fn range_context(self, range: &str) -> Result<T, FMDataError>;

    /// Add sheet name context
    fn sheet_context(self, sheet_name: &str) -> Result<T, FMDataError>;

    /// Add authentication context for Sheets operations
    fn auth_context(self, context: &str) -> Result<T, FMDataError>;

    /// Add data validation context for Sheets operations
    fn data_context(self, description: &str) -> Result<T, FMDataError>;
}

impl<T, E> SheetsResult<T> for Result<T, E>
where
    E: std::fmt::Display,
{
    fn sheets_context(self, operation: &str) -> Result<T, FMDataError> {
        self.map_err(|e| FMDataError::sheets_api(format!("Sheets {operation}: {e}")))
    }

    fn range_context(self, range: &str) -> Result<T, FMDataError> {
        self.map_err(|e| FMDataError::sheets_api(format!("Range '{range}': {e}")))
    }

    fn sheet_context(self, sheet_name: &str) -> Result<T, FMDataError> {
        self.map_err(|e| FMDataError::sheets_api(format!("Sheet '{sheet_name}': {e}")))
    }

    fn auth_context(self, context: &str) -> Result<T, FMDataError> {
        self.map_err(|e| FMDataError::auth(format!("Sheets authentication ({context}): {e}")))
    }

    fn data_context(self, description: &str) -> Result<T, FMDataError> {
        self.map_err(|e| FMDataError::sheets_api(format!("Data {description}: {e}")))
    }
}

/// Domain-specific extension trait for selection/team assignment Results
pub trait SelectionResult<T> {
    /// Add role validation context
    fn role_context(self, role_name: &str) -> Result<T, FMDataError>;

    /// Add player context
    fn player_context(self, player_name: &str) -> Result<T, FMDataError>;

    /// Add category context
    fn category_context(self, category: &str) -> Result<T, FMDataError>;

    /// Add assignment algorithm context
    fn assignment_context(self, description: &str) -> Result<T, FMDataError>;

    /// Add filter processing context
    fn filter_context(self, line_num: usize) -> Result<T, FMDataError>;
}

impl<T, E> SelectionResult<T> for Result<T, E>
where
    E: std::fmt::Display,
{
    fn role_context(self, role_name: &str) -> Result<T, FMDataError> {
        self.map_err(|e| FMDataError::selection(format!("Role '{role_name}': {e}")))
    }

    fn player_context(self, player_name: &str) -> Result<T, FMDataError> {
        self.map_err(|e| FMDataError::selection(format!("Player '{player_name}': {e}")))
    }

    fn category_context(self, category: &str) -> Result<T, FMDataError> {
        self.map_err(|e| FMDataError::selection(format!("Category '{category}': {e}")))
    }

    fn assignment_context(self, description: &str) -> Result<T, FMDataError> {
        self.map_err(|e| FMDataError::selection(format!("Assignment {description}: {e}")))
    }

    fn filter_context(self, line_num: usize) -> Result<T, FMDataError> {
        self.map_err(|e| FMDataError::selection(format!("Filter line {line_num}: {e}")))
    }
}

/// Domain-specific extension trait for table processing Results
pub trait TableResult<T> {
    /// Add table parsing context
    fn table_context(self, operation: &str) -> Result<T, FMDataError>;

    /// Add row-specific context
    fn row_context(self, row_num: usize) -> Result<T, FMDataError>;

    /// Add column-specific context
    fn column_context(self, column_name: &str) -> Result<T, FMDataError>;

    /// Add validation context
    fn validate_context(self, check: &str) -> Result<T, FMDataError>;
}

impl<T, E> TableResult<T> for Result<T, E>
where
    E: std::fmt::Display,
{
    fn table_context(self, operation: &str) -> Result<T, FMDataError> {
        self.map_err(|e| FMDataError::table(format!("Table {operation}: {e}")))
    }

    fn row_context(self, row_num: usize) -> Result<T, FMDataError> {
        self.map_err(|e| FMDataError::table(format!("Row {row_num}: {e}")))
    }

    fn column_context(self, column_name: &str) -> Result<T, FMDataError> {
        self.map_err(|e| FMDataError::table(format!("Column '{column_name}': {e}")))
    }

    fn validate_context(self, check: &str) -> Result<T, FMDataError> {
        self.map_err(|e| FMDataError::table(format!("Validation ({check}): {e}")))
    }
}
