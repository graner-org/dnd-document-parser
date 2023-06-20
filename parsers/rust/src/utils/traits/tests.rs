use std::collections::HashMap;

use serde_json::json;

use crate::{models::common::AbilityScore, utils::traits::To5etools};

#[test]
fn to5etools_vec() {
    assert_eq!(
        vec!["hi", "there"].to_5etools_base(),
        json!(["hi", "there"])
    )
}

#[test]
fn to5etools_map() {
    let map = HashMap::from([(AbilityScore::Strength, 5), (AbilityScore::Dexterity, 3)]);
    assert_eq!(map.to_5etools_base(), json!({"str": 5, "dex": 3}));

    let map = HashMap::from([
        (AbilityScore::Strength, "hi"),
        (AbilityScore::Dexterity, "there"),
    ]);
    assert_eq!(map.to_5etools_base(), json!({"str": "hi", "dex": "there"}));
}

#[test]
#[should_panic]
fn to5etools_map_panic() {
    // Keys are only allowed to be strings
    HashMap::from([(5, 5), (3, 3)]).to_5etools_base();
}
