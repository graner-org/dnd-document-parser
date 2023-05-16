use super::*;

#[test]
fn casting_time_test() {
    let casting_time: CastingTime = CastingTime {
        number: 1,
        unit: CastingTimeUnit::Action(ActionType::Action),
    };

    let casting_time_result: Value = json!([{
        "number": 1,
        "unit": "action"
    }]);

    assert_eq!(casting_time.to_5etools_spell(), casting_time_result);
}

#[test]
fn duration_test() {
    assert_eq!(
        Duration::Instantaneous.to_5etools_spell(),
        json!([{
            "type": "instant"
        }])
    );

    assert_eq!(
        Duration::Timed(TimedDuration {
            number: 1,
            unit: TimeUnit::Round,
            concentration: true,
        })
        .to_5etools_spell(),
        json!([{
            "type": "timed",
            "duration": {
                "type": "round",
                "amount": 1,
            },
            "concentration": true
        }])
    );
}

#[test]
fn range_test() {
    assert_eq!(
        Range::Self_.to_5etools_spell(),
        json!({
            "type": "point",
            "distance": {
                "type": "self"
            }
        })
    );

    assert_eq!(
        Range::Touch.to_5etools_spell(),
        json!({
            "type": "point",
            "distance": {
                "type": "touch"
            }
        })
    );

    assert_eq!(
        Range::Special.to_5etools_spell(),
        json!({
            "type": "special"
        })
    );

    assert_eq!(
        Range::Ranged {
            type_: TargetType::Point,
            range: 90,
            unit: RangeUnit::Feet,
        }
        .to_5etools_spell(),
        json!({
            "type": "point",
            "distance": {
                "type": "feet",
                "amount": 90,
            }
        })
    );

    assert_eq!(
        Range::Ranged {
            type_: TargetType::Cone,
            range: 90,
            unit: RangeUnit::Feet,
        }
        .to_5etools_spell(),
        json!({
            "type": "cone",
            "distance": {
                "type": "feet",
                "amount": 90,
            }
        })
    );
}

#[test]
fn component_test() {
    assert_eq!(
        Components {
            verbal: true,
            somatic: false,
            material: None,
        }
        .to_5etools_spell(),
        json!({"v": true})
    );
    assert_eq!(
        Components {
            verbal: false,
            somatic: true,
            material: None,
        }
        .to_5etools_spell(),
        json!({"s": true})
    );
    assert_eq!(
        Components {
            verbal: true,
            somatic: true,
            material: Some(MaterialComponent {
                component: "diamonds worth 300 gp, which the spell consumes".to_owned(),
                value: Some(ItemValue {
                    value: 300,
                    unit: Currency::Gold
                }),
                consumed: true
            }),
        }
        .to_5etools_spell(),
        json!({
            "v": true,
            "s": true,
            "m": {
                "text": "diamonds worth 300 gp, which the spell consumes",
                "cost": 30000,
                "consume": true
            }
        })
    );
}

#[test]
fn spell_test() {
    let revivify = Spell {
        source: Source {
            source_book: "PHB",
            page: 272,
        },
        name: "Revivify".to_owned(),
        level: 3,
        school: MagicSchool::Necromancy,
        casting_time: CastingTime {
            number: 1,
            unit: CastingTimeUnit::Action(ActionType::Action),
        },
        ritual: false,
        duration: Duration::Instantaneous,
        range: Range::Touch,
        components: Components {
            verbal: true,
            somatic: true,
            material: Some(MaterialComponent {
                component: "diamonds worth 300 gp, which the spell consumes".to_owned(),
                value: Some(ItemValue {
                    value: 300,
                    unit: Currency::Gold,
                }),
                consumed: true,
            }),
        },
        damage_types: None,
        description: vec!["You touch a creature that has died within the last minute. That creature returns to life with 1 hit point. This spell can't return to life a creature that has died of old age, nor can it restore any missing body parts.".to_owned()],
        at_higher_levels: None,
        classes: vec![Classes::Cleric, Classes::Paladin, Classes::Artificer],
    };

    let revivify_5etools_json = json!({
        "name": "Revivify",
        "source": "PHB",
        "page": 272,
        "level": 3,
        "school": "N",
        "time": [
            {
                "number": 1,
                "unit": "action"
            }
        ],
        "range": {
            "type": "point",
            "distance": {
                "type": "touch"
            }
        },
        "components": {
            "v": true,
            "s": true,
            "m": {
                "text": "diamonds worth 300 gp, which the spell consumes",
                "cost": 30000,
                "consume": true
            }
        },
        "duration": [
            {
                "type": "instant"
            }
        ],
        "entries": [
            "You touch a creature that has died within the last minute. That creature returns to life with 1 hit point. This spell can't return to life a creature that has died of old age, nor can it restore any missing body parts."
        ],
        "classes": {
            "fromClassList": [
                {
                    "name": "Cleric",
                    "source": "PHB"
                },
                {
                    "name": "Paladin",
                    "source": "PHB"
                },
                {
                    "name": "Artificer",
                    "source": "TCE"
                }
            ],
        }
    });

    assert_eq!(revivify.to_5etools_spell(), revivify_5etools_json)
}
