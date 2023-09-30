use serde::{Deserialize, Serialize};

use super::ability_score::AbilityScoreType;

#[derive(Serialize, Deserialize, Clone)]
pub struct Race {
    /// Name of the referenced race
    pub name: String,
    /// Base move speed for this race (in feet per round).
    pub speed: i32,
    /// List of traits given by this race
    pub included_traits: Vec<RacialTrait>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RacialTrait {
    /// Name of the referenced trait
    pub name: String,
    /// Description of the referenced trait
    pub description: String,
    /// The type of action to take from this trait.
    pub trait_type: RacialTraitType,
    /// Whether this trait requires a choice to be made.
    pub requires_choice: bool,
    /// The choice to make from this trait if it requires a choice.
    pub choice: RacialTraitChoice,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum RacialTraitType {
    AbilityScoreIncrease(AbilityScoreType),
    AllAbilityScoresIncrease(i32),
    Age,
    Alignment,
    Size,
    Speed,
    Language,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RacialTraitChoice {
    /// The formal title of this choice.
    pub formal_title: String,
    /// A formal description of this choice.
    pub formal_description: String,
    /// The number of choices to make from this list.
    pub count: i32,
    /// The list of choices to make.
    pub choices: Vec<RacialTraitChoiceOption>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RacialTraitChoiceOption {
    /// The name of the option.
    pub name: String,
    /// The description of the option.
    pub description: String,
    /// The number of options to choose from this list.
    pub count: i32,
    /// The list of options to choose from.
    pub options: Vec<String>,
}
