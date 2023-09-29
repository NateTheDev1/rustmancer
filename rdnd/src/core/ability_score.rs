use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct AbilityScore {
    pub identifier: AbilityScoreType,
    pub value: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum AbilityScoreType {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}
