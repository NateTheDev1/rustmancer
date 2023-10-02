use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Skill {
    /// The name of the ability that this skill is based on. For 5e, use the abbreviated ability name.
    pub ability: String,
    /// A bonus to add to the skill roll.
    pub bonus: i32,
    /// Whether or not the character is proficient in this skill.
    pub proficient: bool,
    /// Whether or not the character has double proficiency (a.k.a. expertise) in this skill.
    pub double_proficiency: bool,
    /// The name of the skill (e.g. Perception, Stealth, etc.).
    pub name: String,
}
