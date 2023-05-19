use crate::traits::To5etools;

use super::common::*;
use super::items::*;
use serde_json::{json, Value};

#[cfg(test)]
mod tests;

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
    fn to_5etools_base(&self) -> Value {
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
#[derive(Debug, PartialEq, Clone)]
pub enum CastingTimeUnit {
    Action(ActionType),
    Time(TimeUnit),
}

impl To5etools for CastingTimeUnit {
    fn to_5etools_base(&self) -> Value {
        use CastingTimeUnit::*;
        match self {
            Action(action_type) => action_type.to_5etools_spell(),
            Time(time_unit) => time_unit.to_5etools_spell(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum TargetType {
    Point,
    Radius,
    Cone,
}

impl To5etools for TargetType {
    fn to_5etools_base(&self) -> Value {
        use TargetType::*;
        json!(match self {
            Point => "point",
            Radius => "radius",
            Cone => "cone",
        })
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
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
    fn to_5etools_base(&self) -> Value {
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
                "type": type_.to_5etools_spell(),
                "distance": {
                    "type": unit.to_5etools_spell(),
                    "amount": range
                }
            }),
            Special => json!({ "type": "special" }),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MaterialComponent {
    pub component: String,
    pub value: Option<ItemValue>,
    pub consumed: bool,
}

impl To5etools for MaterialComponent {
    fn to_5etools_base(&self) -> Value {
        if !self.consumed && self.value.is_none() {
            json!(self.component)
        } else {
            let text = json!({ "text": self.component });
            let value = match self.value {
                Some(value) => json!({ "cost": value.to_5etools_spell() }),
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

#[derive(Debug, PartialEq)]
pub struct Components {
    pub verbal: bool,
    pub somatic: bool,
    pub material: Option<MaterialComponent>,
}

impl To5etools for Components {
    fn to_5etools_base(&self) -> Value {
        let verbal = match self.verbal {
            true => json!({"v": true}),
            false => json!({}),
        };
        let somatic = match self.somatic {
            true => json!({"s": true}),
            false => json!({}),
        };
        let material = match &self.material {
            Some(material) => json!({ "m": material.to_5etools_spell() }),
            None => json!({}),
        };
        merge_json(vec![verbal, somatic, material])
    }
}

#[derive(Debug, PartialEq)]
pub enum Duration {
    Instantaneous,
    Timed(TimedDuration),
}

impl To5etools for Duration {
    fn to_5etools_base(&self) -> Value {
        use Duration::*;
        let duration = match self {
            Instantaneous => json!({"type": "instant"}),
            Timed(duration) => duration.to_5etools_base(),
        };
        json!([duration])
    }
}

#[derive(Debug, PartialEq)]
pub struct TimedDuration {
    pub number: u8,
    pub unit: TimeUnit,
    pub concentration: bool,
}

impl To5etools for TimedDuration {
    fn to_5etools_base(&self) -> Value {
        let duration = json!({
            "type": "timed",
            "duration": {
                "type": self.unit.to_5etools_spell(),
                "amount": self.number,
            }
        });
        let concentration = match self.concentration {
            true => json!({"concentration": true}),
            false => json!({}),
        };
        merge_json(vec![duration, concentration])
    }
}

#[derive(Debug, PartialEq)]
pub struct CastingTime {
    pub number: u8,
    pub unit: CastingTimeUnit,
}

impl To5etools for CastingTime {
    fn to_5etools_base(&self) -> Value {
        let condition = match &self.unit {
            CastingTimeUnit::Action(ActionType::Reaction { condition }) => json!({
                "condition": condition,
            }),
            _ => json!({}),
        };
        let number_and_unit = json!({
            "number": self.number,
            "unit": self.unit.to_5etools_spell(),
        });
        json!([merge_json(vec![number_and_unit, condition])])
    }
}

#[derive(Debug)]
pub struct Spell<'a> {
    pub source: Source<'a>,
    pub name: String,
    pub level: u8,
    pub school: MagicSchool,
    pub casting_time: CastingTime,
    pub ritual: bool,
    pub duration: Duration,
    pub range: Range,
    pub components: Components,
    pub damage_types: Option<Vec<DamageType>>,
    pub description: Vec<String>,
    pub at_higher_levels: Option<String>,
    pub classes: Vec<Classes>,
}

impl<'a> To5etools for Spell<'a> {
    fn to_5etools_base(&self) -> Value {
        let source = self.source.to_5etools_base();
        let main_body = json!({
            "name": self.name,
            "level": self.level,
            "school": self.school.to_5etools_spell(),
            "time": self.casting_time.to_5etools_spell(),
            "range": self.range.to_5etools_spell(),
            "components": self.components.to_5etools_spell(),
            "duration": self.duration.to_5etools_spell(),
            "entries": self.description,
            "classes": json!({
                "fromClassList": self.classes.to_5etools_spell()
            }),
        });
        let damage_type = match &self.damage_types {
            Some(damage_types) => json!({
                "damageInflict": damage_types.to_5etools_spell(),
            }),
            None => json!({}),
        };
        let at_higher_levels = match &self.at_higher_levels {
            Some(entries) => json!({
                "entriesHigherLevel": [{
                    "type": "entries",
                    "name": "At Higher Levels",
                    "entries": [ entries ],
                }]
            }),
            None => json!({}),
        };
        let ritual = match self.ritual {
            true => json!({
                "meta": {
                    "ritual": true,
                },
            }),
            false => json!({}),
        };
        merge_json(vec![
            source,
            main_body,
            damage_type,
            at_higher_levels,
            ritual,
        ])
    }
}
