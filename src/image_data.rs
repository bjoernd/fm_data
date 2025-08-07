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

    // Priority 1: Look for full names in early lines (usually first few lines contain the full name)
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if !trimmed.is_empty()
            && !trimmed.contains("TECHNICAL")
            && !trimmed.contains("MENTAL")
            && !trimmed.contains("PHYSICAL")
            && !trimmed.contains("GOALKEEPING")
            && !trimmed.contains("Overview")
            && !trimmed.contains("Contract")
            && !trimmed.contains("Transfer")
            && !trimmed.contains("POSITIONS")
            && i < 5
        // Focus on first 5 lines for full names
        {
            // Try to extract a full name from this line
            if let Some(name) = extract_name_from_text(trimmed) {
                // Prefer longer names (full names) over shorter ones
                if name.split_whitespace().count() >= 2 {
                    log::debug!(
                        "Found full name from priority match: '{}' in line: '{}'",
                        name,
                        trimmed
                    );
                    return Ok(name);
                }
            }
        }
    }

    // Priority 2: Look for "X years old" lines and extract names from them
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

    // Priority 3: Look for lines that contain recognizable name patterns (original logic)
    for line in lines {
        let trimmed = line.trim();
        if !trimmed.is_empty()
            && !trimmed.contains("TECHNICAL")
            && !trimmed.contains("MENTAL")
            && !trimmed.contains("PHYSICAL")
            && !trimmed.contains("GOALKEEPING")
            && !trimmed.contains("LEFT FOOT")
            && !trimmed.contains("RIGHT FOOT")
            && !trimmed.contains("Overview")
            && !trimmed.contains("Contract")
            && !trimmed.contains("Transfer")
            && !trimmed.contains("Reports")
            && !trimmed.contains("Contracted to")
            && !trimmed.contains("Goalkeeper") // Exclude team/position lines
            && !trimmed.contains("Swedish") // Exclude nationality lines
            && !trimmed.contains("caps") // Exclude stats lines
            && trimmed.len() > 2
        {
            // Look for names that might be after a hyphen (common OCR artifact)
            if let Some(hyphen_pos) = trimmed.find('-') {
                let after_hyphen = &trimmed[hyphen_pos + 1..];
                if let Some(name) = extract_name_from_text(after_hyphen) {
                    log::debug!(
                        "Found name from hyphen pattern: '{}' in line: '{}'",
                        name,
                        trimmed
                    );
                    return Ok(name);
                }
            }

            // Look for names in the full line
            if let Some(name) = extract_name_from_text(trimmed) {
                log::debug!(
                    "Found name from general pattern: '{}' in line: '{}'",
                    name,
                    trimmed
                );
                return Ok(name);
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
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            log::debug!("Processing OCR line for attributes: '{}'", trimmed);
        }
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
            log::info!(
                "🎯 EXTRACTED: {} = {} (from line: '{}')",
                extracted_attr,
                value,
                line.trim()
            );
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

    // Try exact match first
    if let Some(start_pos) = line_upper.find(&attr_upper) {
        // Found the attribute name, now look for the value after it
        let after_attr = &line[start_pos + attr_name.len()..];
        let parts: Vec<&str> = after_attr.split_whitespace().collect();

        log::debug!(
            "  🔍 Found '{}' in line, looking for value in: '{}'",
            attr_name,
            after_attr.trim()
        );

        // Look for the first valid number (1-20 range for FM attributes)
        for part in parts {
            if let Ok(value) = part.parse::<u8>() {
                if (1..=20).contains(&value) {
                    log::debug!("    ✅ Found valid value {} for {}", value, attr_name);
                    return Some((attr_name.to_string(), value));
                } else {
                    log::debug!(
                        "    ❌ Value {} for {} is out of range (1-20)",
                        value,
                        attr_name
                    );
                }
            }
            // Handle cases like "n", "ll", "rn" which might be OCR garbled numbers
            if matches!(part, "n" | "ll" | "rn") {
                log::debug!(
                    "    🔍 Found OCR garbled character '{}' for {}, trying to infer value",
                    part,
                    attr_name
                );

                match part {
                    "n" => {
                        log::debug!(
                            "    ✅ Inferring value 11 from OCR garbled 'n' for {}",
                            attr_name
                        );
                        return Some((attr_name.to_string(), 11));
                    }
                    "ll" => {
                        log::debug!(
                            "    ✅ Inferring value 11 from OCR garbled 'll' for {}",
                            attr_name
                        );
                        return Some((attr_name.to_string(), 11));
                    }
                    "rn" => {
                        log::debug!(
                            "    ✅ Inferring value 12 from OCR garbled 'rn' for {}",
                            attr_name
                        );
                        return Some((attr_name.to_string(), 12));
                    }
                    _ => unreachable!(),
                }
            }
        }
        log::debug!("    ❌ No valid numeric value found for {}", attr_name);
        return None;
    }

    // Try comprehensive fuzzy matching for common OCR issues
    let fuzzy_matches = get_fuzzy_attribute_patterns(attr_name);
    for pattern in fuzzy_matches {
        if let Some(start_pos) = line_upper.find(&pattern.to_uppercase()) {
            let after_attr = &line[start_pos + pattern.len()..];
            let parts: Vec<&str> = after_attr.split_whitespace().collect();
            log::debug!(
                "  🔍 Found fuzzy match for '{}' as '{}', looking for value in: '{}'",
                attr_name,
                pattern,
                after_attr.trim()
            );

            for part in parts {
                if let Ok(value) = part.parse::<u8>() {
                    if (1..=20).contains(&value) {
                        log::debug!(
                            "    ✅ Found valid value {} for {} (fuzzy match)",
                            value,
                            attr_name
                        );
                        return Some((attr_name.to_string(), value));
                    }
                }
                // Handle cases like "n", "ll", "u" which might be OCR garbled numbers
                if matches!(part, "n" | "ll" | "u" | "rn" | "m") {
                    log::debug!(
                        "    🔍 Found OCR garbled character '{}' for {}, trying to infer value",
                        part,
                        attr_name
                    );

                    // Special case: "n" often represents "11" in OCR, "rn" represents "m" etc.
                    match (part, attr_name) {
                        // "n" commonly represents "11" in OCR when two 1's are close together
                        ("n", _) => {
                            log::debug!(
                                "    ✅ Inferring value 11 from OCR garbled 'n' for {}",
                                attr_name
                            );
                            return Some((attr_name.to_string(), 11));
                        }
                        // "ll" might represent "11" in some fonts
                        ("ll", _) => {
                            log::debug!(
                                "    ✅ Inferring value 11 from OCR garbled 'll' for {}",
                                attr_name
                            );
                            return Some((attr_name.to_string(), 11));
                        }
                        // "rn" commonly represents "12" in OCR
                        ("rn", _) => {
                            log::debug!(
                                "    ✅ Inferring value 12 from OCR garbled 'rn' for {}",
                                attr_name
                            );
                            return Some((attr_name.to_string(), 12));
                        }
                        _ => {
                            // For other garbled characters, we can't reliably infer the value
                            log::debug!(
                                "    ❌ Cannot reliably infer value from '{}' for {}",
                                part,
                                attr_name
                            );
                        }
                    }
                }
            }
        }
    }

    None
}

/// Get common OCR variations for Football Manager attribute names
fn get_fuzzy_attribute_patterns(attr_name: &str) -> Vec<&'static str> {
    match attr_name {
        // Physical attributes with common OCR issues
        "Agility" => vec![
            "Agtity", "Agtlity", "Agtlty", "Agllity", "Agyity", "Agility",
        ],
        "Acceleration" => vec!["Acceleratlon", "Acceleraton", "Acceleratton"],
        "Balance" => vec!["Baiance", "Baiance", "Balancc"],
        "Jumping Reach" => vec!["Jumplng Reach", "Jumping Rcach", "Jumping Rcach"],
        "Natural Fitness" => vec!["Naturai Fitness", "Natural Fltness"],
        "Stamina" => vec!["Stamlna", "Stamtna"],

        // Mental attributes
        "Anticipation" => vec!["Antlclpation", "Anticipatlon", "Antlcipatlon"],
        "Composure" => vec!["Composurc", "Composurc"],
        "Concentration" => vec!["Concentratlon", "Concentraton"],
        "Decisions" => vec!["Declslons", "Decistons"],
        "Determination" => vec!["Determlnatlon", "Determlnation"],
        "Off The Ball" => vec!["OffThe Ball", "Off Thc Ball", "Off The Bali", "OffTheBall"],
        "Positioning" => vec!["Posltlonlng", "Positionlng"],
        "Teamwork" => vec!["Tcamwork", "Teamworlk"],
        "Work Rate" => vec!["Worlk Rate", "Work Ratc"],

        // Technical attributes
        "Crossing" => vec!["Crosslng", "Crosslng"],
        "Dribbling" => vec!["Drlbbllng", "Drlbbling"],
        "Finishing" => vec!["Flnlshlng", "Flnishing"],
        "First Touch" => vec!["Flrst Touch", "First Toucli"],
        "Free Kick Taking" => vec!["Free Klck Taking", "Frce Kick Taking"],
        "Long Shots" => vec!["Long Shols", "Long Shots"],
        "Long Throws" => vec!["Long Throws", "Long Throvvs"],
        "Passing" => vec!["Passlng", "Passlng"],
        "Penalty Taking" => vec!["Penalty Taklng", "Penaliy Taking"],
        "Tackling" => vec!["Tackllng", "Tackhng"],
        "Technique" => vec!["Technlque", "Technlquc", "TecHnicat"],

        // Goalkeeping attributes
        "Aerial Reach" => vec!["Aerlal Reach", "Aerlal Rcach"],
        "Command Of Area" => vec!["Command OI Area", "Command Of Arca"],
        "Communication" => vec!["Communlcatlon", "Communlcation"],
        "Eccentricity" => vec!["Eccentrlclty", "Eccentrlcity"],
        "Handling" => vec!["Handllng", "Handllng"],
        "Kicking" => vec!["Klcklng", "Klcking"],
        "One On Ones" => vec!["One On Oncs", "Onc On Ones"],
        "Punching" => vec!["Punchlng", "Punchlng"],
        "Reflexes" => vec!["Reflexcs", "Rcflexes"],
        "Rushing Out" => vec!["Rushlng Out", "Rushlng Out"],
        "Throwing" => vec!["Throwlng", "Throwlng"],

        // Default case - no fuzzy matches
        _ => vec![],
    }
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
    fn test_extract_player_name_with_ocr_artifacts() {
        // Test OCR with artifacts like the reported issue (with hyphen)
        let ocr_text = "Ee 4S e 2 q 1-Alexander Westberg\n. Goalkeeper -Vigabyholms IK\nTECHNICAL\nCrossing 8\n";
        let result = extract_player_name(ocr_text).unwrap();
        assert_eq!(result, "Alexander Westberg");

        // Test OCR with artifacts like the actual current issue (no hyphen)
        let ocr_text =
            "ey A q 1Alexander Westberg\nmy Goalkeeper Vigabyholms Ik\nTECHNICAL\nCrossing 8\n";
        let result = extract_player_name(ocr_text).unwrap();
        assert_eq!(result, "Alexander Westberg");
    }

    #[test]
    fn test_extract_player_name_after_hyphen() {
        let ocr_text = "gibberish-John Smith\nGoalkeeper\nTECHNICAL\nCrossing 8\n";
        let result = extract_player_name(ocr_text).unwrap();
        assert_eq!(result, "John Smith");
    }

    #[test]
    fn test_extract_player_name_with_extra_text() {
        let ocr_text = "random text-David de Gea\nGoalkeeper Position\nTECHNICAL\nCrossing 8\n";
        let result = extract_player_name(ocr_text).unwrap();
        assert_eq!(result, "David de Gea");
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
    fn test_improved_name_extraction_patterns() {
        // Test priority 1: Direct name patterns - the issue is that "ey A q 1Alexander Westberg"
        // doesn't produce a valid name from extract_name_from_text due to artifacts
        let ocr_text = "Alexander Westberg artifacts\nmy Goalkeeper Vigabyholms Ik\nTECHNICAL\n";
        let result = extract_player_name(ocr_text).unwrap();
        assert_eq!(result, "Alexander Westberg");

        // Test priority 2: Years old pattern
        let ocr_text = "Random text\nWESTBERG 25 years old (2052001)\nTECHNICAL\n";
        let result = extract_player_name(ocr_text).unwrap();
        assert_eq!(result, "WESTBERG");
    }

    #[test]
    fn test_fuzzy_attribute_patterns() {
        // Test that fuzzy patterns are returned for known attributes
        let agility_patterns = get_fuzzy_attribute_patterns("Agility");
        assert!(agility_patterns.contains(&"Agtity"));
        assert!(agility_patterns.contains(&"Agtlity"));

        let off_ball_patterns = get_fuzzy_attribute_patterns("Off The Ball");
        assert!(off_ball_patterns.contains(&"OffThe Ball"));

        // Test unknown attribute returns empty
        let unknown_patterns = get_fuzzy_attribute_patterns("Unknown Attribute");
        assert!(unknown_patterns.is_empty());
    }

    #[test]
    fn test_ocr_garbled_number_inference() {
        // Test that find_attribute_in_line correctly infers values from garbled OCR characters
        // This tests the "n" -> "11" inference for Agility
        assert_eq!(
            find_attribute_in_line("B Command Of Area 8 Anticipation 9 Agtlity n", "Agility"),
            Some(("Agility".to_string(), 11))
        );

        // Test "rn" -> "12" inference (exact match case)
        assert_eq!(
            find_attribute_in_line("Balance rn", "Balance"),
            Some(("Balance".to_string(), 12))
        );

        // Test "ll" -> "11" inference (exact match case)
        assert_eq!(
            find_attribute_in_line("Pace ll", "Pace"),
            Some(("Pace".to_string(), 11))
        );
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
