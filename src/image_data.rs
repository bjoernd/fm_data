use crate::error::FMDataError;
use crate::image_processor;
use anyhow::Result;
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
    log::info!("Full OCR text with improvements:\n{}", ocr_text);

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
                    && part.chars().filter(|c| c.is_alphabetic()).count() > part.len() / 2
                {
                    name_parts.push(part);
                }
            }

            if !name_parts.is_empty() {
                let name = name_parts.join(" ");
                if name.len() > 2 {
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

    Err(FMDataError::image("Unable to extract player name from OCR text").into())
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
                if cleaned.len() >= 2 {
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
        let is_mostly_letters =
            word_to_check.chars().filter(|c| c.is_alphabetic()).count() > word_to_check.len() / 2;
        let starts_with_capital = word_to_check
            .chars()
            .next()
            .is_some_and(|c| c.is_uppercase());
        let is_connector_word = word_to_check.len() <= 3
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

    if name_words.len() >= 2 && name_words.len() <= 4 && found_capital {
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
    let lines: Vec<&str> = ocr_text.lines().collect();

    // Use structured parsing based on player type
    match player.player_type {
        PlayerType::Goalkeeper => parse_goalkeeper_attributes(player, &lines)?,
        PlayerType::FieldPlayer => parse_field_player_attributes(player, &lines)?,
    }

    log::debug!("Parsed {} total attributes", player.attributes.len());
    Ok(())
}

// Structured layouts for different player types
static FIELD_PLAYER_LAYOUT: &[&[&str]] = &[
    &["TECHNICAL", "MENTAL", "PHYSICAL"],
    &["Corners", "Aggression", "Acceleration"],
    &["Crossing", "Anticipation", "Agility"],
    &["Dribbling", "Bravery", "Balance"],
    &["Finishing", "Composure", "Jumping Reach"],
    &["First Touch", "Concentration", "Natural Fitness"],
    &["Free Kick Taking", "Decisions", "Pace"],
    &["Heading", "Determination", "Stamina"],
    &["Long Shots", "Flair", "Strength"],
    &["Long Throws", "Leadership"],
    &["Marking", "Off the Ball"],
    &["Passing", "Positioning"],
    &["Penalty Taking", "Teamwork"],
    &["Tackling", "Vision"],
    &["Technique", "Work Rate"],
];

static GOALKEEPER_LAYOUT: &[&[&str]] = &[
    &["GOALKEEPING", "MENTAL", "PHYSICAL"],
    &["Aerial Reach", "Aggression", "Acceleration"],
    &["Command Of Area", "Anticipation", "Agility"],
    &["Communication", "Bravery", "Balance"],
    &["Eccentricity", "Composure", "Jumping Reach"],
    &["First Touch", "Concentration", "Natural Fitness"],
    &["Handling", "Decisions", "Pace"],
    &["Kicking", "Determination", "Stamina"],
    &["One On Ones", "Flair", "Strength"],
    &["Passing", "Leadership"],
    &["Punching (Tendency)", "Off the Ball"],
    &["Reflexes", "Positioning"],
    &["Rushing Out (Tendency)", "Teamwork"],
    &["Throwing", "Vision"],
    &["", "Work Rate"],
];

fn parse_field_player_attributes(player: &mut ImagePlayer, lines: &[&str]) -> Result<()> {
    parse_structured_attributes(player, lines, FIELD_PLAYER_LAYOUT, "TECHNICAL")
}

fn parse_goalkeeper_attributes(player: &mut ImagePlayer, lines: &[&str]) -> Result<()> {
    parse_structured_attributes(player, lines, GOALKEEPER_LAYOUT, "GOALKEEPING")
}

fn parse_structured_attributes(
    player: &mut ImagePlayer,
    lines: &[&str],
    layout: &[&[&str]],
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

    // Instead of rigid line-by-line parsing, search through all relevant lines for each attribute
    let search_lines = &lines[start_idx..];

    // Parse each expected attribute by searching through all lines
    for (layout_idx, expected_attrs) in layout.iter().enumerate().skip(1) {
        log::debug!(
            "=== Processing layout row {}: {:?} ===",
            layout_idx,
            expected_attrs
        );

        for &attr_name in expected_attrs.iter() {
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

                if let Some(value) = find_attribute_value_in_line(line, attr_name) {
                    // Determine the correct section prefix for this attribute
                    let section_prefix = get_correct_section_prefix(attr_name, &player.player_type);
                    let full_attr_name = format!(
                        "{}_{}",
                        section_prefix,
                        attr_name
                            .to_lowercase()
                            .replace(" ", "_")
                            .replace("(", "")
                            .replace(")", "")
                    );

                    player.add_attribute(full_attr_name.clone(), value);

                    log::debug!(
                        "Found and added attribute: {} = {} (from line {}: '{}')",
                        full_attr_name,
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

fn find_attribute_value_in_line(line: &str, attr_name: &str) -> Option<u8> {
    // Look for the attribute name in the line, case-insensitive
    let line_lower = line.to_lowercase();
    let attr_lower = attr_name.to_lowercase();

    // First try exact match
    if let Some(attr_pos) = line_lower.find(&attr_lower) {
        return extract_value_after_position(line, attr_pos, attr_name.len(), attr_name);
    }

    // Handle common OCR attribute name typos
    let corrected_patterns = match attr_name {
        "Rushing Out (Tendency)" => vec!["rushing out (tendeney)"],
        "Punching (Tendency)" => vec!["punching (tendeney)"],
        "Agility" => vec!["agtity", "agtlty"],
        "Dribbling" => vec!["dribbting"],
        "Off the Ball" => vec!["offthe ball", "offtheball"],
        "Tackling" => vec!["tackting"],
        "Positioning" => vec!["postioning", "posttioning"],
        _ => vec![],
    };

    for pattern in corrected_patterns {
        if let Some(attr_pos) = line_lower.find(pattern) {
            log::debug!(
                "Found OCR attribute name correction: '{}' -> '{}'",
                pattern,
                attr_name
            );
            return extract_value_after_position(line, attr_pos, pattern.len(), attr_name);
        }
    }

    None
}

fn extract_value_after_position(
    line: &str,
    attr_pos: usize,
    attr_len: usize,
    attr_name: &str,
) -> Option<u8> {
    // Get the text after the attribute name
    let after_attr = &line[attr_pos + attr_len..];
    let words: Vec<&str> = after_attr.split_whitespace().collect();

    // Look for the first valid number after the attribute name
    for word in words {
        // Try to extract and validate attribute value
        if let Some(validated_value) = extract_and_validate_attribute_value(word, attr_name) {
            return Some(validated_value);
        }
    }
    None
}

/// Extract and validate attribute values, ensuring they fall within the valid 1-20 range.
/// Handles OCR errors and provides corrections when possible.
fn extract_and_validate_attribute_value(word: &str, attr_name: &str) -> Option<u8> {
    // Handle specific OCR corrections first before normal parsing
    if word == "40" {
        log::debug!("Found {} = {} (OCR garbled '40' -> 10)", attr_name, 10);
        return Some(10);
    }

    // Then try direct parsing
    if let Ok(num) = word.parse::<u8>() {
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

    // Handle common OCR garbled patterns
    let corrected_value = match word {
        // Common OCR misreads for specific numbers
        "n" | "ll" => Some((11, "OCR garbled 'n'/'ll' -> 11")),
        "rn" => Some((12, "OCR garbled 'rn' -> 12")),
        "rn)" => Some((12, "OCR garbled 'rn)' -> 12")),
        "n)" => Some((11, "OCR garbled 'n)' -> 11")),
        "rl" => Some((12, "OCR garbled 'rl' -> 12")),
        "rT" => Some((11, "OCR garbled 'rT' -> 11")),
        "ri" => Some((11, "OCR garbled 'ri' -> 11")),
        "nn" => Some((11, "OCR garbled 'nn' -> 11")),
        "l" => Some((1, "OCR garbled 'l' -> 1")),
        "I" => Some((1, "OCR garbled 'I' -> 1")),
        "TT" => Some((11, "OCR garbled 'nn' -> 11")),
        "T" => Some((7, "OCR garbled 'T' -> 7")),
        "S" => Some((7, "OCR garbled 'S' -> 8")),
        "a" => Some((9, "OCR garbled 'Oo' -> 9")),
        "Oo" => Some((9, "OCR garbled 'Oo' -> 9")),
        "O" => Some((0, "OCR garbled 'O' -> invalid, ignoring")),
        "o" => Some((0, "OCR garbled 'o' -> invalid, ignoring")),
        _ => None,
    };

    if let Some((value, explanation)) = corrected_value {
        if (1..=20).contains(&value) {
            log::debug!("Found {} = {} ({})", attr_name, value, explanation);
            Some(value)
        } else {
            log::warn!(
                "Found {} = {} ({}, out of valid range 1-20, ignoring)",
                attr_name,
                value,
                explanation
            );
            None
        }
    } else {
        // Try to handle partial OCR corruption - look for digits within the word
        let digits: String = word.chars().filter(|c| c.is_ascii_digit()).collect();
        if !digits.is_empty() {
            if let Ok(num) = digits.parse::<u8>() {
                if (1..=20).contains(&num) {
                    log::debug!(
                        "Found {} = {} (extracted digits from '{}')",
                        attr_name,
                        num,
                        word
                    );
                    return Some(num);
                } else {
                    log::warn!(
                        "Found {} = {} (extracted from '{}', out of valid range 1-20, ignoring)",
                        attr_name,
                        num,
                        word
                    );
                }
            }
        }

        log::debug!(
            "Could not extract valid attribute value from '{}' for {}",
            word,
            attr_name
        );
        None
    }
}

fn get_correct_section_prefix(attr_name: &str, player_type: &PlayerType) -> &'static str {
    // For goalkeepers, some technical attributes appear in the goalkeeping column
    // but should still be stored as technical attributes
    match player_type {
        PlayerType::Goalkeeper => {
            match attr_name {
                "Passing" | "First Touch" => "technical", // These are technical attributes even for GKs
                "Aerial Reach"
                | "Command Of Area"
                | "Communication"
                | "Eccentricity"
                | "Handling"
                | "Kicking"
                | "One On Ones"
                | "Punching (Tendency)"
                | "Reflexes"
                | "Rushing Out (Tendency)"
                | "Throwing" => "goalkeeping",
                "Aggression" | "Anticipation" | "Bravery" | "Composure" | "Concentration"
                | "Decisions" | "Determination" | "Flair" | "Leadership" | "Off the Ball"
                | "Positioning" | "Teamwork" | "Vision" | "Work Rate" => "mental",
                "Acceleration" | "Agility" | "Balance" | "Jumping Reach" | "Natural Fitness"
                | "Pace" | "Stamina" | "Strength" => "physical",
                _ => "technical", // Default fallback
            }
        }
        PlayerType::FieldPlayer => {
            match attr_name {
                "Corners" | "Crossing" | "Dribbling" | "Finishing" | "First Touch"
                | "Free Kick Taking" | "Heading" | "Long Shots" | "Long Throws" | "Marking"
                | "Passing" | "Penalty Taking" | "Tackling" | "Technique" => "technical",
                "Aggression" | "Anticipation" | "Bravery" | "Composure" | "Concentration"
                | "Decisions" | "Determination" | "Flair" | "Leadership" | "Off the Ball"
                | "Positioning" | "Teamwork" | "Vision" | "Work Rate" => "mental",
                "Acceleration" | "Agility" | "Balance" | "Jumping Reach" | "Natural Fitness"
                | "Pace" | "Stamina" | "Strength" => "physical",
                _ => "technical", // Default fallback
            }
        }
    }
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
    fn test_parse_player_from_ocr_field_player() {
        // Create OCR text that matches the structured layout expected by the new parser
        let ocr_text = "Virgil van Dijk\n32 years old\nTECHNICAL MENTAL PHYSICAL\nCorners 7 Aggression 8 Acceleration 11\nCrossing 9 Anticipation 8 Agility 7\nDribbling 10 Bravery 8 Balance 7\nFinishing 6 Composure 18 Jumping Reach 10\nFirst Touch 10 Concentration 8 Natural Fitness 12\nFree Kick Taking 7 Decisions 8 Pace 12\nHeading 7 Determination 10 Stamina 11\nLong Shots 9 Flair 6 Strength 19\nLong Throws 8 Leadership 7\nMarking 9 Off the Ball 9\nPassing 9 Positioning 9\nPenalty Taking 8 Teamwork 9\nTackling 8 Vision 15\nTechnique 10 Work Rate 9\n";
        // Use dummy path for test - footedness detection will fail gracefully and default to BothFooted
        let player = parse_player_from_ocr(ocr_text, "/nonexistent/test.png").unwrap();

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

    #[test]
    fn test_extract_and_validate_attribute_value_valid_range() {
        assert_eq!(extract_and_validate_attribute_value("1", "Test"), Some(1));
        assert_eq!(extract_and_validate_attribute_value("10", "Test"), Some(10));
        assert_eq!(extract_and_validate_attribute_value("20", "Test"), Some(20));
        assert_eq!(extract_and_validate_attribute_value("15", "Test"), Some(15));
    }

    #[test]
    fn test_extract_and_validate_attribute_value_invalid_range() {
        assert_eq!(extract_and_validate_attribute_value("0", "Test"), None);
        assert_eq!(extract_and_validate_attribute_value("21", "Test"), None);
        assert_eq!(extract_and_validate_attribute_value("100", "Test"), None);
        assert_eq!(extract_and_validate_attribute_value("255", "Test"), None);
    }

    #[test]
    fn test_extract_and_validate_attribute_value_ocr_corrections() {
        // Common OCR garbled patterns
        assert_eq!(extract_and_validate_attribute_value("n", "Test"), Some(11));
        assert_eq!(extract_and_validate_attribute_value("ll", "Test"), Some(11));
        assert_eq!(extract_and_validate_attribute_value("rn", "Test"), Some(12));
        assert_eq!(
            extract_and_validate_attribute_value("rn)", "Test"),
            Some(12)
        );
        assert_eq!(extract_and_validate_attribute_value("n)", "Test"), Some(11));
        assert_eq!(extract_and_validate_attribute_value("rl", "Test"), Some(12));
        assert_eq!(extract_and_validate_attribute_value("ri", "Test"), Some(11));
        assert_eq!(extract_and_validate_attribute_value("nn", "Test"), Some(11));
        assert_eq!(extract_and_validate_attribute_value("l", "Test"), Some(1));
        assert_eq!(extract_and_validate_attribute_value("I", "Test"), Some(1));
        assert_eq!(extract_and_validate_attribute_value("T", "Test"), Some(7));
        assert_eq!(extract_and_validate_attribute_value("Oo", "Test"), Some(9));
        assert_eq!(extract_and_validate_attribute_value("40", "Test"), Some(10));
    }

    #[test]
    fn test_extract_and_validate_attribute_value_ocr_invalid() {
        // OCR patterns that map to invalid values should be rejected
        assert_eq!(extract_and_validate_attribute_value("O", "Test"), None);
        assert_eq!(extract_and_validate_attribute_value("o", "Test"), None);
    }

    #[test]
    fn test_extract_and_validate_attribute_value_digit_extraction() {
        // Extract digits from corrupted words
        assert_eq!(
            extract_and_validate_attribute_value("15x", "Test"),
            Some(15)
        );
        assert_eq!(
            extract_and_validate_attribute_value("abc8def", "Test"),
            Some(8)
        );
        assert_eq!(
            extract_and_validate_attribute_value("~12!", "Test"),
            Some(12)
        );
        assert_eq!(extract_and_validate_attribute_value("(5)", "Test"), Some(5));
    }

    #[test]
    fn test_extract_and_validate_attribute_value_invalid_extractions() {
        // Invalid digit extractions
        assert_eq!(extract_and_validate_attribute_value("25x", "Test"), None); // Out of range
        assert_eq!(extract_and_validate_attribute_value("abc", "Test"), None); // No digits
        assert_eq!(extract_and_validate_attribute_value("", "Test"), None); // Empty
        assert_eq!(extract_and_validate_attribute_value("xyz", "Test"), None); // No digits
    }

    #[test]
    fn test_find_attribute_value_in_line_with_validation() {
        // Valid cases
        assert_eq!(
            find_attribute_value_in_line("Crossing 15 Mental", "Crossing"),
            Some(15)
        );
        assert_eq!(
            find_attribute_value_in_line("Pace 8 Strength", "Pace"),
            Some(8)
        );
        assert_eq!(
            find_attribute_value_in_line("Corners rn Aggression", "Corners"),
            Some(12)
        ); // OCR correction

        // Invalid range cases
        assert_eq!(
            find_attribute_value_in_line("Crossing 0 Mental", "Crossing"),
            None
        );
        assert_eq!(
            find_attribute_value_in_line("Pace 25 Strength", "Pace"),
            None
        );
        assert_eq!(
            find_attribute_value_in_line("Speed 100 Power", "Speed"),
            None
        );

        // OCR corrections
        assert_eq!(
            find_attribute_value_in_line("Finishing n Defense", "Finishing"),
            Some(11)
        );
        assert_eq!(
            find_attribute_value_in_line("Tackling ll Vision", "Tackling"),
            Some(11)
        );

        // Digit extraction from corrupted text
        assert_eq!(
            find_attribute_value_in_line("Dribbling 7x Composure", "Dribbling"),
            Some(7)
        );

        // Attribute name OCR corrections
        assert_eq!(
            find_attribute_value_in_line(
                "Rushing Out (Tendeney) 10 Teamwork",
                "Rushing Out (Tendency)"
            ),
            Some(10)
        );
        assert_eq!(
            find_attribute_value_in_line(
                "Punching (Tendeney) 15 Off the Ball",
                "Punching (Tendency)"
            ),
            Some(15)
        );

        // Combined attribute name + value OCR corrections
        assert_eq!(
            find_attribute_value_in_line("Agtity n Balance", "Agility"),
            Some(11)
        );

        // Value OCR corrections
        assert_eq!(
            find_attribute_value_in_line("Leadership T Off the Ball", "Leadership"),
            Some(7)
        );

        // Attribute name spacing corrections
        assert_eq!(
            find_attribute_value_in_line("OffThe Ball 15 Positioning", "Off the Ball"),
            Some(15)
        );

        // Combined attribute name typo + value OCR corrections
        assert_eq!(
            find_attribute_value_in_line("Postioning Oo Teamwork", "Positioning"),
            Some(9)
        );
    }
}
