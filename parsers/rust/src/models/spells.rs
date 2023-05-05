use super::common::*;
use super::items::*;
use serde_json::{json, Value};

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
    fn to_5etools(self) -> Value {
        use MagicSchool::*;
        json!(match self {
            Abjuration => "A",
            Conjuration => "C",
            Divination => "D",
            Enchantment => "E",
            Evocation => "V",
            Illusion => "I",
            Necromancy => "N",
            Transmutation => "T",
        })
    }
}

#[derive(Debug)]
pub enum CastingTimeUnit {
    Action(ActionType),
    Time(TimeUnit),
}

impl To5etools for CastingTimeUnit {
    fn to_5etools(self) -> Value {
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
    fn to_5etools(self) -> Value {
        use TargetType::*;
        json!(match self {
            Point => "point",
            Radius => "radius",
            Cone => "cone",
        })
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
    fn to_5etools(self) -> Value {
        use Range::*;
        match self {
            Self_ => json!({
                "type": "point",
                "distance": {
                    "type": "self"
                }
            }),
            Touch => json!({
                "type": "point",
                "distance": {
                    "type": "touch"
                }
            }),
            Ranged { type_, range, unit } => json!({
                "type": type_.to_5etools(),
                "distance": {
                    "type": unit.to_5etools(),
                    "amount": range
                }
            }),
            Special => json!({ "type": "special" }),
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
