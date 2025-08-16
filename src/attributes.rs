use crate::types::PlayerType;
use anyhow;
use std::collections::HashMap;
use std::sync::LazyLock;

/// Technical attributes for field players (14 attributes)
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TechnicalAttribute {
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
}

/// Goalkeeping attributes (14 attributes, replaces technical for goalkeepers)
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GoalkeepingAttribute {
    AerialReach = 0,
    CommandOfArea = 1,
    Communication = 2,
    Eccentricity = 3,
    FirstTouch = 4,
    Handling = 5,
    Kicking = 6,
    OneOnOnes = 7,
    Passing = 8,
    PunchingTendency = 9,
    Reflexes = 10,
    RushingOutTendency = 11,
    Throwing = 12,
    WorkRate = 13,
}

/// Mental attributes (14 attributes, common to all players)
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MentalAttribute {
    Aggression = 0,
    Anticipation = 1,
    Bravery = 2,
    Composure = 3,
    Concentration = 4,
    Decisions = 5,
    Determination = 6,
    Flair = 7,
    Leadership = 8,
    OffTheBall = 9,
    Positioning = 10,
    Teamwork = 11,
    Vision = 12,
    WorkRate = 13,
}

/// Physical attributes (8 attributes, common to all players)
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PhysicalAttribute {
    Acceleration = 0,
    Agility = 1,
    Balance = 2,
    JumpingReach = 3,
    NaturalFitness = 4,
    Pace = 5,
    Stamina = 6,
    Strength = 7,
}

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

    /// Check if this attribute is valid for field players
    pub const fn is_technical(&self) -> bool {
        matches!(*self as usize, 0..=13)
    }

    /// Check if this attribute is mental (valid for all players)
    pub const fn is_mental(&self) -> bool {
        matches!(*self as usize, 14..=27)
    }

    /// Check if this attribute is physical (valid for all players)
    pub const fn is_physical(&self) -> bool {
        matches!(*self as usize, 28..=35)
    }

    /// Check if this attribute is goalkeeping (valid for goalkeepers only)
    pub const fn is_goalkeeping(&self) -> bool {
        matches!(*self as usize, 36..=49)
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

/// Structured attribute storage for optimal performance
#[derive(Debug, Clone)]
pub struct AttributeSet {
    // First section attributes (14 values)
    first_section: [u8; 14],
    // Mental attributes (14 values)
    mental: [u8; 14],
    // Physical attributes (8 values)
    physical: [u8; 8],
    // Player type to determine first section interpretation
    player_type: PlayerType,
    // Extra attributes that don't fit the structured format (for backward compatibility)
    extra_attributes: HashMap<String, u8>,
}

impl Default for AttributeSet {
    fn default() -> Self {
        Self::new(PlayerType::FieldPlayer) // Default to field player
    }
}

impl AttributeSet {
    /// Create new attribute set for given player type
    pub fn new(player_type: PlayerType) -> Self {
        Self {
            first_section: [0; 14],
            mental: [0; 14],
            physical: [0; 8],
            player_type,
            extra_attributes: HashMap::new(),
        }
    }

    /// Get technical attribute value (field players only)
    pub fn get_technical(&self, attr: TechnicalAttribute) -> u8 {
        debug_assert_eq!(self.player_type, PlayerType::FieldPlayer);
        self.first_section[attr as usize]
    }

    /// Set technical attribute value (field players only)
    pub fn set_technical(&mut self, attr: TechnicalAttribute, value: u8) {
        debug_assert_eq!(self.player_type, PlayerType::FieldPlayer);
        self.first_section[attr as usize] = value;
    }

    /// Get goalkeeping attribute value (goalkeepers only)
    pub fn get_goalkeeping(&self, attr: GoalkeepingAttribute) -> u8 {
        debug_assert_eq!(self.player_type, PlayerType::Goalkeeper);
        self.first_section[attr as usize]
    }

    /// Set goalkeeping attribute value (goalkeepers only)
    pub fn set_goalkeeping(&mut self, attr: GoalkeepingAttribute, value: u8) {
        debug_assert_eq!(self.player_type, PlayerType::Goalkeeper);
        self.first_section[attr as usize] = value;
    }

    /// Get mental attribute value
    pub fn get_mental(&self, attr: MentalAttribute) -> u8 {
        self.mental[attr as usize]
    }

    /// Set mental attribute value
    pub fn set_mental(&mut self, attr: MentalAttribute, value: u8) {
        self.mental[attr as usize] = value;
    }

    /// Get physical attribute value
    pub fn get_physical(&self, attr: PhysicalAttribute) -> u8 {
        self.physical[attr as usize]
    }

    /// Set physical attribute value
    pub fn set_physical(&mut self, attr: PhysicalAttribute, value: u8) {
        self.physical[attr as usize] = value;
    }

    /// Get player type
    pub fn player_type(&self) -> &PlayerType {
        &self.player_type
    }

    /// Convert from HashMap-based attributes (for backward compatibility)
    pub fn from_hashmap(attributes: &HashMap<String, u8>, player_type: &PlayerType) -> Self {
        let mut attr_set = Self::new(*player_type);

        // Store any attributes that don't fit the structured format in extra_attributes
        for (key, &value) in attributes {
            attr_set.extra_attributes.insert(key.to_owned(), value);
        }

        match *player_type {
            PlayerType::FieldPlayer => {
                // Technical attributes
                attr_set.set_technical(
                    TechnicalAttribute::Corners,
                    attributes.get("technical_corners").copied().unwrap_or(0),
                );
                attr_set.set_technical(
                    TechnicalAttribute::Crossing,
                    attributes.get("technical_crossing").copied().unwrap_or(0),
                );
                attr_set.set_technical(
                    TechnicalAttribute::Dribbling,
                    attributes.get("technical_dribbling").copied().unwrap_or(0),
                );
                attr_set.set_technical(
                    TechnicalAttribute::Finishing,
                    attributes.get("technical_finishing").copied().unwrap_or(0),
                );
                attr_set.set_technical(
                    TechnicalAttribute::FirstTouch,
                    attributes
                        .get("technical_first_touch")
                        .copied()
                        .unwrap_or(0),
                );
                attr_set.set_technical(
                    TechnicalAttribute::FreeKickTaking,
                    attributes
                        .get("technical_free_kick_taking")
                        .copied()
                        .unwrap_or(0),
                );
                attr_set.set_technical(
                    TechnicalAttribute::Heading,
                    attributes.get("technical_heading").copied().unwrap_or(0),
                );
                attr_set.set_technical(
                    TechnicalAttribute::LongShots,
                    attributes.get("technical_long_shots").copied().unwrap_or(0),
                );
                attr_set.set_technical(
                    TechnicalAttribute::LongThrows,
                    attributes
                        .get("technical_long_throws")
                        .copied()
                        .unwrap_or(0),
                );
                attr_set.set_technical(
                    TechnicalAttribute::Marking,
                    attributes.get("technical_marking").copied().unwrap_or(0),
                );
                attr_set.set_technical(
                    TechnicalAttribute::Passing,
                    attributes.get("technical_passing").copied().unwrap_or(0),
                );
                attr_set.set_technical(
                    TechnicalAttribute::PenaltyTaking,
                    attributes
                        .get("technical_penalty_taking")
                        .copied()
                        .unwrap_or(0),
                );
                attr_set.set_technical(
                    TechnicalAttribute::Tackling,
                    attributes.get("technical_tackling").copied().unwrap_or(0),
                );
                attr_set.set_technical(
                    TechnicalAttribute::Technique,
                    attributes.get("technical_technique").copied().unwrap_or(0),
                );
            }
            PlayerType::Goalkeeper => {
                // Goalkeeping attributes
                attr_set.set_goalkeeping(
                    GoalkeepingAttribute::AerialReach,
                    attributes
                        .get("goalkeeping_aerial_reach")
                        .copied()
                        .unwrap_or(0),
                );
                attr_set.set_goalkeeping(
                    GoalkeepingAttribute::CommandOfArea,
                    attributes
                        .get("goalkeeping_command_of_area")
                        .copied()
                        .unwrap_or(0),
                );
                attr_set.set_goalkeeping(
                    GoalkeepingAttribute::Communication,
                    attributes
                        .get("goalkeeping_communication")
                        .copied()
                        .unwrap_or(0),
                );
                attr_set.set_goalkeeping(
                    GoalkeepingAttribute::Eccentricity,
                    attributes
                        .get("goalkeeping_eccentricity")
                        .copied()
                        .unwrap_or(0),
                );
                attr_set.set_goalkeeping(
                    GoalkeepingAttribute::FirstTouch,
                    attributes
                        .get("goalkeeping_first_touch")
                        .copied()
                        .unwrap_or(0),
                );
                attr_set.set_goalkeeping(
                    GoalkeepingAttribute::Handling,
                    attributes.get("goalkeeping_handling").copied().unwrap_or(0),
                );
                attr_set.set_goalkeeping(
                    GoalkeepingAttribute::Kicking,
                    attributes.get("goalkeeping_kicking").copied().unwrap_or(0),
                );
                attr_set.set_goalkeeping(
                    GoalkeepingAttribute::OneOnOnes,
                    attributes
                        .get("goalkeeping_one_on_ones")
                        .copied()
                        .unwrap_or(0),
                );
                attr_set.set_goalkeeping(
                    GoalkeepingAttribute::Passing,
                    attributes.get("goalkeeping_passing").copied().unwrap_or(0),
                );
                attr_set.set_goalkeeping(
                    GoalkeepingAttribute::PunchingTendency,
                    attributes
                        .get("goalkeeping_punching_tendency")
                        .copied()
                        .unwrap_or(0),
                );
                attr_set.set_goalkeeping(
                    GoalkeepingAttribute::Reflexes,
                    attributes.get("goalkeeping_reflexes").copied().unwrap_or(0),
                );
                attr_set.set_goalkeeping(
                    GoalkeepingAttribute::RushingOutTendency,
                    attributes
                        .get("goalkeeping_rushing_out_tendency")
                        .copied()
                        .unwrap_or(0),
                );
                attr_set.set_goalkeeping(
                    GoalkeepingAttribute::Throwing,
                    attributes.get("goalkeeping_throwing").copied().unwrap_or(0),
                );
                attr_set.set_goalkeeping(
                    GoalkeepingAttribute::WorkRate,
                    attributes
                        .get("goalkeeping_work_rate")
                        .copied()
                        .unwrap_or(0),
                );
            }
        }

        // Mental attributes (common to all players)
        attr_set.set_mental(
            MentalAttribute::Aggression,
            attributes.get("mental_aggression").copied().unwrap_or(0),
        );
        attr_set.set_mental(
            MentalAttribute::Anticipation,
            attributes.get("mental_anticipation").copied().unwrap_or(0),
        );
        attr_set.set_mental(
            MentalAttribute::Bravery,
            attributes.get("mental_bravery").copied().unwrap_or(0),
        );
        attr_set.set_mental(
            MentalAttribute::Composure,
            attributes.get("mental_composure").copied().unwrap_or(0),
        );
        attr_set.set_mental(
            MentalAttribute::Concentration,
            attributes.get("mental_concentration").copied().unwrap_or(0),
        );
        attr_set.set_mental(
            MentalAttribute::Decisions,
            attributes.get("mental_decisions").copied().unwrap_or(0),
        );
        attr_set.set_mental(
            MentalAttribute::Determination,
            attributes.get("mental_determination").copied().unwrap_or(0),
        );
        attr_set.set_mental(
            MentalAttribute::Flair,
            attributes.get("mental_flair").copied().unwrap_or(0),
        );
        attr_set.set_mental(
            MentalAttribute::Leadership,
            attributes.get("mental_leadership").copied().unwrap_or(0),
        );
        attr_set.set_mental(
            MentalAttribute::OffTheBall,
            attributes.get("mental_off_the_ball").copied().unwrap_or(0),
        );
        attr_set.set_mental(
            MentalAttribute::Positioning,
            attributes.get("mental_positioning").copied().unwrap_or(0),
        );
        attr_set.set_mental(
            MentalAttribute::Teamwork,
            attributes.get("mental_teamwork").copied().unwrap_or(0),
        );
        attr_set.set_mental(
            MentalAttribute::Vision,
            attributes.get("mental_vision").copied().unwrap_or(0),
        );
        attr_set.set_mental(
            MentalAttribute::WorkRate,
            attributes.get("mental_work_rate").copied().unwrap_or(0),
        );

        // Physical attributes (common to all players)
        attr_set.set_physical(
            PhysicalAttribute::Acceleration,
            attributes
                .get("physical_acceleration")
                .copied()
                .unwrap_or(0),
        );
        attr_set.set_physical(
            PhysicalAttribute::Agility,
            attributes.get("physical_agility").copied().unwrap_or(0),
        );
        attr_set.set_physical(
            PhysicalAttribute::Balance,
            attributes.get("physical_balance").copied().unwrap_or(0),
        );
        attr_set.set_physical(
            PhysicalAttribute::JumpingReach,
            attributes
                .get("physical_jumping_reach")
                .copied()
                .unwrap_or(0),
        );
        attr_set.set_physical(
            PhysicalAttribute::NaturalFitness,
            attributes
                .get("physical_natural_fitness")
                .copied()
                .unwrap_or(0),
        );
        attr_set.set_physical(
            PhysicalAttribute::Pace,
            attributes.get("physical_pace").copied().unwrap_or(0),
        );
        attr_set.set_physical(
            PhysicalAttribute::Stamina,
            attributes.get("physical_stamina").copied().unwrap_or(0),
        );
        attr_set.set_physical(
            PhysicalAttribute::Strength,
            attributes.get("physical_strength").copied().unwrap_or(0),
        );

        attr_set
    }

    /// Convert to HashMap with all attributes (for backward compatibility)
    pub fn to_hashmap(&self) -> HashMap<String, u8> {
        let mut map = HashMap::new();

        match self.player_type {
            PlayerType::FieldPlayer => {
                // Technical attributes
                map.insert(
                    "technical_corners".to_string(),
                    self.get_technical(TechnicalAttribute::Corners),
                );
                map.insert(
                    "technical_crossing".to_string(),
                    self.get_technical(TechnicalAttribute::Crossing),
                );
                map.insert(
                    "technical_dribbling".to_string(),
                    self.get_technical(TechnicalAttribute::Dribbling),
                );
                map.insert(
                    "technical_finishing".to_string(),
                    self.get_technical(TechnicalAttribute::Finishing),
                );
                map.insert(
                    "technical_first_touch".to_string(),
                    self.get_technical(TechnicalAttribute::FirstTouch),
                );
                map.insert(
                    "technical_free_kick_taking".to_string(),
                    self.get_technical(TechnicalAttribute::FreeKickTaking),
                );
                map.insert(
                    "technical_heading".to_string(),
                    self.get_technical(TechnicalAttribute::Heading),
                );
                map.insert(
                    "technical_long_shots".to_string(),
                    self.get_technical(TechnicalAttribute::LongShots),
                );
                map.insert(
                    "technical_long_throws".to_string(),
                    self.get_technical(TechnicalAttribute::LongThrows),
                );
                map.insert(
                    "technical_marking".to_string(),
                    self.get_technical(TechnicalAttribute::Marking),
                );
                map.insert(
                    "technical_passing".to_string(),
                    self.get_technical(TechnicalAttribute::Passing),
                );
                map.insert(
                    "technical_penalty_taking".to_string(),
                    self.get_technical(TechnicalAttribute::PenaltyTaking),
                );
                map.insert(
                    "technical_tackling".to_string(),
                    self.get_technical(TechnicalAttribute::Tackling),
                );
                map.insert(
                    "technical_technique".to_string(),
                    self.get_technical(TechnicalAttribute::Technique),
                );
            }
            PlayerType::Goalkeeper => {
                // Goalkeeping attributes
                map.insert(
                    "goalkeeping_aerial_reach".to_string(),
                    self.get_goalkeeping(GoalkeepingAttribute::AerialReach),
                );
                map.insert(
                    "goalkeeping_command_of_area".to_string(),
                    self.get_goalkeeping(GoalkeepingAttribute::CommandOfArea),
                );
                map.insert(
                    "goalkeeping_communication".to_string(),
                    self.get_goalkeeping(GoalkeepingAttribute::Communication),
                );
                map.insert(
                    "goalkeeping_eccentricity".to_string(),
                    self.get_goalkeeping(GoalkeepingAttribute::Eccentricity),
                );
                map.insert(
                    "goalkeeping_first_touch".to_string(),
                    self.get_goalkeeping(GoalkeepingAttribute::FirstTouch),
                );
                map.insert(
                    "goalkeeping_handling".to_string(),
                    self.get_goalkeeping(GoalkeepingAttribute::Handling),
                );
                map.insert(
                    "goalkeeping_kicking".to_string(),
                    self.get_goalkeeping(GoalkeepingAttribute::Kicking),
                );
                map.insert(
                    "goalkeeping_one_on_ones".to_string(),
                    self.get_goalkeeping(GoalkeepingAttribute::OneOnOnes),
                );
                map.insert(
                    "goalkeeping_passing".to_string(),
                    self.get_goalkeeping(GoalkeepingAttribute::Passing),
                );
                map.insert(
                    "goalkeeping_punching_tendency".to_string(),
                    self.get_goalkeeping(GoalkeepingAttribute::PunchingTendency),
                );
                map.insert(
                    "goalkeeping_reflexes".to_string(),
                    self.get_goalkeeping(GoalkeepingAttribute::Reflexes),
                );
                map.insert(
                    "goalkeeping_rushing_out_tendency".to_string(),
                    self.get_goalkeeping(GoalkeepingAttribute::RushingOutTendency),
                );
                map.insert(
                    "goalkeeping_throwing".to_string(),
                    self.get_goalkeeping(GoalkeepingAttribute::Throwing),
                );
                map.insert(
                    "goalkeeping_work_rate".to_string(),
                    self.get_goalkeeping(GoalkeepingAttribute::WorkRate),
                );
            }
        }

        // Mental attributes (common to all players)
        map.insert(
            "mental_aggression".to_string(),
            self.get_mental(MentalAttribute::Aggression),
        );
        map.insert(
            "mental_anticipation".to_string(),
            self.get_mental(MentalAttribute::Anticipation),
        );
        map.insert(
            "mental_bravery".to_string(),
            self.get_mental(MentalAttribute::Bravery),
        );
        map.insert(
            "mental_composure".to_string(),
            self.get_mental(MentalAttribute::Composure),
        );
        map.insert(
            "mental_concentration".to_string(),
            self.get_mental(MentalAttribute::Concentration),
        );
        map.insert(
            "mental_decisions".to_string(),
            self.get_mental(MentalAttribute::Decisions),
        );
        map.insert(
            "mental_determination".to_string(),
            self.get_mental(MentalAttribute::Determination),
        );
        map.insert(
            "mental_flair".to_string(),
            self.get_mental(MentalAttribute::Flair),
        );
        map.insert(
            "mental_leadership".to_string(),
            self.get_mental(MentalAttribute::Leadership),
        );
        map.insert(
            "mental_off_the_ball".to_string(),
            self.get_mental(MentalAttribute::OffTheBall),
        );
        map.insert(
            "mental_positioning".to_string(),
            self.get_mental(MentalAttribute::Positioning),
        );
        map.insert(
            "mental_teamwork".to_string(),
            self.get_mental(MentalAttribute::Teamwork),
        );
        map.insert(
            "mental_vision".to_string(),
            self.get_mental(MentalAttribute::Vision),
        );
        map.insert(
            "mental_work_rate".to_string(),
            self.get_mental(MentalAttribute::WorkRate),
        );

        // Physical attributes (common to all players)
        map.insert(
            "physical_acceleration".to_string(),
            self.get_physical(PhysicalAttribute::Acceleration),
        );
        map.insert(
            "physical_agility".to_string(),
            self.get_physical(PhysicalAttribute::Agility),
        );
        map.insert(
            "physical_balance".to_string(),
            self.get_physical(PhysicalAttribute::Balance),
        );
        map.insert(
            "physical_jumping_reach".to_string(),
            self.get_physical(PhysicalAttribute::JumpingReach),
        );
        map.insert(
            "physical_natural_fitness".to_string(),
            self.get_physical(PhysicalAttribute::NaturalFitness),
        );
        map.insert(
            "physical_pace".to_string(),
            self.get_physical(PhysicalAttribute::Pace),
        );
        map.insert(
            "physical_stamina".to_string(),
            self.get_physical(PhysicalAttribute::Stamina),
        );
        map.insert(
            "physical_strength".to_string(),
            self.get_physical(PhysicalAttribute::Strength),
        );

        // Add extra attributes
        map.extend(self.extra_attributes.iter().map(|(k, &v)| (k.clone(), v)));

        map
    }

    /// Convert to HashMap with only non-zero attributes
    pub fn to_non_zero_hashmap(&self) -> HashMap<String, u8> {
        self.to_hashmap()
            .into_iter()
            .filter(|(_, v)| *v > 0)
            .collect()
    }

    /// Set attribute by name without HashMap conversion (optimized for performance)
    pub fn set_by_name(&mut self, name: &str, value: u8) -> anyhow::Result<()> {
        // Handle different attribute sections based on the name prefix and player type
        if let Some(attr_name) = name.strip_prefix("technical_") {
            if self.player_type == PlayerType::FieldPlayer {
                self.set_technical_by_name(attr_name, value)?
            } else {
                // For goalkeepers, some technical attributes map to goalkeeping attributes
                if let Ok(()) = self.set_goalkeeper_technical_mapping(attr_name, value) {
                    // Successfully mapped to goalkeeping attribute
                } else {
                    // Fall back to extra_attributes for backward compatibility
                    self.extra_attributes.insert(name.to_string(), value);
                }
            }
        } else if let Some(attr_name) = name.strip_prefix("goalkeeping_") {
            if self.player_type != PlayerType::Goalkeeper {
                return Err(anyhow::anyhow!(
                    "Goalkeeping attributes not available for field players"
                ));
            }
            self.set_goalkeeping_by_name(attr_name, value)?
        } else if let Some(attr_name) = name.strip_prefix("mental_") {
            self.set_mental_by_name(attr_name, value)?
        } else if let Some(attr_name) = name.strip_prefix("physical_") {
            self.set_physical_by_name(attr_name, value)?
        } else {
            // Fall back to extra_attributes for unknown attributes
            self.extra_attributes.insert(name.to_string(), value);
        }
        Ok(())
    }

    /// Get attribute by name without HashMap conversion (optimized for performance)
    pub fn get_by_name(&self, name: &str) -> Option<u8> {
        // Handle different attribute sections based on the name prefix and player type
        if let Some(attr_name) = name.strip_prefix("technical_") {
            if self.player_type == PlayerType::FieldPlayer {
                self.get_technical_by_name(attr_name)
            } else {
                // For goalkeepers, check if technical attribute maps to goalkeeping attribute
                if let Some(value) = self.get_goalkeeper_technical_mapping(attr_name) {
                    Some(value)
                } else {
                    // Check extra_attributes for backward compatibility
                    self.extra_attributes.get(name).copied()
                }
            }
        } else if let Some(attr_name) = name.strip_prefix("goalkeeping_") {
            if self.player_type == PlayerType::Goalkeeper {
                self.get_goalkeeping_by_name(attr_name)
            } else {
                None
            }
        } else if let Some(attr_name) = name.strip_prefix("mental_") {
            self.get_mental_by_name(attr_name)
        } else if let Some(attr_name) = name.strip_prefix("physical_") {
            self.get_physical_by_name(attr_name)
        } else {
            // Check extra_attributes for unknown attributes
            self.extra_attributes.get(name).copied()
        }
    }

    // Helper methods for direct attribute setting by name
    fn set_technical_by_name(&mut self, attr_name: &str, value: u8) -> anyhow::Result<()> {
        let attr = match attr_name {
            "corners" => TechnicalAttribute::Corners,
            "crossing" => TechnicalAttribute::Crossing,
            "dribbling" => TechnicalAttribute::Dribbling,
            "finishing" => TechnicalAttribute::Finishing,
            "first_touch" => TechnicalAttribute::FirstTouch,
            "free_kick_taking" => TechnicalAttribute::FreeKickTaking,
            "heading" => TechnicalAttribute::Heading,
            "long_shots" => TechnicalAttribute::LongShots,
            "long_throws" => TechnicalAttribute::LongThrows,
            "marking" => TechnicalAttribute::Marking,
            "passing" => TechnicalAttribute::Passing,
            "penalty_taking" => TechnicalAttribute::PenaltyTaking,
            "tackling" => TechnicalAttribute::Tackling,
            "technique" => TechnicalAttribute::Technique,
            _ => {
                return Err(anyhow::anyhow!(
                    "Unknown technical attribute: {}",
                    attr_name
                ))
            }
        };
        self.set_technical(attr, value);
        Ok(())
    }

    fn set_goalkeeping_by_name(&mut self, attr_name: &str, value: u8) -> anyhow::Result<()> {
        let attr = match attr_name {
            "aerial_reach" => GoalkeepingAttribute::AerialReach,
            "command_of_area" => GoalkeepingAttribute::CommandOfArea,
            "communication" => GoalkeepingAttribute::Communication,
            "eccentricity" => GoalkeepingAttribute::Eccentricity,
            "first_touch" => GoalkeepingAttribute::FirstTouch,
            "handling" => GoalkeepingAttribute::Handling,
            "kicking" => GoalkeepingAttribute::Kicking,
            "one_on_ones" => GoalkeepingAttribute::OneOnOnes,
            "passing" => GoalkeepingAttribute::Passing,
            "punching_tendency" => GoalkeepingAttribute::PunchingTendency,
            "reflexes" => GoalkeepingAttribute::Reflexes,
            "rushing_out_tendency" => GoalkeepingAttribute::RushingOutTendency,
            "throwing" => GoalkeepingAttribute::Throwing,
            "work_rate" => GoalkeepingAttribute::WorkRate,
            _ => {
                return Err(anyhow::anyhow!(
                    "Unknown goalkeeping attribute: {}",
                    attr_name
                ))
            }
        };
        self.set_goalkeeping(attr, value);
        Ok(())
    }

    fn set_mental_by_name(&mut self, attr_name: &str, value: u8) -> anyhow::Result<()> {
        let attr = match attr_name {
            "aggression" => MentalAttribute::Aggression,
            "anticipation" => MentalAttribute::Anticipation,
            "bravery" => MentalAttribute::Bravery,
            "composure" => MentalAttribute::Composure,
            "concentration" => MentalAttribute::Concentration,
            "decisions" => MentalAttribute::Decisions,
            "determination" => MentalAttribute::Determination,
            "flair" => MentalAttribute::Flair,
            "leadership" => MentalAttribute::Leadership,
            "off_the_ball" => MentalAttribute::OffTheBall,
            "positioning" => MentalAttribute::Positioning,
            "teamwork" => MentalAttribute::Teamwork,
            "vision" => MentalAttribute::Vision,
            "work_rate" => MentalAttribute::WorkRate,
            _ => return Err(anyhow::anyhow!("Unknown mental attribute: {}", attr_name)),
        };
        self.set_mental(attr, value);
        Ok(())
    }

    fn set_physical_by_name(&mut self, attr_name: &str, value: u8) -> anyhow::Result<()> {
        let attr = match attr_name {
            "acceleration" => PhysicalAttribute::Acceleration,
            "agility" => PhysicalAttribute::Agility,
            "balance" => PhysicalAttribute::Balance,
            "jumping_reach" => PhysicalAttribute::JumpingReach,
            "natural_fitness" => PhysicalAttribute::NaturalFitness,
            "pace" => PhysicalAttribute::Pace,
            "stamina" => PhysicalAttribute::Stamina,
            "strength" => PhysicalAttribute::Strength,
            _ => return Err(anyhow::anyhow!("Unknown physical attribute: {}", attr_name)),
        };
        self.set_physical(attr, value);
        Ok(())
    }

    // Helper methods for direct attribute getting by name
    fn get_technical_by_name(&self, attr_name: &str) -> Option<u8> {
        let attr = match attr_name {
            "corners" => TechnicalAttribute::Corners,
            "crossing" => TechnicalAttribute::Crossing,
            "dribbling" => TechnicalAttribute::Dribbling,
            "finishing" => TechnicalAttribute::Finishing,
            "first_touch" => TechnicalAttribute::FirstTouch,
            "free_kick_taking" => TechnicalAttribute::FreeKickTaking,
            "heading" => TechnicalAttribute::Heading,
            "long_shots" => TechnicalAttribute::LongShots,
            "long_throws" => TechnicalAttribute::LongThrows,
            "marking" => TechnicalAttribute::Marking,
            "passing" => TechnicalAttribute::Passing,
            "penalty_taking" => TechnicalAttribute::PenaltyTaking,
            "tackling" => TechnicalAttribute::Tackling,
            "technique" => TechnicalAttribute::Technique,
            _ => return None,
        };
        Some(self.get_technical(attr))
    }

    fn get_goalkeeping_by_name(&self, attr_name: &str) -> Option<u8> {
        let attr = match attr_name {
            "aerial_reach" => GoalkeepingAttribute::AerialReach,
            "command_of_area" => GoalkeepingAttribute::CommandOfArea,
            "communication" => GoalkeepingAttribute::Communication,
            "eccentricity" => GoalkeepingAttribute::Eccentricity,
            "first_touch" => GoalkeepingAttribute::FirstTouch,
            "handling" => GoalkeepingAttribute::Handling,
            "kicking" => GoalkeepingAttribute::Kicking,
            "one_on_ones" => GoalkeepingAttribute::OneOnOnes,
            "passing" => GoalkeepingAttribute::Passing,
            "punching_tendency" => GoalkeepingAttribute::PunchingTendency,
            "reflexes" => GoalkeepingAttribute::Reflexes,
            "rushing_out_tendency" => GoalkeepingAttribute::RushingOutTendency,
            "throwing" => GoalkeepingAttribute::Throwing,
            "work_rate" => GoalkeepingAttribute::WorkRate,
            _ => return None,
        };
        Some(self.get_goalkeeping(attr))
    }

    fn get_mental_by_name(&self, attr_name: &str) -> Option<u8> {
        let attr = match attr_name {
            "aggression" => MentalAttribute::Aggression,
            "anticipation" => MentalAttribute::Anticipation,
            "bravery" => MentalAttribute::Bravery,
            "composure" => MentalAttribute::Composure,
            "concentration" => MentalAttribute::Concentration,
            "decisions" => MentalAttribute::Decisions,
            "determination" => MentalAttribute::Determination,
            "flair" => MentalAttribute::Flair,
            "leadership" => MentalAttribute::Leadership,
            "off_the_ball" => MentalAttribute::OffTheBall,
            "positioning" => MentalAttribute::Positioning,
            "teamwork" => MentalAttribute::Teamwork,
            "vision" => MentalAttribute::Vision,
            "work_rate" => MentalAttribute::WorkRate,
            _ => return None,
        };
        Some(self.get_mental(attr))
    }

    fn get_physical_by_name(&self, attr_name: &str) -> Option<u8> {
        let attr = match attr_name {
            "acceleration" => PhysicalAttribute::Acceleration,
            "agility" => PhysicalAttribute::Agility,
            "balance" => PhysicalAttribute::Balance,
            "jumping_reach" => PhysicalAttribute::JumpingReach,
            "natural_fitness" => PhysicalAttribute::NaturalFitness,
            "pace" => PhysicalAttribute::Pace,
            "stamina" => PhysicalAttribute::Stamina,
            "strength" => PhysicalAttribute::Strength,
            _ => return None,
        };
        Some(self.get_physical(attr))
    }

    // Helper methods for mapping technical attributes to goalkeeping attributes
    fn set_goalkeeper_technical_mapping(
        &mut self,
        attr_name: &str,
        value: u8,
    ) -> anyhow::Result<()> {
        // Map some technical attributes to goalkeeping attributes for backward compatibility
        match attr_name {
            "first_touch" => {
                self.set_goalkeeping(GoalkeepingAttribute::FirstTouch, value);
                Ok(())
            }
            "passing" => {
                self.set_goalkeeping(GoalkeepingAttribute::Passing, value);
                Ok(())
            }
            _ => Err(anyhow::anyhow!(
                "No mapping for technical attribute {} to goalkeeping",
                attr_name
            )),
        }
    }

    fn get_goalkeeper_technical_mapping(&self, attr_name: &str) -> Option<u8> {
        // Map some technical attributes to goalkeeping attributes for backward compatibility
        match attr_name {
            "first_touch" => Some(self.get_goalkeeping(GoalkeepingAttribute::FirstTouch)),
            "passing" => Some(self.get_goalkeeping(GoalkeepingAttribute::Passing)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_attribute_set() {
        let field_set = AttributeSet::new(PlayerType::FieldPlayer);
        assert_eq!(*field_set.player_type(), PlayerType::FieldPlayer);

        let gk_set = AttributeSet::new(PlayerType::Goalkeeper);
        assert_eq!(*gk_set.player_type(), PlayerType::Goalkeeper);
    }

    #[test]
    fn test_field_player_technical_attributes() {
        let mut attr_set = AttributeSet::new(PlayerType::FieldPlayer);

        // Test setting and getting
        attr_set.set_technical(TechnicalAttribute::Corners, 15);
        assert_eq!(attr_set.get_technical(TechnicalAttribute::Corners), 15);

        attr_set.set_technical(TechnicalAttribute::Technique, 20);
        assert_eq!(attr_set.get_technical(TechnicalAttribute::Technique), 20);
    }

    #[test]
    fn test_goalkeeper_attributes() {
        let mut attr_set = AttributeSet::new(PlayerType::Goalkeeper);

        // Test setting and getting
        attr_set.set_goalkeeping(GoalkeepingAttribute::Reflexes, 18);
        assert_eq!(attr_set.get_goalkeeping(GoalkeepingAttribute::Reflexes), 18);

        attr_set.set_goalkeeping(GoalkeepingAttribute::Handling, 16);
        assert_eq!(attr_set.get_goalkeeping(GoalkeepingAttribute::Handling), 16);
    }

    #[test]
    fn test_mental_attributes() {
        let mut attr_set = AttributeSet::new(PlayerType::FieldPlayer);

        attr_set.set_mental(MentalAttribute::Determination, 19);
        assert_eq!(attr_set.get_mental(MentalAttribute::Determination), 19);

        attr_set.set_mental(MentalAttribute::Vision, 14);
        assert_eq!(attr_set.get_mental(MentalAttribute::Vision), 14);
    }

    #[test]
    fn test_physical_attributes() {
        let mut attr_set = AttributeSet::new(PlayerType::FieldPlayer);

        attr_set.set_physical(PhysicalAttribute::Pace, 17);
        assert_eq!(attr_set.get_physical(PhysicalAttribute::Pace), 17);

        attr_set.set_physical(PhysicalAttribute::Strength, 12);
        assert_eq!(attr_set.get_physical(PhysicalAttribute::Strength), 12);
    }

    #[test]
    fn test_from_hashmap_field_player() {
        let mut attributes = HashMap::new();
        attributes.insert("technical_corners".to_string(), 15);
        attributes.insert("technical_crossing".to_string(), 12);
        attributes.insert("mental_determination".to_string(), 18);
        attributes.insert("physical_pace".to_string(), 16);

        let attr_set = AttributeSet::from_hashmap(&attributes, &PlayerType::FieldPlayer);

        assert_eq!(attr_set.get_technical(TechnicalAttribute::Corners), 15);
        assert_eq!(attr_set.get_technical(TechnicalAttribute::Crossing), 12);
        assert_eq!(attr_set.get_mental(MentalAttribute::Determination), 18);
        assert_eq!(attr_set.get_physical(PhysicalAttribute::Pace), 16);
    }

    #[test]
    fn test_from_hashmap_goalkeeper() {
        let mut attributes = HashMap::new();
        attributes.insert("goalkeeping_reflexes".to_string(), 19);
        attributes.insert("goalkeeping_handling".to_string(), 17);
        attributes.insert("mental_concentration".to_string(), 16);
        attributes.insert("physical_agility".to_string(), 15);

        let attr_set = AttributeSet::from_hashmap(&attributes, &PlayerType::Goalkeeper);

        assert_eq!(attr_set.get_goalkeeping(GoalkeepingAttribute::Reflexes), 19);
        assert_eq!(attr_set.get_goalkeeping(GoalkeepingAttribute::Handling), 17);
        assert_eq!(attr_set.get_mental(MentalAttribute::Concentration), 16);
        assert_eq!(attr_set.get_physical(PhysicalAttribute::Agility), 15);
    }

    #[test]
    fn test_to_hashmap_field_player() {
        let mut attr_set = AttributeSet::new(PlayerType::FieldPlayer);
        attr_set.set_technical(TechnicalAttribute::Finishing, 18);
        attr_set.set_mental(MentalAttribute::Composure, 16);
        attr_set.set_physical(PhysicalAttribute::Acceleration, 14);

        let map = attr_set.to_hashmap();

        assert_eq!(map.get("technical_finishing"), Some(&18));
        assert_eq!(map.get("mental_composure"), Some(&16));
        assert_eq!(map.get("physical_acceleration"), Some(&14));
    }

    #[test]
    fn test_to_hashmap_goalkeeper() {
        let mut attr_set = AttributeSet::new(PlayerType::Goalkeeper);
        attr_set.set_goalkeeping(GoalkeepingAttribute::Kicking, 15);
        attr_set.set_mental(MentalAttribute::Decisions, 17);
        attr_set.set_physical(PhysicalAttribute::JumpingReach, 13);

        let map = attr_set.to_hashmap();

        assert_eq!(map.get("goalkeeping_kicking"), Some(&15));
        assert_eq!(map.get("mental_decisions"), Some(&17));
        assert_eq!(map.get("physical_jumping_reach"), Some(&13));
    }

    #[test]
    fn test_default_values() {
        let attr_set = AttributeSet::new(PlayerType::FieldPlayer);

        // All attributes should start at 0
        assert_eq!(attr_set.get_technical(TechnicalAttribute::Corners), 0);
        assert_eq!(attr_set.get_mental(MentalAttribute::Determination), 0);
        assert_eq!(attr_set.get_physical(PhysicalAttribute::Pace), 0);
    }

    #[test]
    fn test_round_trip_conversion() {
        // Test field player round trip
        let mut original = HashMap::new();
        original.insert("technical_finishing".to_string(), 18);
        original.insert("mental_determination".to_string(), 19);
        original.insert("physical_pace".to_string(), 16);

        let attr_set = AttributeSet::from_hashmap(&original, &PlayerType::FieldPlayer);
        let converted_back = attr_set.to_hashmap();

        assert_eq!(converted_back.get("technical_finishing"), Some(&18));
        assert_eq!(converted_back.get("mental_determination"), Some(&19));
        assert_eq!(converted_back.get("physical_pace"), Some(&16));
    }

    #[test]
    fn test_direct_access_field_player() {
        let mut attr_set = AttributeSet::new(PlayerType::FieldPlayer);

        // Test setting and getting technical attributes
        attr_set.set_by_name("technical_crossing", 15).unwrap();
        assert_eq!(attr_set.get_by_name("technical_crossing"), Some(15));

        // Test setting and getting mental attributes
        attr_set.set_by_name("mental_composure", 18).unwrap();
        assert_eq!(attr_set.get_by_name("mental_composure"), Some(18));

        // Test setting and getting physical attributes
        attr_set.set_by_name("physical_strength", 19).unwrap();
        assert_eq!(attr_set.get_by_name("physical_strength"), Some(19));

        // Test unknown attribute
        assert_eq!(attr_set.get_by_name("unknown_attribute"), None);

        // Test goalkeeping attributes should fail for field players
        assert!(attr_set.set_by_name("goalkeeping_reflexes", 15).is_err());
        assert_eq!(attr_set.get_by_name("goalkeeping_reflexes"), None);
    }

    #[test]
    fn test_direct_access_goalkeeper() {
        let mut attr_set = AttributeSet::new(PlayerType::Goalkeeper);

        // Test setting and getting goalkeeping attributes
        attr_set.set_by_name("goalkeeping_reflexes", 19).unwrap();
        assert_eq!(attr_set.get_by_name("goalkeeping_reflexes"), Some(19));

        // Test setting and getting mental attributes
        attr_set.set_by_name("mental_concentration", 16).unwrap();
        assert_eq!(attr_set.get_by_name("mental_concentration"), Some(16));

        // Test setting and getting physical attributes
        attr_set.set_by_name("physical_agility", 17).unwrap();
        assert_eq!(attr_set.get_by_name("physical_agility"), Some(17));

        // Test unknown attribute
        assert_eq!(attr_set.get_by_name("unknown_attribute"), None);

        // Test technical attributes for goalkeepers are stored as extra attributes for backward compatibility
        attr_set.set_by_name("technical_crossing", 15).unwrap(); // Should not fail
        assert_eq!(attr_set.get_by_name("technical_crossing"), Some(15)); // Should be accessible via extra_attributes

        // Test technical attributes that map to goalkeeping attributes
        attr_set.set_by_name("technical_first_touch", 12).unwrap();
        assert_eq!(attr_set.get_by_name("technical_first_touch"), Some(12)); // Should map to goalkeeping first_touch
    }

    #[test]
    fn test_direct_access_extra_attributes() {
        let mut attr_set = AttributeSet::new(PlayerType::FieldPlayer);

        // Test setting and getting extra attributes (unknown names)
        attr_set.set_by_name("custom_attribute", 12).unwrap();
        assert_eq!(attr_set.get_by_name("custom_attribute"), Some(12));

        // Verify it's stored in extra_attributes and appears in hashmap conversion
        let hashmap = attr_set.to_hashmap();
        assert_eq!(hashmap.get("custom_attribute"), Some(&12));
    }

    #[test]
    fn test_direct_access_performance_equivalence() {
        let mut attr_set = AttributeSet::new(PlayerType::FieldPlayer);

        // Set some attributes using direct access
        attr_set.set_by_name("technical_finishing", 18).unwrap();
        attr_set.set_by_name("mental_determination", 19).unwrap();
        attr_set.set_by_name("physical_pace", 16).unwrap();

        // Verify they match hashmap conversion results
        let hashmap = attr_set.to_hashmap();
        assert_eq!(
            attr_set.get_by_name("technical_finishing"),
            hashmap.get("technical_finishing").copied()
        );
        assert_eq!(
            attr_set.get_by_name("mental_determination"),
            hashmap.get("mental_determination").copied()
        );
        assert_eq!(
            attr_set.get_by_name("physical_pace"),
            hashmap.get("physical_pace").copied()
        );
    }

    #[test]
    fn test_invalid_attribute_names() {
        let mut attr_set = AttributeSet::new(PlayerType::FieldPlayer);

        // Test invalid technical attribute
        assert!(attr_set.set_by_name("technical_invalid", 15).is_err());
        assert_eq!(attr_set.get_by_name("technical_invalid"), None);

        // Test invalid mental attribute
        assert!(attr_set.set_by_name("mental_invalid", 15).is_err());
        assert_eq!(attr_set.get_by_name("mental_invalid"), None);

        // Test invalid physical attribute
        assert!(attr_set.set_by_name("physical_invalid", 15).is_err());
        assert_eq!(attr_set.get_by_name("physical_invalid"), None);
    }

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
        fn test_attribute_enum_properties() {
            // Test technical attributes
            assert!(Attribute::Corners.is_technical());
            assert!(Attribute::Technique.is_technical());
            assert!(!Attribute::Determination.is_technical());
            assert!(!Attribute::Pace.is_technical());
            assert!(!Attribute::Reflexes.is_technical());

            // Test mental attributes
            assert!(Attribute::Determination.is_mental());
            assert!(Attribute::Vision.is_mental());
            assert!(!Attribute::Corners.is_mental());
            assert!(!Attribute::Pace.is_mental());
            assert!(!Attribute::Reflexes.is_mental());

            // Test physical attributes
            assert!(Attribute::Pace.is_physical());
            assert!(Attribute::Strength.is_physical());
            assert!(!Attribute::Corners.is_physical());
            assert!(!Attribute::Determination.is_physical());
            assert!(!Attribute::Reflexes.is_physical());

            // Test goalkeeping attributes
            assert!(Attribute::Reflexes.is_goalkeeping());
            assert!(Attribute::Handling.is_goalkeeping());
            assert!(!Attribute::Corners.is_goalkeeping());
            assert!(!Attribute::Determination.is_goalkeeping());
            assert!(!Attribute::Pace.is_goalkeeping());
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
