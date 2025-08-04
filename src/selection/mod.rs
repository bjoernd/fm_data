pub mod algorithm;
pub mod categories;
pub mod formatter;
pub mod parser;
pub mod types;

// Re-export public API to maintain backward compatibility
pub use algorithm::{
    calculate_assignment_score, find_optimal_assignments, find_optimal_assignments_with_filters,
    is_player_eligible_for_role, parse_player_data,
};
pub use categories::{get_roles_for_category, is_valid_category, role_belongs_to_category};
pub use formatter::format_team_output;
pub use parser::{parse_role_file, parse_role_file_content, validate_roles};
pub use types::{
    Assignment, Footedness, Player, PlayerCategory, PlayerFilter, Role, RoleFileContent, Team,
    ABILITIES, VALID_ROLES,
};

// Keep any high-level orchestration functions here if needed
