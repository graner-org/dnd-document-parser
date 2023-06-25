use std::collections::HashMap;

use crate::utils::traits::{option_to_5etools_creature, To5etools};
use serde_json::{json, Value};

use super::common::{
    merge_json, AbilityScore, Alignment, DamageType, NamedEntry, Skill, Source, StatusCondition,
};

#[cfg(test)]
mod tests;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Size {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
}

impl To5etools for Size {
    fn to_5etools_base(&self) -> Value {
        use Size::{Gargantuan, Huge, Large, Medium, Small, Tiny};
        Value::String(
            match self {
                Tiny => "T",
                Small => "S",
                Medium => "M",
                Large => "L",
                Huge => "H",
                Gargantuan => "G",
            }
            .to_owned(),
        )
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreatureTypeEnum {
    Aberration,
    Beast,
    Celestial,
    Construct,
    Dragon,
    Elemental,
    Fey,
    Fiend,
    Giant,
    Humanoid,
    Monstrosity,
    Ooze,
    Plant,
    Undead,
}

impl To5etools for CreatureTypeEnum {
    fn to_5etools_base(&self) -> Value {
        use CreatureTypeEnum::*;
        Value::String(
            match self {
                Aberration => "aberration",
                Beast => "beast",
                Celestial => "celestial",
                Construct => "construct",
                Dragon => "dragon",
                Elemental => "elemental",
                Fey => "fey",
                Fiend => "fiend",
                Giant => "giant",
                Humanoid => "humanoid",
                Monstrosity => "monstrosity",
                Ooze => "ooze",
                Plant => "plant",
                Undead => "undead",
            }
            .to_owned(),
        )
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HitPointsFormula {
    pub number_of_dice: u8,
    pub die_size: u8,
    pub modifier: u16,
}

impl To5etools for HitPointsFormula {
    fn to_5etools_base(&self) -> Value {
        Value::String(format!(
            "{no_dice}d{die_size} + {modifier}",
            no_dice = self.number_of_dice,
            die_size = self.die_size,
            modifier = self.modifier
        ))
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HitPoints {
    pub average: u16,
    pub formula: HitPointsFormula,
}

impl To5etools for HitPoints {
    fn to_5etools_base(&self) -> Value {
        json!({
            "average": self.average,
            "formula": self.formula.to_5etools_base(),
        })
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlySpeed {
    pub speed: u16,
    pub hover: bool,
}

impl To5etools for FlySpeed {
    fn to_5etools_base(&self) -> Value {
        if self.hover {
            json!({
                "number": self.speed,
                "condition": "(hover)",
            })
        } else {
            json!(self.speed)
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Speed {
    pub walk: u16,
    pub burrow: Option<u16>,
    pub climb: Option<u16>,
    pub crawl: Option<u16>,
    pub fly: Option<FlySpeed>,
    pub swim: Option<u16>,
}

impl To5etools for Speed {
    fn to_5etools_base(&self) -> Value {
        merge_json(vec![
            json!({"walk": self.walk}),
            option_to_5etools_creature(self.burrow.as_ref(), "burrow"),
            option_to_5etools_creature(self.climb.as_ref(), "climb"),
            option_to_5etools_creature(self.crawl.as_ref(), "crawl"),
            option_to_5etools_creature(self.fly.as_ref(), "fly"),
            option_to_5etools_creature(self.swim.as_ref(), "swim"),
        ])
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbilityScores {
    pub strength: u8,
    pub dexterity: u8,
    pub constitution: u8,
    pub intelligence: u8,
    pub wisdom: u8,
    pub charisma: u8,
}

impl To5etools for AbilityScores {
    fn to_5etools_base(&self) -> Value {
        json!({
            "str": self.strength,
            "dex": self.dexterity,
            "con": self.constitution,
            "int": self.constitution,
            "wis": self.wisdom,
            "cha": self.charisma,
        })
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DamageModifierType {
    Immunity,
    Resistance,
    Vulnerability,
}

impl DamageModifierType {
    fn to_string(&self) -> String {
        use DamageModifierType::*;
        match self {
            Immunity => "immune",
            Resistance => "resist",
            Vulnerability => "vulnerable",
        }
        .to_string()
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionalDamageModifier {
    pub modifier_type: DamageModifierType,
    pub damage_types: Vec<DamageType>,
    pub condition: String,
}

impl To5etools for ConditionalDamageModifier {
    fn to_5etools_base(&self) -> Value {
        json!({
            self.modifier_type.to_string(): self.damage_types.to_5etools_base(),
            "note": self.condition,
            "cond": true,
        })
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DamageModifier {
    Conditional(ConditionalDamageModifier),
    Unconditional(DamageType),
}

impl To5etools for DamageModifier {
    fn to_5etools_base(&self) -> Value {
        use DamageModifier::*;
        match self {
            Conditional(conditional) => conditional.to_5etools_base(),
            Unconditional(unconditional) => unconditional.to_5etools_base(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatureType {
    pub main_type: CreatureTypeEnum,
    pub subtypes: Option<Vec<String>>,
}

impl To5etools for CreatureType {
    fn to_5etools_base(&self) -> Value {
        match &self.subtypes {
            None => self.main_type.to_5etools_base(),
            Some(subtypes) => json!({
                "type": self.main_type.to_5etools_base(),
                "tags": subtypes,
            }),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArmorClass {
    pub ac: u8,
    pub armor_type: Option<Vec<String>>,
}

impl To5etools for ArmorClass {
    fn to_5etools_base(&self) -> Value {
        match &self.armor_type {
            None => json!([self.ac]),
            Some(armor) => json!([{
                    "ac": self.ac,
                    "from": armor,
            }]),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChallengeRating {
    WholeNumber(u8),
    Half,
    Quarter,
    Eighth,
}

impl To5etools for ChallengeRating {
    fn to_5etools_base(&self) -> Value {
        use ChallengeRating::*;
        Value::String(match self {
            WholeNumber(number) => number.to_string(),
            Half => "1/2".to_string(),
            Quarter => "1/4".to_string(),
            Eighth => "1/8".to_string(),
        })
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Creature<'a> {
    pub name: String,
    pub source: Source<'a>,
    pub size: Size,
    pub creature_type: CreatureTypeEnum,
    pub alignment: Alignment,
    pub armor_class: ArmorClass,
    pub hit_points: HitPoints,
    pub speed: Speed,
    pub ability_scores: AbilityScores,
    pub saving_throws: Option<HashMap<AbilityScore, u8>>,
    pub skills: Option<HashMap<Skill, i8>>,
    pub senses: Option<Vec<String>>,
    pub passive_perception: u8,
    pub damage_resistance: Option<Vec<DamageModifier>>,
    pub damage_immunity: Option<Vec<DamageModifier>>,
    pub damage_vulnerability: Option<Vec<DamageModifier>>,
    pub condition_immunities: Option<Vec<StatusCondition>>,
    pub languages: Vec<String>,
    pub challenge_rating: ChallengeRating,
    pub abilities: Option<Vec<NamedEntry>>,
    pub actions: Option<Vec<NamedEntry>>,
    pub bonus_actions: Option<Vec<NamedEntry>>,
    pub reactions: Option<Vec<NamedEntry>>,
    pub legendary_actions: Option<Vec<NamedEntry>>,
    pub mythic_actions: Option<Vec<NamedEntry>>,
    pub mythic_header: Option<String>,
}

impl<'a> To5etools for Creature<'a> {
    fn to_5etools_base(&self) -> Value {
        let main_body = json!({
            "name": self.name,
            "size": [self.size.to_5etools_creature()],
            "type": self.creature_type.to_5etools_creature(),
            "alignment": self.alignment.to_5etools_creature(),
            "ac": self.armor_class.to_5etools_creature(),
            "hp": self.hit_points.to_5etools_creature(),
            "speed": self.speed.to_5etools_creature(),
            "passive": self.passive_perception,
            "languages": self.languages,
            "cr": self.challenge_rating.to_5etools_creature(),
        });

        let ability_scores = self.ability_scores.to_5etools_creature();
        let source = self.source.to_5etools_creature();
        let saving_throws = option_to_5etools_creature(self.saving_throws.as_ref(), "save");
        let skills = option_to_5etools_creature(self.skills.as_ref(), "skill");
        let senses = option_to_5etools_creature(self.senses.as_ref(), "senses");
        let damage_resistance =
            option_to_5etools_creature(self.damage_resistance.as_ref(), "resist");
        let damage_immunity = option_to_5etools_creature(self.damage_immunity.as_ref(), "immune");
        let damage_vulnerability =
            option_to_5etools_creature(self.damage_vulnerability.as_ref(), "vulnerable");
        let condition_immunities =
            option_to_5etools_creature(self.condition_immunities.as_ref(), "conditionImmune");
        let abilities = option_to_5etools_creature(self.abilities.as_ref(), "trait");
        let actions = option_to_5etools_creature(self.actions.as_ref(), "action");
        let bonus_actions = option_to_5etools_creature(self.bonus_actions.as_ref(), "bonus");
        let reactions = option_to_5etools_creature(self.reactions.as_ref(), "reaction");
        let legendary_actions =
            option_to_5etools_creature(self.legendary_actions.as_ref(), "legendary");
        let mythic_header = option_to_5etools_creature(self.mythic_header.as_ref(), "mythicHeader");
        let mythic_actions = option_to_5etools_creature(self.mythic_actions.as_ref(), "mythic");

        merge_json(vec![
            source,
            main_body,
            ability_scores,
            saving_throws,
            skills,
            senses,
            damage_resistance,
            damage_immunity,
            damage_vulnerability,
            condition_immunities,
            abilities,
            actions,
            bonus_actions,
            reactions,
            legendary_actions,
            mythic_header,
            mythic_actions,
        ])
    }
}
