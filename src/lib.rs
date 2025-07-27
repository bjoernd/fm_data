pub mod auth;
pub mod config;
pub mod error;
pub mod progress;
pub mod sheets_client;
pub mod table;
pub mod validation;

pub use auth::{create_authenticator_and_token, get_secure_config_dir};
pub use config::Config;
pub use error::{FMDataError, Result};
pub use progress::{NoOpProgress, ProgressCallback, ProgressTracker};
pub use sheets_client::SheetsManager;
pub use table::{process_table_data, read_table, validate_data_size, validate_table_structure};
pub use validation::{DataValidator, IdValidator, PathValidator};
