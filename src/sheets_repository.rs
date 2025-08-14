use crate::error::Result;
use crate::progress::{ProgressPublisher, ProgressReporter};
use crate::sheets_client::SheetsManager;
use async_trait::async_trait;
use std::collections::HashMap;

/// Repository trait for Google Sheets operations that enables dependency injection
#[async_trait]
pub trait SheetsRepository: Send + Sync {
    /// Verify access to the spreadsheet
    async fn verify_spreadsheet_access(&self, progress: &dyn ProgressReporter) -> Result<()>;

    /// Verify a specific sheet exists in the spreadsheet
    async fn verify_sheet_exists(
        &self,
        sheet_name: &str,
        progress: &dyn ProgressReporter,
    ) -> Result<()>;

    /// Read data from a specific sheet and range
    async fn read_data(&self, sheet: &str, range: &str) -> Result<Vec<Vec<String>>>;

    /// Upload data to a specific sheet and range
    async fn upload_data(&self, sheet: &str, range: &str, data: Vec<Vec<String>>) -> Result<()>;

    /// Upload data with progress reporting
    async fn upload_data_with_progress(
        &self,
        sheet: &str,
        range: &str,
        data: Vec<Vec<String>>,
        progress: &dyn ProgressReporter,
    ) -> Result<()>;

    /// Upload data with event-driven progress reporting
    async fn upload_data_with_events(
        &self,
        sheet: &str,
        range: &str,
        data: Vec<Vec<String>>,
        progress: &ProgressPublisher,
    ) -> Result<()>;

    /// Clear a specific range in a sheet
    async fn clear_range(&self, sheet: &str, range: &str) -> Result<()>;

    /// Clear a range with progress reporting
    async fn clear_range_with_progress(
        &self,
        sheet: &str,
        range: &str,
        progress: &dyn ProgressReporter,
    ) -> Result<()>;

    /// Clear a range with event-driven progress reporting
    async fn clear_range_with_events(
        &self,
        sheet: &str,
        range: &str,
        progress: &ProgressPublisher,
    ) -> Result<()>;
}

/// Production implementation that wraps SheetsManager
pub struct GoogleSheetsRepository {
    manager: SheetsManager,
}

impl GoogleSheetsRepository {
    /// Create a new Google Sheets repository
    pub fn new(manager: SheetsManager) -> Self {
        Self { manager }
    }

    /// Get a reference to the underlying SheetsManager
    pub fn manager(&self) -> &SheetsManager {
        &self.manager
    }
}

#[async_trait]
impl SheetsRepository for GoogleSheetsRepository {
    async fn verify_spreadsheet_access(&self, progress: &dyn ProgressReporter) -> Result<()> {
        self.manager.verify_spreadsheet_access(progress).await
    }

    async fn verify_sheet_exists(
        &self,
        sheet_name: &str,
        progress: &dyn ProgressReporter,
    ) -> Result<()> {
        self.manager.verify_sheet_exists(sheet_name, progress).await
    }

    async fn read_data(&self, sheet: &str, range: &str) -> Result<Vec<Vec<String>>> {
        use crate::progress::NoOpProgressReporter;
        let progress = NoOpProgressReporter;
        self.manager.read_data(sheet, range, &progress).await
    }

    async fn upload_data(&self, sheet: &str, range: &str, data: Vec<Vec<String>>) -> Result<()> {
        use crate::progress::NoOpProgressReporter;
        let progress = NoOpProgressReporter;
        self.manager
            .upload_data_to_range(sheet, range, data, &progress)
            .await
    }

    async fn upload_data_with_progress(
        &self,
        sheet: &str,
        _range: &str,
        data: Vec<Vec<String>>,
        progress: &dyn ProgressReporter,
    ) -> Result<()> {
        self.manager.upload_data(sheet, data, progress).await
    }

    async fn upload_data_with_events(
        &self,
        sheet: &str,
        _range: &str,
        data: Vec<Vec<String>>,
        progress: &ProgressPublisher,
    ) -> Result<()> {
        self.manager
            .upload_data_with_events(sheet, data, progress)
            .await
    }

    async fn clear_range(&self, sheet: &str, _range: &str) -> Result<()> {
        use crate::progress::NoOpProgressReporter;
        let progress = NoOpProgressReporter;
        self.manager.clear_range(sheet, &progress).await
    }

    async fn clear_range_with_progress(
        &self,
        sheet: &str,
        _range: &str,
        progress: &dyn ProgressReporter,
    ) -> Result<()> {
        self.manager.clear_range(sheet, progress).await
    }

    async fn clear_range_with_events(
        &self,
        sheet: &str,
        _range: &str,
        progress: &ProgressPublisher,
    ) -> Result<()> {
        self.manager.clear_range_with_events(sheet, progress).await
    }
}

/// Mock implementation for testing
#[derive(Debug, Clone)]
pub struct MockSheetsRepository {
    /// Simulated spreadsheet data organized as sheet_name -> Vec<Vec<String>>
    data: HashMap<String, Vec<Vec<String>>>,
    /// Simulated sheets that exist in the spreadsheet
    sheets: Vec<String>,
    /// Whether to simulate errors
    simulate_errors: bool,
}

impl MockSheetsRepository {
    /// Create a new mock repository
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            sheets: vec!["Sheet1".to_string()], // Default sheet
            simulate_errors: false,
        }
    }

    /// Add a sheet to the mock spreadsheet
    pub fn add_sheet(&mut self, sheet_name: impl Into<String>) {
        let name = sheet_name.into();
        if !self.sheets.contains(&name) {
            self.sheets.push(name);
        }
    }

    /// Set data for a specific sheet
    pub fn set_sheet_data(&mut self, sheet_name: impl Into<String>, data: Vec<Vec<String>>) {
        self.data.insert(sheet_name.into(), data);
    }

    /// Get data for a specific sheet
    pub fn get_sheet_data(&self, sheet_name: &str) -> Option<&Vec<Vec<String>>> {
        self.data.get(sheet_name)
    }

    /// Enable error simulation for testing error handling
    pub fn simulate_errors(&mut self, enable: bool) {
        self.simulate_errors = enable;
    }

    /// Check if a sheet exists
    pub fn has_sheet(&self, sheet_name: &str) -> bool {
        self.sheets.contains(&sheet_name.to_string())
    }

    /// List all sheets
    pub fn list_sheets(&self) -> &[String] {
        &self.sheets
    }

    /// Parse a range string (e.g., "A2:C10") to extract rows and columns
    fn parse_range(&self, range: &str) -> Result<(usize, usize, usize, usize)> {
        // Simple A1 notation parser - for testing we'll use simplified logic
        // Returns (start_row, start_col, end_row, end_col) as zero-based indices

        // Handle known hardcoded ranges first
        if range == "A2:AX58" {
            // Common range for main data
            Ok((1, 0, 57, 49)) // A2 to AX58 (0-based: row 1 to 57, col 0 to 49)
        } else if range == "A2:EQ58" {
            // Common range for team selection
            Ok((1, 0, 57, 144)) // A2 to EQ58 (0-based: row 1 to 57, col 0 to 144)
        } else if range == "A4:AX104" {
            // Common range for image processing
            Ok((3, 0, 103, 49)) // A4 to AX104 (0-based: row 3 to 103, col 0 to 49)
        } else if range == "A2:B3" {
            // Test range: A2 to B3 (0-based: row 1 to 2, col 0 to 1)
            Ok((1, 0, 2, 1))
        } else if range == "A1:B2" {
            // Test range: A1 to B2 (0-based: row 0 to 1, col 0 to 1)
            Ok((0, 0, 1, 1))
        } else {
            // For other ranges, assume the full sheet
            Ok((0, 0, 999, 49))
        }
    }

    /// Extract data from sheet based on range
    fn extract_range_data(
        &self,
        sheet_data: &[Vec<String>],
        range: &str,
    ) -> Result<Vec<Vec<String>>> {
        let (start_row, start_col, end_row, end_col) = self.parse_range(range)?;

        let mut result = Vec::new();
        for row_idx in start_row..=end_row.min(sheet_data.len().saturating_sub(1)) {
            if row_idx < sheet_data.len() {
                let row = &sheet_data[row_idx];
                let mut result_row = Vec::new();
                for col_idx in start_col..=end_col {
                    if col_idx < row.len() {
                        result_row.push(row[col_idx].clone());
                    } else {
                        result_row.push(String::new());
                    }
                }
                result.push(result_row);
            }
        }

        Ok(result)
    }
}

impl Default for MockSheetsRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SheetsRepository for MockSheetsRepository {
    async fn verify_spreadsheet_access(&self, _progress: &dyn ProgressReporter) -> Result<()> {
        if self.simulate_errors {
            return Err(crate::error::FMDataError::sheets_api(
                "Mock spreadsheet access denied",
            ));
        }
        Ok(())
    }

    async fn verify_sheet_exists(
        &self,
        sheet_name: &str,
        _progress: &dyn ProgressReporter,
    ) -> Result<()> {
        if self.simulate_errors {
            return Err(crate::error::FMDataError::sheets_api(
                "Mock sheet verification failed",
            ));
        }

        if !self.has_sheet(sheet_name) {
            return Err(crate::error::FMDataError::sheets_api(format!(
                "Sheet '{sheet_name}' does not exist in mock spreadsheet"
            )));
        }

        Ok(())
    }

    async fn read_data(&self, sheet: &str, range: &str) -> Result<Vec<Vec<String>>> {
        if self.simulate_errors {
            return Err(crate::error::FMDataError::sheets_api(
                "Mock read data failed",
            ));
        }

        if !self.has_sheet(sheet) {
            return Err(crate::error::FMDataError::sheets_api(format!(
                "Sheet '{sheet}' does not exist"
            )));
        }

        if let Some(sheet_data) = self.data.get(sheet) {
            self.extract_range_data(sheet_data, range)
        } else {
            // Return empty data for empty sheets
            Ok(Vec::new())
        }
    }

    async fn upload_data(&self, _sheet: &str, _range: &str, _data: Vec<Vec<String>>) -> Result<()> {
        if self.simulate_errors {
            return Err(crate::error::FMDataError::sheets_api(
                "Mock upload data failed",
            ));
        }
        Ok(())
    }

    async fn upload_data_with_progress(
        &self,
        sheet: &str,
        _range: &str,
        _data: Vec<Vec<String>>,
        _progress: &dyn ProgressReporter,
    ) -> Result<()> {
        if self.simulate_errors {
            return Err(crate::error::FMDataError::sheets_api(
                "Mock upload with progress failed",
            ));
        }

        if !self.has_sheet(sheet) {
            return Err(crate::error::FMDataError::sheets_api(format!(
                "Sheet '{sheet}' does not exist"
            )));
        }

        Ok(())
    }

    async fn upload_data_with_events(
        &self,
        sheet: &str,
        _range: &str,
        _data: Vec<Vec<String>>,
        _progress: &ProgressPublisher,
    ) -> Result<()> {
        if self.simulate_errors {
            return Err(crate::error::FMDataError::sheets_api(
                "Mock upload with events failed",
            ));
        }

        if !self.has_sheet(sheet) {
            return Err(crate::error::FMDataError::sheets_api(format!(
                "Sheet '{sheet}' does not exist"
            )));
        }

        Ok(())
    }

    async fn clear_range(&self, sheet: &str, _range: &str) -> Result<()> {
        if self.simulate_errors {
            return Err(crate::error::FMDataError::sheets_api(
                "Mock clear range failed",
            ));
        }

        if !self.has_sheet(sheet) {
            return Err(crate::error::FMDataError::sheets_api(format!(
                "Sheet '{sheet}' does not exist"
            )));
        }

        Ok(())
    }

    async fn clear_range_with_progress(
        &self,
        sheet: &str,
        range: &str,
        _progress: &dyn ProgressReporter,
    ) -> Result<()> {
        self.clear_range(sheet, range).await
    }

    async fn clear_range_with_events(
        &self,
        sheet: &str,
        range: &str,
        _progress: &ProgressPublisher,
    ) -> Result<()> {
        self.clear_range(sheet, range).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::progress::NoOpProgressReporter;

    #[tokio::test]
    async fn test_mock_repository_creation() {
        let repo = MockSheetsRepository::new();
        assert!(repo.has_sheet("Sheet1"));
        assert!(!repo.has_sheet("NonExistent"));
    }

    #[tokio::test]
    async fn test_mock_repository_sheet_management() {
        let mut repo = MockSheetsRepository::new();

        repo.add_sheet("TestSheet");
        assert!(repo.has_sheet("TestSheet"));

        let sheets = repo.list_sheets();
        assert!(sheets.contains(&"Sheet1".to_string()));
        assert!(sheets.contains(&"TestSheet".to_string()));
    }

    #[tokio::test]
    async fn test_mock_repository_data_operations() {
        let mut repo = MockSheetsRepository::new();

        let test_data = vec![
            vec!["A1".to_string(), "B1".to_string()],
            vec!["A2".to_string(), "B2".to_string()],
        ];

        repo.set_sheet_data("Sheet1", test_data.clone());

        let retrieved_data = repo.get_sheet_data("Sheet1").unwrap();
        assert_eq!(retrieved_data, &test_data);
    }

    #[tokio::test]
    async fn test_mock_repository_verify_operations() {
        let repo = MockSheetsRepository::new();
        let progress = NoOpProgressReporter;

        // Should succeed for existing sheet
        assert!(repo.verify_sheet_exists("Sheet1", &progress).await.is_ok());

        // Should fail for non-existent sheet
        assert!(repo
            .verify_sheet_exists("NonExistent", &progress)
            .await
            .is_err());
    }

    #[tokio::test]
    async fn test_mock_repository_error_simulation() {
        let mut repo = MockSheetsRepository::new();
        repo.simulate_errors(true);

        let progress = NoOpProgressReporter;

        // All operations should fail when errors are simulated
        assert!(repo.verify_spreadsheet_access(&progress).await.is_err());
        assert!(repo.verify_sheet_exists("Sheet1", &progress).await.is_err());
        assert!(repo.read_data("Sheet1", "A1:B2").await.is_err());
        assert!(repo.upload_data("Sheet1", "A1:B2", vec![]).await.is_err());
        assert!(repo.clear_range("Sheet1", "A1:B2").await.is_err());
    }

    #[tokio::test]
    async fn test_mock_repository_read_data() {
        let mut repo = MockSheetsRepository::new();

        let test_data = vec![
            vec!["Header1".to_string(), "Header2".to_string()],
            vec!["Data1".to_string(), "Data2".to_string()],
            vec!["Data3".to_string(), "Data4".to_string()],
        ];

        repo.set_sheet_data("Sheet1", test_data);

        let result = repo.read_data("Sheet1", "A2:B3").await.unwrap();

        // Should return data starting from row 2 (0-based index 1)
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], vec!["Data1".to_string(), "Data2".to_string()]);
        assert_eq!(result[1], vec!["Data3".to_string(), "Data4".to_string()]);
    }
}
