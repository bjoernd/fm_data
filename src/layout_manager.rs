use crate::error::{FMDataError, Result};
use log::{debug, info, warn};
use std::path::Path;
use tokio::fs;

/// Layout manager that handles loading and caching of player attribute layouts
/// from external files or embedded fallbacks
#[derive(Debug, Clone)]
pub struct LayoutManager {
    layout: Vec<Vec<String>>,
}

impl LayoutManager {
    /// Create a new layout manager by loading layout from file
    pub async fn from_files<P: AsRef<Path>>(layout_path: P) -> Result<Self> {
        let layout = Self::load_layout_from_file(&layout_path).await?;

        debug!("Loaded layout: {} rows", layout.len());

        Ok(Self { layout })
    }

    /// Create a layout manager with fallback to embedded layout if file doesn't exist
    pub async fn from_files_with_fallback<P: AsRef<Path>>(layout_path: P) -> Result<Self> {
        match Self::from_files(&layout_path).await {
            Ok(manager) => {
                info!("Successfully loaded layout from external file");
                Ok(manager)
            }
            Err(e) => {
                warn!(
                    "Failed to load layout from file: {}. Using embedded fallback.",
                    e
                );
                Ok(Self::from_embedded())
            }
        }
    }

    /// Create a layout manager using embedded fallback layout
    pub fn from_embedded() -> Self {
        debug!("Using embedded fallback layout");

        // Unified layout that works for all player types
        let layout = vec![
            vec![
                "Corners".to_string(),
                "Aggression".to_string(),
                "Acceleration".to_string(),
            ],
            vec![
                "Crossing".to_string(),
                "Anticipation".to_string(),
                "Agility".to_string(),
            ],
            vec![
                "Dribbling".to_string(),
                "Bravery".to_string(),
                "Balance".to_string(),
            ],
            vec![
                "Finishing".to_string(),
                "Composure".to_string(),
                "Jumping Reach".to_string(),
            ],
            vec![
                "First Touch".to_string(),
                "Concentration".to_string(),
                "Natural Fitness".to_string(),
            ],
            vec![
                "Free Kick Taking".to_string(),
                "Decisions".to_string(),
                "Pace".to_string(),
            ],
            vec![
                "Heading".to_string(),
                "Determination".to_string(),
                "Stamina".to_string(),
            ],
            vec![
                "Long Shots".to_string(),
                "Flair".to_string(),
                "Strength".to_string(),
            ],
            vec!["Long Throws".to_string(), "Leadership".to_string()],
            vec!["Marking".to_string(), "Off the Ball".to_string()],
            vec!["Passing".to_string(), "Positioning".to_string()],
            vec!["Penalty Taking".to_string(), "Teamwork".to_string()],
            vec!["Tackling".to_string(), "Vision".to_string()],
            vec!["Technique".to_string(), "Work Rate".to_string()],
            // Goalkeeping attributes (will be empty for field players)
            vec!["Aerial Reach".to_string()],
            vec!["Command Of Area".to_string()],
            vec!["Communication".to_string()],
            vec!["Eccentricity".to_string()],
            vec!["Handling".to_string()],
            vec!["Kicking".to_string()],
            vec!["One On Ones".to_string()],
            vec!["Punching (Tendency)".to_string()],
            vec!["Reflexes".to_string()],
            vec!["Rushing Out (Tendency)".to_string()],
            vec!["Throwing".to_string()],
        ];

        Self { layout }
    }

    /// Get the unified layout
    pub fn get_layout(&self) -> &[Vec<String>] {
        &self.layout
    }

    /// Load a layout from a tab-separated file
    async fn load_layout_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Vec<String>>> {
        let path = path.as_ref();
        debug!("Loading layout from file: {}", path.display());

        let content = fs::read_to_string(path).await.map_err(|e| {
            FMDataError::image(format!(
                "Failed to read layout file '{}': {}",
                path.display(),
                e
            ))
        })?;

        let mut layout = Vec::new();
        for (line_num, line) in content.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let columns: Vec<String> = line.split('\t').map(|s| s.trim().to_string()).collect();

            if columns.is_empty() {
                continue;
            }

            // Validate that we have 1-3 columns (some rows have fewer columns)
            if columns.len() > 3 {
                return Err(FMDataError::image(format!(
                    "Invalid layout file '{}' at line {}: expected 1-3 columns, got {}",
                    path.display(),
                    line_num + 1,
                    columns.len()
                )));
            }

            layout.push(columns);
        }

        if layout.is_empty() {
            return Err(FMDataError::image(format!(
                "Layout file '{}' is empty or contains no valid rows",
                path.display()
            )));
        }

        info!(
            "Successfully loaded layout with {} rows from '{}'",
            layout.len(),
            path.display()
        );

        Ok(layout)
    }

    /// Validate that the layout has the expected structure
    pub fn validate(&self) -> Result<()> {
        self.validate_layout(&self.layout, "unified layout")?;
        Ok(())
    }

    /// Validate a single layout
    fn validate_layout(&self, layout: &[Vec<String>], layout_name: &str) -> Result<()> {
        if layout.is_empty() {
            return Err(FMDataError::image(format!("{layout_name} layout is empty")));
        }

        // Validate minimum expected rows (at least 14 attribute rows)
        if layout.len() < 14 {
            return Err(FMDataError::image(format!(
                "{} layout should have at least 14 rows, got {}",
                layout_name,
                layout.len()
            )));
        }

        debug!(
            "Validated {} layout with {} rows",
            layout_name,
            layout.len()
        );
        Ok(())
    }
}

/// Default file path for unified layout relative to the project root
pub mod default_paths {
    pub const LAYOUT_FILE: &str = "layout-specs/layout-unified.txt";
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_test_layout_file(content: &str) -> NamedTempFile {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(content.as_bytes()).unwrap();
        temp_file.flush().unwrap();
        temp_file
    }

    #[tokio::test]
    async fn test_embedded_layout_manager() {
        let manager = LayoutManager::from_embedded();

        // Test unified layout
        let layout = manager.get_layout();
        assert!(!layout.is_empty());
        assert_eq!(layout[0][0], "Corners");
        assert_eq!(layout[0][1], "Aggression");
        assert_eq!(layout[0][2], "Acceleration");

        // Should contain both field player and goalkeeper attributes
        assert!(layout.len() >= 20); // At least 20 rows for all attributes
    }

    #[tokio::test]
    async fn test_load_layout_from_valid_file() {
        let content = "Corners\tAggression\tAcceleration\n\
                       Crossing\tAnticipation\tAgility\n\
                       Dribbling\tBravery\tBalance\n";
        let temp_file = create_test_layout_file(content);

        let result = LayoutManager::from_files(temp_file.path()).await;
        assert!(result.is_ok());

        let manager = result.unwrap();
        let layout = manager.get_layout();
        assert_eq!(layout.len(), 3);
        assert_eq!(layout[0], vec!["Corners", "Aggression", "Acceleration"]);
        assert_eq!(layout[1], vec!["Crossing", "Anticipation", "Agility"]);
        assert_eq!(layout[2], vec!["Dribbling", "Bravery", "Balance"]);
    }

    #[tokio::test]
    async fn test_load_layout_from_file_with_empty_lines() {
        let content = "Corners\tAggression\tAcceleration\n\
                       \n\
                       Crossing\tAnticipation\tAgility\n\
                       \n\
                       Dribbling\tBravery\tBalance\n";
        let temp_file = create_test_layout_file(content);

        let result = LayoutManager::from_files(temp_file.path()).await;
        assert!(result.is_ok());

        let manager = result.unwrap();
        let layout = manager.get_layout();
        // Empty lines should be filtered out
        assert_eq!(layout.len(), 3);
    }

    #[tokio::test]
    async fn test_load_layout_from_invalid_file() {
        let result = LayoutManager::from_files("/nonexistent/path.txt").await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Failed to read layout file"));
    }

    #[tokio::test]
    async fn test_load_layout_with_too_many_columns() {
        let content = "A\tB\tC\tD\tE\n"; // Too many columns
        let temp_file = create_test_layout_file(content);

        let result = LayoutManager::from_files(temp_file.path()).await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("expected 1-3 columns"));
    }

    #[tokio::test]
    async fn test_load_layout_from_empty_file() {
        let content = "";
        let temp_file = create_test_layout_file(content);

        let result = LayoutManager::from_files(temp_file.path()).await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("is empty or contains no valid rows"));
    }

    #[tokio::test]
    async fn test_from_files_with_fallback_success() {
        let content = "Corners\tAggression\tAcceleration\n\
                       Crossing\tAnticipation\tAgility\n";
        let temp_file = create_test_layout_file(content);

        let result = LayoutManager::from_files_with_fallback(temp_file.path()).await;
        assert!(result.is_ok());

        let manager = result.unwrap();
        let layout = manager.get_layout();
        assert_eq!(layout.len(), 2);
    }

    #[tokio::test]
    async fn test_from_files_with_fallback_uses_embedded() {
        // Use non-existent file to trigger fallback
        let result = LayoutManager::from_files_with_fallback("/nonexistent/layout.txt").await;
        assert!(result.is_ok());

        let manager = result.unwrap();
        // Should have loaded embedded layout
        let layout = manager.get_layout();
        assert!(!layout.is_empty());
        assert_eq!(layout[0][0], "Corners");
    }

    #[tokio::test]
    async fn test_validate_valid_layouts() {
        let manager = LayoutManager::from_embedded();
        let result = manager.validate();
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validate_invalid_layout() {
        // Create a manager with invalid layout
        let mut manager = LayoutManager::from_embedded();
        manager.layout = vec![]; // Empty layout should fail validation

        let result = manager.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("layout is empty"));
    }

    #[tokio::test]
    async fn test_validate_layout_insufficient_rows() {
        let mut manager = LayoutManager::from_embedded();
        // Keep only few rows - should fail validation
        manager.layout = vec![vec![
            "Corners".to_string(),
            "Aggression".to_string(),
            "Acceleration".to_string(),
        ]];

        let result = manager.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("should have at least 14 rows"));
    }

    #[tokio::test]
    async fn test_validate_layout_empty() {
        let mut manager = LayoutManager::from_embedded();
        // Empty layout should fail validation
        manager.layout = vec![];

        let result = manager.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("layout is empty"));
    }
}
