use crate::utils::traits::To5etools;
use serde_json::{json, Value};

use super::common::{merge_json, DamageType};

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
        let empty_json = json!({});
        merge_json(vec![
            json!({"walk": self.walk}),
            self.burrow
                .map_or_else(|| empty_json.clone(), |speed| json!({ "burrow": speed })),
            self.climb
                .map_or_else(|| empty_json.clone(), |speed| json!({ "climb": speed })),
            self.crawl
                .map_or_else(|| empty_json.clone(), |speed| json!({ "crawl": speed })),
            self.fly.as_ref().map_or_else(
                || empty_json.clone(),
                |fly_speed| json!({ "fly": fly_speed.to_5etools_base() }),
            ),
            self.swim
                .map_or_else(|| empty_json.clone(), |speed| json!({ "swim": speed })),
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
        self.subtypes.as_ref().map_or_else(
            || self.main_type.to_5etools_base(),
            |subtypes| {
                json!({
                    "type": self.main_type.to_5etools_base(),
                    "tags": subtypes,
                })
            },
        )
    }
}
