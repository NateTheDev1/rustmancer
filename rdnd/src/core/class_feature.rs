use serde::{Deserialize, Serialize};

use super::ability::AbilityType;

#[derive(Serialize, Deserialize, Clone)]
pub struct ClassFeature {
    pub id: String,
    pub name: String,
    pub description: String,
    pub level: u8,
    pub logic: ClassFeatureLogic,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ClassFeatureLogic {
    pub steps: Vec<ClassFeatureLogicalStep>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ClassFeatureLogicalStep {
    action: ClassFeatureAction,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CustomClassFeatureAction {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ClassFeatureAction {
    AddAbilityScore(AbilityType, u8),
    AddSkillProficiency(String),
    AddToolProficiency(String),
    AddLanguage(String),
    AddWeaponProficiency(String),
    AddArmorProficiency(String),
    AddSpell(String),
    AddSpellSlot(u8),
    AddSpellSlotLevel(u8),
    AddSpellcastingAbility(AbilityType),
    AddSpellSaveDC(u8),
    AddSpellAttackBonus(u8),
    AddHitPoints(u8),
    AddHitPointsPerLevel(u8),
    AddBonusAction(String),
    AddReaction(String),
    IncreaseActionCount(u8),
    IncreaseBonusActionCount(u8),
    Custom(CustomClassFeatureAction),
}

impl ClassFeatureLogicalStep {
    pub fn get_ability_type(&self) -> Option<AbilityType> {
        match &self.action {
            ClassFeatureAction::AddAbilityScore(ability_type, _) => Some(ability_type.clone()),
            _ => None,
        }
    }

    pub fn get_ability_score(&self) -> Option<u8> {
        match &self.action {
            ClassFeatureAction::AddAbilityScore(_, ability_score) => Some(*ability_score),
            _ => None,
        }
    }

    pub fn get_skill_proficiency(&self) -> Option<String> {
        match &self.action {
            ClassFeatureAction::AddSkillProficiency(skill) => Some(skill.clone()),
            _ => None,
        }
    }

    pub fn get_tool_proficiency(&self) -> Option<String> {
        match &self.action {
            ClassFeatureAction::AddToolProficiency(tool) => Some(tool.clone()),
            _ => None,
        }
    }

    pub fn get_language(&self) -> Option<String> {
        match &self.action {
            ClassFeatureAction::AddLanguage(language) => Some(language.clone()),
            _ => None,
        }
    }

    pub fn get_weapon_proficiency(&self) -> Option<String> {
        match &self.action {
            ClassFeatureAction::AddWeaponProficiency(weapon) => Some(weapon.clone()),
            _ => None,
        }
    }

    pub fn get_armor_proficiency(&self) -> Option<String> {
        match &self.action {
            ClassFeatureAction::AddArmorProficiency(armor) => Some(armor.clone()),
            _ => None,
        }
    }

    pub fn get_spell(&self) -> Option<String> {
        match &self.action {
            ClassFeatureAction::AddSpell(spell) => Some(spell.clone()),
            _ => None,
        }
    }

    pub fn get_spell_slot(&self) -> Option<u8> {
        match &self.action {
            ClassFeatureAction::AddSpellSlot(spell_slot) => Some(*spell_slot),
            _ => None,
        }
    }

    pub fn get_spell_slot_level(&self) -> Option<u8> {
        match &self.action {
            ClassFeatureAction::AddSpellSlotLevel(spell_slot_level) => Some(*spell_slot_level),
            _ => None,
        }
    }

    pub fn get_spellcasting_ability(&self) -> Option<AbilityType> {
        match &self.action {
            ClassFeatureAction::AddSpellcastingAbility(ability_type) => Some(ability_type.clone()),
            _ => None,
        }
    }

    pub fn get_spell_save_dc(&self) -> Option<u8> {
        match &self.action {
            ClassFeatureAction::AddSpellSaveDC(spell_save_dc) => Some(*spell_save_dc),
            _ => None,
        }
    }

    pub fn get_spell_attack_bonus(&self) -> Option<u8> {
        match &self.action {
            ClassFeatureAction::AddSpellAttackBonus(spell_attack_bonus) => {
                Some(*spell_attack_bonus)
            }
            _ => None,
        }
    }

    pub fn get_hit_points(&self) -> Option<u8> {
        match &self.action {
            ClassFeatureAction::AddHitPoints(hit_points) => Some(*hit_points),
            _ => None,
        }
    }

    pub fn get_hit_points_per_level(&self) -> Option<u8> {
        match &self.action {
            ClassFeatureAction::AddHitPointsPerLevel(hit_points_per_level) => {
                Some(*hit_points_per_level)
            }
            _ => None,
        }
    }

    pub fn get_bonus_action(&self) -> Option<String> {
        match &self.action {
            ClassFeatureAction::AddBonusAction(bonus_action) => Some(bonus_action.clone()),
            _ => None,
        }
    }

    pub fn get_reaction(&self) -> Option<String> {
        match &self.action {
            ClassFeatureAction::AddReaction(reaction) => Some(reaction.clone()),
            _ => None,
        }
    }

    pub fn get_action_count(&self) -> Option<u8> {
        match &self.action {
            ClassFeatureAction::IncreaseActionCount(action_count) => Some(*action_count),
            _ => None,
        }
    }

    pub fn get_bonus_action_count(&self) -> Option<u8> {
        match &self.action {
            ClassFeatureAction::IncreaseBonusActionCount(bonus_action_count) => {
                Some(*bonus_action_count)
            }
            _ => None,
        }
    }

    pub fn get_custom_action(&self) -> Option<CustomClassFeatureAction> {
        match &self.action {
            ClassFeatureAction::Custom(custom_action) => Some(custom_action.clone()),
            _ => None,
        }
    }

    pub fn get_action(&self) -> &ClassFeatureAction {
        &self.action
    }
}
