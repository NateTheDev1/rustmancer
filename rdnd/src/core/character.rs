use serde::{Deserialize, Serialize};

use super::{ability_score::AbilityScore, action::Action};

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
}
