use crate::error::FMDataError;
use crate::image_processor;
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub enum PlayerType {
    Goalkeeper,
    FieldPlayer,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Footedness {
    LeftFooted,
    RightFooted,
    BothFooted,
}

#[derive(Debug, Clone)]
pub struct ImagePlayer {
    pub name: String,
    pub age: u8,
    pub player_type: PlayerType,
    pub footedness: Footedness,
    pub attributes: HashMap<String, u8>,
}

impl ImagePlayer {
    pub fn new(name: String, age: u8, player_type: PlayerType, footedness: Footedness) -> Self {
        Self {
            name,
            age,
            player_type,
            footedness,
            attributes: HashMap::new(),
        }
    }

    pub fn add_attribute(&mut self, name: String, value: u8) {
        self.attributes.insert(name, value);
    }

    pub fn get_attribute(&self, name: &str) -> u8 {
        self.attributes.get(name).copied().unwrap_or(0)
    }
}

pub fn parse_player_from_ocr<P: AsRef<Path>>(ocr_text: &str, image_path: P) -> Result<ImagePlayer> {
    log::debug!("Full OCR text to parse:\n{}", ocr_text);

    let name = extract_player_name(ocr_text)?;
    let age = extract_player_age(ocr_text)?;
    let player_type = detect_player_type(ocr_text);

    // Detect footedness from image
    let footedness = image_processor::detect_footedness(&image_path).unwrap_or_else(|e| {
        log::warn!(
            "Failed to detect footedness from image: {}. Defaulting to BothFooted",
            e
        );
        Footedness::BothFooted
    });

    let mut player = ImagePlayer::new(name, age, player_type, footedness);

    parse_attributes(&mut player, ocr_text)?;

    log::debug!("Player has {} total attributes", player.attributes.len());
    for (attr_name, value) in &player.attributes {
        log::debug!("Final attribute: {} = {}", attr_name, value);
    }

    validate_required_attributes(&player)?;

    Ok(player)
}

fn extract_player_name(ocr_text: &str) -> Result<String> {
    let lines: Vec<&str> = ocr_text.lines().collect();

    // Look for the first substantial line that isn't a section header
    for line in lines {
        let trimmed = line.trim();
        if !trimmed.is_empty()
            && !trimmed.contains("TECHNICAL")
            && !trimmed.contains("MENTAL")
            && !trimmed.contains("PHYSICAL")
            && !trimmed.contains("GOALKEEPING")
            && !trimmed.contains("LEFT FOOT")
            && !trimmed.contains("RIGHT FOOT")
            && trimmed.len() > 2
        {
            // Try to extract just the name part (before age if present)
            let name_part = trimmed
                .split_whitespace()
                .take_while(|word| !word.chars().all(|c| c.is_ascii_digit()))
                .collect::<Vec<_>>()
                .join(" ");

            if !name_part.is_empty() {
                return Ok(name_part);
            }
        }
    }

    Err(FMDataError::image("Unable to extract player name from OCR text").into())
}

fn extract_player_age(ocr_text: &str) -> Result<u8> {
    let lines: Vec<&str> = ocr_text.lines().collect();

    // Look for age in the first few lines (player name area), not in attribute sections
    for line in lines.iter().take(5) {
        // Check more lines for age patterns
        let trimmed = line.trim();
        if !trimmed.contains("TECHNICAL")
            && !trimmed.contains("MENTAL")
            && !trimmed.contains("PHYSICAL")
            && !trimmed.contains("GOALKEEPING")
        {
            // First try to find "X years old" pattern
            if let Some(age) = extract_age_from_years_old_pattern(trimmed) {
                return Ok(age);
            }

            // Fallback to looking for standalone numbers
            let words: Vec<&str> = trimmed.split_whitespace().collect();
            for word in words {
                if let Ok(age) = word.parse::<u8>() {
                    if (15..=45).contains(&age) {
                        // Reasonable age range for players
                        return Ok(age);
                    }
                }
            }
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
                if (15..=45).contains(&age) {
                    return Some(age);
                }
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

fn parse_attributes(player: &mut ImagePlayer, ocr_text: &str) -> Result<()> {
    let sections = extract_attribute_sections(ocr_text);

    // Debug logging to see what sections we found
    log::debug!(
        "Extracted {} attribute sections: {:?}",
        sections.len(),
        sections.keys().collect::<Vec<_>>()
    );

    for (section_name, section_text) in sections {
        parse_section_attributes(player, &section_name, &section_text)
            .with_context(|| format!("Failed to parse {section_name} section"))?;
    }

    // Validate that we have required attributes based on player type
    validate_required_attributes(player)?;

    Ok(())
}

fn extract_attribute_sections(ocr_text: &str) -> HashMap<String, String> {
    let mut sections: HashMap<String, String> = HashMap::new();

    // Define known FM attributes and their sections
    let goalkeeping_attrs = [
        "Aerial Reach",
        "Command Of Area",
        "Communication",
        "Eccentricity",
        "First Touch",
        "Handling",
        "Kicking",
        "One On Ones",
        "Passing",
        "Punching",
        "Reflexes",
        "Rushing Out",
        "Throwing",
        "Punching (Tendency)",
        "Rushing Out (Tendency)",
    ];

    let technical_attrs = [
        "Corners",
        "Crossing",
        "Dribbling",
        "Finishing",
        "First Touch",
        "Free Kick Taking",
        "Heading",
        "Long Shots",
        "Long Throws",
        "Marking",
        "Passing",
        "Penalty Taking",
        "Tackling",
        "Technique",
    ];

    let mental_attrs = [
        "Aggression",
        "Anticipation",
        "Bravery",
        "Composure",
        "Concentration",
        "Decisions",
        "Determination",
        "Flair",
        "Leadership",
        "Off The Ball",
        "Positioning",
        "Teamwork",
        "Vision",
        "Work Rate",
    ];

    let physical_attrs = [
        "Acceleration",
        "Agility",
        "Balance",
        "Jumping Reach",
        "Natural Fitness",
        "Pace",
        "Stamina",
        "Strength",
    ];

    // Parse all attribute lines from OCR text
    for line in ocr_text.lines() {
        // Try to extract attributes from this line
        extract_attributes_from_line(line, &goalkeeping_attrs, "GOALKEEPING", &mut sections);
        extract_attributes_from_line(line, &technical_attrs, "TECHNICAL", &mut sections);
        extract_attributes_from_line(line, &mental_attrs, "MENTAL", &mut sections);
        extract_attributes_from_line(line, &physical_attrs, "PHYSICAL", &mut sections);
    }

    sections
}

fn extract_attributes_from_line(
    line: &str,
    known_attrs: &[&str],
    section_name: &str,
    sections: &mut HashMap<String, String>,
) {
    for &attr_name in known_attrs {
        if let Some((extracted_attr, value)) = find_attribute_in_line(line, attr_name) {
            let section_content = sections.entry(section_name.to_string()).or_default();
            if !section_content.is_empty() {
                section_content.push('\n');
            }
            section_content.push_str(&format!("{extracted_attr} {value}"));
        }
    }
}

fn find_attribute_in_line(line: &str, attr_name: &str) -> Option<(String, u8)> {
    let line_upper = line.to_uppercase();
    let attr_upper = attr_name.to_uppercase();

    if let Some(start_pos) = line_upper.find(&attr_upper) {
        // Found the attribute name, now look for the value after it
        let after_attr = &line[start_pos + attr_name.len()..];
        let parts: Vec<&str> = after_attr.split_whitespace().collect();

        // Look for the first valid number (1-20 range for FM attributes)
        for part in parts {
            if let Ok(value) = part.parse::<u8>() {
                if (1..=20).contains(&value) {
                    return Some((attr_name.to_string(), value));
                }
            }
        }
    }

    None
}

fn parse_section_attributes(
    player: &mut ImagePlayer,
    section_name: &str,
    section_text: &str,
) -> Result<()> {
    let lines: Vec<&str> = section_text.lines().collect();
    log::debug!(
        "Parsing section '{}' with {} lines",
        section_name,
        lines.len()
    );

    for line in lines {
        if let Some((attr_name, value)) = parse_attribute_line(line) {
            let full_attr_name = format!("{}_{}", section_name.to_lowercase(), attr_name);
            player.add_attribute(full_attr_name.clone(), value);
            log::debug!("Added attribute: {} = {}", full_attr_name, value);
        }
    }

    Ok(())
}

fn parse_attribute_line(line: &str) -> Option<(String, u8)> {
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.len() >= 2 {
        // Look for the last number in the line (should be the attribute value)
        for i in (1..parts.len()).rev() {
            if let Ok(value) = parts[i].parse::<u8>() {
                if value <= 20 {
                    // FM attributes are typically 1-20
                    let attr_name = parts[0..i].join("_").to_lowercase();
                    return Some((attr_name, value));
                }
            }
        }
    }

    None
}

fn validate_required_attributes(player: &ImagePlayer) -> Result<()> {
    match player.player_type {
        PlayerType::Goalkeeper => {
            // Goalkeepers should have goalkeeping attributes
            if !player
                .attributes
                .keys()
                .any(|k| k.starts_with("goalkeeping_"))
            {
                return Err(FMDataError::image(
                    "Goalkeeper missing required GOALKEEPING attributes",
                )
                .into());
            }
        }
        PlayerType::FieldPlayer => {
            // Field players should have technical, mental, and physical attributes
            let has_technical = player
                .attributes
                .keys()
                .any(|k| k.starts_with("technical_"));
            let has_mental = player.attributes.keys().any(|k| k.starts_with("mental_"));
            let has_physical = player.attributes.keys().any(|k| k.starts_with("physical_"));

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
        assert!(player.attributes.is_empty());
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
    fn test_extract_player_age() {
        let ocr_text = "Player Name 25\nTECHNICAL\nCrossing 15\n";
        let result = extract_player_age(ocr_text).unwrap();
        assert_eq!(result, 25);
    }

    #[test]
    fn test_extract_player_age_years_old_pattern() {
        let ocr_text = "Player Name\n25 years old\nTECHNICAL\nCrossing 15\n";
        let result = extract_player_age(ocr_text).unwrap();
        assert_eq!(result, 25);
    }

    #[test]
    fn test_extract_player_age_year_old_pattern() {
        let ocr_text = "Player Name\n21 year old\nTECHNICAL\nCrossing 15\n";
        let result = extract_player_age(ocr_text).unwrap();
        assert_eq!(result, 21);
    }

    #[test]
    fn test_extract_player_age_mixed_case() {
        let ocr_text = "Player Name\n30 YEARS OLD\nTECHNICAL\nCrossing 15\n";
        let result = extract_player_age(ocr_text).unwrap();
        assert_eq!(result, 30);
    }

    #[test]
    fn test_extract_player_age_invalid_range() {
        let ocr_text = "Player Name 100\nTECHNICAL\nCrossing 8\n";
        assert!(extract_player_age(ocr_text).is_err());
    }

    #[test]
    fn test_extract_player_age_invalid_years_old_range() {
        let ocr_text = "Player Name\n100 years old\nTECHNICAL\nCrossing 8\n";
        assert!(extract_player_age(ocr_text).is_err());
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
        assert_eq!(extract_age_from_years_old_pattern("100 years old"), None); // Out of range
        assert_eq!(extract_age_from_years_old_pattern("5 years old"), None); // Out of range
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

    #[test]
    fn test_extract_attribute_sections() {
        let ocr_text =
            "Player Name\nTECHNICAL\nCrossing 15\nDribbling 12\nMENTAL\nComposure 18\nVision 16\n";
        let sections = extract_attribute_sections(ocr_text);

        assert_eq!(sections.len(), 2);
        assert!(sections.contains_key("TECHNICAL"));
        assert!(sections.contains_key("MENTAL"));
        assert!(sections["TECHNICAL"].contains("Crossing 15"));
        assert!(sections["MENTAL"].contains("Composure 18"));
    }

    #[test]
    fn test_parse_attribute_line() {
        assert_eq!(
            parse_attribute_line("Crossing 15"),
            Some(("crossing".to_string(), 15))
        );
        assert_eq!(
            parse_attribute_line("Long Shots 12"),
            Some(("long_shots".to_string(), 12))
        );
        assert_eq!(parse_attribute_line("Invalid Line"), None);
        assert_eq!(parse_attribute_line("Value Too High 25"), None);
    }

    #[test]
    fn test_parse_player_from_ocr_field_player() {
        let ocr_text = "Virgil van Dijk\n32 years old\nTECHNICAL\nCrossing 8\nDribbling 10\nMENTAL\nComposure 18\nVision 15\nPHYSICAL\nPace 12\nStrength 19\n";
        // Use dummy path for test - footedness detection will fail gracefully and default to BothFooted
        let player = parse_player_from_ocr(ocr_text, "/nonexistent/test.png").unwrap();

        assert_eq!(player.name, "Virgil van Dijk");
        assert_eq!(player.age, 32);
        assert_eq!(player.player_type, PlayerType::FieldPlayer);
        assert_eq!(player.footedness, Footedness::BothFooted); // Default when detection fails
        assert_eq!(player.get_attribute("technical_crossing"), 8);
        assert_eq!(player.get_attribute("mental_composure"), 18);
        assert_eq!(player.get_attribute("physical_strength"), 19);
    }

    #[test]
    fn test_parse_player_from_ocr_goalkeeper() {
        let ocr_text = "Alisson Becker\n30 years old\nTECHNICAL\nFirst Touch 15\nGOALKEEPING\nReflexes 18\nHandling 17\nMENTAL\nComposure 16\nPHYSICAL\nAgility 17\n";
        // Use dummy path for test - footedness detection will fail gracefully and default to BothFooted
        let player = parse_player_from_ocr(ocr_text, "/nonexistent/test.png").unwrap();

        assert_eq!(player.name, "Alisson Becker");
        assert_eq!(player.age, 30);
        assert_eq!(player.player_type, PlayerType::Goalkeeper);
        assert_eq!(player.footedness, Footedness::BothFooted); // Default when detection fails
        assert_eq!(player.get_attribute("goalkeeping_reflexes"), 18);
        assert_eq!(player.get_attribute("goalkeeping_handling"), 17);
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
}
