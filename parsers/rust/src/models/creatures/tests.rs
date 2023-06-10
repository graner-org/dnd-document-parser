use serde_json::json;

use crate::{
    models::{
        common::DamageType,
        creatures::{
            ConditionalDamageModifier, DamageModifier, DamageModifierType, FlySpeed, HitPoints,
            HitPointsFormula, Speed,
        },
    },
    utils::traits::To5etools,
};

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
            "resist": vec!["acid", "fire"],
            "note": "that is non-magical",
            "cond": true,
        })
    );
}
