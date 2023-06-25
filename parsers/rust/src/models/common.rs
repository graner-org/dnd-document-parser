use itertools::Itertools;
use regex::Regex;
use serde_json::{json, Value};

use crate::utils::traits::To5etools;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Description {
    Entry(String),
    List(Vec<Self>),
}

impl To5etools for Description {
    fn to_5etools_base(&self) -> Value {
        use Description::{Entry, List};
        match self {
            Entry(entry) => {
                // Capture e.g. "2d4" or "20d12" or "5d6 + 10"
                let dice_capture = Regex::new(r"(?P<dice>\d+d\d+(?: [\+-] \d+)?)").unwrap();
                Value::String(
                    dice_capture
                        .replace_all(entry, "{@damage $dice}")
                        .to_string(),
                )
            }
            List(list_entries) => json!({
                "type": "list",
                "items": list_entries.iter().map(Self::to_5etools_base).collect_vec()
            }),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ActionType {
    Action,
    BonusAction,
    Reaction { condition: String },
}

impl To5etools for ActionType {
    fn to_5etools_base(&self) -> Value {
        use ActionType::{Action, BonusAction, Reaction};
        json!(match self {
            Action => "action",
            BonusAction => "bonus",
            Reaction { condition: _ } => "reaction",
        })
    }
}

#[must_use]
pub fn merge_json(json_vec: Vec<Value>) -> Value {
    json_vec
        .into_iter()
        .map(|json: Value| json.as_object().unwrap().clone()) // Value -> Map<String, value>
        .reduce(|map1, map2| map1.into_iter().chain(map2).collect()) // Add maps
        .unwrap()
        .into()
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TimeUnit {
    Round,
    Minute,
    Hour,
    Day,
    Year,
}

impl To5etools for TimeUnit {
    fn to_5etools_base(&self) -> Value {
        use TimeUnit::{Day, Hour, Minute, Round, Year};
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
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RangeUnit {
    Feet,
    Miles,
}

impl To5etools for RangeUnit {
    fn to_5etools_base(&self) -> Value {
        use RangeUnit::{Feet, Miles};
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
        use DamageType::{
            Acid, Bludgeoning, Cold, Fire, Force, Lightning, Necrotic, Piercing, Poison, Psychic,
            Radiant, Slashing, Thunder,
        };
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        use Classes::{
            Artificer, Barbarian, Bard, Cleric, Druid, Fighter, Monk, Paladin, Ranger, Rogue,
            Sorcerer, Warlock, Wizard,
        };
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
        use Classes::Artificer;
        match self {
            Artificer => "TCE",
            _ => "PHB",
        }
        .to_owned()
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatusCondition {
    Blinded,
    Charmed,
    Deafened,
    Exhaustion,
    Frightened,
    Grappled,
    Incapacitated,
    Invisible,
    Paralyzed,
    Petrified,
    Poisoned,
    Prone,
    Restrained,
    Stunned,
}

impl To5etools for StatusCondition {
    fn to_5etools_base(&self) -> Value {
        use StatusCondition::*;
        Value::String(
            match self {
                Blinded => "blinded",
                Charmed => "charmed",
                Deafened => "deafened",
                Exhaustion => "exhaustion",
                Frightened => "frightened",
                Grappled => "grappled",
                Incapacitated => "incapacitated",
                Invisible => "invisible",
                Paralyzed => "paralyzed",
                Petrified => "petrified",
                Poisoned => "poisoned",
                Prone => "prone",
                Restrained => "restrained",
                Stunned => "stunned",
            }
            .to_owned(),
        )
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AbilityScore {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

impl To5etools for AbilityScore {
    fn to_5etools_base(&self) -> Value {
        use AbilityScore::*;
        Value::String(
            match self {
                Strength => "str",
                Dexterity => "dex",
                Constitution => "con",
                Intelligence => "int",
                Wisdom => "wis",
                Charisma => "cha",
            }
            .to_owned(),
        )
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Skill {
    Acrobatics,
    AnimalHandling,
    Arcana,
    Athletics,
    Deception,
    History,
    Insight,
    Intimidation,
    Investigation,
    Medicine,
    Nature,
    Perception,
    Performance,
    Persuasion,
    Religion,
    SleightOfHand,
    Stealth,
    Survival,
}

impl To5etools for Skill {
    fn to_5etools_base(&self) -> Value {
        use Skill::*;
        Value::String(
            match self {
                Acrobatics => "acrobatics",
                AnimalHandling => "animal handling",
                Arcana => "arcana",
                Athletics => "athletics",
                Deception => "deception",
                History => "history",
                Insight => "insight",
                Intimidation => "intimidation",
                Investigation => "investigation",
                Medicine => "medicine",
                Nature => "nature",
                Perception => "perception",
                Performance => "performance",
                Persuasion => "persuasion",
                Religion => "religion",
                SleightOfHand => "sleight of hand",
                Stealth => "stealth",
                Survival => "survival",
            }
            .to_owned(),
        )
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Language {
    Common,
    Dwarvish,
    Elvish,
    Giant,
    Gnomish,
    Goblin,
    Halfling,
    Orc,
    Abyssal,
    Celestial,
    Draconic,
    DeepSpeech,
    Infernal,
    Primordial,
    Sylvan,
    Undercommon,
}

impl To5etools for Language {
    fn to_5etools_base(&self) -> Value {
        use Language::*;
        Value::String(
            match self {
                Common => "Common",
                Dwarvish => "Dwarvish",
                Elvish => "Elvish",
                Giant => "Giant",
                Gnomish => "Gnomish",
                Goblin => "Goblin",
                Halfling => "Halfling",
                Orc => "Orc",
                Abyssal => "Abyssal",
                Celestial => "Celestial",
                Draconic => "Draconic",
                DeepSpeech => "Deep Speech",
                Infernal => "Infernal",
                Primordial => "Primordial",
                Sylvan => "Sylvan",
                Undercommon => "Undercommon",
            }
            .to_owned(),
        )
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlignmentAxisOrder {
    Lawful,
    Neutral,
    Chaotic,
}

impl To5etools for AlignmentAxisOrder {
    fn to_5etools_base(&self) -> Value {
        use AlignmentAxisOrder::*;
        Value::String(
            match self {
                Lawful => "L",
                Neutral => "N",
                Chaotic => "C",
            }
            .to_owned(),
        )
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlignmentAxisMoral {
    Good,
    Neutral,
    Evil,
}

impl To5etools for AlignmentAxisMoral {
    fn to_5etools_base(&self) -> Value {
        use AlignmentAxisMoral::*;
        Value::String(
            match self {
                Good => "G",
                Neutral => "N",
                Evil => "E",
            }
            .to_owned(),
        )
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlignmentAxis {
    Order(AlignmentAxisOrder),
    Moral(AlignmentAxisMoral),
}

impl To5etools for AlignmentAxis {
    fn to_5etools_base(&self) -> Value {
        use AlignmentAxis::*;
        match self {
            Order(order) => order.to_5etools_base(),
            Moral(moral) => moral.to_5etools_base(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Alignment {
    OneAxis(AlignmentAxis),
    TwoAxes {
        order: AlignmentAxisOrder,
        moral: AlignmentAxisMoral,
    },
    Any,
    Unaligned,
}

impl To5etools for Alignment {
    fn to_5etools_base(&self) -> Value {
        use Alignment::*;
        json!(match self {
            Any => vec![json!("A")],
            Unaligned => vec![json!("U")],
            OneAxis(axis) => vec![axis.to_5etools_base()],
            TwoAxes {
                order: AlignmentAxisOrder::Neutral,
                moral: AlignmentAxisMoral::Neutral,
            } => vec![json!("N")],
            TwoAxes { order, moral } => vec![order.to_5etools_base(), moral.to_5etools_base()],
        })
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedEntry {
    pub name: String,
    pub entry: String,
}

impl To5etools for NamedEntry {
    fn to_5etools_base(&self) -> Value {
        let attack_type_re =
            Regex::new(r"^(?P<range>(?:Melee)|(?:Ranged)) (?P<type>(?:Weapon)|(?:Spell)) Attack")
                .unwrap();
        let attack_type = attack_type_re.captures(&self.entry).map_or_else(
            || "".to_string(),
            |capture| {
                capture.get(0).map_or_else(
                    || "".to_string(),
                    |attack_type| {
                        attack_type
                            .as_str()
                            .split(' ')
                            .take(2)
                            .map(|word| word.chars().next().unwrap().to_ascii_lowercase())
                            .collect::<String>()
                    },
                )
            },
        );

        let to_hit_re = Regex::new(r"\+(?P<to_hit>[0-9]+) to hit").unwrap();
        let hit_re = Regex::new(r"\. (Hit: )").unwrap();
        let dice_re = Regex::new(r"(?P<dice>\d+d\d+(?: [\+-] \d+)?)").unwrap();
        let entry = attack_type_re.replace(&self.entry, format!("{{@atk {attack_type}}}"));
        let entry = to_hit_re.replace(&entry, "{@hit $to_hit} to hit");
        let entry = hit_re.replace(&entry, ". {@h}");
        let entry = dice_re.replace_all(&entry, "{@damage $dice}");
        json!({
            "name": self.name,
            "entries": [
                entry,
            ]
        })
    }
}
