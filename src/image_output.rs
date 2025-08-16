use crate::attributes::Attribute;
use crate::image_data::ImagePlayer;
use crate::types::{Footedness, PlayerType};
use log::{debug, warn};

/// Check for zero values in relevant attributes and print warnings
/// Uses unified attribute iteration instead of category-specific logic
fn check_zero_attributes(player: &ImagePlayer) {
    // Iterate through all attributes and check based on player type appropriateness
    for attr_index in 0..Attribute::count() {
        let attr = unsafe { std::mem::transmute::<u8, Attribute>(attr_index as u8) };
        let value = player.attributes.get(attr);

        // Only check attributes that are relevant for this player type
        let is_relevant = match player.player_type {
            PlayerType::FieldPlayer => {
                attr.is_technical() || attr.is_mental() || attr.is_physical()
            }
            PlayerType::Goalkeeper => {
                attr.is_goalkeeping() || attr.is_mental() || attr.is_physical()
            }
        };

        // Warn if relevant attribute has zero value
        if is_relevant && value == 0 {
            let category = if attr.is_technical() {
                "technical"
            } else if attr.is_mental() {
                "mental"
            } else if attr.is_physical() {
                "physical"
            } else if attr.is_goalkeeping() {
                "goalkeeping"
            } else {
                "unknown"
            };

            warn!(
                "Warning: {} has 0 value for {} attribute '{}'",
                player.name,
                category,
                attr.display_name()
            );
        }
    }
}

/// Format an ImagePlayer into tab-separated output with the exact attribute order
/// specified in the feature requirements
pub fn format_player_data(player: &ImagePlayer) -> String {
    debug!("Formatting player data for: {}", player.name);

    // Check for zero attribute values and print warnings
    check_zero_attributes(player);

    // Start with basic player information
    let mut output = Vec::new();
    output.push(player.name.clone());
    output.push(player.age.to_string());
    output.push(format_footedness(&player.footedness));

    // Technical attributes (enum indexes 0-13) - always included, 0 for goalkeepers
    for attr_index in 0..=13 {
        let attr = unsafe { std::mem::transmute::<u8, Attribute>(attr_index) };
        output.push(player.attributes.get(attr).to_string());
    }

    // Mental attributes (enum indexes 14-27) - common to all players
    for attr_index in 14..=27 {
        let attr = unsafe { std::mem::transmute::<u8, Attribute>(attr_index) };
        output.push(player.attributes.get(attr).to_string());
    }

    // Physical attributes (enum indexes 28-35) - common to all players
    for attr_index in 28..=35 {
        let attr = unsafe { std::mem::transmute::<u8, Attribute>(attr_index) };
        output.push(player.attributes.get(attr).to_string());
    }

    // Goalkeeping attributes - always included, 0 for field players
    // Using specific enum indexes that match the original specification
    let gk_attrs = [
        36, // AerialReach
        37, // CommandOfArea
        38, // Communication
        39, // Eccentricity
        41, // Handling
        42, // Kicking
        43, // OneOnOnes
        45, // PunchingTendency
        46, // Reflexes
        47, // RushingOutTendency
        48, // Throwing
    ];
    for &attr_index in &gk_attrs {
        let attr = unsafe { std::mem::transmute::<u8, Attribute>(attr_index) };
        output.push(player.attributes.get(attr).to_string());
    }

    let result = output.join("\t");
    debug!("Formatted player data: {} fields", output.len());
    result
}

/// Format player data in detailed KEY -> VALUE format for verbose output
pub fn format_player_data_verbose(player: &ImagePlayer) -> String {
    let mut output = Vec::new();

    // Basic player information
    output.push(format!("Name -> {}", player.name));
    output.push(format!("Age -> {}", player.age));
    output.push(format!("Type -> {:?}", player.player_type));
    output.push(format!(
        "Footedness -> {}",
        format_footedness_verbose(&player.footedness)
    ));

    // Technical attributes using unified system
    output.push("".to_string()); // Empty line for separation
    output.push("TECHNICAL ATTRIBUTES:".to_string());
    add_unified_attribute_group(
        &mut output,
        player,
        &[
            (Attribute::Corners, "Corners"),
            (Attribute::Crossing, "Crossing"),
            (Attribute::Dribbling, "Dribbling"),
            (Attribute::Finishing, "Finishing"),
            (Attribute::FirstTouch, "First Touch"),
            (Attribute::FreeKickTaking, "Free Kick Taking"),
            (Attribute::Heading, "Heading"),
            (Attribute::LongShots, "Long Shots"),
            (Attribute::LongThrows, "Long Throws"),
            (Attribute::Marking, "Marking"),
            (Attribute::Passing, "Passing"),
            (Attribute::PenaltyTaking, "Penalty Taking"),
            (Attribute::Tackling, "Tackling"),
            (Attribute::Technique, "Technique"),
        ],
    );

    // Mental attributes using unified system
    output.push("".to_string());
    output.push("MENTAL ATTRIBUTES:".to_string());
    add_unified_attribute_group(
        &mut output,
        player,
        &[
            (Attribute::Aggression, "Aggression"),
            (Attribute::Anticipation, "Anticipation"),
            (Attribute::Bravery, "Bravery"),
            (Attribute::Composure, "Composure"),
            (Attribute::Concentration, "Concentration"),
            (Attribute::Decisions, "Decisions"),
            (Attribute::Determination, "Determination"),
            (Attribute::Flair, "Flair"),
            (Attribute::Leadership, "Leadership"),
            (Attribute::OffTheBall, "Off the Ball"),
            (Attribute::Positioning, "Positioning"),
            (Attribute::Teamwork, "Teamwork"),
            (Attribute::Vision, "Vision"),
            (Attribute::WorkRate, "Work Rate"),
        ],
    );

    // Physical attributes using unified system
    output.push("".to_string());
    output.push("PHYSICAL ATTRIBUTES:".to_string());
    add_unified_attribute_group(
        &mut output,
        player,
        &[
            (Attribute::Acceleration, "Acceleration"),
            (Attribute::Agility, "Agility"),
            (Attribute::Balance, "Balance"),
            (Attribute::JumpingReach, "Jumping Reach"),
            (Attribute::NaturalFitness, "Natural Fitness"),
            (Attribute::Pace, "Pace"),
            (Attribute::Stamina, "Stamina"),
            (Attribute::Strength, "Strength"),
        ],
    );

    // Check if we should show goalkeeping attributes
    let goalkeeping_attrs = [
        Attribute::AerialReach,
        Attribute::CommandOfArea,
        Attribute::Communication,
        Attribute::Eccentricity,
        Attribute::GoalkeepingFirstTouch,
        Attribute::Handling,
        Attribute::Kicking,
        Attribute::OneOnOnes,
        Attribute::GoalkeepingPassing,
        Attribute::PunchingTendency,
        Attribute::Reflexes,
        Attribute::RushingOutTendency,
        Attribute::Throwing,
        Attribute::GoalkeepingWorkRate,
    ];

    let has_gk_attrs = goalkeeping_attrs
        .iter()
        .any(|&attr| player.attributes.get(attr) > 0);

    if matches!(player.player_type, PlayerType::Goalkeeper) || has_gk_attrs {
        output.push("".to_string());
        output.push("GOALKEEPING ATTRIBUTES:".to_string());
        add_unified_attribute_group(
            &mut output,
            player,
            &[
                (Attribute::AerialReach, "Aerial Reach"),
                (Attribute::CommandOfArea, "Command Of Area"),
                (Attribute::Communication, "Communication"),
                (Attribute::Eccentricity, "Eccentricity"),
                (Attribute::GoalkeepingFirstTouch, "First Touch"),
                (Attribute::Handling, "Handling"),
                (Attribute::Kicking, "Kicking"),
                (Attribute::OneOnOnes, "One On Ones"),
                (Attribute::GoalkeepingPassing, "Passing"),
                (Attribute::PunchingTendency, "Punching (Tendency)"),
                (Attribute::Reflexes, "Reflexes"),
                (Attribute::RushingOutTendency, "Rushing Out (Tendency)"),
                (Attribute::Throwing, "Throwing"),
                (Attribute::GoalkeepingWorkRate, "Work Rate"),
            ],
        );
    }

    output.join("\n")
}

/// Add all attributes from a specific group to the output using unified attribute system
fn add_unified_attribute_group(
    output: &mut Vec<String>,
    player: &ImagePlayer,
    attrs: &[(Attribute, &'static str)],
) {
    for &(attr, display_name) in attrs {
        let value = player.attributes.get(attr);
        output.push(format!("  {display_name} -> {value}"));
    }
}

/// Format footedness enum into verbose string representation
fn format_footedness_verbose(footedness: &Footedness) -> String {
    match footedness {
        Footedness::LeftFooted => "Left Footed".to_string(),
        Footedness::RightFooted => "Right Footed".to_string(),
        Footedness::BothFooted => "Both Footed".to_string(),
    }
}

/// Format footedness enum into the required string representation
fn format_footedness(footedness: &Footedness) -> String {
    match footedness {
        Footedness::LeftFooted => "l".to_string(),
        Footedness::RightFooted => "r".to_string(),
        Footedness::BothFooted => "lr".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::image_data::ImagePlayer;
    use crate::types::PlayerType;

    fn create_test_field_player() -> ImagePlayer {
        let mut player = ImagePlayer::new(
            "Virgil van Dijk".to_string(),
            32,
            PlayerType::FieldPlayer,
            Footedness::LeftFooted,
        );

        // Add some sample technical attributes
        player.add_attribute("technical_corners".to_string(), 8);
        player.add_attribute("technical_crossing".to_string(), 5);
        player.add_attribute("technical_heading".to_string(), 18);
        player.add_attribute("technical_passing".to_string(), 16);
        player.add_attribute("technical_technique".to_string(), 15);

        // Add some sample mental attributes
        player.add_attribute("mental_composure".to_string(), 18);
        player.add_attribute("mental_concentration".to_string(), 17);
        player.add_attribute("mental_decisions".to_string(), 16);
        player.add_attribute("mental_positioning".to_string(), 19);

        // Add some sample physical attributes
        player.add_attribute("physical_pace".to_string(), 12);
        player.add_attribute("physical_strength".to_string(), 19);
        player.add_attribute("physical_agility".to_string(), 14);

        player
    }

    fn create_test_goalkeeper() -> ImagePlayer {
        let mut player = ImagePlayer::new(
            "Alisson Becker".to_string(),
            30,
            PlayerType::Goalkeeper,
            Footedness::RightFooted,
        );

        // Add some sample technical attributes
        player.add_attribute("technical_first_touch".to_string(), 15);
        player.add_attribute("technical_passing".to_string(), 14);
        player.add_attribute("technical_technique".to_string(), 13);

        // Add some sample mental attributes
        player.add_attribute("mental_composure".to_string(), 16);
        player.add_attribute("mental_concentration".to_string(), 18);
        player.add_attribute("mental_decisions".to_string(), 17);

        // Add some sample physical attributes
        player.add_attribute("physical_agility".to_string(), 17);
        player.add_attribute("physical_pace".to_string(), 8);

        // Add goalkeeping attributes
        player.add_attribute("goalkeeping_reflexes".to_string(), 18);
        player.add_attribute("goalkeeping_handling".to_string(), 17);
        player.add_attribute("goalkeeping_command_of_area".to_string(), 16);
        player.add_attribute("goalkeeping_kicking".to_string(), 15);
        player.add_attribute("goalkeeping_one_on_ones".to_string(), 16);

        player
    }

    #[test]
    fn test_format_footedness() {
        assert_eq!(format_footedness(&Footedness::LeftFooted), "l");
        assert_eq!(format_footedness(&Footedness::RightFooted), "r");
        assert_eq!(format_footedness(&Footedness::BothFooted), "lr");
    }

    #[test]
    fn test_format_player_data_field_player() {
        let player = create_test_field_player();
        let result = format_player_data(&player);

        // Split by tabs to verify structure
        let fields: Vec<&str> = result.split('\t').collect();

        // Should have exactly 50 fields (name + age + footedness + 47 attributes)
        assert_eq!(fields.len(), 50);

        // Verify basic player info
        assert_eq!(fields[0], "Virgil van Dijk");
        assert_eq!(fields[1], "32");
        assert_eq!(fields[2], "l");

        // Verify some technical attributes (positions 3-16)
        assert_eq!(fields[3], "8"); // corners
        assert_eq!(fields[4], "5"); // crossing
        assert_eq!(fields[5], "0"); // dribbling (not set, should be 0)
        assert_eq!(fields[9], "18"); // heading
        assert_eq!(fields[13], "16"); // passing
        assert_eq!(fields[16], "15"); // technique

        // Verify some mental attributes (positions 17-30)
        assert_eq!(fields[20], "18"); // composure
        assert_eq!(fields[21], "17"); // concentration
        assert_eq!(fields[22], "16"); // decisions
        assert_eq!(fields[27], "19"); // positioning

        // Verify some physical attributes (positions 31-38)
        assert_eq!(fields[36], "12"); // pace (position 36 = 31+5)
        assert_eq!(fields[38], "19"); // strength (position 38 = 31+7)
        assert_eq!(fields[32], "14"); // agility (position 32 = 31+1)

        // Verify all goalkeeping attributes are 0 (positions 39-49)
        #[allow(clippy::needless_range_loop)]
        for i in 39..50 {
            assert_eq!(
                fields[i], "0",
                "Goalkeeping attribute at position {i} should be 0 for field player"
            );
        }
    }

    #[test]
    fn test_format_player_data_goalkeeper() {
        let player = create_test_goalkeeper();
        let result = format_player_data(&player);

        // Split by tabs to verify structure
        let fields: Vec<&str> = result.split('\t').collect();

        // Should have exactly 50 fields
        assert_eq!(fields.len(), 50);

        // Verify basic player info
        assert_eq!(fields[0], "Alisson Becker");
        assert_eq!(fields[1], "30");
        assert_eq!(fields[2], "r");

        // Verify some technical attributes are present
        assert_eq!(fields[7], "15"); // first_touch
        assert_eq!(fields[13], "14"); // passing
        assert_eq!(fields[16], "13"); // technique

        // Verify goalkeeping attributes are not 0 (positions 39-49)
        assert_eq!(fields[47], "18"); // reflexes (position 39+8)
        assert_eq!(fields[43], "17"); // handling (position 39+4)
        assert_eq!(fields[40], "16"); // command_of_area (position 39+1)
        assert_eq!(fields[44], "15"); // kicking (position 39+5)
        assert_eq!(fields[45], "16"); // one_on_ones (position 39+6)
    }

    #[test]
    fn test_format_player_data_missing_attributes_as_zeros() {
        // Create a player with minimal attributes
        let mut player = ImagePlayer::new(
            "Test Player".to_string(),
            25,
            PlayerType::FieldPlayer,
            Footedness::BothFooted,
        );

        // Only add one attribute
        player.add_attribute("technical_crossing".to_string(), 10);

        let result = format_player_data(&player);
        let fields: Vec<&str> = result.split('\t').collect();

        // Should still have 50 fields
        assert_eq!(fields.len(), 50);

        // Basic info
        assert_eq!(fields[0], "Test Player");
        assert_eq!(fields[1], "25");
        assert_eq!(fields[2], "lr");

        // The one attribute we set
        assert_eq!(fields[4], "10"); // crossing

        // All other attributes should be 0
        #[allow(clippy::needless_range_loop)]
        for i in 3..50 {
            if i != 4 {
                // Skip the crossing attribute we set
                assert_eq!(fields[i], "0", "Attribute at position {i} should be 0");
            }
        }
    }

    #[test]
    fn test_format_player_data_tab_separation() {
        let player = create_test_field_player();
        let result = format_player_data(&player);

        // Verify it contains tabs
        assert!(result.contains('\t'));

        // Verify it doesn't contain other common separators
        assert!(!result.contains(','));
        assert!(!result.contains(';'));
        assert!(!result.contains('|'));

        // Count tabs - should be 49 (50 fields - 1)
        let tab_count = result.matches('\t').count();
        assert_eq!(tab_count, 49);
    }

    #[test]
    fn test_format_player_data_both_footed_player() {
        let player = ImagePlayer::new(
            "Ambidextrous Player".to_string(),
            28,
            PlayerType::FieldPlayer,
            Footedness::BothFooted,
        );

        let result = format_player_data(&player);
        let fields: Vec<&str> = result.split('\t').collect();

        assert_eq!(fields[0], "Ambidextrous Player");
        assert_eq!(fields[1], "28");
        assert_eq!(fields[2], "lr");
    }

    #[test]
    fn test_zero_attribute_warnings_field_player() {
        use crate::image_data::ImagePlayer;

        // Create a field player with some zero values
        let mut player = ImagePlayer::new(
            "Test Player".to_string(),
            25,
            PlayerType::FieldPlayer,
            Footedness::RightFooted,
        );

        // Add only some attributes, leaving others at 0
        player.add_attribute("technical_crossing".to_string(), 15);
        player.add_attribute("mental_composure".to_string(), 18);
        player.add_attribute("physical_pace".to_string(), 12);

        // This will trigger warnings for all zero attributes
        let _result = format_player_data(&player);
        // Warnings are logged, so we can't easily test them in unit tests
        // but this ensures the function runs without panicking
    }

    #[test]
    fn test_zero_attribute_warnings_goalkeeper() {
        use crate::image_data::ImagePlayer;

        // Create a goalkeeper with some zero values
        let mut player = ImagePlayer::new(
            "Test Keeper".to_string(),
            28,
            PlayerType::Goalkeeper,
            Footedness::LeftFooted,
        );

        // Add only some attributes, leaving others at 0
        player.add_attribute("goalkeeping_reflexes".to_string(), 18);
        player.add_attribute("mental_concentration".to_string(), 16);
        player.add_attribute("physical_agility".to_string(), 14);

        // This will trigger warnings for all zero attributes
        let _result = format_player_data(&player);
        // Warnings are logged, so we can't easily test them in unit tests
        // but this ensures the function runs without panicking
    }
}
