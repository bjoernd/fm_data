pub mod app_builder;
pub mod app_runner;
pub mod attributes;
pub mod auth;
pub mod cli;
#[cfg(feature = "image-processing")]
pub mod clipboard;
pub mod config;
pub mod constants;
pub mod domain;
pub mod error;
pub mod error_helpers;
pub mod error_messages;
#[cfg(feature = "image-processing")]
pub mod image_constants;
#[cfg(feature = "image-processing")]
pub mod image_data;
#[cfg(feature = "image-processing")]
pub mod image_output;
#[cfg(feature = "image-processing")]
pub mod image_processor;
#[cfg(feature = "image-processing")]
pub mod image_processor_pool;
#[cfg(feature = "image-processing")]
pub mod layout_manager;
#[cfg(feature = "image-processing")]
pub mod ocr_corrections;
pub mod progress;
pub mod selection;
pub mod setup_commands;
pub mod sheets_client;
pub mod sheets_repository;
pub mod table;
pub mod test_builders;
pub mod test_helpers;
pub mod traits;
pub mod types;
pub mod validation;
pub mod validators;

pub use app_builder::AppRunnerBuilder;
pub use app_runner::{AppRunner, CLIArgumentValidator};
pub use attributes::{
    AttributeSet, GoalkeepingAttribute, MentalAttribute, PhysicalAttribute, TechnicalAttribute,
};
pub use auth::{create_authenticator_and_token, get_secure_config_dir};
pub use cli::{
    BinarySpecificCLI, CommonArgs, CommonCLIArgs, ImageCLI, SelectorCLI, StandardCLIWrapper,
    UploaderCLI,
};
pub use config::Config;
pub use domain::{PlayerId, RoleId, SpreadsheetId};
pub use error::{FMDataError, Result};
pub use error_messages::{ErrorBuilder, ErrorCode};
#[cfg(feature = "image-processing")]
pub use image_data::{parse_player_from_ocr, ImagePlayer};
#[cfg(feature = "image-processing")]
pub use image_output::{format_player_data, format_player_data_verbose};
#[cfg(feature = "image-processing")]
pub use image_processor::{
    detect_footedness_optional, extract_text_from_image, load_image, preprocess_image,
    ImageProcessor, ProcessingConfig,
};
#[cfg(feature = "image-processing")]
pub use image_processor_pool::{ImageProcessorPool, ImageProcessorPoolBuilder};
#[cfg(feature = "image-processing")]
pub use layout_manager::{default_paths, LayoutManager};
#[cfg(feature = "image-processing")]
pub use ocr_corrections::OCRCorrector;
pub use progress::{
    create_progress_reporter, NoOpProgress, NoOpProgressReporter, ProgressCallback,
    ProgressReporter, ProgressTracker,
};
pub use selection::{
    find_optimal_assignments, find_optimal_assignments_with_filters, format_team_output,
    parse_player_data, parse_role_file, parse_role_file_content, Assignment, Footedness, Player,
    PlayerCategory, PlayerFilter, Role, RoleFileContent, Team,
};
pub use setup_commands::{
    AuthenticationSetup, ImageProcessorSetup, PlayerUploaderSetup, SetupCommand, TeamSelectorSetup,
};
pub use sheets_client::SheetsManager;
pub use table::{process_table_data, read_table, validate_data_size, validate_table_structure};
pub use test_builders::{
    ConfigDataBuilder, ImageDataBuilder, PlayerDataBuilder, PlayersDataBuilder, RoleFileBuilder,
};
pub use traits::{
    DataUploader, DefaultTableProcessor, DefaultTeamSelector, PlayerDataSource, SheetsDataUploader,
    SheetsPlayerDataSource, TableProcessor, TeamSelector,
};
pub use types::{Footedness as TypesFootedness, PlayerType};
pub use validation::Validator;
pub use validators::{
    AuthValidator, ConfigValidator, DataValidator, FileValidator, PlayerValidator, RoleValidator,
};
