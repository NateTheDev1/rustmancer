use serde::{Deserialize, Serialize};

use super::{
    ability_score::AbilityScore,
    action::{Action, DamageType},
    class::Class,
    health::Health,
    wallet::Wallet,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Character {
    /// The character's ability scores.
    pub ability_scores: Vec<AbilityScore>,
    /// The character's actions.
    pub actions: Vec<Action>,
    /// The character's age.
    pub age: String,
    /// The character's alignment.
    pub alignment: String,
    // The character's armor class not including bonuses.
    pub armor_class: i32,
    // A long description of the character's appearance.
    pub appearance: String,
    /// A non-player character's challenge rating.
    pub challenge_rating: i32,
    // An list of character classes and their associated levels.
    pub classes: Vec<Class>,
    /// A non-player character's condition immunities.
    pub condition_immunities: Vec<String>,
    /// The character's wallet to track currency.
    pub wallet: Wallet,
    /// The character's current health tracker.
    pub health: Health,
    pub damage_immunities: Vec<CharacterDamageAdjustment>,
    pub damage_resistances: Vec<CharacterDamageAdjustment>,
    pub damage_vulnerabilities: Vec<CharacterDamageAdjustment>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CharacterDamageAdjustment {
    /// The condition associated with the damage adjustment (e.g. silvered, magical, etc.)
    pub condition: String,
    ///  The name of the damage type to adjust (e.g. Slashing, Piercing, etc.).
    pub damage_type: DamageType,
}
