#[derive(Debug)]
pub enum ActionType {
    Action,
    BonusAction,
    Reaction,
}

#[derive(Debug)]
pub enum TimeUnit {
    Round,
    Minute,
    Hour,
    Day,
    Year,
}

#[derive(Debug)]
pub enum DurationUnit {
    Instantaneous,
    Time(TimeUnit),
}

#[derive(Debug)]
pub enum RangeUnit {
    Feet,
    Mile,
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
