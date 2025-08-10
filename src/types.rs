/// Shared types used across different modules

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
