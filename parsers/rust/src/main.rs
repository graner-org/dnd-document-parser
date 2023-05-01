use models::common::*;
use models::spells::*;

mod models;

fn main() {
    let s = Spell {
        name: "Blade of Disaster".to_owned(),
        level: 9,
        school: MagicSchool::Conjuration,
        casting_time: CastingTime {
            number: 1,
            unit: CastingTimeUnit::Action(ActionType::Action),
        },
        duration: Duration {
            number: 1,
            unit: DurationUnit::Time(TimeUnit::Minute),
            concentration: true,
        },
        range: Range::Point {
            range: 60,
            unit: RangeUnit::Feet,
        },
        components: Components {
            verbal: true,
            somatic: true,
            material: None,
        },
        damage_type: Some(DamageType::Force),
        description: [
            "You create a blade-shaped planar rift about 3 feet long in an unoccupied space you can see within range. The blade lasts for the duration. When you cast this spell, you can make up to two melee spell attacks with the blade, each one against a creature, loose object, or structure within 5 feet of the blade. On a hit, the target takes {@damage 4d12} force damage. This attack scores a critical hit if the number on the {@dice d20} is 18 or higher. On a critical hit, the blade deals an extra {@damage 8d12} force damage (for a total of {@damage 12d12} force damage).",
			"As a bonus action on your turn, you can move the blade up to 30 feet to an unoccupied space you can see and then make up to two melee spell attacks with it again.",
			"The blade can harmlessly pass through any barrier, including a {@spell wall of force}."
        ].map(|s| s.to_owned()).to_vec(),
        at_higher_levels: None,
    };
    println!("{:?}", s);
}
