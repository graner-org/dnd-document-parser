use super::common::*;
use super::items::*;

#[derive(Debug)]
pub enum MagicSchool {
    Abjuration,
    Conjuration,
    Divination,
    Enchantment,
    Evocation,
    Illusion,
    Necromancy,
    Transmutation,
}

impl To5etools for MagicSchool {
    fn to_5etools(self) -> String {
        use MagicSchool::*;
        match self {
            Abjuration => "A",
            Conjuration => "C",
            Divination => "D",
            Enchantment => "E",
            Evocation => "V",
            Illusion => "I",
            Necromancy => "N",
            Transmutation => "T",
        }
        .to_owned()
    }
}

#[derive(Debug)]
pub enum CastingTimeUnit {
    Action(ActionType),
    Time(TimeUnit),
}

impl To5etools for CastingTimeUnit {
    fn to_5etools(self) -> String {
        use CastingTimeUnit::*;
        match self {
            Action(action_type) => action_type.to_5etools(),
            Time(time_unit) => time_unit.to_5etools(),
        }
    }
}

#[derive(Debug)]
pub enum TargetType {
    Point,
    Radius,
    Cone,
}

impl To5etools for TargetType {
    fn to_5etools(self) -> String {
        use TargetType::*;
        match self {
            Point => "point",
            Radius => "radius",
            Cone => "cone",
        }
        .to_owned()
    }
}

#[derive(Debug)]
pub enum Range {
    Self_,
    Touch,
    Ranged {
        type_: TargetType,
        range: u16,
        unit: RangeUnit,
    },
    Special,
}

impl To5etools for Range {
    fn to_5etools(self) -> String {
        use Range::*;
        match self {
            Self_ => r#"{ "type": "point", "distance": { "type": "self" } }"#.to_owned(),
            Touch => r#"{ "type": "point", "distance": { "type": "touch" } }"#.to_owned(),
            Ranged { type_, range, unit } => format!(
                // Braces { and } are escaped by doubling.
                r#"{{ "type": "{}", "distance": {{ "type": "{}", "amount": {} }} }}"#,
                type_.to_5etools(),
                unit.to_5etools(),
                range
            ),
            Special => r#"{ "type": "special" }"#.to_owned(),
        }
    }
}

#[derive(Debug)]
pub struct MaterialComponent {
    pub component: String,
    pub value: Option<ItemValue>,
    pub consumed: bool,
}

#[derive(Debug)]
pub struct Components {
    pub verbal: bool,
    pub somatic: bool,
    pub material: Option<MaterialComponent>,
}

#[derive(Debug)]
pub struct Duration {
    pub number: u8,
    pub unit: DurationUnit,
    pub concentration: bool,
}

#[derive(Debug)]
pub struct CastingTime {
    pub number: u8,
    pub unit: CastingTimeUnit,
}

#[derive(Debug)]
pub struct Spell {
    pub name: String,
    pub level: u8,
    pub school: MagicSchool,
    pub casting_time: CastingTime,
    pub duration: Duration,
    pub range: Range,
    pub components: Components,
    pub damage_type: Option<DamageType>,
    pub description: Vec<String>,
    pub at_higher_levels: Option<String>,
    pub classes: Vec<Classes>,
}
