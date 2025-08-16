use crate::attributes::PlayerAttributes;
use crate::error::FMDataError;
use crate::image_constants::age_name;
use crate::image_processor;
use crate::layout_manager::{default_paths, LayoutManager};
use crate::ocr_corrections::OCRCorrector;
use crate::types::{Footedness, PlayerType};
use anyhow::Result;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct ImagePlayer {
    pub name: String,
    pub age: u8,
    pub player_type: PlayerType,
    pub footedness: Footedness,
    pub attributes: PlayerAttributes,
}

impl ImagePlayer {
    pub fn new(
        name: impl Into<String>,
        age: u8,
        player_type: PlayerType,
        footedness: Footedness,
    ) -> Self {
        Self {
            name: name.into(),
            age,
            player_type,
            footedness,
            attributes: PlayerAttributes::new(),
        }
    }

    pub fn add_attribute(&mut self, name: impl Into<String>, value: u8) {
        let name = name.into();
        // Use unified attribute system - much simpler now
        match self.attributes.set_by_name(&name, value) {
            Ok(()) => {}
            Err(err) => {
                // Log the error but continue - unknown attributes are simply ignored
                log::debug!("Failed to set attribute {}: {}", name, err);
            }
        }
    }

    pub fn get_attribute(&self, name: &str) -> u8 {
        // Use direct attribute access for performance
        self.attributes.get_by_name(name).unwrap_or(0)
    }
}

pub async fn parse_player_from_ocr<P: AsRef<Path>>(
    ocr_text: &str,
    image_path: P,
) -> Result<ImagePlayer> {
    log::info!("Full OCR text with improvements:\n{}", ocr_text);

    let name = extract_player_name(ocr_text)?;
    let age = extract_player_age(ocr_text)?;
    let player_type = detect_player_type(ocr_text);

    // Detect footedness from image (using optional detection with built-in fallback)
    let footedness = image_processor::detect_footedness_optional(&image_path)?;

    let mut player = ImagePlayer::new(name, age, player_type, footedness);

    // Load layout manager with fallback to embedded layouts
    let layout_manager = LayoutManager::from_files_with_fallback(
        default_paths::FIELD_LAYOUT_FILE,
        default_paths::GOALKEEPER_LAYOUT_FILE,
    )
    .await
    .map_err(|e| FMDataError::image(format!("Failed to load layouts: {e}")))?;

    parse_attributes(&mut player, ocr_text, &layout_manager)?;

    let attr_hashmap = player.attributes.to_hashmap();
    log::debug!("Player has {} total attributes", attr_hashmap.len());
    for (attr_name, value) in &attr_hashmap {
        log::debug!("Final attribute: {} = {}", attr_name, value);
    }

    validate_required_attributes(&player)?;

    Ok(player)
}

fn extract_player_name(ocr_text: &str) -> Result<String> {
    let lines: Vec<&str> = ocr_text.lines().collect();

    // Always look in the first line for the player name
    if let Some(first_line) = lines.first() {
        let trimmed = first_line.trim();
        if let Some(name) = extract_name_from_text(trimmed) {
            log::debug!(
                "Found player name in first line: '{}' from: '{}'",
                name,
                trimmed
            );
            return Ok(name);
        }
    }

    // Fallback: look for "X years old" lines and extract names from them
    for line in lines.iter() {
        let trimmed = line.trim();
        if trimmed.contains("years old") || trimmed.contains("year old") {
            // Extract everything before the age
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            let mut name_parts = Vec::new();

            for part in parts {
                if part.chars().all(|c| c.is_ascii_digit()) {
                    break; // Stop when we hit the age number
                }
                if part.chars().next().is_some_and(|c| c.is_uppercase())
                    && part.chars().filter(|c| c.is_alphabetic()).count()
                        > part.len() / age_name::MIN_ALPHABETIC_FRACTION
                {
                    name_parts.push(part);
                }
            }

            if !name_parts.is_empty() {
                let name = name_parts.join(" ");
                if name.len() > age_name::MIN_NAME_LENGTH {
                    log::debug!(
                        "Found name from 'years old' pattern: '{}' in line: '{}'",
                        name,
                        trimmed
                    );
                    return Ok(name);
                }
            }
        }
    }

    log::warn!("Unable to extract player name from OCR text");
    Ok("N. N.".to_string())
}

/// Extract a plausible player name from a piece of text
fn extract_name_from_text(text: &str) -> Option<String> {
    let words: Vec<&str> = text.split_whitespace().collect();

    // Look for sequences of words that could be a name
    let mut name_words = Vec::new();
    let mut found_capital = false;

    for word in words {
        // Handle OCR artifacts where digits are stuck to names (e.g., "1Alexander" -> "Alexander")
        let word_to_check =
            if word.len() > 1 && word.chars().next().is_some_and(|c| c.is_ascii_digit()) {
                // Remove leading digits and non-letter characters to extract potential name
                let cleaned = word
                    .chars()
                    .skip_while(|c| !c.is_alphabetic())
                    .collect::<String>();
                if cleaned.len() >= age_name::MIN_CLEANED_NAME_LENGTH {
                    cleaned
                } else {
                    word.to_string()
                }
            } else {
                word.to_string()
            };

        // Skip obvious non-name words
        if word_to_check.chars().all(|c| c.is_ascii_digit())
            || word_to_check.len() == 1
            || word_to_check.contains("Goalkeeper")
            || word_to_check.contains("years")
            || word_to_check.contains("old")
            || word_to_check.contains("(")
            || word_to_check.contains(")")
            || word_to_check.contains("Overview")
            || word_to_check.contains("Contract")
            || word_to_check.contains("Transfer")
            || word_to_check.contains("United")
            || word_to_check.contains("City")
            || word_to_check.contains("FC")
            || word_to_check.contains("Club")
        {
            // If we already have name words, stop here
            if !name_words.is_empty() {
                break;
            }
            continue;
        }

        // Check if this looks like a name word
        let is_mostly_letters = word_to_check.chars().filter(|c| c.is_alphabetic()).count()
            > word_to_check.len() / age_name::MIN_ALPHABETIC_FRACTION;
        let starts_with_capital = word_to_check
            .chars()
            .next()
            .is_some_and(|c| c.is_uppercase());
        let is_connector_word = word_to_check.len() <= age_name::MAX_CONNECTOR_WORD_LENGTH
            && (word_to_check == "van"
                || word_to_check == "de"
                || word_to_check == "la"
                || word_to_check == "el"
                || word_to_check == "da"
                || word_to_check == "von");

        if is_mostly_letters && (starts_with_capital || (is_connector_word && found_capital)) {
            name_words.push(word_to_check);
            if starts_with_capital {
                found_capital = true;
            }
        } else if !name_words.is_empty() && found_capital {
            // Stop when we hit non-name-like words after finding some name words
            break;
        }
    }

    if name_words.len() >= age_name::MIN_NAME_WORDS
        && name_words.len() <= age_name::MAX_NAME_WORDS
        && found_capital
    {
        // Reasonable name length (first + last, or first + middle + last) and at least one capital
        Some(name_words.join(" "))
    } else {
        None
    }
}

fn extract_player_age(ocr_text: &str) -> Result<u8> {
    let lines: Vec<&str> = ocr_text.lines().collect();

    // Only look for "X years old" pattern - more reliable than standalone numbers
    for line in lines.iter() {
        let trimmed = line.trim();
        if let Some(age) = extract_age_from_years_old_pattern(trimmed) {
            return Ok(age);
        }
    }

    Err(FMDataError::image("Unable to extract player age from OCR text").into())
}

/// Extract age from patterns like "25 years old", "30 year old", etc.
fn extract_age_from_years_old_pattern(text: &str) -> Option<u8> {
    let text_lower = text.to_lowercase();

    // Look for patterns like "25 years old", "30 year old"
    let words: Vec<&str> = text_lower.split_whitespace().collect();

    for i in 0..words.len().saturating_sub(2) {
        // Check for "X years old" or "X year old" patterns
        if (words[i + 1] == "year" || words[i + 1] == "years") && words[i + 2] == "old" {
            if let Ok(age) = words[i].parse::<u8>() {
                if !(age_name::MIN_PLAYER_AGE..=age_name::MAX_PLAYER_AGE).contains(&age) {
                    log::warn!("Player age {age} looks suspicious. Gracefully proceeding anyway.");
                }
                return Some(age);
            }
        }
    }

    None
}

fn detect_player_type(ocr_text: &str) -> PlayerType {
    if ocr_text.contains("GOALKEEPING") {
        PlayerType::Goalkeeper
    } else {
        PlayerType::FieldPlayer
    }
}

fn parse_attributes(
    player: &mut ImagePlayer,
    ocr_text: &str,
    layout_manager: &LayoutManager,
) -> Result<()> {
    let lines: Vec<&str> = ocr_text.lines().collect();

    // Use structured parsing with dynamic layout
    let layout = layout_manager.get_layout(&player.player_type);
    let first_section_name = layout_manager.get_first_section_name(&player.player_type);

    parse_structured_attributes(player, &lines, layout, first_section_name)?;

    let attr_hashmap = player.attributes.to_hashmap();
    log::debug!("Parsed {} total attributes", attr_hashmap.len());
    Ok(())
}

// Structured layouts are now loaded dynamically via LayoutManager

fn parse_structured_attributes(
    player: &mut ImagePlayer,
    lines: &[&str],
    layout: &[Vec<String>],
    first_section_name: &str,
) -> Result<()> {
    // Find the start of attribute section by looking for the header line
    let mut attr_start_line = None;
    for (line_idx, line) in lines.iter().enumerate() {
        if line.contains(first_section_name) {
            attr_start_line = Some(line_idx);
            break;
        }
    }

    let start_idx = attr_start_line.ok_or_else(|| {
        FMDataError::image(format!(
            "Could not find {first_section_name} section header"
        ))
    })?;

    log::debug!("Found attribute section starting at line {}", start_idx);

    // Create OCR corrector for handling attribute name and value corrections
    let corrector = OCRCorrector::new();

    // Instead of rigid line-by-line parsing, search through all relevant lines for each attribute
    let search_lines = &lines[start_idx..];

    // Parse each expected attribute by searching through all lines
    for (layout_idx, expected_attrs) in layout.iter().enumerate().skip(1) {
        log::debug!(
            "=== Processing layout row {}: {:?} ===",
            layout_idx,
            expected_attrs
        );

        for attr_name in expected_attrs.iter() {
            if attr_name.is_empty() {
                continue;
            }

            // Search through all lines in the attribute section for this attribute
            let mut found = false;
            for (search_idx, line) in search_lines.iter().enumerate() {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }

                if let Some(value) = find_attribute_value_in_line(line, attr_name, &corrector) {
                    // Use unified attribute system - try multiple name formats
                    // The PlayerAttributes system handles name resolution automatically
                    player.add_attribute(attr_name, value);

                    log::debug!(
                        "Found and added attribute: {} = {} (from line {}: '{}')",
                        attr_name,
                        value,
                        start_idx + search_idx,
                        line
                    );
                    found = true;
                    break;
                }
            }

            if !found {
                log::debug!("Attribute '{}' not found in any line", attr_name);
            }
        }
    }

    Ok(())
}

fn find_attribute_value_in_line(
    line: &str,
    attr_name: &str,
    corrector: &OCRCorrector,
) -> Option<u8> {
    // Use OCR corrector to find attribute name (handles OCR errors)
    if let Some(attr_pos) = corrector.find_corrected_attribute_in_line(line, attr_name) {
        return extract_value_after_position(line, attr_pos, attr_name, corrector);
    }
    None
}

fn extract_value_after_position(
    line: &str,
    attr_pos: usize,
    attr_name: &str,
    corrector: &OCRCorrector,
) -> Option<u8> {
    // Get all words starting from the found position
    let line_after_pos = &line[attr_pos..];
    let words: Vec<&str> = line_after_pos.split_whitespace().collect();

    // Skip the first word (attribute name) and look for values
    for word in words.iter().skip(1) {
        // Use OCR corrector for value validation and correction
        if let Some(validated_value) = corrector.correct_attribute_value(word, attr_name) {
            return Some(validated_value);
        }
    }
    None
}

fn validate_required_attributes(player: &ImagePlayer) -> Result<()> {
    match player.player_type {
        PlayerType::Goalkeeper => {
            // Goalkeepers should have non-zero goalkeeping attributes
            let attr_hashmap = player.attributes.to_hashmap();
            if !attr_hashmap
                .iter()
                .any(|(k, &v)| k.starts_with("goalkeeping_") && v > 0)
            {
                return Err(FMDataError::image(
                    "Goalkeeper missing required GOALKEEPING attributes",
                )
                .into());
            }
        }
        PlayerType::FieldPlayer => {
            // Field players should have non-zero technical, mental, and physical attributes
            let attr_hashmap = player.attributes.to_hashmap();
            let has_technical = attr_hashmap
                .iter()
                .any(|(k, &v)| k.starts_with("technical_") && v > 0);
            let has_mental = attr_hashmap
                .iter()
                .any(|(k, &v)| k.starts_with("mental_") && v > 0);
            let has_physical = attr_hashmap
                .iter()
                .any(|(k, &v)| k.starts_with("physical_") && v > 0);

            if !has_technical || !has_mental || !has_physical {
                return Err(
                    FMDataError::image("Field player missing required attribute sections").into(),
                );
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_player_creation() {
        let player = ImagePlayer::new(
            "Virgil van Dijk".to_string(),
            32,
            PlayerType::FieldPlayer,
            Footedness::LeftFooted,
        );

        assert_eq!(player.name, "Virgil van Dijk");
        assert_eq!(player.age, 32);
        assert_eq!(player.player_type, PlayerType::FieldPlayer);
        assert_eq!(player.footedness, Footedness::LeftFooted);
        let attr_hashmap = player.attributes.to_non_zero_hashmap();
        assert!(attr_hashmap.is_empty());
    }

    #[test]
    fn test_add_and_get_attributes() {
        let mut player = ImagePlayer::new(
            "Test Player".to_string(),
            25,
            PlayerType::FieldPlayer,
            Footedness::RightFooted,
        );

        player.add_attribute("technical_crossing".to_string(), 15);
        player.add_attribute("mental_composure".to_string(), 18);

        assert_eq!(player.get_attribute("technical_crossing"), 15);
        assert_eq!(player.get_attribute("mental_composure"), 18);
        assert_eq!(player.get_attribute("nonexistent_attr"), 0);
    }

    #[test]
    fn test_extract_player_name_simple() {
        let ocr_text = "Virgil van Dijk\n32\nTECHNICAL\nCrossing 8\n";
        let result = extract_player_name(ocr_text).unwrap();
        assert_eq!(result, "Virgil van Dijk");
    }

    #[test]
    fn test_extract_player_name_with_age() {
        let ocr_text = "Mohamed Salah 31\nTECHNICAL\nCrossing 15\n";
        let result = extract_player_name(ocr_text).unwrap();
        assert_eq!(result, "Mohamed Salah");
    }

    #[test]
    fn test_extract_name_from_text() {
        assert_eq!(
            extract_name_from_text("Alexander Westberg"),
            Some("Alexander Westberg".to_string())
        );
        assert_eq!(
            extract_name_from_text("Virgil van Dijk 32"),
            Some("Virgil van Dijk".to_string())
        );
        assert_eq!(
            extract_name_from_text("John Smith Goalkeeper"),
            Some("John Smith".to_string())
        );
        assert_eq!(
            extract_name_from_text("Mohamed Salah 31 years old"),
            Some("Mohamed Salah".to_string())
        );
        assert_eq!(extract_name_from_text("25 years old"), None); // No name
        assert_eq!(extract_name_from_text("A B C D E F"), None); // Too many words
        assert_eq!(extract_name_from_text("OnlyOne"), None); // Single name
    }

    #[test]
    fn test_extract_player_age_years_old_pattern() {
        let ocr_text = "Player Name\n25 years old\nTECHNICAL\nCrossing 15\n";
        let result = extract_player_age(ocr_text).unwrap();
        assert_eq!(result, 25);
    }

    #[test]
    fn test_extract_age_from_years_old_pattern() {
        assert_eq!(extract_age_from_years_old_pattern("25 years old"), Some(25));
        assert_eq!(extract_age_from_years_old_pattern("30 year old"), Some(30));
        assert_eq!(extract_age_from_years_old_pattern("19 YEARS OLD"), Some(19));
        assert_eq!(
            extract_age_from_years_old_pattern("Player Name 22 years old Striker"),
            Some(22)
        );
        assert_eq!(extract_age_from_years_old_pattern("25 years"), None); // Missing "old"
        assert_eq!(extract_age_from_years_old_pattern("years old 25"), None); // Wrong order
    }

    #[test]
    fn test_detect_player_type_field_player() {
        let ocr_text = "Player Name\nTECHNICAL\nCrossing 15\nMENTAL\nComposure 12\n";
        let result = detect_player_type(ocr_text);
        assert_eq!(result, PlayerType::FieldPlayer);
    }

    #[test]
    fn test_detect_player_type_goalkeeper() {
        let ocr_text = "Player Name\nTECHNICAL\nCrossing 15\nGOALKEEPING\nReflexes 18\n";
        let result = detect_player_type(ocr_text);
        assert_eq!(result, PlayerType::Goalkeeper);
    }

    #[tokio::test]
    async fn test_parse_player_from_ocr_field_player() {
        // Create OCR text that matches the structured layout expected by the new parser
        let ocr_text = "Virgil van Dijk\n32 years old\nTECHNICAL MENTAL PHYSICAL\nCorners 7 Aggression 8 Acceleration 11\nCrossing 9 Anticipation 8 Agility 7\nDribbling 10 Bravery 8 Balance 7\nFinishing 6 Composure 18 Jumping Reach 10\nFirst Touch 10 Concentration 8 Natural Fitness 12\nFree Kick Taking 7 Decisions 8 Pace 12\nHeading 7 Determination 10 Stamina 11\nLong Shots 9 Flair 6 Strength 19\nLong Throws 8 Leadership 7\nMarking 9 Off the Ball 9\nPassing 9 Positioning 9\nPenalty Taking 8 Teamwork 9\nTackling 8 Vision 15\nTechnique 10 Work Rate 9\n";
        // Use dummy path for test - footedness detection will fail gracefully and default to BothFooted
        let player = parse_player_from_ocr(ocr_text, "/nonexistent/test.png")
            .await
            .unwrap();

        assert_eq!(player.name, "Virgil van Dijk");
        assert_eq!(player.age, 32);
        assert_eq!(player.player_type, PlayerType::FieldPlayer);
        assert_eq!(player.footedness, Footedness::BothFooted); // Default when detection fails

        // Check that some attributes were parsed correctly
        assert_eq!(player.get_attribute("technical_crossing"), 9);
        assert_eq!(player.get_attribute("mental_composure"), 18);
        assert_eq!(player.get_attribute("physical_strength"), 19);
    }

    #[test]
    fn test_validate_required_attributes_field_player_missing() {
        let mut player = ImagePlayer::new(
            "Test Player".to_string(),
            25,
            PlayerType::FieldPlayer,
            Footedness::RightFooted,
        );
        player.add_attribute("technical_crossing".to_string(), 15);
        // Missing mental and physical attributes

        assert!(validate_required_attributes(&player).is_err());
    }

    #[test]
    fn test_validate_required_attributes_goalkeeper_missing() {
        let mut player = ImagePlayer::new(
            "Test Keeper".to_string(),
            25,
            PlayerType::Goalkeeper,
            Footedness::RightFooted,
        );
        player.add_attribute("technical_crossing".to_string(), 15);
        // Missing goalkeeping attributes

        assert!(validate_required_attributes(&player).is_err());
    }

    // OCR correction tests have been moved to the ocr_corrections module

    #[test]
    fn test_find_attribute_value_in_line_with_validation() {
        let corrector = OCRCorrector::new();

        // Valid cases
        assert_eq!(
            find_attribute_value_in_line("Crossing 15 Mental", "Crossing", &corrector),
            Some(15)
        );
        assert_eq!(
            find_attribute_value_in_line("Pace 8 Strength", "Pace", &corrector),
            Some(8)
        );
        assert_eq!(
            find_attribute_value_in_line("Corners rn Aggression", "Corners", &corrector),
            Some(12)
        ); // OCR correction

        // Invalid range cases
        assert_eq!(
            find_attribute_value_in_line("Crossing 0 Mental", "Crossing", &corrector),
            None
        );
        assert_eq!(
            find_attribute_value_in_line("Pace 25 Strength", "Pace", &corrector),
            None
        );
        assert_eq!(
            find_attribute_value_in_line("Speed 100 Power", "Speed", &corrector),
            None
        );

        // OCR corrections
        assert_eq!(
            find_attribute_value_in_line("Finishing n Defense", "Finishing", &corrector),
            Some(11)
        );
        assert_eq!(
            find_attribute_value_in_line("Tackling ll Vision", "Tackling", &corrector),
            Some(11)
        );

        // Digit extraction from corrupted text
        assert_eq!(
            find_attribute_value_in_line("Dribbling 7x Composure", "Dribbling", &corrector),
            Some(7)
        );

        // Attribute name OCR corrections
        assert_eq!(
            find_attribute_value_in_line(
                "Rushing Out (Tendeney) 10 Teamwork",
                "Rushing Out (Tendency)",
                &corrector
            ),
            Some(10)
        );
        assert_eq!(
            find_attribute_value_in_line(
                "Punching (Tendeney) 15 Off the Ball",
                "Punching (Tendency)",
                &corrector
            ),
            Some(15)
        );

        // Combined attribute name + value OCR corrections
        assert_eq!(
            find_attribute_value_in_line("Agtity n Balance", "Agility", &corrector),
            Some(11)
        );

        // Value OCR corrections
        assert_eq!(
            find_attribute_value_in_line("Leadership T Off the Ball", "Leadership", &corrector),
            Some(7)
        );

        // Attribute name spacing corrections
        assert_eq!(
            find_attribute_value_in_line("OffThe Ball 15 Positioning", "Off the Ball", &corrector),
            Some(15)
        );

        // Combined attribute name typo + value OCR corrections
        assert_eq!(
            find_attribute_value_in_line("Postioning Oo Teamwork", "Positioning", &corrector),
            Some(9)
        );
    }
}
