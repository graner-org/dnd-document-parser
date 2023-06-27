use std::{
    assert_eq,
    fs::File,
    io::{read_to_string, BufReader},
};

use crate::{
    models::{
        common::{Alignment, AlignmentAxis, AlignmentAxisMoral, AlignmentAxisOrder},
        creatures::{
            AbilityScores, ArmorClass, CreatureType, CreatureTypeEnum, FlySpeed, HitPoints,
            HitPointsFormula, Size, Speed,
        },
    },
    parsers::creatures::{
        extract_stat_blocks, parse_first_group, parse_second_group, parse_third_group,
    },
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
fn parse_second_group_test() {
    assert_eq!(
        parse_second_group(vec![
            "- **Armor Class** 10".to_string(),
            "- **Hit Points** 10 (1d10 + 4)".to_string(),
            "- **Speed** 30 ft.".to_string(),
        ]),
        Ok((
            ArmorClass {
                ac: 10,
                armor_type: None,
            },
            HitPoints {
                average: 10,
                formula: HitPointsFormula {
                    number_of_dice: 1,
                    die_size: 10,
                    modifier: 4,
                },
            },
            Speed {
                walk: 30,
                burrow: None,
                climb: None,
                crawl: None,
                fly: None,
                swim: None,
            }
        ))
    )
}

#[test]
fn parse_third_group_test() {
    assert_eq!(
        parse_third_group(vec![
            "|STR|DEX|CON|INT|WIS|CHA|".to_string(),
            "|:---:|:---:|:---:|:---:|:---:|:---:|".to_string(),
            "|25 (+7)|11 (+0)|21 (+5)|15 (+2)|15 (+2)|4 (-3)|".to_string(),
        ]),
        Ok(AbilityScores {
            strength: 25,
            dexterity: 11,
            constitution: 21,
            intelligence: 15,
            wisdom: 15,
            charisma: 4,
        })
    )
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

#[test]
fn armor_class() {
    assert_eq!(
        "10".try_into(),
        Ok(ArmorClass {
            ac: 10,
            armor_type: None,
        })
    );

    assert_eq!(
        "10 (Natural Armor, Shield)".try_into(),
        Ok(ArmorClass {
            ac: 10,
            armor_type: Some(vec!["Natural Armor".to_string(), "Shield".to_string()]),
        })
    );
}

#[test]
fn hit_points() {
    assert_eq!(
        "10 (1d10 + 4)".try_into(),
        Ok(HitPoints {
            average: 10,
            formula: HitPointsFormula {
                number_of_dice: 1,
                die_size: 10,
                modifier: 4,
            }
        })
    );

    assert_eq!(
        "2 (1d10-4)".try_into(),
        Ok(HitPoints {
            average: 2,
            formula: HitPointsFormula {
                number_of_dice: 1,
                die_size: 10,
                modifier: -4,
            }
        })
    );

    assert_eq!(
        "6 (1d10)".try_into(),
        Ok(HitPoints {
            average: 6,
            formula: HitPointsFormula {
                number_of_dice: 1,
                die_size: 10,
                modifier: 0,
            }
        })
    );
}

#[test]
fn speed() {
    assert_eq!(
        "30 ft.".try_into(),
        Ok(Speed {
            walk: 30,
            burrow: None,
            climb: None,
            crawl: None,
            fly: None,
            swim: None,
        })
    );

    assert_eq!(
        "30 ft., climb 30 ft., burrow 30 ft.".try_into(),
        Ok(Speed {
            walk: 30,
            burrow: Some(30),
            climb: Some(30),
            crawl: None,
            fly: None,
            swim: None,
        })
    );

    assert_eq!(
        "30 ft., burrow 30 ft., climb 30 ft., crawl 30 ft., fly 30 ft. (hover), swim 30 ft."
            .try_into(),
        Ok(Speed {
            walk: 30,
            burrow: Some(30),
            climb: Some(30),
            crawl: Some(30),
            fly: Some(FlySpeed {
                speed: 30,
                hover: true,
            }),
            swim: Some(30),
        })
    );
}
