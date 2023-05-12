use crate::models::common::{ActionType, TimeUnit};
use crate::models::spells::CastingTimeUnit;

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
    assert_eq!(reaction, Ok(Action(ActionType::Reaction)));
    assert_eq!(hour, Ok(Time(Hour)));
    assert_eq!(fail, Err(()));
}
