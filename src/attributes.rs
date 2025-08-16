use anyhow;
use std::collections::HashMap;
use std::sync::LazyLock;

/// Unified attribute enum containing all possible Football Manager attributes
/// Uses sequential indexing for O(1) array access
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Attribute {
    // Technical attributes (0-13) - field players only
    Corners = 0,
    Crossing = 1,
    Dribbling = 2,
    Finishing = 3,
    FirstTouch = 4,
    FreeKickTaking = 5,
    Heading = 6,
    LongShots = 7,
    LongThrows = 8,
    Marking = 9,
    Passing = 10,
    PenaltyTaking = 11,
    Tackling = 12,
    Technique = 13,

    // Mental attributes (14-27) - common to all players
    Aggression = 14,
    Anticipation = 15,
    Bravery = 16,
    Composure = 17,
    Concentration = 18,
    Decisions = 19,
    Determination = 20,
    Flair = 21,
    Leadership = 22,
    OffTheBall = 23,
    Positioning = 24,
    Teamwork = 25,
    Vision = 26,
    WorkRate = 27,

    // Physical attributes (28-35) - common to all players
    Acceleration = 28,
    Agility = 29,
    Balance = 30,
    JumpingReach = 31,
    NaturalFitness = 32,
    Pace = 33,
    Stamina = 34,
    Strength = 35,

    // Goalkeeping attributes (36-49) - goalkeepers only
    AerialReach = 36,
    CommandOfArea = 37,
    Communication = 38,
    Eccentricity = 39,
    GoalkeepingFirstTouch = 40,
    Handling = 41,
    Kicking = 42,
    OneOnOnes = 43,
    GoalkeepingPassing = 44,
    PunchingTendency = 45,
    Reflexes = 46,
    RushingOutTendency = 47,
    Throwing = 48,
    GoalkeepingWorkRate = 49,
}

impl Attribute {
    /// Get the total number of attributes
    pub const fn count() -> usize {
        50
    }

    /// Get the display name for this attribute
    pub const fn display_name(&self) -> &'static str {
        match self {
            // Technical attributes
            Attribute::Corners => "Corners",
            Attribute::Crossing => "Crossing",
            Attribute::Dribbling => "Dribbling",
            Attribute::Finishing => "Finishing",
            Attribute::FirstTouch => "First Touch",
            Attribute::FreeKickTaking => "Free Kick Taking",
            Attribute::Heading => "Heading",
            Attribute::LongShots => "Long Shots",
            Attribute::LongThrows => "Long Throws",
            Attribute::Marking => "Marking",
            Attribute::Passing => "Passing",
            Attribute::PenaltyTaking => "Penalty Taking",
            Attribute::Tackling => "Tackling",
            Attribute::Technique => "Technique",

            // Mental attributes
            Attribute::Aggression => "Aggression",
            Attribute::Anticipation => "Anticipation",
            Attribute::Bravery => "Bravery",
            Attribute::Composure => "Composure",
            Attribute::Concentration => "Concentration",
            Attribute::Decisions => "Decisions",
            Attribute::Determination => "Determination",
            Attribute::Flair => "Flair",
            Attribute::Leadership => "Leadership",
            Attribute::OffTheBall => "Off the Ball",
            Attribute::Positioning => "Positioning",
            Attribute::Teamwork => "Teamwork",
            Attribute::Vision => "Vision",
            Attribute::WorkRate => "Work Rate",

            // Physical attributes
            Attribute::Acceleration => "Acceleration",
            Attribute::Agility => "Agility",
            Attribute::Balance => "Balance",
            Attribute::JumpingReach => "Jumping Reach",
            Attribute::NaturalFitness => "Natural Fitness",
            Attribute::Pace => "Pace",
            Attribute::Stamina => "Stamina",
            Attribute::Strength => "Strength",

            // Goalkeeping attributes
            Attribute::AerialReach => "Aerial Reach",
            Attribute::CommandOfArea => "Command of Area",
            Attribute::Communication => "Communication",
            Attribute::Eccentricity => "Eccentricity",
            Attribute::GoalkeepingFirstTouch => "First Touch",
            Attribute::Handling => "Handling",
            Attribute::Kicking => "Kicking",
            Attribute::OneOnOnes => "1on1s",
            Attribute::GoalkeepingPassing => "Passing",
            Attribute::PunchingTendency => "Punching",
            Attribute::Reflexes => "Reflexes",
            Attribute::RushingOutTendency => "Rushing Out",
            Attribute::Throwing => "Throwing",
            Attribute::GoalkeepingWorkRate => "Work Rate",
        }
    }
}

/// Global attribute name lookup table for O(1) name-based access
static ATTRIBUTE_LOOKUP: LazyLock<HashMap<&'static str, Attribute>> = LazyLock::new(|| {
    let mut map = HashMap::new();

    // Technical attributes with both prefixed and clean names
    map.insert("technical_corners", Attribute::Corners);
    map.insert("corners", Attribute::Corners);
    map.insert("Corners", Attribute::Corners);

    map.insert("technical_crossing", Attribute::Crossing);
    map.insert("crossing", Attribute::Crossing);
    map.insert("Crossing", Attribute::Crossing);

    map.insert("technical_dribbling", Attribute::Dribbling);
    map.insert("dribbling", Attribute::Dribbling);
    map.insert("Dribbling", Attribute::Dribbling);

    map.insert("technical_finishing", Attribute::Finishing);
    map.insert("finishing", Attribute::Finishing);
    map.insert("Finishing", Attribute::Finishing);

    map.insert("technical_first_touch", Attribute::FirstTouch);
    map.insert("first_touch", Attribute::FirstTouch);
    map.insert("First Touch", Attribute::FirstTouch);

    map.insert("technical_free_kick_taking", Attribute::FreeKickTaking);
    map.insert("free_kick_taking", Attribute::FreeKickTaking);
    map.insert("Free Kick Taking", Attribute::FreeKickTaking);

    map.insert("technical_heading", Attribute::Heading);
    map.insert("heading", Attribute::Heading);
    map.insert("Heading", Attribute::Heading);

    map.insert("technical_long_shots", Attribute::LongShots);
    map.insert("long_shots", Attribute::LongShots);
    map.insert("Long Shots", Attribute::LongShots);

    map.insert("technical_long_throws", Attribute::LongThrows);
    map.insert("long_throws", Attribute::LongThrows);
    map.insert("Long Throws", Attribute::LongThrows);

    map.insert("technical_marking", Attribute::Marking);
    map.insert("marking", Attribute::Marking);
    map.insert("Marking", Attribute::Marking);

    map.insert("technical_passing", Attribute::Passing);
    map.insert("passing", Attribute::Passing);
    map.insert("Passing", Attribute::Passing);

    map.insert("technical_penalty_taking", Attribute::PenaltyTaking);
    map.insert("penalty_taking", Attribute::PenaltyTaking);
    map.insert("Penalty Taking", Attribute::PenaltyTaking);

    map.insert("technical_tackling", Attribute::Tackling);
    map.insert("tackling", Attribute::Tackling);
    map.insert("Tackling", Attribute::Tackling);

    map.insert("technical_technique", Attribute::Technique);
    map.insert("technique", Attribute::Technique);
    map.insert("Technique", Attribute::Technique);

    // Mental attributes
    map.insert("mental_aggression", Attribute::Aggression);
    map.insert("aggression", Attribute::Aggression);
    map.insert("Aggression", Attribute::Aggression);

    map.insert("mental_anticipation", Attribute::Anticipation);
    map.insert("anticipation", Attribute::Anticipation);
    map.insert("Anticipation", Attribute::Anticipation);

    map.insert("mental_bravery", Attribute::Bravery);
    map.insert("bravery", Attribute::Bravery);
    map.insert("Bravery", Attribute::Bravery);

    map.insert("mental_composure", Attribute::Composure);
    map.insert("composure", Attribute::Composure);
    map.insert("Composure", Attribute::Composure);

    map.insert("mental_concentration", Attribute::Concentration);
    map.insert("concentration", Attribute::Concentration);
    map.insert("Concentration", Attribute::Concentration);

    map.insert("mental_decisions", Attribute::Decisions);
    map.insert("decisions", Attribute::Decisions);
    map.insert("Decisions", Attribute::Decisions);

    map.insert("mental_determination", Attribute::Determination);
    map.insert("determination", Attribute::Determination);
    map.insert("Determination", Attribute::Determination);

    map.insert("mental_flair", Attribute::Flair);
    map.insert("flair", Attribute::Flair);
    map.insert("Flair", Attribute::Flair);

    map.insert("mental_leadership", Attribute::Leadership);
    map.insert("leadership", Attribute::Leadership);
    map.insert("Leadership", Attribute::Leadership);

    map.insert("mental_off_the_ball", Attribute::OffTheBall);
    map.insert("off_the_ball", Attribute::OffTheBall);
    map.insert("Off the Ball", Attribute::OffTheBall);

    map.insert("mental_positioning", Attribute::Positioning);
    map.insert("positioning", Attribute::Positioning);
    map.insert("Positioning", Attribute::Positioning);

    map.insert("mental_teamwork", Attribute::Teamwork);
    map.insert("teamwork", Attribute::Teamwork);
    map.insert("Teamwork", Attribute::Teamwork);

    map.insert("mental_vision", Attribute::Vision);
    map.insert("vision", Attribute::Vision);
    map.insert("Vision", Attribute::Vision);

    map.insert("mental_work_rate", Attribute::WorkRate);
    map.insert("work_rate", Attribute::WorkRate);
    map.insert("Work Rate", Attribute::WorkRate);

    // Physical attributes
    map.insert("physical_acceleration", Attribute::Acceleration);
    map.insert("acceleration", Attribute::Acceleration);
    map.insert("Acceleration", Attribute::Acceleration);

    map.insert("physical_agility", Attribute::Agility);
    map.insert("agility", Attribute::Agility);
    map.insert("Agility", Attribute::Agility);

    map.insert("physical_balance", Attribute::Balance);
    map.insert("balance", Attribute::Balance);
    map.insert("Balance", Attribute::Balance);

    map.insert("physical_jumping_reach", Attribute::JumpingReach);
    map.insert("jumping_reach", Attribute::JumpingReach);
    map.insert("Jumping Reach", Attribute::JumpingReach);

    map.insert("physical_natural_fitness", Attribute::NaturalFitness);
    map.insert("natural_fitness", Attribute::NaturalFitness);
    map.insert("Natural Fitness", Attribute::NaturalFitness);

    map.insert("physical_pace", Attribute::Pace);
    map.insert("pace", Attribute::Pace);
    map.insert("Pace", Attribute::Pace);

    map.insert("physical_stamina", Attribute::Stamina);
    map.insert("stamina", Attribute::Stamina);
    map.insert("Stamina", Attribute::Stamina);

    map.insert("physical_strength", Attribute::Strength);
    map.insert("strength", Attribute::Strength);
    map.insert("Strength", Attribute::Strength);

    // Goalkeeping attributes
    map.insert("goalkeeping_aerial_reach", Attribute::AerialReach);
    map.insert("aerial_reach", Attribute::AerialReach);
    map.insert("Aerial Reach", Attribute::AerialReach);

    map.insert("goalkeeping_command_of_area", Attribute::CommandOfArea);
    map.insert("command_of_area", Attribute::CommandOfArea);
    map.insert("Command of Area", Attribute::CommandOfArea);

    map.insert("goalkeeping_communication", Attribute::Communication);
    map.insert("communication", Attribute::Communication);
    map.insert("Communication", Attribute::Communication);

    map.insert("goalkeeping_eccentricity", Attribute::Eccentricity);
    map.insert("eccentricity", Attribute::Eccentricity);
    map.insert("Eccentricity", Attribute::Eccentricity);

    map.insert("goalkeeping_first_touch", Attribute::GoalkeepingFirstTouch);

    map.insert("goalkeeping_handling", Attribute::Handling);
    map.insert("handling", Attribute::Handling);
    map.insert("Handling", Attribute::Handling);

    map.insert("goalkeeping_kicking", Attribute::Kicking);
    map.insert("kicking", Attribute::Kicking);
    map.insert("Kicking", Attribute::Kicking);

    map.insert("goalkeeping_one_on_ones", Attribute::OneOnOnes);
    map.insert("one_on_ones", Attribute::OneOnOnes);
    map.insert("1on1s", Attribute::OneOnOnes);

    map.insert("goalkeeping_passing", Attribute::GoalkeepingPassing);

    map.insert("goalkeeping_punching_tendency", Attribute::PunchingTendency);
    map.insert("punching_tendency", Attribute::PunchingTendency);
    map.insert("Punching", Attribute::PunchingTendency);

    map.insert("goalkeeping_reflexes", Attribute::Reflexes);
    map.insert("reflexes", Attribute::Reflexes);
    map.insert("Reflexes", Attribute::Reflexes);

    map.insert(
        "goalkeeping_rushing_out_tendency",
        Attribute::RushingOutTendency,
    );
    map.insert("rushing_out_tendency", Attribute::RushingOutTendency);
    map.insert("Rushing Out", Attribute::RushingOutTendency);

    map.insert("goalkeeping_throwing", Attribute::Throwing);
    map.insert("throwing", Attribute::Throwing);
    map.insert("Throwing", Attribute::Throwing);

    map.insert("goalkeeping_work_rate", Attribute::GoalkeepingWorkRate);

    map
});

/// Simplified player attributes storage with unified access
#[derive(Debug, Clone)]
pub struct PlayerAttributes {
    // All attributes stored in a single Vec with O(1) access by index
    attributes: Vec<u8>,
}

impl Default for PlayerAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl PlayerAttributes {
    /// Create new empty player attributes
    pub fn new() -> Self {
        Self {
            attributes: vec![0; Attribute::count()],
        }
    }

    /// Get attribute value by enum
    pub fn get(&self, attr: Attribute) -> u8 {
        self.attributes[attr as usize]
    }

    /// Set attribute value by enum
    pub fn set(&mut self, attr: Attribute, value: u8) {
        self.attributes[attr as usize] = value;
    }

    /// Get attribute value by name
    pub fn get_by_name(&self, name: &str) -> Option<u8> {
        ATTRIBUTE_LOOKUP.get(name).map(|&attr| self.get(attr))
    }

    /// Set attribute value by name
    pub fn set_by_name(&mut self, name: &str, value: u8) -> anyhow::Result<()> {
        match ATTRIBUTE_LOOKUP.get(name) {
            Some(&attr) => {
                self.set(attr, value);
                Ok(())
            }
            None => Err(anyhow::anyhow!("Unknown attribute: {}", name)),
        }
    }

    /// Convert to HashMap with all attributes (for backward compatibility)
    pub fn to_hashmap(&self) -> HashMap<String, u8> {
        let mut map = HashMap::new();

        // Technical attributes
        for i in 0..=13 {
            let attr = unsafe { std::mem::transmute::<u8, Attribute>(i) };
            let key = match attr {
                Attribute::Corners => "technical_corners",
                Attribute::Crossing => "technical_crossing",
                Attribute::Dribbling => "technical_dribbling",
                Attribute::Finishing => "technical_finishing",
                Attribute::FirstTouch => "technical_first_touch",
                Attribute::FreeKickTaking => "technical_free_kick_taking",
                Attribute::Heading => "technical_heading",
                Attribute::LongShots => "technical_long_shots",
                Attribute::LongThrows => "technical_long_throws",
                Attribute::Marking => "technical_marking",
                Attribute::Passing => "technical_passing",
                Attribute::PenaltyTaking => "technical_penalty_taking",
                Attribute::Tackling => "technical_tackling",
                Attribute::Technique => "technical_technique",
                _ => unreachable!(),
            };
            map.insert(key.to_string(), self.get(attr));
        }

        // Mental attributes
        for i in 14..=27 {
            let attr = unsafe { std::mem::transmute::<u8, Attribute>(i) };
            let key = match attr {
                Attribute::Aggression => "mental_aggression",
                Attribute::Anticipation => "mental_anticipation",
                Attribute::Bravery => "mental_bravery",
                Attribute::Composure => "mental_composure",
                Attribute::Concentration => "mental_concentration",
                Attribute::Decisions => "mental_decisions",
                Attribute::Determination => "mental_determination",
                Attribute::Flair => "mental_flair",
                Attribute::Leadership => "mental_leadership",
                Attribute::OffTheBall => "mental_off_the_ball",
                Attribute::Positioning => "mental_positioning",
                Attribute::Teamwork => "mental_teamwork",
                Attribute::Vision => "mental_vision",
                Attribute::WorkRate => "mental_work_rate",
                _ => unreachable!(),
            };
            map.insert(key.to_string(), self.get(attr));
        }

        // Physical attributes
        for i in 28..=35 {
            let attr = unsafe { std::mem::transmute::<u8, Attribute>(i) };
            let key = match attr {
                Attribute::Acceleration => "physical_acceleration",
                Attribute::Agility => "physical_agility",
                Attribute::Balance => "physical_balance",
                Attribute::JumpingReach => "physical_jumping_reach",
                Attribute::NaturalFitness => "physical_natural_fitness",
                Attribute::Pace => "physical_pace",
                Attribute::Stamina => "physical_stamina",
                Attribute::Strength => "physical_strength",
                _ => unreachable!(),
            };
            map.insert(key.to_string(), self.get(attr));
        }

        // Goalkeeping attributes
        for i in 36..=49 {
            let attr = unsafe { std::mem::transmute::<u8, Attribute>(i) };
            let key = match attr {
                Attribute::AerialReach => "goalkeeping_aerial_reach",
                Attribute::CommandOfArea => "goalkeeping_command_of_area",
                Attribute::Communication => "goalkeeping_communication",
                Attribute::Eccentricity => "goalkeeping_eccentricity",
                Attribute::GoalkeepingFirstTouch => "goalkeeping_first_touch",
                Attribute::Handling => "goalkeeping_handling",
                Attribute::Kicking => "goalkeeping_kicking",
                Attribute::OneOnOnes => "goalkeeping_one_on_ones",
                Attribute::GoalkeepingPassing => "goalkeeping_passing",
                Attribute::PunchingTendency => "goalkeeping_punching_tendency",
                Attribute::Reflexes => "goalkeeping_reflexes",
                Attribute::RushingOutTendency => "goalkeeping_rushing_out_tendency",
                Attribute::Throwing => "goalkeeping_throwing",
                Attribute::GoalkeepingWorkRate => "goalkeeping_work_rate",
                _ => unreachable!(),
            };
            map.insert(key.to_string(), self.get(attr));
        }

        map
    }

    /// Convert from HashMap (for backward compatibility)
    pub fn from_hashmap(map: &HashMap<String, u8>) -> Self {
        let mut attributes = Self::new();

        for (key, &value) in map {
            if let Ok(()) = attributes.set_by_name(key, value) {
                // Successfully set the attribute
            }
            // Ignore unknown attributes for backward compatibility
        }

        attributes
    }

    /// Convert to HashMap with only non-zero attributes
    pub fn to_non_zero_hashmap(&self) -> HashMap<String, u8> {
        self.to_hashmap()
            .into_iter()
            .filter(|(_, v)| *v > 0)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // New tests for unified PlayerAttributes system
    mod unified_attributes_tests {
        use super::*;

        #[test]
        fn test_new_player_attributes() {
            let attrs = PlayerAttributes::new();
            assert_eq!(attrs.get(Attribute::Corners), 0);
            assert_eq!(attrs.get(Attribute::Determination), 0);
            assert_eq!(attrs.get(Attribute::Pace), 0);
            assert_eq!(attrs.get(Attribute::Reflexes), 0);
        }

        #[test]
        fn test_direct_attribute_access() {
            let mut attrs = PlayerAttributes::new();

            // Test technical attributes
            attrs.set(Attribute::Corners, 15);
            assert_eq!(attrs.get(Attribute::Corners), 15);

            attrs.set(Attribute::Finishing, 18);
            assert_eq!(attrs.get(Attribute::Finishing), 18);

            // Test mental attributes
            attrs.set(Attribute::Determination, 19);
            assert_eq!(attrs.get(Attribute::Determination), 19);

            attrs.set(Attribute::Vision, 16);
            assert_eq!(attrs.get(Attribute::Vision), 16);

            // Test physical attributes
            attrs.set(Attribute::Pace, 17);
            assert_eq!(attrs.get(Attribute::Pace), 17);

            attrs.set(Attribute::Strength, 14);
            assert_eq!(attrs.get(Attribute::Strength), 14);

            // Test goalkeeping attributes
            attrs.set(Attribute::Reflexes, 20);
            assert_eq!(attrs.get(Attribute::Reflexes), 20);

            attrs.set(Attribute::Handling, 18);
            assert_eq!(attrs.get(Attribute::Handling), 18);
        }

        #[test]
        fn test_name_based_access() {
            let mut attrs = PlayerAttributes::new();

            // Test prefixed names
            attrs.set_by_name("technical_corners", 15).unwrap();
            assert_eq!(attrs.get_by_name("technical_corners"), Some(15));

            attrs.set_by_name("mental_determination", 19).unwrap();
            assert_eq!(attrs.get_by_name("mental_determination"), Some(19));

            attrs.set_by_name("physical_pace", 17).unwrap();
            assert_eq!(attrs.get_by_name("physical_pace"), Some(17));

            attrs.set_by_name("goalkeeping_reflexes", 20).unwrap();
            assert_eq!(attrs.get_by_name("goalkeeping_reflexes"), Some(20));

            // Test clean names
            attrs.set_by_name("corners", 12).unwrap();
            assert_eq!(attrs.get_by_name("corners"), Some(12));
            assert_eq!(attrs.get_by_name("technical_corners"), Some(12)); // Should be same value

            attrs.set_by_name("Corners", 10).unwrap();
            assert_eq!(attrs.get_by_name("Corners"), Some(10));
            assert_eq!(attrs.get_by_name("corners"), Some(10)); // Should be same value

            // Test unknown attributes
            assert!(attrs.set_by_name("unknown_attribute", 15).is_err());
            assert_eq!(attrs.get_by_name("unknown_attribute"), None);
        }

        #[test]
        fn test_display_names() {
            assert_eq!(Attribute::Corners.display_name(), "Corners");
            assert_eq!(Attribute::FirstTouch.display_name(), "First Touch");
            assert_eq!(Attribute::FreeKickTaking.display_name(), "Free Kick Taking");
            assert_eq!(Attribute::OffTheBall.display_name(), "Off the Ball");
            assert_eq!(Attribute::JumpingReach.display_name(), "Jumping Reach");
            assert_eq!(Attribute::OneOnOnes.display_name(), "1on1s");
            assert_eq!(Attribute::PunchingTendency.display_name(), "Punching");
            assert_eq!(Attribute::RushingOutTendency.display_name(), "Rushing Out");
        }

        #[test]
        fn test_hashmap_conversion() {
            let mut attrs = PlayerAttributes::new();

            // Set some test values
            attrs.set(Attribute::Corners, 15);
            attrs.set(Attribute::Determination, 19);
            attrs.set(Attribute::Pace, 17);
            attrs.set(Attribute::Reflexes, 20);

            let map = attrs.to_hashmap();

            // Verify all attributes are present
            assert_eq!(map.len(), 50); // All 50 attributes should be present

            // Verify set values
            assert_eq!(map.get("technical_corners"), Some(&15));
            assert_eq!(map.get("mental_determination"), Some(&19));
            assert_eq!(map.get("physical_pace"), Some(&17));
            assert_eq!(map.get("goalkeeping_reflexes"), Some(&20));

            // Verify unset values are 0
            assert_eq!(map.get("technical_crossing"), Some(&0));
            assert_eq!(map.get("mental_aggression"), Some(&0));
            assert_eq!(map.get("physical_strength"), Some(&0));
            assert_eq!(map.get("goalkeeping_handling"), Some(&0));
        }

        #[test]
        fn test_from_hashmap() {
            let mut map = HashMap::new();
            map.insert("technical_corners".to_string(), 15);
            map.insert("mental_determination".to_string(), 19);
            map.insert("physical_pace".to_string(), 17);
            map.insert("goalkeeping_reflexes".to_string(), 20);
            map.insert("unknown_attribute".to_string(), 99); // Should be ignored

            let attrs = PlayerAttributes::from_hashmap(&map);

            assert_eq!(attrs.get(Attribute::Corners), 15);
            assert_eq!(attrs.get(Attribute::Determination), 19);
            assert_eq!(attrs.get(Attribute::Pace), 17);
            assert_eq!(attrs.get(Attribute::Reflexes), 20);

            // Other attributes should be 0
            assert_eq!(attrs.get(Attribute::Crossing), 0);
            assert_eq!(attrs.get(Attribute::Aggression), 0);
        }

        #[test]
        fn test_round_trip_conversion() {
            let mut original_attrs = PlayerAttributes::new();
            original_attrs.set(Attribute::Finishing, 18);
            original_attrs.set(Attribute::Vision, 16);
            original_attrs.set(Attribute::Acceleration, 15);
            original_attrs.set(Attribute::Handling, 19);

            let map = original_attrs.to_hashmap();
            let converted_attrs = PlayerAttributes::from_hashmap(&map);

            // Verify all values are preserved
            assert_eq!(converted_attrs.get(Attribute::Finishing), 18);
            assert_eq!(converted_attrs.get(Attribute::Vision), 16);
            assert_eq!(converted_attrs.get(Attribute::Acceleration), 15);
            assert_eq!(converted_attrs.get(Attribute::Handling), 19);

            // Verify other attributes remain 0
            assert_eq!(converted_attrs.get(Attribute::Corners), 0);
            assert_eq!(converted_attrs.get(Attribute::Determination), 0);
        }

        #[test]
        fn test_non_zero_hashmap() {
            let mut attrs = PlayerAttributes::new();
            attrs.set(Attribute::Corners, 15);
            attrs.set(Attribute::Determination, 0); // Should be filtered out
            attrs.set(Attribute::Pace, 17);
            attrs.set(Attribute::Reflexes, 0); // Should be filtered out

            let non_zero_map = attrs.to_non_zero_hashmap();

            // Should only include non-zero values
            assert!(non_zero_map.contains_key("technical_corners"));
            assert!(non_zero_map.contains_key("physical_pace"));
            assert!(!non_zero_map.contains_key("mental_determination"));
            assert!(!non_zero_map.contains_key("goalkeeping_reflexes"));

            assert_eq!(non_zero_map.get("technical_corners"), Some(&15));
            assert_eq!(non_zero_map.get("physical_pace"), Some(&17));
        }

        #[test]
        fn test_attribute_count() {
            assert_eq!(Attribute::count(), 50);
        }

        #[test]
        fn test_goalkeeping_first_touch_separation() {
            let mut attrs = PlayerAttributes::new();

            // Set both FirstTouch and GoalkeepingFirstTouch
            attrs.set(Attribute::FirstTouch, 15);
            attrs.set(Attribute::GoalkeepingFirstTouch, 18);

            // They should be separate values
            assert_eq!(attrs.get(Attribute::FirstTouch), 15);
            assert_eq!(attrs.get(Attribute::GoalkeepingFirstTouch), 18);

            // Verify they map to different names
            assert_eq!(attrs.get_by_name("technical_first_touch"), Some(15));
            assert_eq!(attrs.get_by_name("goalkeeping_first_touch"), Some(18));

            let map = attrs.to_hashmap();
            assert_eq!(map.get("technical_first_touch"), Some(&15));
            assert_eq!(map.get("goalkeeping_first_touch"), Some(&18));
        }

        #[test]
        fn test_goalkeeping_work_rate_separation() {
            let mut attrs = PlayerAttributes::new();

            // Set both WorkRate and GoalkeepingWorkRate
            attrs.set(Attribute::WorkRate, 16);
            attrs.set(Attribute::GoalkeepingWorkRate, 19);

            // They should be separate values
            assert_eq!(attrs.get(Attribute::WorkRate), 16);
            assert_eq!(attrs.get(Attribute::GoalkeepingWorkRate), 19);

            // Verify they map to different names
            assert_eq!(attrs.get_by_name("mental_work_rate"), Some(16));
            assert_eq!(attrs.get_by_name("goalkeeping_work_rate"), Some(19));

            let map = attrs.to_hashmap();
            assert_eq!(map.get("mental_work_rate"), Some(&16));
            assert_eq!(map.get("goalkeeping_work_rate"), Some(&19));
        }

        #[test]
        fn test_passing_separation() {
            let mut attrs = PlayerAttributes::new();

            // Set both Passing and GoalkeepingPassing
            attrs.set(Attribute::Passing, 14);
            attrs.set(Attribute::GoalkeepingPassing, 17);

            // They should be separate values
            assert_eq!(attrs.get(Attribute::Passing), 14);
            assert_eq!(attrs.get(Attribute::GoalkeepingPassing), 17);

            // Verify they map to different names
            assert_eq!(attrs.get_by_name("technical_passing"), Some(14));
            assert_eq!(attrs.get_by_name("goalkeeping_passing"), Some(17));

            let map = attrs.to_hashmap();
            assert_eq!(map.get("technical_passing"), Some(&14));
            assert_eq!(map.get("goalkeeping_passing"), Some(&17));
        }
    }
}
