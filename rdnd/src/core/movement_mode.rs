use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct MovementMode {
    /// The distance the character can move in this mode.
    distance: i32,
    /// The type of movement.
    mode: MovementModeType,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum MovementModeType {
    Walk,
    Fly,
    Swim,
    Burrow,
    Climb,
    Other(String),
}
