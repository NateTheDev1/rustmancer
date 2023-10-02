use serde::{Deserialize, Serialize};

use super::{action::ActionStepDamage, saving_throw::SavingThrow};

#[derive(Serialize, Deserialize, Clone)]
pub struct Spell {
    /// Whether or not the spell can be cast as a ritual.
    pub can_be_cast_as_ritual: bool,
    /// Whether or not the spell can be cast at higher levels.
    pub can_be_cast_at_higher_levels: bool,
    /// The casting time for the spell (e.g. 1 Action, 10 minutes, etc.).
    pub casting_time: String,
    /// An array of the components required to cast the spell. For 5e, the options are V, S, and M for verbal, somatic, and material, respectively
    pub components: Vec<String>,
    /// An array of damage dice rolls.
    pub damage: Vec<ActionStepDamage>,
    /// The description of the spell.
    pub description: String,
    /// The duration of the spell (e.g. Instantaneous, 1 minute, etc.).
    pub duration: String,
    /// The description of how the spell changes when cast at higher levels.
    pub higher_level_description: String,
    /// An array of higher level spell effects.
    pub higher_levels: Vec<SpellHigherLevel>,
    /// The level of the spell.
    pub level: i32,
    /// The material components required to cast the spell.
    pub materials: String,
    /// The name of the spell.
    pub name: String,
    /// The range of the spell.
    pub range: String,
    /// Whether or not the spell requires concentration to maintain effects.
    pub requires_concentration: bool,
    /// Whether or not the spell rolls an attack.
    pub rolls_attack: bool,
    /// The saving throw associated with the spell.
    pub saving_throw: SavingThrow,
    /// The school of magic the spell belongs to.
    pub school: String,
    /// An array of tags associated with the spell. These can be custom or describe common spell metadata such as Damage to indicate that it deals damage or Wizard to indicate that it is a Wizard spell.
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SpellHigherLevel {
    /// An array of levels at which this effect is applied.
    pub apply_at_levels: Vec<i32>,
    /// An array of damage dice rolls.
    pub damage: Vec<ActionStepDamage>,
    /// The type of effect
    pub effect_type: SpellEffectType,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum SpellEffectType {
    PerSlot,
    PerLevel,
}
