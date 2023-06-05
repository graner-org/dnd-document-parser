use crate::models::common::Description;
use crate::utils::traits::To5etools;
use serde_json::{json, Value};

#[test]
fn description_test() {
    use Description::{Entry, List};
    let description = Entry("Entry Line".to_owned());
    let description_result: Value = json!("Entry Line");

    assert_eq!(description.to_5etools_spell(), description_result);

    let description = List(vec![Entry("Line 1".to_owned()), Entry("Line 2".to_owned())]);
    let description_result: Value = json!({
        "type": "list",
        "items": [
            "Line 1",
            "Line 2",
        ]
    });

    assert_eq!(description.to_5etools_spell(), description_result);

    let description = List(vec![
        Entry("Line 1".to_owned()),
        List(vec![Entry("Line 2".to_owned())]),
    ]);
    let description_result: Value = json!({
        "type": "list",
        "items": [
            "Line 1",
            {
                "type": "list",
                "items": [
                    "Line 2",
                ]
            }
        ]
    });

    assert_eq!(description.to_5etools_spell(), description_result);
}
