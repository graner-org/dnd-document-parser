use serde_json::{json, Value};

use crate::utils::traits::To5etools;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct Source<'a> {
    pub source_book: &'a str,
    pub page: i16,
}

impl<'a> To5etools for Source<'a> {
    fn to_5etools_base(&self) -> Value {
        json!({
            "source": self.source_book,
            "page": self.page,
        })
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum ActionType {
    Action,
    BonusAction,
    Reaction { condition: String },
}

impl To5etools for ActionType {
    fn to_5etools_base(&self) -> Value {
        use ActionType::*;
        json!(match self {
            Action => "action",
            BonusAction => "bonus",
            Reaction { condition: _ } => "reaction",
        })
    }
}

pub fn merge_json(json_vec: Vec<Value>) -> Value {
    json_vec
        .into_iter()
        .map(|json: Value| json.as_object().unwrap().clone()) // Value -> Map<String, value>
        .reduce(|map1, map2| map1.into_iter().chain(map2).collect()) // Add maps
        .unwrap()
        .into()
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TimeUnit {
    Round,
    Minute,
    Hour,
    Day,
    Year,
}

impl To5etools for TimeUnit {
    fn to_5etools_base(&self) -> Value {
        use TimeUnit::*;
        json!(match self {
            Round => "round",
            Minute => "minute",
            Hour => "hour",
            Day => "day",
            Year => "year",
        })
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum RangeUnit {
    Feet,
    Miles,
}

impl To5etools for RangeUnit {
    fn to_5etools_base(&self) -> Value {
        use RangeUnit::*;
        json!(match self {
            Feet => "feet",
            Miles => "miles",
        })
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DamageType {
    Acid,
    Bludgeoning,
    Cold,
    Fire,
    Force,
    Lightning,
    Necrotic,
    Piercing,
    Poison,
    Psychic,
    Radiant,
    Slashing,
    Thunder,
}

impl To5etools for DamageType {
    fn to_5etools_base(&self) -> Value {
        use DamageType::*;
        json!(match self {
            Acid => "acid",
            Bludgeoning => "bludgeoning",
            Cold => "cold",
            Fire => "fire",
            Force => "force",
            Lightning => "lightning",
            Necrotic => "necrotic",
            Piercing => "piercing",
            Poison => "poison",
            Psychic => "psychic",
            Radiant => "radiant",
            Slashing => "slashing",
            Thunder => "thunder",
        })
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Classes {
    Artificer,
    Barbarian,
    Bard,
    Cleric,
    Druid,
    Fighter,
    Monk,
    Paladin,
    Ranger,
    Rogue,
    Sorcerer,
    Warlock,
    Wizard,
}

impl To5etools for Classes {
    fn to_5etools_base(&self) -> Value {
        use Classes::*;
        json!(match self {
            Artificer => "Artificer",
            Barbarian => "Barbarian",
            Bard => "Bard",
            Cleric => "Cleric",
            Druid => "Druid",
            Fighter => "Fighter",
            Monk => "Monk",
            Paladin => "Paladin",
            Ranger => "Ranger",
            Rogue => "Rogue",
            Sorcerer => "Sorcerer",
            Warlock => "Warlock",
            Wizard => "Wizard",
        })
    }
    fn to_5etools_spell(&self) -> Value {
        json!({
            "name": self.to_5etools_base(),
            "source": self.source_book(),
        })
    }
}

impl Classes {
    fn source_book(self) -> String {
        use Classes::*;
        match self {
            Artificer => "TCE",
            _ => "PHB",
        }
        .to_owned()
    }
}
