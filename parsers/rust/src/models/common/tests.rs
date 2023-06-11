use crate::models::common::{
    Alignment, AlignmentAxis, AlignmentAxisMoral, AlignmentAxisOrder, Description, NamedEntry,
};
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
        Entry("Entry 20d12 + 10 Line 2d6 - 4".to_owned()).to_5etools_spell(),
        json!("Entry {@damage 20d12 + 10} Line {@damage 2d6 - 4}")
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

#[test]
fn alignment_test() {
    use Alignment::*;

    assert_eq!(Any.to_5etools_creature(), json!(["A"]));

    assert_eq!(
        OneAxis(AlignmentAxis::Moral(AlignmentAxisMoral::Good)).to_5etools_creature(),
        json!(["G"])
    );

    assert_eq!(
        TwoAxes {
            moral: AlignmentAxisMoral::Evil,
            order: AlignmentAxisOrder::Neutral,
        }
        .to_5etools_creature(),
        json!(["N", "E"])
    );
}

#[test]
fn named_entry() {
    assert_eq!(
        NamedEntry {
            name: "Entry".to_string(),
            entry: "Attack +7 to hit. Hit: 2d4 - 3 acid damage.".to_string()
        }
        .to_5etools_base(),
        json!({
            "name": "Entry",
            "entries": [
                "Attack {@hit 7} to hit. {@h}{@damage 2d4 - 3} acid damage."
            ]
        })
    );

    assert_eq!(
        NamedEntry {
            name: "Entry".to_string(),
            entry: "Melee Weapon Attack +7 to hit. Hit: 2d4 - 3 acid damage.".to_string()
        }
        .to_5etools_base(),
        json!({
            "name": "Entry",
            "entries": [
                "{@atk mw} {@hit 7} to hit. {@h}{@damage 2d4 - 3} acid damage."
            ]
        })
    );

    assert_eq!(
        NamedEntry {
            name: "Entry".to_string(),
            entry: "Ranged Spell Attack +7 to hit. Hit: 2d4 - 3 acid damage.".to_string()
        }
        .to_5etools_base(),
        json!({
            "name": "Entry",
            "entries": [
                "{@atk rs} {@hit 7} to hit. {@h}{@damage 2d4 - 3} acid damage."
            ]
        })
    );
}
