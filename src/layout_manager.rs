use crate::error::{FMDataError, Result};
use crate::types::PlayerType;
use log::{debug, info, warn};
use std::fs;
use std::path::Path;

/// Layout manager that handles loading and caching of player attribute layouts
/// from external files or embedded fallbacks
#[derive(Debug, Clone)]
pub struct LayoutManager {
    field_layout: Vec<Vec<String>>,
    goalkeeper_layout: Vec<Vec<String>>,
}

impl LayoutManager {
    /// Create a new layout manager by loading layouts from files
    pub fn from_files<P: AsRef<Path>>(field_path: P, gk_path: P) -> Result<Self> {
        let field_layout = Self::load_layout_from_file(&field_path)?;
        let goalkeeper_layout = Self::load_layout_from_file(&gk_path)?;

        debug!(
            "Loaded layouts: {} field rows, {} GK rows",
            field_layout.len(),
            goalkeeper_layout.len()
        );

        Ok(Self {
            field_layout,
            goalkeeper_layout,
        })
    }

    /// Create a layout manager with fallback to embedded layouts if files don't exist
    pub fn from_files_with_fallback<P: AsRef<Path>>(field_path: P, gk_path: P) -> Result<Self> {
        match Self::from_files(&field_path, &gk_path) {
            Ok(manager) => {
                info!("Successfully loaded layouts from external files");
                Ok(manager)
            }
            Err(e) => {
                warn!(
                    "Failed to load layouts from files: {}. Using embedded fallbacks.",
                    e
                );
                Ok(Self::from_embedded())
            }
        }
    }

    /// Create a layout manager using embedded fallback layouts
    pub fn from_embedded() -> Self {
        debug!("Using embedded fallback layouts");

        let field_layout = vec![
            vec![
                "TECHNICAL".to_string(),
                "MENTAL".to_string(),
                "PHYSICAL".to_string(),
            ],
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
        ];

        let goalkeeper_layout = vec![
            vec![
                "GOALKEEPING".to_string(),
                "MENTAL".to_string(),
                "PHYSICAL".to_string(),
            ],
            vec![
                "Aerial Reach".to_string(),
                "Aggression".to_string(),
                "Acceleration".to_string(),
            ],
            vec![
                "Command Of Area".to_string(),
                "Anticipation".to_string(),
                "Agility".to_string(),
            ],
            vec![
                "Communication".to_string(),
                "Bravery".to_string(),
                "Balance".to_string(),
            ],
            vec![
                "Eccentricity".to_string(),
                "Composure".to_string(),
                "Jumping Reach".to_string(),
            ],
            vec![
                "First Touch".to_string(),
                "Concentration".to_string(),
                "Natural Fitness".to_string(),
            ],
            vec![
                "Handling".to_string(),
                "Decisions".to_string(),
                "Pace".to_string(),
            ],
            vec![
                "Kicking".to_string(),
                "Determination".to_string(),
                "Stamina".to_string(),
            ],
            vec![
                "One On Ones".to_string(),
                "Flair".to_string(),
                "Strength".to_string(),
            ],
            vec!["Passing".to_string(), "Leadership".to_string()],
            vec![
                "Punching (Tendency)".to_string(),
                "Off the Ball".to_string(),
            ],
            vec!["Reflexes".to_string(), "Positioning".to_string()],
            vec!["Rushing Out (Tendency)".to_string(), "Teamwork".to_string()],
            vec!["Throwing".to_string(), "Vision".to_string()],
            vec!["".to_string(), "Work Rate".to_string()],
        ];

        Self {
            field_layout,
            goalkeeper_layout,
        }
    }

    /// Get the layout for the specified player type
    pub fn get_layout(&self, player_type: &PlayerType) -> &[Vec<String>] {
        match player_type {
            PlayerType::FieldPlayer => &self.field_layout,
            PlayerType::Goalkeeper => &self.goalkeeper_layout,
        }
    }

    /// Get the first section name for the specified player type
    pub fn get_first_section_name(&self, player_type: &PlayerType) -> &str {
        match player_type {
            PlayerType::FieldPlayer => "TECHNICAL",
            PlayerType::Goalkeeper => "GOALKEEPING",
        }
    }

    /// Load a layout from a tab-separated file
    fn load_layout_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Vec<String>>> {
        let path = path.as_ref();
        debug!("Loading layout from file: {}", path.display());

        let content = fs::read_to_string(path).map_err(|e| {
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
        self.validate_layout(&self.field_layout, "field player")?;
        self.validate_layout(&self.goalkeeper_layout, "goalkeeper")?;
        Ok(())
    }

    /// Validate a single layout
    fn validate_layout(&self, layout: &[Vec<String>], layout_name: &str) -> Result<()> {
        if layout.is_empty() {
            return Err(FMDataError::image(format!("{layout_name} layout is empty")));
        }

        // First row should be headers
        let headers = &layout[0];
        if headers.len() != 3 {
            return Err(FMDataError::image(format!(
                "{} layout header row should have exactly 3 columns, got {}",
                layout_name,
                headers.len()
            )));
        }

        // Validate minimum expected rows (header + at least 8 data rows)
        if layout.len() < 9 {
            return Err(FMDataError::image(format!(
                "{} layout should have at least 9 rows (header + 8 data), got {}",
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

/// Default file paths for layouts relative to the project root
pub mod default_paths {
    pub const FIELD_LAYOUT_FILE: &str = "layout-specs/layout-field.txt";
    pub const GOALKEEPER_LAYOUT_FILE: &str = "layout-specs/layout-gk.txt";
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

    #[test]
    fn test_embedded_layout_manager() {
        let manager = LayoutManager::from_embedded();

        // Test field player layout
        let field_layout = manager.get_layout(&PlayerType::FieldPlayer);
        assert!(!field_layout.is_empty());
        assert_eq!(field_layout[0][0], "TECHNICAL");
        assert_eq!(field_layout[0][1], "MENTAL");
        assert_eq!(field_layout[0][2], "PHYSICAL");

        // Test goalkeeper layout
        let gk_layout = manager.get_layout(&PlayerType::Goalkeeper);
        assert!(!gk_layout.is_empty());
        assert_eq!(gk_layout[0][0], "GOALKEEPING");
        assert_eq!(gk_layout[0][1], "MENTAL");
        assert_eq!(gk_layout[0][2], "PHYSICAL");

        // Test first section names
        assert_eq!(
            manager.get_first_section_name(&PlayerType::FieldPlayer),
            "TECHNICAL"
        );
        assert_eq!(
            manager.get_first_section_name(&PlayerType::Goalkeeper),
            "GOALKEEPING"
        );
    }

    #[test]
    fn test_load_layout_from_valid_file() {
        let content = "TECHNICAL\tMENTAL\tPHYSICAL\n\
                       Corners\tAggression\tAcceleration\n\
                       Crossing\tAnticipation\tAgility\n";
        let temp_file = create_test_layout_file(content);
        let temp_file2 = create_test_layout_file(content);

        let result = LayoutManager::from_files(temp_file.path(), temp_file2.path());
        assert!(result.is_ok());

        let manager = result.unwrap();
        let layout = manager.get_layout(&PlayerType::FieldPlayer);
        assert_eq!(layout.len(), 3);
        assert_eq!(layout[0], vec!["TECHNICAL", "MENTAL", "PHYSICAL"]);
        assert_eq!(layout[1], vec!["Corners", "Aggression", "Acceleration"]);
        assert_eq!(layout[2], vec!["Crossing", "Anticipation", "Agility"]);
    }

    #[test]
    fn test_load_layout_from_file_with_empty_lines() {
        let content = "TECHNICAL\tMENTAL\tPHYSICAL\n\
                       \n\
                       Corners\tAggression\tAcceleration\n\
                       \n\
                       Crossing\tAnticipation\tAgility\n";
        let temp_file = create_test_layout_file(content);
        let temp_file2 = create_test_layout_file(content);

        let result = LayoutManager::from_files(temp_file.path(), temp_file2.path());
        assert!(result.is_ok());

        let manager = result.unwrap();
        let layout = manager.get_layout(&PlayerType::FieldPlayer);
        // Empty lines should be filtered out
        assert_eq!(layout.len(), 3);
    }

    #[test]
    fn test_load_layout_from_invalid_file() {
        let result = LayoutManager::from_files("/nonexistent/path1.txt", "/nonexistent/path2.txt");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Failed to read layout file"));
    }

    #[test]
    fn test_load_layout_with_too_many_columns() {
        let content = "A\tB\tC\tD\tE\n"; // Too many columns
        let temp_file = create_test_layout_file(content);
        let temp_file2 = create_test_layout_file("GOALKEEPING\tMENTAL\tPHYSICAL\n");

        let result = LayoutManager::from_files(temp_file.path(), temp_file2.path());
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("expected 1-3 columns"));
    }

    #[test]
    fn test_load_layout_from_empty_file() {
        let content = "";
        let temp_file = create_test_layout_file(content);
        let temp_file2 = create_test_layout_file("GOALKEEPING\tMENTAL\tPHYSICAL\n");

        let result = LayoutManager::from_files(temp_file.path(), temp_file2.path());
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("is empty or contains no valid rows"));
    }

    #[test]
    fn test_from_files_with_fallback_success() {
        let content = "TECHNICAL\tMENTAL\tPHYSICAL\n\
                       Corners\tAggression\tAcceleration\n";
        let temp_file = create_test_layout_file(content);
        let temp_file2 = create_test_layout_file(content);

        let result = LayoutManager::from_files_with_fallback(temp_file.path(), temp_file2.path());
        assert!(result.is_ok());

        let manager = result.unwrap();
        let layout = manager.get_layout(&PlayerType::FieldPlayer);
        assert_eq!(layout.len(), 2);
    }

    #[test]
    fn test_from_files_with_fallback_uses_embedded() {
        // Use non-existent files to trigger fallback
        let result = LayoutManager::from_files_with_fallback(
            "/nonexistent/field.txt",
            "/nonexistent/gk.txt",
        );
        assert!(result.is_ok());

        let manager = result.unwrap();
        // Should have loaded embedded layouts
        let field_layout = manager.get_layout(&PlayerType::FieldPlayer);
        assert!(!field_layout.is_empty());
        assert_eq!(field_layout[0][0], "TECHNICAL");
    }

    #[test]
    fn test_validate_valid_layouts() {
        let manager = LayoutManager::from_embedded();
        let result = manager.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_invalid_layout() {
        // Create a manager with invalid layout
        let mut manager = LayoutManager::from_embedded();
        manager.field_layout = vec![]; // Empty layout should fail validation

        let result = manager.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("layout is empty"));
    }

    #[test]
    fn test_validate_layout_insufficient_rows() {
        let mut manager = LayoutManager::from_embedded();
        // Keep only header row - should fail validation
        manager.field_layout = vec![vec![
            "TECHNICAL".to_string(),
            "MENTAL".to_string(),
            "PHYSICAL".to_string(),
        ]];

        let result = manager.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("should have at least 9 rows"));
    }

    #[test]
    fn test_validate_layout_invalid_header() {
        let mut manager = LayoutManager::from_embedded();
        // Invalid header with wrong number of columns
        manager.field_layout[0] = vec!["TECHNICAL".to_string()]; // Only 1 column instead of 3

        let result = manager.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("header row should have exactly 3 columns"));
    }
}
