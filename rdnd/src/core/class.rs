use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Class {
    pub identifier: ClassType,
    pub level: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ClassType {
    Barbarian,
    Bard,
    Cleric,
    Druid,
    Fighter,
    Monk,
    Paladin,
    Ranger,
    Rogue,
    Sorcerer,
    Warlock,
    Wizard,
    Other(String),
}
