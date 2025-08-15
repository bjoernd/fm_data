use crate::error::Result;
use crate::selection::types::{Player, PlayerFilter, Role, Team};
use async_trait::async_trait;

/// Trait for team selection functionality
/// Enables dependency injection and multiple selection algorithm implementations
#[async_trait]
pub trait TeamSelector: Send + Sync {
    /// Select optimal team assignments from players and roles
    async fn select_team(
        &self,
        players: Vec<Player>,
        roles: Vec<Role>,
        filters: &[PlayerFilter],
    ) -> Result<Team>;

    /// Select optimal team assignments without filters (backward compatibility)
    async fn select_team_simple(&self, players: Vec<Player>, roles: Vec<Role>) -> Result<Team>;
}

/// Trait for loading player data from various sources
/// Abstracts data source to enable testing with mock data and support multiple backends
#[async_trait]
pub trait PlayerDataSource: Send + Sync {
    /// Load players from the data source
    async fn load_players(&self) -> Result<Vec<Player>>;

    /// Load players from a specific range or sheet
    async fn load_players_from_range(&self, sheet: &str, range: &str) -> Result<Vec<Player>>;

    /// Verify the data source is accessible
    async fn verify_access(&self) -> Result<()>;
}

/// Trait for uploading/exporting data to various destinations
/// Enables multiple output formats and destinations
#[async_trait]
pub trait DataUploader: Send + Sync {
    /// Upload tabular data to the destination
    async fn upload_data(&self, data: Vec<Vec<String>>) -> Result<()>;

    /// Upload data to a specific location/sheet
    async fn upload_data_to_location(&self, location: &str, data: Vec<Vec<String>>) -> Result<()>;

    /// Clear data at a specific location before uploading
    async fn upload_data_with_clear(&self, location: &str, data: Vec<Vec<String>>) -> Result<()>;
}

/// Trait for processing HTML table data
/// Abstracts table processing to enable different parsing strategies
pub trait TableProcessor: Send + Sync {
    /// Process HTML content and extract table data
    fn process_html(&self, html_content: &str) -> Result<Vec<Vec<String>>>;

    /// Process HTML file and extract table data
    fn process_html_file(&self, file_path: &str) -> Result<Vec<Vec<String>>>;

    /// Validate table structure and consistency
    fn validate_table(&self, data: &[Vec<String>]) -> Result<()>;
}

/// Default implementation of TeamSelector using the existing algorithm
pub struct DefaultTeamSelector;

impl DefaultTeamSelector {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DefaultTeamSelector {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl TeamSelector for DefaultTeamSelector {
    async fn select_team(
        &self,
        players: Vec<Player>,
        roles: Vec<Role>,
        filters: &[PlayerFilter],
    ) -> Result<Team> {
        // Use the existing algorithm implementation
        crate::selection::find_optimal_assignments_with_filters(players, roles, filters)
    }

    async fn select_team_simple(&self, players: Vec<Player>, roles: Vec<Role>) -> Result<Team> {
        // Use the existing algorithm implementation without filters
        crate::selection::find_optimal_assignments(players, roles)
    }
}

/// Google Sheets implementation of PlayerDataSource
pub struct SheetsPlayerDataSource {
    repository: Box<dyn crate::sheets_repository::SheetsRepository>,
    sheet_name: String,
    range: String,
}

impl SheetsPlayerDataSource {
    pub fn new(
        repository: Box<dyn crate::sheets_repository::SheetsRepository>,
        sheet_name: String,
        range: String,
    ) -> Self {
        Self {
            repository,
            sheet_name,
            range,
        }
    }
}

#[async_trait]
impl PlayerDataSource for SheetsPlayerDataSource {
    async fn load_players(&self) -> Result<Vec<Player>> {
        let data = self
            .repository
            .read_data(&self.sheet_name, &self.range)
            .await?;
        crate::selection::parse_player_data(data)
    }

    async fn load_players_from_range(&self, sheet: &str, range: &str) -> Result<Vec<Player>> {
        let data = self.repository.read_data(sheet, range).await?;
        crate::selection::parse_player_data(data)
    }

    async fn verify_access(&self) -> Result<()> {
        self.repository
            .verify_sheet_exists(&self.sheet_name, &crate::progress::NoOpProgressReporter)
            .await
    }
}

/// Google Sheets implementation of DataUploader
pub struct SheetsDataUploader {
    repository: Box<dyn crate::sheets_repository::SheetsRepository>,
    sheet_name: String,
    range: String,
}

impl SheetsDataUploader {
    pub fn new(
        repository: Box<dyn crate::sheets_repository::SheetsRepository>,
        sheet_name: String,
        range: String,
    ) -> Self {
        Self {
            repository,
            sheet_name,
            range,
        }
    }
}

#[async_trait]
impl DataUploader for SheetsDataUploader {
    async fn upload_data(&self, data: Vec<Vec<String>>) -> Result<()> {
        self.repository
            .upload_data(&self.sheet_name, &self.range, data)
            .await
    }

    async fn upload_data_to_location(&self, location: &str, data: Vec<Vec<String>>) -> Result<()> {
        self.repository
            .upload_data(&self.sheet_name, location, data)
            .await
    }

    async fn upload_data_with_clear(&self, location: &str, data: Vec<Vec<String>>) -> Result<()> {
        self.repository
            .clear_range(&self.sheet_name, location)
            .await?;
        self.repository
            .upload_data(&self.sheet_name, location, data)
            .await
    }
}

/// Default implementation of TableProcessor using existing table processing logic
pub struct DefaultTableProcessor;

impl DefaultTableProcessor {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DefaultTableProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl TableProcessor for DefaultTableProcessor {
    fn process_html(&self, html_content: &str) -> Result<Vec<Vec<String>>> {
        use table_extract::Table;

        let table = Table::find_first(html_content).ok_or_else(|| {
            crate::error_helpers::table_processing_error(
                None,
                None,
                "No table found in the provided HTML content",
            )
        })?;

        crate::table::process_table_data(&table)
    }

    fn process_html_file(&self, file_path: &str) -> Result<Vec<Vec<String>>> {
        // For sync trait, we need to use blocking operations
        // This could be improved with async trait methods in the future
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| crate::error::FMDataError::config(format!("Runtime error: {e}")))?;

        rt.block_on(async {
            let table = crate::table::read_table(file_path).await?;
            crate::table::validate_table_structure(&table)?;
            crate::table::process_table_data(&table)
        })
    }

    fn validate_table(&self, data: &[Vec<String>]) -> Result<()> {
        crate::validators::DataValidator::validate_row_consistency(data)?;
        crate::validators::DataValidator::validate_non_empty_data(data)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::PlayerId;
    use crate::selection::types::{Footedness, PlayerCategory, PlayerFilter};

    fn create_test_player(name: &str) -> Player {
        Player::new(
            name.to_string(),
            25,
            Footedness::Right,
            vec![Some(10.0); 47], // All abilities at 10.0
            Some(50.0),
            vec![Some(15.0); 94], // All role ratings at 15.0
        )
        .unwrap()
    }

    fn create_test_role(name: &str) -> Role {
        Role::new(name).unwrap()
    }

    #[tokio::test]
    async fn test_default_team_selector() {
        let selector = DefaultTeamSelector::new();

        // Create a valid team setup with enough players and exactly 11 roles
        let players = (1..=15)
            .map(|i| create_test_player(&format!("Player {i}")))
            .collect::<Vec<_>>();

        // Create exactly 11 roles for a valid formation
        let roles = vec![
            create_test_role("GK"),
            create_test_role("CD(d)"),
            create_test_role("CD(s)"),
            create_test_role("FB(d) R"),
            create_test_role("FB(d) L"),
            create_test_role("CM(d)"),
            create_test_role("CM(s)"),
            create_test_role("CM(a)"),
            create_test_role("W(s) R"),
            create_test_role("W(s) L"),
            create_test_role("CF(s)"),
        ];

        // Test with filters - only allow Player 1 to be goalkeeper
        let filters = vec![PlayerFilter::new(
            PlayerId::new("Player 1").unwrap(),
            vec![PlayerCategory::Goal],
        )];

        let team = selector
            .select_team(players.clone(), roles.clone(), &filters)
            .await;
        assert!(team.is_ok(), "Team selection with filters should succeed");

        if let Ok(team_result) = team {
            // Should have exactly 11 assignments
            assert_eq!(
                team_result.assignments.len(),
                11,
                "Team should have exactly 11 assignments"
            );
        }

        // Test without filters - should work with 11 roles
        let team_simple = selector.select_team_simple(players, roles).await;
        assert!(team_simple.is_ok(), "Simple team selection should succeed");

        if let Ok(team_simple_result) = team_simple {
            assert_eq!(
                team_simple_result.assignments.len(),
                11,
                "Simple team should have exactly 11 assignments"
            );
        }
    }

    #[test]
    fn test_default_table_processor() {
        let processor = DefaultTableProcessor::new();

        // Test HTML processing with a simple table
        let html_content = r#"
            <table>
                <tr><td>Name</td><td>Age</td></tr>
                <tr><td>Player 1</td><td>25</td></tr>
                <tr><td>Player 2</td><td>27</td></tr>
            </table>
        "#;

        let result = processor.process_html(html_content);
        assert!(result.is_ok(), "HTML processing should succeed");

        let data = result.unwrap();
        assert_eq!(data.len(), 3, "Should have 3 rows (header + 2 data)");
        assert_eq!(data[0], vec!["Name", "Age"], "Header row should match");
        assert_eq!(
            data[1],
            vec!["Player 1", "25"],
            "First data row should match"
        );
        assert_eq!(
            data[2],
            vec!["Player 2", "27"],
            "Second data row should match"
        );
    }

    #[test]
    fn test_table_validation() {
        let processor = DefaultTableProcessor::new();

        // Test valid data
        let valid_data = vec![
            vec!["Name".to_string(), "Age".to_string()],
            vec!["Player 1".to_string(), "25".to_string()],
            vec!["Player 2".to_string(), "27".to_string()],
        ];

        let result = processor.validate_table(&valid_data);
        assert!(result.is_ok(), "Valid table should pass validation");

        // Test invalid data (inconsistent row lengths)
        let invalid_data = vec![
            vec!["Name".to_string(), "Age".to_string()],
            vec!["Player 1".to_string()], // Missing age
        ];

        let result = processor.validate_table(&invalid_data);
        assert!(result.is_err(), "Invalid table should fail validation");
    }

    #[test]
    fn test_trait_object_creation() {
        // Test that we can create trait objects for dependency injection
        let team_selector: Box<dyn TeamSelector> = Box::new(DefaultTeamSelector::new());
        let table_processor: Box<dyn TableProcessor> = Box::new(DefaultTableProcessor::new());

        // Verify they implement the expected traits (just check they can be created)
        drop(team_selector);
        drop(table_processor);
    }
}
