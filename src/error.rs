use thiserror::Error;

#[derive(Error, Debug)]
pub enum FMDataError {
    #[error("Configuration error: {message}")]
    Config { message: String },

    #[error("Authentication error: {message}")]
    Auth { message: String },

    #[error("Table processing error: {message}")]
    Table { message: String },

    #[error("Google Sheets API error: {message}")]
    SheetsApi { message: String },

    #[error("Progress tracking error: {message}")]
    Progress { message: String },

    #[error("Selection error: {message}")]
    Selection { message: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("OAuth2 error: {0}")]
    OAuth2(#[from] yup_oauth2::Error),
}

impl FMDataError {
    pub fn config<T: Into<String>>(message: T) -> Self {
        Self::Config {
            message: message.into(),
        }
    }

    pub fn auth<T: Into<String>>(message: T) -> Self {
        Self::Auth {
            message: message.into(),
        }
    }

    pub fn table<T: Into<String>>(message: T) -> Self {
        Self::Table {
            message: message.into(),
        }
    }

    pub fn sheets_api<T: Into<String>>(message: T) -> Self {
        Self::SheetsApi {
            message: message.into(),
        }
    }

    pub fn progress<T: Into<String>>(message: T) -> Self {
        Self::Progress {
            message: message.into(),
        }
    }

    pub fn selection<T: Into<String>>(message: T) -> Self {
        Self::Selection {
            message: message.into(),
        }
    }
}

pub type Result<T> = std::result::Result<T, FMDataError>;
