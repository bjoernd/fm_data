pub mod app_runner;
pub mod auth;
pub mod config;
pub mod constants;
pub mod error;
pub mod progress;
pub mod selection;
pub mod sheets_client;
pub mod table;
pub mod test_helpers;
pub mod validation;

pub use app_runner::{AppRunner, CLIArgumentValidator};
pub use auth::{create_authenticator_and_token, get_secure_config_dir};
pub use config::Config;
pub use error::{FMDataError, Result};
pub use progress::{NoOpProgress, ProgressCallback, ProgressTracker};
pub use selection::{
    find_optimal_assignments, format_team_output, parse_player_data, parse_role_file, Assignment,
    Footedness, Player, Role, Team,
};
pub use sheets_client::SheetsManager;
pub use table::{process_table_data, read_table, validate_data_size, validate_table_structure};
pub use validation::Validator;
