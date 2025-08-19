use crate::browser::BrowserPlayer;
use crate::error::Result;
use crate::progress::{ProgressPublisher, ProgressReporter};
use crate::sheets_repository::SheetsRepository;
use async_trait::async_trait;
use log::{debug, info, warn};

/// Repository for browser data operations with Google Sheets integration
///
/// This repository handles fetching player data from Google Sheets in the A2:EQ58 range
/// and converting it to BrowserPlayer objects for use in the terminal UI.
pub struct BrowserRepository<T: SheetsRepository> {
    sheets_repository: T,
}

impl<T: SheetsRepository> BrowserRepository<T> {
    /// Create a new browser repository with the provided sheets repository
    pub fn new(sheets_repository: T) -> Self {
        Self { sheets_repository }
    }

    /// Fetch all valid players from the specified sheet
    ///
    /// Reads data from range A2:EQ58 (145 columns, up to 57 players) and filters out:
    /// - Rows with empty or whitespace-only names
    /// - Rows that fail to parse due to insufficient data
    ///
    /// # Arguments
    /// * `sheet_name` - Name of the sheet to read from (e.g., "Squad")
    ///
    /// # Returns
    /// Vector of valid BrowserPlayer objects, sorted by name
    pub async fn fetch_players(&self, sheet_name: &str) -> Result<Vec<BrowserPlayer>> {
        debug!("Fetching players from sheet: {}", sheet_name);

        // Fetch raw data from Google Sheets (A2:EQ58 = 145 columns, up to 57 rows)
        let raw_data = self
            .sheets_repository
            .read_data(sheet_name, "A2:EQ58")
            .await?;

        info!("Retrieved {} rows from {}", raw_data.len(), sheet_name);

        // Convert raw rows to BrowserPlayer objects, filtering out invalid ones
        let mut players: Vec<BrowserPlayer> = raw_data
            .into_iter()
            .enumerate()
            .filter_map(|(row_index, row)| match BrowserPlayer::from_row(&row) {
                Some(player) => {
                    if player.is_valid() {
                        Some(player)
                    } else {
                        debug!(
                            "Filtered out player with invalid name at row {}",
                            row_index + 2
                        );
                        None
                    }
                }
                None => {
                    debug!("Failed to parse player data at row {}", row_index + 2);
                    None
                }
            })
            .collect();

        // Sort players alphabetically by name for consistent display
        players.sort_by(|a, b| a.name.cmp(&b.name));

        let filtered_count = players.len();
        if filtered_count == 0 {
            warn!("No valid players found in sheet {}", sheet_name);
        } else {
            info!(
                "Successfully parsed {} valid players from {}",
                filtered_count, sheet_name
            );
        }

        Ok(players)
    }

    /// Fetch players with progress reporting
    ///
    /// Same as `fetch_players` but provides progress updates during the operation.
    /// Useful for UI applications that want to show loading progress.
    pub async fn fetch_players_with_progress(
        &self,
        sheet_name: &str,
        progress: &dyn ProgressReporter,
    ) -> Result<Vec<BrowserPlayer>> {
        progress.set_message(&format!("Fetching player data from {sheet_name}..."));

        debug!("Fetching players from sheet: {}", sheet_name);

        // Fetch raw data from Google Sheets
        let raw_data = self
            .sheets_repository
            .read_data(sheet_name, "A2:EQ58")
            .await?;

        progress.set_message("Processing player data...");
        info!("Retrieved {} rows from {}", raw_data.len(), sheet_name);

        // Convert and filter player data
        let mut players: Vec<BrowserPlayer> = raw_data
            .into_iter()
            .enumerate()
            .filter_map(|(row_index, row)| match BrowserPlayer::from_row(&row) {
                Some(player) => {
                    if player.is_valid() {
                        Some(player)
                    } else {
                        debug!(
                            "Filtered out player with invalid name at row {}",
                            row_index + 2
                        );
                        None
                    }
                }
                None => {
                    debug!("Failed to parse player data at row {}", row_index + 2);
                    None
                }
            })
            .collect();

        // Sort players alphabetically by name
        players.sort_by(|a, b| a.name.cmp(&b.name));

        let filtered_count = players.len();
        progress.set_message(&format!("Loaded {filtered_count} players"));

        if filtered_count == 0 {
            warn!("No valid players found in sheet {}", sheet_name);
        } else {
            info!(
                "Successfully parsed {} valid players from {}",
                filtered_count, sheet_name
            );
        }

        progress.inc(1);
        Ok(players)
    }

    /// Fetch players with event-driven progress reporting
    ///
    /// Same as `fetch_players_with_progress` but uses the event-driven progress system.
    pub async fn fetch_players_with_events(
        &self,
        sheet_name: &str,
        progress: &ProgressPublisher,
    ) -> Result<Vec<BrowserPlayer>> {
        progress.message(format!("Fetching player data from {sheet_name}..."));

        debug!("Fetching players from sheet: {}", sheet_name);

        // Fetch raw data from Google Sheets
        let raw_data = self
            .sheets_repository
            .read_data(sheet_name, "A2:EQ58")
            .await?;

        progress.message("Processing player data...");
        info!("Retrieved {} rows from {}", raw_data.len(), sheet_name);

        // Convert and filter player data
        let mut players: Vec<BrowserPlayer> = raw_data
            .into_iter()
            .enumerate()
            .filter_map(|(row_index, row)| match BrowserPlayer::from_row(&row) {
                Some(player) => {
                    if player.is_valid() {
                        Some(player)
                    } else {
                        debug!(
                            "Filtered out player with invalid name at row {}",
                            row_index + 2
                        );
                        None
                    }
                }
                None => {
                    debug!("Failed to parse player data at row {}", row_index + 2);
                    None
                }
            })
            .collect();

        // Sort players alphabetically by name
        players.sort_by(|a, b| a.name.cmp(&b.name));

        let filtered_count = players.len();
        progress.message(format!("Loaded {filtered_count} players"));

        if filtered_count == 0 {
            warn!("No valid players found in sheet {}", sheet_name);
        } else {
            info!(
                "Successfully parsed {} valid players from {}",
                filtered_count, sheet_name
            );
        }

        progress.increment(1);
        Ok(players)
    }

    /// Verify that the repository can access the spreadsheet and specified sheet
    ///
    /// This method performs basic connectivity and access checks without fetching data.
    /// Useful for validating configuration before attempting to load player data.
    pub async fn verify_access(
        &self,
        sheet_name: &str,
        progress: &dyn ProgressReporter,
    ) -> Result<()> {
        // Verify spreadsheet access
        self.sheets_repository
            .verify_spreadsheet_access(progress)
            .await?;

        // Verify the specific sheet exists
        self.sheets_repository
            .verify_sheet_exists(sheet_name, progress)
            .await?;

        Ok(())
    }

    /// Get a reference to the underlying sheets repository
    ///
    /// Provides access to the underlying SheetsRepository for advanced operations
    /// or integration with other parts of the application.
    pub fn sheets_repository(&self) -> &T {
        &self.sheets_repository
    }
}

/// Trait for browser data operations to enable testing with mock repositories
#[async_trait]
pub trait BrowserDataSource: Send + Sync {
    /// Fetch all valid players from the specified sheet
    async fn fetch_players(&self, sheet_name: &str) -> Result<Vec<BrowserPlayer>>;

    /// Fetch players with progress reporting
    async fn fetch_players_with_progress(
        &self,
        sheet_name: &str,
        progress: &dyn ProgressReporter,
    ) -> Result<Vec<BrowserPlayer>>;

    /// Verify access to the spreadsheet and sheet
    async fn verify_access(&self, sheet_name: &str, progress: &dyn ProgressReporter) -> Result<()>;
}

#[async_trait]
impl<T: SheetsRepository> BrowserDataSource for BrowserRepository<T> {
    async fn fetch_players(&self, sheet_name: &str) -> Result<Vec<BrowserPlayer>> {
        self.fetch_players(sheet_name).await
    }

    async fn fetch_players_with_progress(
        &self,
        sheet_name: &str,
        progress: &dyn ProgressReporter,
    ) -> Result<Vec<BrowserPlayer>> {
        self.fetch_players_with_progress(sheet_name, progress).await
    }

    async fn verify_access(&self, sheet_name: &str, progress: &dyn ProgressReporter) -> Result<()> {
        self.verify_access(sheet_name, progress).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::FMDataError;
    use crate::progress::NoOpProgressReporter;
    use crate::sheets_repository::SheetsRepository;
    use async_trait::async_trait;
    use std::collections::HashMap;

    /// Mock sheets repository for testing
    struct MockSheetsRepository {
        data: HashMap<String, Vec<Vec<String>>>,
        should_fail: bool,
    }

    impl MockSheetsRepository {
        fn new() -> Self {
            Self {
                data: HashMap::new(),
                should_fail: false,
            }
        }

        fn with_data(mut self, sheet: &str, range: &str, data: Vec<Vec<String>>) -> Self {
            let key = format!("{sheet}!{range}");
            self.data.insert(key, data);
            self
        }

        fn with_failure(mut self) -> Self {
            self.should_fail = true;
            self
        }
    }

    #[async_trait]
    impl SheetsRepository for MockSheetsRepository {
        async fn verify_spreadsheet_access(&self, _progress: &dyn ProgressReporter) -> Result<()> {
            if self.should_fail {
                Err(FMDataError::sheets_api("Mock failure"))
            } else {
                Ok(())
            }
        }

        async fn verify_sheet_exists(
            &self,
            _sheet_name: &str,
            _progress: &dyn ProgressReporter,
        ) -> Result<()> {
            if self.should_fail {
                Err(FMDataError::sheets_api("Mock failure"))
            } else {
                Ok(())
            }
        }

        async fn read_data(&self, sheet: &str, range: &str) -> Result<Vec<Vec<String>>> {
            if self.should_fail {
                return Err(FMDataError::sheets_api("Mock failure"));
            }

            let key = format!("{sheet}!{range}");
            Ok(self.data.get(&key).cloned().unwrap_or_default())
        }

        async fn upload_data(
            &self,
            _sheet: &str,
            _range: &str,
            _data: Vec<Vec<String>>,
        ) -> Result<()> {
            Ok(())
        }

        async fn upload_data_with_progress(
            &self,
            _sheet: &str,
            _range: &str,
            _data: Vec<Vec<String>>,
            _progress: &dyn ProgressReporter,
        ) -> Result<()> {
            Ok(())
        }

        async fn upload_data_with_events(
            &self,
            _sheet: &str,
            _range: &str,
            _data: Vec<Vec<String>>,
            _progress: &ProgressPublisher,
        ) -> Result<()> {
            Ok(())
        }

        async fn clear_range(&self, _sheet: &str, _range: &str) -> Result<()> {
            Ok(())
        }

        async fn clear_range_with_progress(
            &self,
            _sheet: &str,
            _range: &str,
            _progress: &dyn ProgressReporter,
        ) -> Result<()> {
            Ok(())
        }

        async fn clear_range_with_events(
            &self,
            _sheet: &str,
            _range: &str,
            _progress: &ProgressPublisher,
        ) -> Result<()> {
            Ok(())
        }
    }

    fn create_valid_player_row(name: &str) -> Vec<String> {
        let mut row = vec![name.to_string(), "25.0".to_string(), "Right".to_string()];
        // Add remaining 142 fields (145 total)
        for i in 3..145 {
            if i % 5 == 0 {
                row.push("--".to_string());
            } else {
                row.push(format!("{}", i % 20));
            }
        }
        row
    }

    #[tokio::test]
    async fn test_fetch_players_success() {
        let test_data = vec![
            create_valid_player_row("Alice"),
            create_valid_player_row("Bob"),
            create_valid_player_row("Charlie"),
        ];

        let mock_sheets = MockSheetsRepository::new().with_data("Squad", "A2:EQ58", test_data);

        let repository = BrowserRepository::new(mock_sheets);
        let players = repository.fetch_players("Squad").await.unwrap();

        assert_eq!(players.len(), 3);
        // Players should be sorted by name
        assert_eq!(players[0].name, "Alice");
        assert_eq!(players[1].name, "Bob");
        assert_eq!(players[2].name, "Charlie");
    }

    #[tokio::test]
    async fn test_fetch_players_filters_invalid_names() {
        let test_data = vec![
            create_valid_player_row("Valid Player"),
            vec!["".to_string(); 145],    // Empty name
            vec!["   ".to_string(); 145], // Whitespace name
            create_valid_player_row("Another Valid Player"),
        ];

        let mock_sheets = MockSheetsRepository::new().with_data("Squad", "A2:EQ58", test_data);

        let repository = BrowserRepository::new(mock_sheets);
        let players = repository.fetch_players("Squad").await.unwrap();

        assert_eq!(players.len(), 2);
        assert_eq!(players[0].name, "Another Valid Player");
        assert_eq!(players[1].name, "Valid Player");
    }

    #[tokio::test]
    async fn test_fetch_players_handles_insufficient_columns() {
        let test_data = vec![
            create_valid_player_row("Valid Player"),
            vec!["Short Row".to_string()], // Only 1 column instead of 145
        ];

        let mock_sheets = MockSheetsRepository::new().with_data("Squad", "A2:EQ58", test_data);

        let repository = BrowserRepository::new(mock_sheets);
        let players = repository.fetch_players("Squad").await.unwrap();

        assert_eq!(players.len(), 1);
        assert_eq!(players[0].name, "Valid Player");
    }

    #[tokio::test]
    async fn test_fetch_players_empty_sheet() {
        let mock_sheets = MockSheetsRepository::new().with_data("Squad", "A2:EQ58", vec![]);

        let repository = BrowserRepository::new(mock_sheets);
        let players = repository.fetch_players("Squad").await.unwrap();

        assert_eq!(players.len(), 0);
    }

    #[tokio::test]
    async fn test_fetch_players_with_progress() {
        let test_data = vec![create_valid_player_row("Test Player")];

        let mock_sheets = MockSheetsRepository::new().with_data("Squad", "A2:EQ58", test_data);

        let repository = BrowserRepository::new(mock_sheets);
        let progress = NoOpProgressReporter;
        let players = repository
            .fetch_players_with_progress("Squad", &progress)
            .await
            .unwrap();

        assert_eq!(players.len(), 1);
        assert_eq!(players[0].name, "Test Player");
    }

    #[tokio::test]
    async fn test_verify_access_success() {
        let mock_sheets = MockSheetsRepository::new();
        let repository = BrowserRepository::new(mock_sheets);
        let progress = NoOpProgressReporter;

        let result = repository.verify_access("Squad", &progress).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_verify_access_failure() {
        let mock_sheets = MockSheetsRepository::new().with_failure();
        let repository = BrowserRepository::new(mock_sheets);
        let progress = NoOpProgressReporter;

        let result = repository.verify_access("Squad", &progress).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_sheets_repository_access() {
        let mock_sheets = MockSheetsRepository::new();
        let repository = BrowserRepository::new(mock_sheets);

        // Should be able to access the underlying repository
        let _sheets_ref = repository.sheets_repository();
    }

    #[tokio::test]
    async fn test_browser_data_source_trait() {
        let test_data = vec![create_valid_player_row("Trait Test Player")];
        let mock_sheets = MockSheetsRepository::new().with_data("Squad", "A2:EQ58", test_data);

        let repository: Box<dyn BrowserDataSource> = Box::new(BrowserRepository::new(mock_sheets));
        let players = repository.fetch_players("Squad").await.unwrap();

        assert_eq!(players.len(), 1);
        assert_eq!(players[0].name, "Trait Test Player");
    }
}
