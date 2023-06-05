use crate::models::common::Description;
use crate::utils::traits::To5etools;
use serde_json::json;

#[test]
fn description_test() {
    use Description::{Entry, List};

    assert_eq!(
        Entry("Entry Line".to_owned()).to_5etools_spell(),
        json!("Entry Line")
    );

    assert_eq!(
        Entry("Entry 2d4 Line".to_owned()).to_5etools_spell(),
        json!("Entry {@damage 2d4} Line")
    );

    assert_eq!(
        Entry("Entry 20d12 Line 2d6".to_owned()).to_5etools_spell(),
        json!("Entry {@damage 20d12} Line {@damage 2d6}")
    );

    assert_eq!(
        List(vec![Entry("Line 1".to_owned()), Entry("Line 2".to_owned())]).to_5etools_spell(),
        json!({
            "type": "list",
            "items": [
                "Line 1",
                "Line 2",
            ]
        })
    );

    assert_eq!(
        List(vec![
            Entry("Line 1".to_owned()),
            List(vec![Entry("Line 2d4".to_owned())]),
        ])
        .to_5etools_spell(),
        json!({
            "type": "list",
            "items": [
                "Line 1",
                {
                    "type": "list",
                    "items": [
                        "Line {@damage 2d4}",
                    ]
                }
            ]
        })
    );
}
