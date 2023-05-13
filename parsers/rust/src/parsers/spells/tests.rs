use super::parse_casting_time;
use crate::models::common::{ActionType, RangeUnit, TimeUnit};
use crate::models::spells::{CastingTime, CastingTimeUnit, Range, TargetType};
use crate::parsers::spells::parse_range;

#[test]
fn casting_time_unit_parse_test() {
    use CastingTimeUnit::*;
    use TimeUnit::Hour;
    type Res = Result<CastingTimeUnit, ()>;
    let action: Res = "action".try_into();
    let reaction: Res = "reaction".try_into();
    let hour: Res = "hour".try_into();
    let fail: Res = "fail".try_into();

    assert_eq!(action, Ok(Action(ActionType::Action)));
    assert_eq!(
        reaction,
        Ok(Action(ActionType::Reaction {
            condition: "".to_owned()
        }))
    );
    assert_eq!(hour, Ok(Time(Hour)));
    assert_eq!(fail, Err(()));
}

#[test]
fn casting_time_parse_test() {
    assert_eq!(
        parse_casting_time(&"1 action".to_owned()),
        Ok(CastingTime {
            number: 1,
            unit: CastingTimeUnit::Action(ActionType::Action)
        }),
    );
    assert_eq!(
        parse_casting_time(&"10 minutes".to_owned()),
        Ok(CastingTime {
            number: 10,
            unit: CastingTimeUnit::Time(TimeUnit::Minute),
        }),
    );
    assert_eq!(
        parse_casting_time(&"1 reaction when condition is met".to_owned()),
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
        parse_range(&"60 feet".to_owned()),
        Ok(Range::Ranged {
            type_: TargetType::Point,
            range: 60,
            unit: RangeUnit::Feet
        })
    );
    assert_eq!(
        parse_range(&"self 60 mile radius".to_owned()),
        Ok(Range::Ranged {
            type_: TargetType::Radius,
            range: 60,
            unit: RangeUnit::Miles
        })
    );
    assert_eq!(parse_range(&"self".to_owned()), Ok(Range::Self_));
    assert_eq!(parse_range(&"touch".to_owned()), Ok(Range::Touch));
    assert_eq!(
        parse_range(&"self 10 foot cone".to_owned()),
        Ok(Range::Ranged {
            type_: TargetType::Cone,
            range: 10,
            unit: RangeUnit::Feet
        })
    );
}
