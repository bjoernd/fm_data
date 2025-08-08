pub mod app_builder;
pub mod app_runner;
pub mod auth;
pub mod cli;
pub mod config;
pub mod constants;
pub mod error;
pub mod error_helpers;
pub mod image_data;
pub mod image_output;
pub mod image_processor;
pub mod progress;
pub mod selection;
pub mod sheets_client;
pub mod table;
pub mod test_helpers;
pub mod validation;
pub mod validators;

pub use app_builder::AppRunnerBuilder;
pub use app_runner::{AppRunner, CLIArgumentValidator};
pub use auth::{create_authenticator_and_token, get_secure_config_dir};
pub use cli::{CommonArgs, CommonCLIArgs, ImageCLI, SelectorCLI, UploaderCLI};
pub use config::Config;
pub use error::{FMDataError, Result};
pub use image_data::{
    parse_player_from_ocr, Footedness as ImageFootedness, ImagePlayer, PlayerType,
};
pub use image_output::{format_player_data, format_player_data_verbose};
pub use image_processor::{extract_text_from_image, load_image, preprocess_image};
pub use progress::{
    create_progress_reporter, NoOpProgress, NoOpProgressReporter, ProgressCallback,
    ProgressReporter, ProgressTracker,
};
pub use selection::{
    find_optimal_assignments, find_optimal_assignments_with_filters, format_team_output,
    parse_player_data, parse_role_file, parse_role_file_content, Assignment, Footedness, Player,
    PlayerCategory, PlayerFilter, Role, RoleFileContent, Team,
};
pub use sheets_client::SheetsManager;
pub use table::{process_table_data, read_table, validate_data_size, validate_table_structure};
pub use validation::Validator;
pub use validators::{
    AuthValidator, ConfigValidator, DataValidator, FileValidator, PlayerValidator, RoleValidator,
};
