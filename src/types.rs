/// Shared types used across different modules

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlayerType {
    Goalkeeper,
    FieldPlayer,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Footedness {
    LeftFooted,
    RightFooted,
    BothFooted,
}
