use super::{parse_casting_time, parse_entries};
use crate::models::common::{ActionType, Classes, DamageType, Description, RangeUnit, TimeUnit};
use crate::models::items::{Currency, ItemValue};
use crate::models::spells::{
    CastingTime, CastingTimeUnit, Components, Duration, MaterialComponent, Range, TargetType,
    TimedDuration,
};
use crate::parsers::spells::{parse_classes, parse_components, parse_duration, parse_range};
use crate::utils::error::ParseError;

#[test]
fn casting_time_unit_parse_test() {
    use CastingTimeUnit::{Action, Time};
    use TimeUnit::Hour;
    type Res = Result<CastingTimeUnit, ParseError>;
    let action: Res = "action".try_into();
    let reaction: Res = "reaction".try_into();
    let hour: Res = "hour".try_into();
    let fail: Res = "fail".try_into();

    assert_eq!(action, Ok(Action(ActionType::Action)));
    assert_eq!(
        reaction,
        Ok(Action(ActionType::Reaction {
            condition: String::new()
        }))
    );
    assert_eq!(hour, Ok(Time(Hour)));
    assert_eq!(
        fail,
        Err(ParseError {
            string: "fail".to_owned(),
            parsing_step: "CastingTimeUnit".to_owned(),
            problem: Some("Neither ActionType nor TimeUnit".to_owned())
        })
    );
}

#[test]
fn casting_time_parse_test() {
    assert_eq!(
        parse_casting_time("1 action"),
        Ok(CastingTime {
            number: 1,
            unit: CastingTimeUnit::Action(ActionType::Action)
        }),
    );
    assert_eq!(
        parse_casting_time("10 minutes"),
        Ok(CastingTime {
            number: 10,
            unit: CastingTimeUnit::Time(TimeUnit::Minute),
        }),
    );
    assert_eq!(
        parse_casting_time("1 reaction when condition is met"),
        Ok(CastingTime {
            number: 1,
            unit: CastingTimeUnit::Action(ActionType::Reaction {
                condition: "when condition is met".to_owned()
            }),
        }),
    );
}

#[test]
fn range_parse_test() {
    assert_eq!(
        parse_range("60 feet"),
        Ok(Range::Ranged {
            type_: TargetType::Point,
            range: 60,
            unit: RangeUnit::Feet
        })
    );
    assert_eq!(
        parse_range("self 60 mile radius"),
        Ok(Range::Ranged {
            type_: TargetType::Radius,
            range: 60,
            unit: RangeUnit::Miles
        })
    );
    assert_eq!(parse_range("self"), Ok(Range::Self_));
    assert_eq!(parse_range("touch"), Ok(Range::Touch));
    assert_eq!(
        parse_range("self 10 foot cone"),
        Ok(Range::Ranged {
            type_: TargetType::Cone,
            range: 10,
            unit: RangeUnit::Feet
        })
    );
}

#[test]
fn components_parse_test() {
    assert_eq!(
        parse_components("v s".to_owned()),
        Ok(Components {
            verbal: true,
            somatic: true,
            material: None
        })
    );
    assert_eq!(
        parse_components("s".to_owned()),
        Ok(Components {
            verbal: false,
            somatic: true,
            material: None
        })
    );
    assert_eq!(
        parse_components("v".to_owned()),
        Ok(Components {
            verbal: true,
            somatic: false,
            material: None
        })
    );
    assert_eq!(
        parse_components("v s m component".to_owned()),
        Ok(Components {
            verbal: true,
            somatic: true,
            material: Some(MaterialComponent {
                component: "component".to_owned(),
                value: None,
                consumed: false
            })
        })
    );
    assert_eq!(
        parse_components("v s m component which the spell consumes".to_owned()),
        Ok(Components {
            verbal: true,
            somatic: true,
            material: Some(MaterialComponent {
                component: "component which the spell consumes".to_owned(),
                value: None,
                consumed: true
            })
        })
    );
    assert_eq!(
        parse_components("m component worth 40 pp which the spell consumes".to_owned()),
        Ok(Components {
            verbal: false,
            somatic: false,
            material: Some(MaterialComponent {
                component: "component worth 40 pp which the spell consumes".to_owned(),
                value: Some(ItemValue {
                    value: 40,
                    unit: Currency::Platinum
                }),
                consumed: true
            })
        })
    );
}

#[test]
fn parse_duration_test() {
    assert_eq!(
        parse_duration("instantaneous".to_owned()),
        Ok(Duration::Instantaneous)
    );
    assert_eq!(
        parse_duration("1 round".to_owned()),
        Ok(Duration::Timed(TimedDuration {
            number: 1,
            unit: TimeUnit::Round,
            concentration: false
        }))
    );
    assert_eq!(
        parse_duration("Up to 5 minutes".to_owned()),
        Ok(Duration::Timed(TimedDuration {
            number: 5,
            unit: TimeUnit::Minute,
            concentration: false
        }))
    );
    assert_eq!(
        parse_duration("concentration up to 10 minutes".to_owned()),
        Ok(Duration::Timed(TimedDuration {
            number: 10,
            unit: TimeUnit::Minute,
            concentration: true
        }))
    );
}

#[test]
fn parse_classes_test() {
    use Classes::{Artificer, Warlock, Wizard};
    assert_eq!(
        parse_classes("wizard warlock".to_owned()),
        Ok(vec![Wizard, Warlock])
    );
    assert_eq!(parse_classes("artificer".to_owned()), Ok(vec![Artificer]));
    assert_eq!(
        parse_classes("non_existing_class".to_owned()),
        Err(ParseError {
            string: "non_existing_class".to_owned(),
            parsing_step: "Classes".to_owned(),
            problem: Some("No classes could be parsed.".to_owned())
        }
        .into())
    );
}

#[test]
fn parse_entries_test() {
    use DamageType::{Acid, Necrotic};
    use Description::{Entry, List};
    assert_eq!(
        parse_entries(
            vec![
                vec!["entry 1"],
                vec!["entry 2"],
                vec!["**At higher levels.**  Entry 3"],
            ]
            .iter()
        ),
        Ok((
            None,
            vec![Entry("entry 1".to_owned()), Entry("entry 2".to_owned())],
            Some("Entry 3".to_owned())
        )),
    );
    assert_eq!(
        parse_entries(
            vec![
                vec!["AcId 1"],
                vec!["entry neCRotic 2"],
                vec!["**At higher levels.** Entry 3"],
            ]
            .iter()
        ),
        Ok((
            Some(vec![Acid, Necrotic]),
            vec![
                Entry("AcId 1".to_owned()),
                Entry("entry neCRotic 2".to_owned())
            ],
            Some("Entry 3".to_owned())
        )),
    );
    assert_eq!(
        parse_entries(vec![vec!["entry 1"], vec!["entry 2"],].iter()),
        Ok((
            None,
            vec![Entry("entry 1".to_owned()), Entry("entry 2".to_owned())],
            None
        )),
    );
    assert_eq!(
        parse_entries(
            vec![
                vec!["- Line 1"],
                vec!["- Line 2 acid"],
                vec!["**At higher levels.** Entry 3"],
            ]
            .iter()
        ),
        Ok((
            Some(vec![Acid]),
            vec![List(vec![
                Entry("Line 1".to_owned()),
                Entry("Line 2 acid".to_owned())
            ])],
            Some("Entry 3".to_owned()),
        ))
    );
}
