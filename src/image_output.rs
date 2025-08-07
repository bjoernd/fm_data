use crate::image_data::{Footedness, ImagePlayer};
use log::debug;

/// Format an ImagePlayer into tab-separated output with the exact attribute order
/// specified in the feature requirements
#[allow(clippy::vec_init_then_push)]
pub fn format_player_data(player: &ImagePlayer) -> String {
    debug!("Formatting player data for: {}", player.name);

    // Start with basic player information
    let mut output = Vec::new();
    output.push(player.name.clone());
    output.push(player.age.to_string());
    output.push(format_footedness(&player.footedness));

    // Technical attributes (in exact specification order)
    output.push(player.get_attribute("technical_corners").to_string());
    output.push(player.get_attribute("technical_crossing").to_string());
    output.push(player.get_attribute("technical_dribbling").to_string());
    output.push(player.get_attribute("technical_finishing").to_string());
    output.push(player.get_attribute("technical_first_touch").to_string());
    output.push(
        player
            .get_attribute("technical_free_kick_taking")
            .to_string(),
    );
    output.push(player.get_attribute("technical_heading").to_string());
    output.push(player.get_attribute("technical_long_shots").to_string());
    output.push(player.get_attribute("technical_long_throws").to_string());
    output.push(player.get_attribute("technical_marking").to_string());
    output.push(player.get_attribute("technical_passing").to_string());
    output.push(player.get_attribute("technical_penalty_taking").to_string());
    output.push(player.get_attribute("technical_tackling").to_string());
    output.push(player.get_attribute("technical_technique").to_string());

    // Mental attributes (in exact specification order)
    output.push(player.get_attribute("mental_aggression").to_string());
    output.push(player.get_attribute("mental_anticipation").to_string());
    output.push(player.get_attribute("mental_bravery").to_string());
    output.push(player.get_attribute("mental_composure").to_string());
    output.push(player.get_attribute("mental_concentration").to_string());
    output.push(player.get_attribute("mental_decisions").to_string());
    output.push(player.get_attribute("mental_determination").to_string());
    output.push(player.get_attribute("mental_flair").to_string());
    output.push(player.get_attribute("mental_leadership").to_string());
    output.push(player.get_attribute("mental_off_the_ball").to_string());
    output.push(player.get_attribute("mental_positioning").to_string());
    output.push(player.get_attribute("mental_teamwork").to_string());
    output.push(player.get_attribute("mental_vision").to_string());
    output.push(player.get_attribute("mental_work_rate").to_string());

    // Physical attributes (in exact specification order)
    output.push(player.get_attribute("physical_acceleration").to_string());
    output.push(player.get_attribute("physical_agility").to_string());
    output.push(player.get_attribute("physical_balance").to_string());
    output.push(player.get_attribute("physical_jumping_reach").to_string());
    output.push(player.get_attribute("physical_natural_fitness").to_string());
    output.push(player.get_attribute("physical_pace").to_string());
    output.push(player.get_attribute("physical_stamina").to_string());
    output.push(player.get_attribute("physical_strength").to_string());

    // Goalkeeping attributes (in exact specification order)
    // These will be 0 for field players, actual values for goalkeepers
    output.push(player.get_attribute("goalkeeping_aerial_reach").to_string());
    output.push(
        player
            .get_attribute("goalkeeping_command_of_area")
            .to_string(),
    );
    output.push(
        player
            .get_attribute("goalkeeping_communication")
            .to_string(),
    );
    output.push(player.get_attribute("goalkeeping_eccentricity").to_string());
    output.push(player.get_attribute("goalkeeping_handling").to_string());
    output.push(player.get_attribute("goalkeeping_kicking").to_string());
    output.push(player.get_attribute("goalkeeping_one_on_ones").to_string());
    output.push(
        player
            .get_attribute("goalkeeping_punching_tendency")
            .to_string(),
    );
    output.push(player.get_attribute("goalkeeping_reflexes").to_string());
    output.push(
        player
            .get_attribute("goalkeeping_rushing_out_tendency")
            .to_string(),
    );
    output.push(player.get_attribute("goalkeeping_throwing").to_string());

    let result = output.join("\t");
    debug!("Formatted player data: {} fields", output.len());
    result
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
    use crate::image_data::{ImagePlayer, PlayerType};

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
}
