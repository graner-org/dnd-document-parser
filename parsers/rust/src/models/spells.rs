use super::common::*;
use super::items::*;

#[derive(Debug)]
pub enum MagicSchool {
    Conjuration,
    Necromancy,
    Evocation,
    Abjuration,
    Transmutation,
    Divination,
    Enchantment,
    Illusion,
}

#[derive(Debug)]
pub enum CastingTimeUnit {
    Action(ActionType),
    Time(TimeUnit),
}

#[derive(Debug)]
pub enum TargetType {
    SingleTarget,
    MultipleTargets,
    Circle {
        radius: u16,
        unit: RangeUnit,
    },
    Cone {
        width: u16,
        unit: RangeUnit,
    },
    Cube {
        side_length: u16,
        unit: RangeUnit,
    },
    Cylinder {
        radius: u16,
        height: u16,
        unit: RangeUnit,
    },
    Hemisphere {
        radius: u16,
        unit: RangeUnit,
    },
    Line {
        length: u16,
        width: u16,
        unit: RangeUnit,
    },
    Sphere {
        radius: u16,
        unit: RangeUnit,
    },
    Square {
        side_length: u16,
        unit: RangeUnit,
    },
    Wall {
        length: u16,
        height: u16,
        unit: RangeUnit,
    },
}

#[derive(Debug)]
pub enum Range {
    Self_,
    Touch,
    Point {
        range: u16,
        unit: RangeUnit,
    },
    Area {
        range: u16,
        unit: RangeUnit,
        target_type: TargetType,
    },
    SelfArea(TargetType),
    Special,
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
