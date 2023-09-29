use serde::{Deserialize, Serialize};

use super::{dice::Die, saving_throw::SavingThrow};

#[derive(Serialize, Deserialize, Clone)]
pub struct Action {
    pub name: String,
    pub description: String,
    pub sort_order_preference: i32,
    pub steps: Vec<ActionStep>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ActionStep {
    /// The type of action step.
    pub action_type: ActionStepType,
    /// If this is of type `ActionStepType.Attack`, this contains the details of the attack.
    pub attack: Option<ActionStepAttack>,
    /// If this is of type `ActionStepType.DiceRoll`, this contains the details of the dice roll command.
    pub dice_roll: Vec<ActionStepDamage>,
    /// If this is of type `ActionStepType.SkillCheck`, this contains the details of the skill check command.
    pub skill_check: Option<ActionStepSkillCheck>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ActionStepType {
    Attack,
    DiceRoll,
    SkillCheck,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ActionStepAttack {
    /// The ability score to use for the attack roll.
    pub ability: String,
    /// The type of action. This will be related to the struct type.
    pub action_type: ActionType,
    /// The bonus to the attack roll.
    pub bonus: i32,
    /// The number at which the attack is considered a critical hit.
    pub crit: i32,
    /// A list of damage structs that describe the damage to roll.
    pub damage: Vec<ActionStepDamage>,
    /// Whether the character is proficient in the attack and thus whether to apply the character's proficiency bonus to the attack roll.
    pub proficient: bool,
    /// Whether this is a ranged attack.
    pub ranged: bool,
    /// The name of the attack.
    pub name: String,
    /// The range of the attack.
    pub range: i32,
    /// The threshold for long range, or the point at which disadvantage is applied to the attack roll.
    pub range_max: i32,
    /// Whether this attack rolls an attack roll.
    pub rolls_attack: bool,
    /// If this attack is a saving throw, this contains the details of the saving throw.
    pub saving_throw: Option<SavingThrow>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ActionStepSkillCheck {
    /// The modifier to apply to the skill check.
    modifier: ActionStepSkillCheckModifier,
    /// The name of a skill present on the character to roll a skill check for.
    skill: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ActionStepSkillCheckModifier {
    Advantage,
    Disadvantage,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ActionType {
    Action,
    BonusAction,
    Reaction,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ActionStepDamage {
    ///  The ability score to use for the damage roll.
    pub ability: String,
    /// An arbitrary bonus to add to the damage roll.
    pub bonus: i32,
    /// The dice to roll for the damage.
    pub dice: Vec<Die>,
    /// The type of damage to deal.
    pub damage_type: DamageType,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum DamageType {
    Acid,
    Bludgeoning,
    Cold,
    Fire,
    Force,
    Lightning,
    Necrotic,
    Piercing,
    Poison,
    Psychic,
    Radiant,
    Slashing,
    Thunder,
    /// A damage type that is not listed in the PHB.
    Other(String),
}
