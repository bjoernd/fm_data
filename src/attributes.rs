use crate::types::PlayerType;
use std::collections::HashMap;

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
    pub fn from_hashmap(attributes: &HashMap<String, u8>, player_type: PlayerType) -> Self {
        let mut attr_set = Self::new(player_type.clone());

        // Store any attributes that don't fit the structured format in extra_attributes
        for (key, &value) in attributes {
            attr_set.extra_attributes.insert(key.clone(), value);
        }

        match player_type {
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
        map.extend(self.extra_attributes.clone());

        map
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

        let attr_set = AttributeSet::from_hashmap(&attributes, PlayerType::FieldPlayer);

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

        let attr_set = AttributeSet::from_hashmap(&attributes, PlayerType::Goalkeeper);

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

        let attr_set = AttributeSet::from_hashmap(&original, PlayerType::FieldPlayer);
        let converted_back = attr_set.to_hashmap();

        assert_eq!(converted_back.get("technical_finishing"), Some(&18));
        assert_eq!(converted_back.get("mental_determination"), Some(&19));
        assert_eq!(converted_back.get("physical_pace"), Some(&16));
    }
}
