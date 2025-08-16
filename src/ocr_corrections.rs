use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Configuration structure for OCR corrections loaded from JSON file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OCRCorrectionsConfig {
    pub attribute_names: HashMap<String, String>,
    pub attribute_values: HashMap<String, u8>,
}

impl Default for OCRCorrectionsConfig {
    fn default() -> Self {
        Self {
            attribute_names: default_attribute_name_corrections(),
            attribute_values: default_attribute_value_corrections(),
        }
    }
}

/// OCR correction system that handles both attribute name and value corrections
#[derive(Debug, Clone)]
pub struct OCRCorrector {
    name_corrections: HashMap<String, String>,
    value_corrections: HashMap<String, u8>,
}

impl OCRCorrector {
    /// Create a new OCR corrector with default corrections
    pub fn new() -> Self {
        Self::from_config(OCRCorrectionsConfig::default())
    }

    /// Create an OCR corrector from a configuration
    pub fn from_config(config: OCRCorrectionsConfig) -> Self {
        Self {
            name_corrections: config.attribute_names,
            value_corrections: config.attribute_values,
        }
    }

    /// Load OCR corrections from a JSON configuration file
    pub async fn from_file<P: AsRef<Path>>(config_path: P) -> Result<Self> {
        let config_content = tokio::fs::read_to_string(config_path).await?;
        let config: OCRCorrectionsConfig = serde_json::from_str(&config_content)?;
        Ok(Self::from_config(config))
    }

    /// Load OCR corrections from a JSON file with fallback to defaults
    pub async fn from_file_with_fallback<P: AsRef<Path>>(config_path: P) -> Self {
        match Self::from_file(config_path).await {
            Ok(corrector) => {
                log::debug!("Loaded OCR corrections from config file");
                corrector
            }
            Err(e) => {
                log::debug!(
                    "Failed to load OCR corrections config file: {}. Using defaults.",
                    e
                );
                Self::new()
            }
        }
    }

    /// Correct attribute name OCR errors
    pub fn correct_attribute_name(&self, name: &str) -> String {
        let name_lower = name.to_lowercase();

        // Check for exact matches first
        if let Some(corrected) = self.name_corrections.get(&name_lower) {
            return corrected.clone();
        }

        // Return original name if no correction found
        name.to_string()
    }

    /// Correct attribute value OCR errors, returning None if value is invalid
    pub fn correct_attribute_value(&self, value: &str, attr_name: &str) -> Option<u8> {
        // Check OCR corrections table first (handles special cases like "40" -> 10)
        // Use lowercase for case-insensitive matching
        if let Some(&corrected_value) = self.value_corrections.get(&value.to_lowercase()) {
            if (1..=20).contains(&corrected_value) {
                log::debug!(
                    "Found {} = {} (OCR correction: '{}' -> {})",
                    attr_name,
                    corrected_value,
                    value,
                    corrected_value
                );
                return Some(corrected_value);
            } else {
                log::warn!(
                    "Found {} = {} (OCR correction: '{}' -> {}, out of valid range 1-20, ignoring)",
                    attr_name,
                    corrected_value,
                    value,
                    corrected_value
                );
                return None;
            }
        }

        // Then try direct parsing for valid numeric values
        if let Ok(num) = value.parse::<u8>() {
            if (1..=20).contains(&num) {
                log::debug!("Found {} = {} (valid range)", attr_name, num);
                return Some(num);
            } else {
                log::warn!(
                    "Found {} = {} (out of valid range 1-20, ignoring)",
                    attr_name,
                    num
                );
                return None;
            }
        }

        // Try to extract digits from corrupted text as fallback
        let digits: String = value.chars().filter(|c| c.is_ascii_digit()).collect();
        if !digits.is_empty() {
            if let Ok(num) = digits.parse::<u8>() {
                if (1..=20).contains(&num) {
                    log::debug!(
                        "Found {} = {} (extracted digits from '{}')",
                        attr_name,
                        num,
                        value
                    );
                    return Some(num);
                } else {
                    log::warn!(
                        "Found {} = {} (extracted from '{}', out of valid range 1-20, ignoring)",
                        attr_name,
                        num,
                        value
                    );
                }
            }
        }

        log::debug!(
            "Could not extract valid attribute value from '{}' for {}",
            value,
            attr_name
        );
        None
    }

    /// Find and correct an attribute name in a line of text, considering OCR errors
    pub fn find_corrected_attribute_in_line(&self, line: &str, attr_name: &str) -> Option<usize> {
        let line_lower = line.to_lowercase();
        let attr_lower = attr_name.to_lowercase();

        // First try exact match
        if let Some(pos) = line_lower.find(&attr_lower) {
            return Some(pos);
        }

        // Try OCR-corrected patterns
        for (pattern, corrected_name) in &self.name_corrections {
            if corrected_name.to_lowercase() == attr_lower {
                if let Some(pos) = line_lower.find(pattern) {
                    log::debug!(
                        "Found OCR attribute name correction: '{}' -> '{}'",
                        pattern,
                        attr_name
                    );
                    return Some(pos);
                }
            }
        }

        None
    }
}

impl Default for OCRCorrector {
    fn default() -> Self {
        Self::new()
    }
}

/// Default attribute name corrections for common OCR errors
fn default_attribute_name_corrections() -> HashMap<String, String> {
    let mut corrections = HashMap::new();

    // Tendency misspellings (updated for unified attribute names)
    corrections.insert(
        "rushing out (tendeney)".to_string(),
        "Rushing Out".to_string(),
    );
    corrections.insert("punching (tendeney)".to_string(), "Punching".to_string());
    corrections.insert(
        "rushing out (tendency)".to_string(),
        "Rushing Out".to_string(),
    );
    corrections.insert("punching (tendency)".to_string(), "Punching".to_string());

    // Spacing issues
    corrections.insert("offthe ball".to_string(), "Off the Ball".to_string());
    corrections.insert("offtheball".to_string(), "Off the Ball".to_string());

    // Character typos
    corrections.insert("agtity".to_string(), "Agility".to_string());
    corrections.insert("agtlty".to_string(), "Agility".to_string());
    corrections.insert("dribbting".to_string(), "Dribbling".to_string());
    corrections.insert("tackting".to_string(), "Tackling".to_string());
    corrections.insert("taking".to_string(), "Tackling".to_string());
    corrections.insert("postioning".to_string(), "Positioning".to_string());
    corrections.insert("posttioning".to_string(), "Positioning".to_string());
    corrections.insert("postioning".to_string(), "Positioning".to_string());
    corrections.insert("vison".to_string(), "Vision".to_string());

    corrections
}

/// Default attribute value corrections for common OCR errors
fn default_attribute_value_corrections() -> HashMap<String, u8> {
    let mut corrections = HashMap::new();

    // Common OCR misreads for specific numbers
    corrections.insert("n".to_string(), 11);
    corrections.insert("ll".to_string(), 11);
    corrections.insert("rn".to_string(), 12);
    corrections.insert("rn)".to_string(), 12);
    corrections.insert("n)".to_string(), 11);
    corrections.insert("rl".to_string(), 12);
    corrections.insert("rt".to_string(), 11);
    corrections.insert("ri".to_string(), 11);
    corrections.insert("nn".to_string(), 11);
    corrections.insert("tt".to_string(), 11);
    corrections.insert("l".to_string(), 1);
    corrections.insert("i".to_string(), 1);
    corrections.insert("t".to_string(), 7);
    corrections.insert("s".to_string(), 8);
    corrections.insert("a".to_string(), 9);
    corrections.insert("oo".to_string(), 9);
    corrections.insert("40".to_string(), 10); // Special case: OCR often reads "10" as "40"
    corrections.insert("41".to_string(), 11); // Special case: OCR often reads "11" as "41"
    corrections.insert("42".to_string(), 12); // Special case: OCR often reads "12" as "42"
    corrections.insert("43".to_string(), 13); // Special case: OCR often reads "13" as "43"
    corrections.insert("44".to_string(), 14); // Special case: OCR often reads "14" as "44"

    // Invalid values that should be ignored (mapped to 0, which will be filtered out)
    corrections.insert("o".to_string(), 0);
    corrections.insert("o".to_string(), 0);

    corrections
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_ocr_corrector_new() {
        let corrector = OCRCorrector::new();
        assert!(!corrector.name_corrections.is_empty());
        assert!(!corrector.value_corrections.is_empty());
    }

    #[test]
    fn test_correct_attribute_name_exact_match() {
        let corrector = OCRCorrector::new();

        // Test exact corrections
        assert_eq!(
            corrector.correct_attribute_name("rushing out (tendeney)"),
            "Rushing Out"
        );
        assert_eq!(
            corrector.correct_attribute_name("offthe ball"),
            "Off the Ball"
        );
        assert_eq!(corrector.correct_attribute_name("agtity"), "Agility");

        // Test no correction needed
        assert_eq!(corrector.correct_attribute_name("Crossing"), "Crossing");
    }

    #[test]
    fn test_correct_attribute_value_valid_range() {
        let corrector = OCRCorrector::new();

        assert_eq!(corrector.correct_attribute_value("1", "Test"), Some(1));
        assert_eq!(corrector.correct_attribute_value("10", "Test"), Some(10));
        assert_eq!(corrector.correct_attribute_value("20", "Test"), Some(20));
        assert_eq!(corrector.correct_attribute_value("15", "Test"), Some(15));
    }

    #[test]
    fn test_correct_attribute_value_invalid_range() {
        let corrector = OCRCorrector::new();

        assert_eq!(corrector.correct_attribute_value("0", "Test"), None);
        assert_eq!(corrector.correct_attribute_value("21", "Test"), None);
        assert_eq!(corrector.correct_attribute_value("100", "Test"), None);
        assert_eq!(corrector.correct_attribute_value("255", "Test"), None);
    }

    #[test]
    fn test_correct_attribute_value_ocr_corrections() {
        let corrector = OCRCorrector::new();

        // Common OCR garbled patterns
        assert_eq!(corrector.correct_attribute_value("n", "Test"), Some(11));
        assert_eq!(corrector.correct_attribute_value("ll", "Test"), Some(11));
        assert_eq!(corrector.correct_attribute_value("rn", "Test"), Some(12));
        assert_eq!(corrector.correct_attribute_value("rn)", "Test"), Some(12));
        assert_eq!(corrector.correct_attribute_value("n)", "Test"), Some(11));
        assert_eq!(corrector.correct_attribute_value("rl", "Test"), Some(12));
        assert_eq!(corrector.correct_attribute_value("ri", "Test"), Some(11));
        assert_eq!(corrector.correct_attribute_value("nn", "Test"), Some(11));
        assert_eq!(corrector.correct_attribute_value("l", "Test"), Some(1));
        assert_eq!(corrector.correct_attribute_value("i", "Test"), Some(1));
        assert_eq!(corrector.correct_attribute_value("t", "Test"), Some(7));
        assert_eq!(corrector.correct_attribute_value("oo", "Test"), Some(9));
        assert_eq!(corrector.correct_attribute_value("40", "Test"), Some(10));
    }

    #[test]
    fn test_correct_attribute_value_ocr_invalid() {
        let corrector = OCRCorrector::new();

        // OCR patterns that map to invalid values should be rejected
        assert_eq!(corrector.correct_attribute_value("o", "Test"), None);
        assert_eq!(corrector.correct_attribute_value("o", "Test"), None);
    }

    #[test]
    fn test_correct_attribute_value_digit_extraction() {
        let corrector = OCRCorrector::new();

        // Extract digits from corrupted words
        assert_eq!(corrector.correct_attribute_value("15x", "Test"), Some(15));
        assert_eq!(
            corrector.correct_attribute_value("abc8def", "Test"),
            Some(8)
        );
        assert_eq!(corrector.correct_attribute_value("~12!", "Test"), Some(12));
        assert_eq!(corrector.correct_attribute_value("(5)", "Test"), Some(5));

        // Invalid digit extractions
        assert_eq!(corrector.correct_attribute_value("25x", "Test"), None); // Out of range
        assert_eq!(corrector.correct_attribute_value("abc", "Test"), None); // No digits
        assert_eq!(corrector.correct_attribute_value("", "Test"), None); // Empty
        assert_eq!(corrector.correct_attribute_value("xyz", "Test"), None); // No digits
    }

    #[test]
    fn test_find_corrected_attribute_in_line() {
        let corrector = OCRCorrector::new();

        // Exact match
        assert_eq!(
            corrector.find_corrected_attribute_in_line("Crossing 15 Mental", "Crossing"),
            Some(0)
        );

        // OCR correction match
        assert_eq!(
            corrector.find_corrected_attribute_in_line(
                "Rushing Out (Tendeney) 10 Teamwork",
                "Rushing Out"
            ),
            Some(0)
        );

        // OCR correction match - spacing
        assert_eq!(
            corrector
                .find_corrected_attribute_in_line("OffThe Ball 15 Positioning", "Off the Ball"),
            Some(0)
        );

        // No match
        assert_eq!(
            corrector.find_corrected_attribute_in_line("Pace 8 Strength", "Crossing"),
            None
        );
    }

    #[test]
    fn test_from_config() {
        let mut name_corrections = HashMap::new();
        name_corrections.insert("test_typo".to_string(), "Test Correct".to_string());

        let mut value_corrections = HashMap::new();
        value_corrections.insert("x".to_string(), 5);

        let config = OCRCorrectionsConfig {
            attribute_names: name_corrections,
            attribute_values: value_corrections,
        };

        let corrector = OCRCorrector::from_config(config);

        assert_eq!(
            corrector.correct_attribute_name("test_typo"),
            "Test Correct"
        );
        assert_eq!(corrector.correct_attribute_value("x", "Test"), Some(5));
    }

    #[tokio::test]
    async fn test_from_file_with_fallback() {
        // Test with non-existent file - should fall back to defaults
        let corrector = OCRCorrector::from_file_with_fallback("/nonexistent/config.json").await;

        // Should have default corrections
        assert_eq!(corrector.correct_attribute_name("agtity"), "Agility");
        assert_eq!(corrector.correct_attribute_value("n", "Test"), Some(11));
    }

    #[tokio::test]
    async fn test_from_file_valid_config() {
        // Create a temporary config file
        let mut temp_file = NamedTempFile::new().unwrap();
        let config_content = r#"{
            "attribute_names": {
                "test_name": "Test Corrected"
            },
            "attribute_values": {
                "y": 3
            }
        }"#;
        temp_file.write_all(config_content.as_bytes()).unwrap();

        let corrector = OCRCorrector::from_file(temp_file.path()).await.unwrap();

        assert_eq!(
            corrector.correct_attribute_name("test_name"),
            "Test Corrected"
        );
        assert_eq!(corrector.correct_attribute_value("y", "Test"), Some(3));
    }

    #[test]
    fn test_default_corrections_comprehensive() {
        let corrector = OCRCorrector::new();

        // Test all default attribute name corrections
        assert_eq!(
            corrector.correct_attribute_name("rushing out (tendeney)"),
            "Rushing Out"
        );
        assert_eq!(
            corrector.correct_attribute_name("punching (tendeney)"),
            "Punching"
        );
        assert_eq!(
            corrector.correct_attribute_name("offthe ball"),
            "Off the Ball"
        );
        assert_eq!(
            corrector.correct_attribute_name("offtheball"),
            "Off the Ball"
        );
        assert_eq!(corrector.correct_attribute_name("agtity"), "Agility");
        assert_eq!(corrector.correct_attribute_name("agtlty"), "Agility");
        assert_eq!(corrector.correct_attribute_name("dribbting"), "Dribbling");
        assert_eq!(corrector.correct_attribute_name("tackting"), "Tackling");
        assert_eq!(
            corrector.correct_attribute_name("postioning"),
            "Positioning"
        );
        assert_eq!(
            corrector.correct_attribute_name("posttioning"),
            "Positioning"
        );
    }
}
