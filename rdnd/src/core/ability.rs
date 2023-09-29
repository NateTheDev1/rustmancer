use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum AbilityType {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}
