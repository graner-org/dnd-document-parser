use std::{fs::File, io::BufReader};

use serde_json::{json, Value};

use crate::{
    models::{
        common::{DamageType, NamedEntry, Source},
        creatures::{
            ArmorClass, ConditionalDamageModifier, CreatureType, CreatureTypeEnum, DamageModifier,
            DamageModifierType, FlySpeed, HitPoints, HitPointsFormula, Speed,
        },
    },
    utils::{compare::json_compare, traits::To5etools},
};

use super::{Creature, Size};

#[test]
fn hit_points() {
    assert_eq!(
        HitPoints {
            average: 91,
            formula: HitPointsFormula {
                number_of_dice: 14,
                die_size: 8,
                modifier: 28
            }
        }
        .to_5etools_base(),
        json!({"average": 91, "formula": "14d8 + 28"})
    )
}

#[test]
fn speed() {
    assert_eq!(
        Speed {
            walk: 30,
            burrow: None,
            climb: None,
            crawl: None,
            fly: None,
            swim: None,
        }
        .to_5etools_base(),
        json!({"walk": 30})
    );

    assert_eq!(
        Speed {
            walk: 30,
            burrow: Some(40),
            climb: None,
            crawl: None,
            fly: None,
            swim: Some(10),
        }
        .to_5etools_base(),
        json!({"walk": 30, "burrow": 40, "swim": 10})
    );

    assert_eq!(
        Speed {
            walk: 0,
            burrow: None,
            climb: None,
            crawl: None,
            fly: Some(FlySpeed {
                speed: 60,
                hover: false
            }),
            swim: Some(10),
        }
        .to_5etools_base(),
        json!({"walk": 0, "fly": 60, "swim": 10})
    );

    assert_eq!(
        Speed {
            walk: 0,
            burrow: None,
            climb: None,
            crawl: None,
            fly: Some(FlySpeed {
                speed: 60,
                hover: true
            }),
            swim: Some(10),
        }
        .to_5etools_base(),
        json!({"walk": 0, "fly": { "number": 60, "condition": "(hover)" }, "swim": 10})
    );
}

#[test]
fn damage_resistance() {
    use DamageType::{Acid, Fire};
    assert_eq!(
        DamageModifier::Unconditional(Acid).to_5etools_base(),
        json!("acid")
    );

    assert_eq!(
        DamageModifier::Conditional(ConditionalDamageModifier {
            modifier_type: DamageModifierType::Resistance,
            damage_types: vec![Acid, Fire],
            condition: "that is non-magical".to_string()
        })
        .to_5etools_base(),
        json!({
            "resist": ["acid", "fire"],
            "note": "that is non-magical",
            "cond": true,
        })
    );
}

#[test]
fn creature_type() {
    use CreatureTypeEnum::Fiend;
    assert_eq!(
        CreatureType {
            main_type: Fiend,
            subtypes: None,
        }
        .to_5etools_base(),
        json!("fiend")
    );

    assert_eq!(
        CreatureType {
            main_type: Fiend,
            subtypes: Some(vec!["demon".to_string()]),
        }
        .to_5etools_base(),
        json!({
            "type": "fiend",
            "tags": ["demon"]
        })
    );
}

#[test]
fn armor_class() {
    assert_eq!(
        ArmorClass {
            ac: 10,
            armor_type: None
        }
        .to_5etools_base(),
        json!([10])
    );

    assert_eq!(
        ArmorClass {
            ac: 10,
            armor_type: Some("Natural Armor".to_string())
        }
        .to_5etools_base(),
        json!([
            {
                "ac": 10,
                "from": ["Natural Armor"]
            }
        ])
    );
}

fn read_json_file(filename: String) -> Value {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}

#[test]
fn creature() {
    let expected_json = read_json_file("resources/test/creatures/unit.json".to_string())
        .get("monster")
        .and_then(|array| array.get(0))
        .unwrap()
        .to_owned();

    let creature = Creature {
        name: "test".to_string(),
        source: Source {
            source_book: "book",
            page: 0,
        },
        size: Size::Medium,
        creature_type: CreatureTypeEnum::Beast,
        alignment: crate::models::common::Alignment::Unaligned,
        armor_class: ArmorClass {
            ac: 10,
            armor_type: None,
        },
        hit_points: HitPoints {
            average: 10,
            formula: HitPointsFormula {
                number_of_dice: 1,
                die_size: 10,
                modifier: 4,
            },
        },
        speed: Speed {
            walk: 30,
            burrow: None,
            climb: None,
            crawl: None,
            fly: None,
            swim: None,
        },
        ability_scores: super::AbilityScores {
            strength: 10,
            dexterity: 10,
            constitution: 10,
            intelligence: 10,
            wisdom: 10,
            charisma: 10,
        },
        saving_throws: None,
        skills: None,
        senses: None,
        passive_perception: 10,
        damage_resistance: None,
        damage_immunity: None,
        damage_vulnerability: None,
        condition_immunities: None,
        languages: vec!["Common".to_string()],
        challenge_rating: super::ChallengeRating::WholeNumber(2),
        abilities: None,
        actions: Some(vec![NamedEntry {
            name: "attack".to_string(),
            entry:
                "Melee Weapon Attack: +5 to hit, reach 5 ft. Hit: 10 (1d10 + 4) slashing damage."
                    .to_string(),
        }]),
        bonus_actions: None,
        reactions: None,
        legendary_actions: None,
        mythic_actions: None,
        mythic_header: None,
    };
    json_compare(creature.to_5etools_creature(), expected_json).unwrap()
}
