use std::{
    assert_eq,
    collections::HashMap,
    fs::File,
    io::{read_to_string, BufReader},
};

use crate::{
    models::{
        common::{
            AbilityScore, Alignment, AlignmentAxis, AlignmentAxisMoral, AlignmentAxisOrder,
            DamageType, Skill, StatusCondition, ALL_DAMAGE_TYPES,
        },
        creatures::{
            AbilityScores, ArmorClass, ChallengeRating, ConditionalDamageModifier, CreatureType,
            CreatureTypeEnum, DamageModifier, DamageModifierType, FlySpeed, HitPoints,
            HitPointsFormula, Size, Speed,
        },
    },
    parsers::creatures::{
        extract_stat_blocks, parse_challenge_rating, parse_condition_immunities,
        parse_damage_modifier, parse_first_group, parse_fourth_group, parse_languages,
        parse_saving_throws, parse_second_group, parse_senses, parse_skills, parse_third_group,
        SavingThrows, Skills,
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
fn parse_fourth_group_test() {
    use AbilityScore::{Constitution, Wisdom};
    use DamageModifier::{Conditional, Unconditional};
    use DamageType::{Cold, Fire, Piercing};
    use Skill::{Athletics, Perception};
    use StatusCondition::{Charmed, Frightened};
    let (saves, skills, damres, damimm, damvul, condimm, senses, passperc, langs, cr) =
        match parse_fourth_group(vec![
            "- **Saving Throws** CON +3, WIS +2".to_string(),
            "- **Skills** Athletics +5, Perception +3".to_string(),
            "- **Damage Resistances** Piercing from non-magical attacks".to_string(),
            "- **Damage Immunities** Cold".to_string(),
            "- **Damage Vulnerabilities** Fire".to_string(),
            "- **Condition Immunities** Charmed, Frightened".to_string(),
            "- **Senses** Passive Perception 15, blindsight 60 ft.".to_string(),
            "- **Languages** Common, Giant".to_string(),
            "- **Challenge** 16 (15,000 XP)".to_string(),
        ]) {
            Ok(ret) => ret,
            Err(err) => panic!("{err:?}"),
        };

    assert_eq!(
        saves,
        Some(HashMap::from_iter(vec![
            (Constitution, 3 as i8),
            (Wisdom, 2 as i8),
        ]))
    );

    assert_eq!(
        skills,
        Some(HashMap::from_iter(vec![
            (Athletics, 5 as i8),
            (Perception, 3 as i8),
        ]))
    );

    assert_eq!(
        damres,
        Some(vec![Conditional(ConditionalDamageModifier {
            modifier_type: DamageModifierType::Resistance,
            damage_types: vec![Piercing],
            condition: "from non-magical attacks".to_string(),
        })]),
    );

    assert_eq!(damimm, Some(vec![Unconditional(Cold)]));

    assert_eq!(damvul, Some(vec![Unconditional(Fire)]));

    assert_eq!(condimm, Some(vec![Charmed, Frightened]));

    assert_eq!(senses, vec!["blindsight 60 ft.".to_string()]);

    assert_eq!(passperc, 15);

    assert_eq!(langs, vec!["Common".to_string(), "Giant".to_string()]);

    assert_eq!(cr, ChallengeRating::WholeNumber(16));

    let (saves, skills, damres, damimm, damvul, condimm, senses, passperc, langs, cr) =
        match parse_fourth_group(vec![
            "- **Damage Resistances** Piercing from non-magical attacks".to_string(),
            "- **Damage Vulnerabilities** Fire".to_string(),
            "- **Senses** Passive Perception 15, blindsight 60 ft.".to_string(),
            "- **Languages** Common, Giant".to_string(),
            "- **Challenge** 16 (15,000 XP)".to_string(),
        ]) {
            Ok(ret) => ret,
            Err(err) => panic!("{err:?}"),
        };

    assert_eq!(saves, None);

    assert_eq!(skills, None);

    assert_eq!(
        damres,
        Some(vec![Conditional(ConditionalDamageModifier {
            modifier_type: DamageModifierType::Resistance,
            damage_types: vec![Piercing],
            condition: "from non-magical attacks".to_string(),
        })]),
    );

    assert_eq!(damimm, None);

    assert_eq!(damvul, Some(vec![Unconditional(Fire)]));

    assert_eq!(condimm, None);

    assert_eq!(senses, vec!["blindsight 60 ft.".to_string()]);

    assert_eq!(passperc, 15);

    assert_eq!(langs, vec!["Common".to_string(), "Giant".to_string()]);

    assert_eq!(cr, ChallengeRating::WholeNumber(16));
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

#[test]
fn saving_throws() {
    use AbilityScore::{Charisma, Strength};
    assert_eq!(
        parse_saving_throws("STR +3, CHA -2"),
        Ok(HashMap::from([(Strength, 3), (Charisma, -2)]))
    );
}

#[test]
fn skills() {
    use Skill::{Athletics, Perception};
    assert_eq!(
        parse_skills("Athletics +3, Perception -2"),
        Ok(HashMap::from([(Athletics, 3), (Perception, -2)]))
    );
}

#[test]
fn damage_modifier() {
    use DamageModifier::{Conditional, Unconditional};
    use DamageModifierType::{Resistance, Vulnerability};
    use DamageType::{Acid, Cold, Fire};

    assert_eq!(
        parse_damage_modifier(Resistance, "Fire, Cold"),
        Ok(vec![Unconditional(Fire), Unconditional(Cold)]),
    );

    assert_eq!(
        parse_damage_modifier(
            Vulnerability,
            "Fire; Cold, and Acid from non-magical attacks"
        ),
        Ok(vec![
            Unconditional(Fire),
            Conditional(ConditionalDamageModifier {
                modifier_type: Vulnerability,
                damage_types: vec![Cold, Acid],
                condition: "from non-magical attacks".to_string(),
            })
        ])
    );

    assert_eq!(
        parse_damage_modifier(Resistance, "Attacks made with disadvantage"),
        Ok(vec![Conditional(ConditionalDamageModifier {
            modifier_type: Resistance,
            damage_types: ALL_DAMAGE_TYPES.into(),
            condition: "attacks made with disadvantage".to_string(),
        })])
    )
}

#[test]
fn condition_immunities() {
    use StatusCondition::{Charmed, Frightened};
    assert_eq!(
        parse_condition_immunities("Charmed, frightened"),
        Ok(vec![Charmed, Frightened])
    )
}

#[test]
fn senses() {
    assert_eq!(
        parse_senses("Darkvision 60 ft., Passive Perception 17"),
        Ok((17, vec!["Darkvision 60 ft.".to_string()]))
    )
}

#[test]
fn languages() {
    assert_eq!(
        parse_languages("Common, Auran"),
        Ok(vec!["Common".to_string(), "Auran".to_string()])
    )
}

#[test]
fn challenge_rating() {
    use ChallengeRating::{Quarter, WholeNumber};
    assert_eq!(parse_challenge_rating("11 (7,200 XP)"), Ok(WholeNumber(11)));
    assert_eq!(parse_challenge_rating("1/4 (400 XP)"), Ok(Quarter));
}
