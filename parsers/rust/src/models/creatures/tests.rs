use serde_json::json;

use crate::{
    models::creatures::{FlySpeed, HitPoints, HitPointsFormula, Speed},
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
