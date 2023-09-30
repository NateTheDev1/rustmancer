use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Proficiency {
    /// The name of the proficiency.
    pub name: String,
    /// The type of proficiency.
    pub prof_type: ProficiencyType,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ProficiencyType {
    Armor,
    Weapon,
    Tool,
    Skill,
    Language,
    Other(String),
}
