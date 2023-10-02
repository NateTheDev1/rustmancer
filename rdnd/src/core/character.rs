use serde::{Deserialize, Serialize};

use super::{
    ability_score::AbilityScore,
    action::{Action, DamageType},
    class::Class,
    dice::HitDie,
    health::Health,
    movement_mode::MovementMode,
    proficiencies::Proficiency,
    race::Race,
    sense::Sense,
    skill::Skill,
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
    /// A non-player character's damage immunities.
    pub damage_immunities: Vec<CharacterDamageAdjustment>,
    /// A non-player character's damage resistances.
    pub damage_resistances: Vec<CharacterDamageAdjustment>,
    /// A non-player character's damage vulnerabilities.
    pub damage_vulnerabilities: Vec<CharacterDamageAdjustment>,
    /// A description of the character.
    pub description: String,
    /// The character's eye color
    pub eye_color: String,
    /// The amount of experience the character has.
    pub exp: i32,
    /// The character's hair color.
    pub hair_color: String,
    /// The character's height.
    pub height: String,
    /// The character's hit dice.
    pub hit_dice: HitDie,
    /// A URI for the character's avatar or profile picture.
    pub avatar_url: String,
    /// The character's total initiative bonus.
    pub initiative_bonus: i32,
    /// Indicates whether this character is a spellcaster or not. This is used to trigger the display of spellcasting features in the UI.
    pub is_spellcaster: bool,
    /// Indicates whether this character is a legendary creature.
    pub legendary: bool,
    /// A list of movement modes and their associated speeds.
    pub movement_modes: Vec<MovementMode>,
    /// The character's maximum hit points.
    pub max_hp: i32,
    /// The character's name.
    pub name: String,
    /// An array of skills, languages, saving throws, etc. that the character is proficient in.
    pub proficiencies: Vec<Proficiency>,
    /// The character's proficiency bonus.
    pub proficiency_bonus: i32,
    /// The character's race
    pub race: Race,
    /// A non-player character's senses.
    pub senses: Vec<Sense>,
    /// The character's skills.
    pub skills: Vec<Skill>,
    /// The color of the character's skin.
    pub skin_color: String,
    /// The player character's movement speed per turn.
    pub speed: i32,
    /// The character's size.
    pub size: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CharacterDamageAdjustment {
    /// The condition associated with the damage adjustment (e.g. silvered, magical, etc.)
    pub condition: String,
    ///  The name of the damage type to adjust (e.g. Slashing, Piercing, etc.).
    pub damage_type: DamageType,
}
