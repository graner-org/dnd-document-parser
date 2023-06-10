use crate::utils::traits::To5etools;
use serde_json::{json, Value};

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
pub enum CreatureType {
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

impl To5etools for CreatureType {
    fn to_5etools_base(&self) -> Value {
        use CreatureType::*;
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
