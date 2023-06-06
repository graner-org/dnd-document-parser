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
