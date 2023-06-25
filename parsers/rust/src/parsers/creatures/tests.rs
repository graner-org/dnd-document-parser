use std::{
    assert_eq,
    fs::File,
    io::{read_to_string, BufReader},
};

use crate::{
    models::{
        common::{Alignment, AlignmentAxis, AlignmentAxisMoral, AlignmentAxisOrder},
        creatures::{CreatureType, CreatureTypeEnum, Size},
    },
    parsers::creatures::{extract_stat_blocks, parse_first_group},
};

#[test]
fn extract_stat_blocks_test() {
    let filename = "resources/test/creatures/unit_input.md";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let document = read_to_string(reader).unwrap();
    let extracted_stat_blocks = extract_stat_blocks(document);

    assert!(
        extracted_stat_blocks.len() == 3,
        "Incorrect number of stat blocks extracted.",
    );

    let first_extracted_block = extracted_stat_blocks[0].clone();
    let expected_first_block = vec!["Unparsable entity", "With multiple lines"];
    assert_eq!(
        first_extracted_block, expected_first_block,
        "Stat block not parsed correctly.",
    )
}

#[test]
fn parse_first_group_test() {
    let first_group = vec![
        "## test".to_string(),
        "*Medium beast, unaligned*".to_string(),
    ];
    let expected_result = Ok((
        "test".to_string(),
        Size::Medium,
        CreatureType {
            main_type: CreatureTypeEnum::Beast,
            subtypes: None,
        },
        Alignment::Unaligned,
    ));

    assert_eq!(parse_first_group(first_group), expected_result);

    assert!(parse_first_group(vec![
        "Unparsable entity".to_string(),
        "With multiple lines".to_string()
    ])
    .is_err());
}

#[test]
fn creature_type() {
    assert_eq!(
        "fiend (demon)".try_into(),
        Ok(CreatureType {
            main_type: CreatureTypeEnum::Fiend,
            subtypes: Some(vec!["demon".to_string()])
        })
    );

    assert_eq!(
        "fiend/undead".try_into(),
        Ok(CreatureType {
            main_type: CreatureTypeEnum::Fiend,
            subtypes: Some(vec!["undead".to_string()])
        })
    );

    assert_eq!(
        "fiend".try_into(),
        Ok(CreatureType {
            main_type: CreatureTypeEnum::Fiend,
            subtypes: None,
        })
    );
}

#[test]
fn alignment() {
    use Alignment::{Any, OneAxis, TwoAxes, Unaligned};
    use AlignmentAxis::Order;
    use AlignmentAxisMoral::Evil;
    use AlignmentAxisOrder::Chaotic;

    assert_eq!(
        "chaotic evil".try_into(),
        Ok(TwoAxes {
            order: Chaotic,
            moral: Evil,
        })
    );

    assert_eq!(
        "any chaotic alignment".try_into(),
        Ok(OneAxis(Order(Chaotic)))
    );

    assert_eq!("any alignment".try_into(), Ok(Any));

    assert_eq!("unaligned".try_into(), Ok(Unaligned));

    assert_eq!(
        "neutral".try_into(),
        Ok(TwoAxes {
            order: AlignmentAxisOrder::Neutral,
            moral: AlignmentAxisMoral::Neutral,
        })
    )
}
