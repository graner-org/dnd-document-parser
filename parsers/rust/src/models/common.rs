pub trait To5etools {
    fn to_5etools(self) -> String;
}

#[derive(Debug)]
pub enum ActionType {
    Action,
    BonusAction,
    Reaction,
}

impl To5etools for ActionType {
    fn to_5etools(self) -> String {
        use ActionType::*;
        match self {
            Action => "action",
            BonusAction => "bonus",
            Reaction => "reaction",
        }
        .to_owned()
    }
}

#[derive(Debug)]
pub enum TimeUnit {
    Round,
    Minute,
    Hour,
    Day,
    Year,
}

impl To5etools for TimeUnit {
    fn to_5etools(self) -> String {
        use TimeUnit::*;
        match self {
            Round => "round",
            Minute => "minute",
            Hour => "hour",
            Day => "day",
            Year => "year",
        }
        .to_owned()
    }
}

#[derive(Debug)]
pub enum DurationUnit {
    Instantaneous,
    Time(TimeUnit),
}

impl To5etools for DurationUnit {
    fn to_5etools(self) -> String {
        use DurationUnit::*;
        match self {
            Instantaneous => "instant".to_owned(),
            Time(unit) => unit.to_5etools(),
        }
    }
}

#[derive(Debug)]
pub enum RangeUnit {
    Feet,
    Miles,
}

impl To5etools for RangeUnit {
    fn to_5etools(self) -> String {
        use RangeUnit::*;
        match self {
            Feet => "feet",
            Miles => "miles",
        }
        .to_owned()
    }
}

#[derive(Debug)]
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
    fn to_5etools(self) -> String {
        use DamageType::*;
        match self {
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
        }
        .to_owned()
    }
}

#[derive(Debug)]
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
    fn to_5etools(self) -> String {
        use Classes::*;
        match self {
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
        }
        .to_owned()
    }
}
