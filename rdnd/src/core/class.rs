use serde::{Deserialize, Serialize};

use super::{ability::AbilityType, class_feature::ClassFeature, dice::Dice};

#[derive(Serialize, Deserialize, Clone)]
pub struct Class {
    pub id: String,
    pub name: String,
    pub description: String,
    pub hit_die: Dice,
    pub primary_ability: AbilityType,
    pub saving_throws: Vec<AbilityType>,
    pub class_features: Vec<ClassFeature>,
}
