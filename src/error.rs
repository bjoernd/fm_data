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

macro_rules! error_constructors {
    ($($name:ident => $variant:ident),*) => {
        $(
            pub fn $name<T: Into<String>>(message: T) -> Self {
                Self::$variant { message: message.into() }
            }
        )*
    };
}

impl FMDataError {
    error_constructors!(
        config => Config,
        auth => Auth,
        table => Table,
        sheets_api => SheetsApi,
        progress => Progress,
        selection => Selection
    );
}

pub type Result<T> = std::result::Result<T, FMDataError>;
