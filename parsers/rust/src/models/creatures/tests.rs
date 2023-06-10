use serde_json::json;

use crate::{
    models::creatures::{HitPoints, HitPointsFormula},
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
