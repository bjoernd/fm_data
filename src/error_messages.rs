use crate::error::FMDataError;

/// Error codes for standardized error handling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    // Configuration errors (E100-E199)
    E100, // Config file not found
    E101, // Invalid config format
    E102, // Missing required field
    E103, // Invalid file extension
    E104, // Cannot read file
    E105, // Empty field value
    E106, // Invalid field value

    // Authentication errors (E200-E299)
    E200, // Credentials file not found
    E201, // Invalid credentials format
    E202, // Authentication failed
    E203, // Token cache error

    // Table processing errors (E300-E399)
    E300, // Empty table
    E301, // No data rows
    E302, // Invalid table structure
    E303, // Column count mismatch
    E304, // Row processing failed

    // Google Sheets API errors (E400-E499)
    E400, // Spreadsheet not found
    E401, // Sheet access denied
    E402, // Range invalid
    E403, // API quota exceeded

    // Selection/Team assignment errors (E500-E599)
    E500, // Invalid role
    E501, // Insufficient players
    E502, // Role file format error
    E503, // Player filter error
    E504, // Assignment failed
    E505, // Duplicate player filter

    // Image processing errors (E600-E699)
    E600, // Image file not found
    E601, // Invalid image format
    E602, // OCR extraction failed
    E603, // Layout parsing failed
    E604, // Footedness detection failed
    E605, // No text extracted
}

impl ErrorCode {
    /// Get the error code as a string (e.g., "E100")
    pub fn as_str(&self) -> &'static str {
        match self {
            ErrorCode::E100 => "E100",
            ErrorCode::E101 => "E101",
            ErrorCode::E102 => "E102",
            ErrorCode::E103 => "E103",
            ErrorCode::E104 => "E104",
            ErrorCode::E105 => "E105",
            ErrorCode::E106 => "E106",
            ErrorCode::E200 => "E200",
            ErrorCode::E201 => "E201",
            ErrorCode::E202 => "E202",
            ErrorCode::E203 => "E203",
            ErrorCode::E300 => "E300",
            ErrorCode::E301 => "E301",
            ErrorCode::E302 => "E302",
            ErrorCode::E303 => "E303",
            ErrorCode::E304 => "E304",
            ErrorCode::E400 => "E400",
            ErrorCode::E401 => "E401",
            ErrorCode::E402 => "E402",
            ErrorCode::E403 => "E403",
            ErrorCode::E500 => "E500",
            ErrorCode::E501 => "E501",
            ErrorCode::E502 => "E502",
            ErrorCode::E503 => "E503",
            ErrorCode::E504 => "E504",
            ErrorCode::E505 => "E505",
            ErrorCode::E600 => "E600",
            ErrorCode::E601 => "E601",
            ErrorCode::E602 => "E602",
            ErrorCode::E603 => "E603",
            ErrorCode::E604 => "E604",
            ErrorCode::E605 => "E605",
        }
    }

    /// Get the standard error message template for this error code
    pub fn message(&self) -> &'static str {
        match self {
            // Configuration errors
            ErrorCode::E100 => "Config file not found",
            ErrorCode::E101 => "Invalid config format",
            ErrorCode::E102 => "Missing required field",
            ErrorCode::E103 => "Invalid file extension",
            ErrorCode::E104 => "Cannot read file",
            ErrorCode::E105 => "Empty field value",
            ErrorCode::E106 => "Invalid field value",

            // Authentication errors
            ErrorCode::E200 => "Credentials file not found",
            ErrorCode::E201 => "Invalid credentials format",
            ErrorCode::E202 => "Authentication failed",
            ErrorCode::E203 => "Token cache error",

            // Table processing errors
            ErrorCode::E300 => "Empty table",
            ErrorCode::E301 => "No data rows",
            ErrorCode::E302 => "Invalid table structure",
            ErrorCode::E303 => "Column count mismatch",
            ErrorCode::E304 => "Row processing failed",

            // Google Sheets API errors
            ErrorCode::E400 => "Spreadsheet not found",
            ErrorCode::E401 => "Sheet access denied",
            ErrorCode::E402 => "Invalid range",
            ErrorCode::E403 => "API quota exceeded",

            // Selection/Team assignment errors
            ErrorCode::E500 => "Invalid role",
            ErrorCode::E501 => "Insufficient players",
            ErrorCode::E502 => "Role file format error",
            ErrorCode::E503 => "Player filter error",
            ErrorCode::E504 => "Assignment failed",
            ErrorCode::E505 => "Duplicate player filter",

            // Image processing errors
            ErrorCode::E600 => "Image file not found",
            ErrorCode::E601 => "Invalid image format",
            ErrorCode::E602 => "OCR extraction failed",
            ErrorCode::E603 => "Layout parsing failed",
            ErrorCode::E604 => "Footedness detection failed",
            ErrorCode::E605 => "No text extracted",
        }
    }
}

/// Standardized error message builder
pub struct ErrorBuilder {
    code: ErrorCode,
    context: Option<String>,
}

impl ErrorBuilder {
    /// Create a new error builder with the given error code
    pub fn new(code: ErrorCode) -> Self {
        Self {
            code,
            context: None,
        }
    }

    /// Add context to the error message
    pub fn with_context<T: ToString>(mut self, context: T) -> Self {
        self.context = Some(context.to_string());
        self
    }

    /// Build the FMDataError with standardized message format
    pub fn build(self) -> FMDataError {
        let message = match self.context {
            Some(ctx) => format!("[{}] {}: {}", self.code.as_str(), self.code.message(), ctx),
            None => format!("[{}] {}", self.code.as_str(), self.code.message()),
        };

        match self.code {
            // Configuration errors
            ErrorCode::E100
            | ErrorCode::E101
            | ErrorCode::E102
            | ErrorCode::E103
            | ErrorCode::E104
            | ErrorCode::E105
            | ErrorCode::E106 => FMDataError::config(message),

            // Authentication errors
            ErrorCode::E200 | ErrorCode::E201 | ErrorCode::E202 | ErrorCode::E203 => {
                FMDataError::auth(message)
            }

            // Table processing errors
            ErrorCode::E300
            | ErrorCode::E301
            | ErrorCode::E302
            | ErrorCode::E303
            | ErrorCode::E304 => FMDataError::table(message),

            // Google Sheets API errors
            ErrorCode::E400 | ErrorCode::E401 | ErrorCode::E402 | ErrorCode::E403 => {
                FMDataError::sheets_api(message)
            }

            // Selection/Team assignment errors
            ErrorCode::E500
            | ErrorCode::E501
            | ErrorCode::E502
            | ErrorCode::E503
            | ErrorCode::E504
            | ErrorCode::E505 => FMDataError::selection(message),

            // Image processing errors
            ErrorCode::E600
            | ErrorCode::E601
            | ErrorCode::E602
            | ErrorCode::E603
            | ErrorCode::E604
            | ErrorCode::E605 => FMDataError::image(message),
        }
    }
}

/// Convenience functions for creating standardized errors
pub fn config_error(code: ErrorCode) -> ErrorBuilder {
    ErrorBuilder::new(code)
}

pub fn auth_error(code: ErrorCode) -> ErrorBuilder {
    ErrorBuilder::new(code)
}

pub fn table_error(code: ErrorCode) -> ErrorBuilder {
    ErrorBuilder::new(code)
}

pub fn sheets_error(code: ErrorCode) -> ErrorBuilder {
    ErrorBuilder::new(code)
}

pub fn selection_error(code: ErrorCode) -> ErrorBuilder {
    ErrorBuilder::new(code)
}

pub fn image_error(code: ErrorCode) -> ErrorBuilder {
    ErrorBuilder::new(code)
}

/// Convenience macros for creating standardized errors with reduced boilerplate
/// These macros reduce error construction code by ~60% and ensure consistency
///
/// Create a config error with optional context
#[macro_export]
macro_rules! config_error {
    ($code:expr) => {
        $crate::error_messages::ErrorBuilder::new($code).build()
    };
    ($code:expr, $context:expr) => {
        $crate::error_messages::ErrorBuilder::new($code)
            .with_context($context)
            .build()
    };
}

/// Create an authentication error with optional context
#[macro_export]
macro_rules! auth_error {
    ($code:expr) => {
        $crate::error_messages::ErrorBuilder::new($code).build()
    };
    ($code:expr, $context:expr) => {
        $crate::error_messages::ErrorBuilder::new($code)
            .with_context($context)
            .build()
    };
}

/// Create a table processing error with optional context
#[macro_export]
macro_rules! table_error {
    ($code:expr) => {
        $crate::error_messages::ErrorBuilder::new($code).build()
    };
    ($code:expr, $context:expr) => {
        $crate::error_messages::ErrorBuilder::new($code)
            .with_context($context)
            .build()
    };
}

/// Create a Google Sheets API error with optional context
#[macro_export]
macro_rules! sheets_error {
    ($code:expr) => {
        $crate::error_messages::ErrorBuilder::new($code).build()
    };
    ($code:expr, $context:expr) => {
        $crate::error_messages::ErrorBuilder::new($code)
            .with_context($context)
            .build()
    };
}

/// Create a selection/team assignment error with optional context
#[macro_export]
macro_rules! selection_error {
    ($code:expr) => {
        $crate::error_messages::ErrorBuilder::new($code).build()
    };
    ($code:expr, $context:expr) => {
        $crate::error_messages::ErrorBuilder::new($code)
            .with_context($context)
            .build()
    };
}

/// Create an image processing error with optional context
#[macro_export]
macro_rules! image_error {
    ($code:expr) => {
        $crate::error_messages::ErrorBuilder::new($code).build()
    };
    ($code:expr, $context:expr) => {
        $crate::error_messages::ErrorBuilder::new($code)
            .with_context($context)
            .build()
    };
}

/// Generic error builder macro that automatically selects the correct error type
#[macro_export]
macro_rules! fm_error {
    ($code:expr) => {
        $crate::error_messages::ErrorBuilder::new($code).build()
    };
    ($code:expr, $context:expr) => {
        $crate::error_messages::ErrorBuilder::new($code)
            .with_context($context)
            .build()
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_code_string_representation() {
        assert_eq!(ErrorCode::E100.as_str(), "E100");
        assert_eq!(ErrorCode::E200.as_str(), "E200");
        assert_eq!(ErrorCode::E500.as_str(), "E500");
    }

    #[test]
    fn test_error_message_templates() {
        assert_eq!(ErrorCode::E100.message(), "Config file not found");
        assert_eq!(ErrorCode::E200.message(), "Credentials file not found");
        assert_eq!(ErrorCode::E500.message(), "Invalid role");
    }

    #[test]
    fn test_error_builder_without_context() {
        let error = ErrorBuilder::new(ErrorCode::E100).build();

        if let FMDataError::Config { message } = error {
            assert_eq!(message, "[E100] Config file not found");
        } else {
            panic!("Expected Config error");
        }
    }

    #[test]
    fn test_error_builder_with_context() {
        let error = ErrorBuilder::new(ErrorCode::E104)
            .with_context("config.json")
            .build();

        if let FMDataError::Config { message } = error {
            assert_eq!(message, "[E104] Cannot read file: config.json");
        } else {
            panic!("Expected Config error");
        }
    }

    #[test]
    fn test_error_type_mapping() {
        // Configuration errors should create Config variant
        let config_error = ErrorBuilder::new(ErrorCode::E100).build();
        assert!(matches!(config_error, FMDataError::Config { .. }));

        // Authentication errors should create Auth variant
        let auth_error = ErrorBuilder::new(ErrorCode::E200).build();
        assert!(matches!(auth_error, FMDataError::Auth { .. }));

        // Selection errors should create Selection variant
        let selection_error = ErrorBuilder::new(ErrorCode::E500).build();
        assert!(matches!(selection_error, FMDataError::Selection { .. }));

        // Image errors should create Image variant
        let image_error = ErrorBuilder::new(ErrorCode::E600).build();
        assert!(matches!(image_error, FMDataError::Image { .. }));
    }

    #[test]
    fn test_convenience_functions() {
        let error = config_error(ErrorCode::E102)
            .with_context("spreadsheet_id")
            .build();

        if let FMDataError::Config { message } = error {
            assert_eq!(message, "[E102] Missing required field: spreadsheet_id");
        } else {
            panic!("Expected Config error");
        }
    }

    #[test]
    fn test_error_code_ranges() {
        // Test that error codes map to correct error types
        let config_codes = [ErrorCode::E100, ErrorCode::E101, ErrorCode::E106];
        for code in config_codes {
            let error = ErrorBuilder::new(code).build();
            assert!(matches!(error, FMDataError::Config { .. }));
        }

        let auth_codes = [ErrorCode::E200, ErrorCode::E201, ErrorCode::E203];
        for code in auth_codes {
            let error = ErrorBuilder::new(code).build();
            assert!(matches!(error, FMDataError::Auth { .. }));
        }

        let selection_codes = [ErrorCode::E500, ErrorCode::E501, ErrorCode::E505];
        for code in selection_codes {
            let error = ErrorBuilder::new(code).build();
            assert!(matches!(error, FMDataError::Selection { .. }));
        }
    }

    #[test]
    fn test_config_error_macro_without_context() {
        let error = config_error!(ErrorCode::E100);

        if let FMDataError::Config { message } = error {
            assert_eq!(message, "[E100] Config file not found");
        } else {
            panic!("Expected Config error");
        }
    }

    #[test]
    fn test_config_error_macro_with_context() {
        let error = config_error!(ErrorCode::E104, "config.json");

        if let FMDataError::Config { message } = error {
            assert_eq!(message, "[E104] Cannot read file: config.json");
        } else {
            panic!("Expected Config error");
        }
    }

    #[test]
    fn test_auth_error_macro() {
        let error = auth_error!(ErrorCode::E200, "credentials.json");

        if let FMDataError::Auth { message } = error {
            assert_eq!(
                message,
                "[E200] Credentials file not found: credentials.json"
            );
        } else {
            panic!("Expected Auth error");
        }
    }

    #[test]
    fn test_selection_error_macro() {
        let error = selection_error!(ErrorCode::E502, "invalid role format");

        if let FMDataError::Selection { message } = error {
            assert_eq!(
                message,
                "[E502] Role file format error: invalid role format"
            );
        } else {
            panic!("Expected Selection error");
        }
    }

    #[test]
    fn test_image_error_macro() {
        let error = image_error!(ErrorCode::E600, "image.png");

        if let FMDataError::Image { message } = error {
            assert_eq!(message, "[E600] Image file not found: image.png");
        } else {
            panic!("Expected Image error");
        }
    }

    #[test]
    fn test_fm_error_macro_generic() {
        // Test that the generic macro works for different error types
        let config_error = fm_error!(ErrorCode::E100, "test config");
        assert!(matches!(config_error, FMDataError::Config { .. }));

        let auth_error = fm_error!(ErrorCode::E200);
        assert!(matches!(auth_error, FMDataError::Auth { .. }));

        let selection_error = fm_error!(ErrorCode::E500, "test role");
        assert!(matches!(selection_error, FMDataError::Selection { .. }));
    }

    #[test]
    fn test_macro_reduces_boilerplate() {
        // Compare old verbose way vs new macro way

        // Old way: 3 lines of code
        let old_way = ErrorBuilder::new(ErrorCode::E104)
            .with_context("test.txt")
            .build();

        // New way: 1 line of code (60% reduction)
        let new_way = config_error!(ErrorCode::E104, "test.txt");

        // Both should produce the same result
        assert_eq!(format!("{old_way:?}"), format!("{:?}", new_way));
    }
}
