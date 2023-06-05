use crate::utils::traits::To5etools;

use super::common::{
    merge_json, ActionType, Classes, DamageType, Description, RangeUnit, Source, TimeUnit,
};
use super::items::ItemValue;
use itertools::Itertools;
use serde_json::{json, Value};

#[cfg(test)]
mod tests;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
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
        use MagicSchool::{
            Abjuration, Conjuration, Divination, Enchantment, Evocation, Illusion, Necromancy,
            Transmutation,
        };
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
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CastingTimeUnit {
    Action(ActionType),
    Time(TimeUnit),
}

impl To5etools for CastingTimeUnit {
    fn to_5etools_base(&self) -> Value {
        use CastingTimeUnit::{Action, Time};
        match self {
            Action(action_type) => action_type.to_5etools_spell(),
            Time(time_unit) => time_unit.to_5etools_spell(),
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TargetType {
    Point,
    Radius,
    Cone,
}

impl To5etools for TargetType {
    fn to_5etools_base(&self) -> Value {
        use TargetType::{Cone, Point, Radius};
        json!(match self {
            Point => "point",
            Radius => "radius",
            Cone => "cone",
        })
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
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
        use Range::{Ranged, Self_, Special, Touch};
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

#[derive(Debug, Clone, PartialEq, Eq)]
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
            let value = self.value.map_or_else(
                || json!({}),
                |value| json!({ "cost": value.to_5etools_spell() }),
            );
            let consumed = if self.consumed {
                json!({"consume": true})
            } else {
                json!({})
            };
            merge_json(vec![text, value, consumed])
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Components {
    pub verbal: bool,
    pub somatic: bool,
    pub material: Option<MaterialComponent>,
}

impl To5etools for Components {
    fn to_5etools_base(&self) -> Value {
        let verbal = if self.verbal {
            json!({"v": true})
        } else {
            json!({})
        };
        let somatic = if self.somatic {
            json!({"s": true})
        } else {
            json!({})
        };
        let material = self.material.as_ref().map_or_else(
            || json!({}),
            |material| json!({ "m": material.to_5etools_spell() }),
        );
        merge_json(vec![verbal, somatic, material])
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Duration {
    Instantaneous,
    Timed(TimedDuration),
}

impl To5etools for Duration {
    fn to_5etools_base(&self) -> Value {
        use Duration::{Instantaneous, Timed};
        let duration = match self {
            Instantaneous => json!({"type": "instant"}),
            Timed(duration) => duration.to_5etools_base(),
        };
        json!([duration])
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
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
        let concentration = if self.concentration {
            json!({"concentration": true})
        } else {
            json!({})
        };
        merge_json(vec![duration, concentration])
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
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
    pub description: Vec<Description>,
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
            // "entries": description_serialization(&self.description),
            "entries": self.description.iter().map(To5etools::to_5etools_spell).collect_vec(),
            "classes": json!({
                "fromClassList": self.classes.to_5etools_spell()
            }),
        });
        let damage_type = self.damage_types.as_ref().map_or_else(
            || json!({}),
            |damage_types| {
                json!({
                    "damageInflict": damage_types.to_5etools_spell(),
                })
            },
        );
        let at_higher_levels = self.at_higher_levels.as_ref().map_or_else(
            || json!({}),
            |entries| {
                json!({
                    "entriesHigherLevel": [{
                        "type": "entries",
                        "name": "At Higher Levels",
                        "entries": [ entries ],
                    }]
                })
            },
        );
        let ritual = if self.ritual {
            json!({
                "meta": {
                    "ritual": true,
                },
            })
        } else {
            json!({})
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
