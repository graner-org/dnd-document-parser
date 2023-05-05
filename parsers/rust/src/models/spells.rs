use super::common::*;
use super::items::*;
use serde_json::{json, Value};

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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

impl To5etools for MaterialComponent {
    fn to_5etools(self) -> Value {
        if !self.consumed && self.value.is_none() {
            self.component.into()
        } else {
            let text = json!({ "text": self.component });
            let value = match self.value {
                Some(value) => json!({ "cost": value.to_5etools() }),
                None => json!({}),
            };
            let consumed = if self.consumed {
                json!({"consume": true})
            } else {
                json!({})
            };
            merge_json(vec![text, value, consumed])
        }
    }
}

#[derive(Debug)]
pub struct Components {
    pub verbal: bool,
    pub somatic: bool,
    pub material: Option<MaterialComponent>,
}

impl To5etools for Components {
    fn to_5etools(self) -> Value {
        let verbal = match self.verbal {
            true => json!({"v": true}),
            false => json!({}),
        };
        let somatic = match self.somatic {
            true => json!({"s": true}),
            false => json!({}),
        };
        let material = match self.material {
            Some(material) => json!({ "m": material.to_5etools() }),
            None => json!({}),
        };
        merge_json(vec![verbal, somatic, material])
    }
}

#[derive(Debug)]
pub struct Duration {
    pub number: u8,
    pub unit: DurationUnit,
    pub concentration: bool,
}

impl To5etools for Duration {
    fn to_5etools(self) -> Value {
        use DurationUnit::*;
        let duration = match self.unit {
            Instantaneous => json!({"type": self.unit.to_5etools()}),
            Time(unit) => {
                let duration = json!({
                    "type": "timed",
                    "duration": {
                        "type": unit.to_5etools(),
                        "amount": self.number,
                    }
                });
                let concentration = match self.concentration {
                    true => json!({"concentration": true}),
                    false => json!({}),
                };
                merge_json(vec![duration, concentration])
            }
        };
        json!([duration])
    }
}

#[derive(Debug)]
pub struct CastingTime {
    pub number: u8,
    pub unit: CastingTimeUnit,
}

impl To5etools for CastingTime {
    fn to_5etools(self) -> Value {
        json!([{
            "number": self.number,
            "unit": self.unit.to_5etools(),
        }])
    }
}

#[derive(Debug)]
pub struct Spell {
    pub name: String,
    pub level: u8,
    pub school: MagicSchool,
    pub casting_time: CastingTime,
    pub ritual: bool,
    pub duration: Duration,
    pub range: Range,
    pub components: Components,
    pub damage_type: Option<DamageType>,
    pub description: Vec<String>,
    pub at_higher_levels: Option<String>,
    pub classes: Vec<Classes>,
}
