use std::collections::HashMap;

use itertools::Itertools;

use crate::{
    models::{
        common::{
            AbilityScore, Alignment, AlignmentAxis, AlignmentAxisMoral, AlignmentAxisOrder,
            DamageType, NamedEntry, Skill, StatusCondition, ALL_DAMAGE_TYPES,
        },
        creatures::{
            AbilityScores, ArmorClass, ChallengeRating, ConditionalDamageModifier, CreatureType,
            CreatureTypeEnum, DamageModifier, DamageModifierType, FlySpeed, HitPoints,
            HitPointsFormula, Size, Speed,
        },
    },
    utils::error::{Error, OutOfBoundsError, ParseError, Result},
};

type Name = String;
type SavingThrows = HashMap<AbilityScore, i8>;
type Skills = HashMap<Skill, i8>;
type DamageResistances = Vec<DamageModifier>;
type DamageImmunities = Vec<DamageModifier>;
type DamageVulnerabilities = Vec<DamageModifier>;
type ConditionImmunities = Vec<StatusCondition>;
type Senses = Vec<String>;
type PassivePerception = u8;
type Languages = Vec<String>;
type Traits = Vec<NamedEntry>;
type Actions = Vec<NamedEntry>;
type BonusActions = Vec<NamedEntry>;
type Reactions = Vec<NamedEntry>;
type LegendaryActions = Vec<NamedEntry>;
type MythicHeader = String;
type MythicActions = Vec<NamedEntry>;

#[cfg(test)]
mod tests;

/// Extract stat blocks from a document containing multiple stat blocks.
///
/// * `document` - The document to extract stat blocks from
/// Returns: Vector of raw stat blocks.
fn extract_stat_blocks(document: String) -> Vec<Vec<String>> {
    document
        .split('\n')
        // Stat blocks always start with `>`
        .group_by(|line| line.starts_with('>'))
        .into_iter()
        .flat_map(|(is_stat_block, line_group)| {
            if is_stat_block {
                Some(
                    line_group
                        // Remove `>` and potential leading spaces. Equivalent to s/^>\s*//
                        .map(|line| line.replacen('>', "", 1).trim().to_string())
                        .collect_vec(),
                )
            } else {
                None
            }
        })
        .collect_vec()
}

fn clean_stat_block_line(line: &String) -> Result<(String, &str)> {
    line.rsplit_once("**")
        .map(|(line_type, line)| {
            (
                line_type.replacen("- **", "", 1).to_lowercase(),
                line.trim(),
            )
        })
        .ok_or_else(|| {
            ParseError {
                string: line.to_string(),
                parsing_step: "Removing `**<line type def>**`".to_string(),
                problem: None,
            }
            .into()
        })
}

fn parse_first_group(first_group: Vec<String>) -> Result<(Name, Size, CreatureType, Alignment)> {
    fn clean_name(name: &String) -> Result<Name> {
        name.strip_prefix("## ")
            .ok_or_else(|| {
                ParseError {
                    string: name.to_string(),
                    parsing_step: "Name".to_string(),
                    problem: Some("Name line does not start with `## `".to_string()),
                }
                .into()
            })
            .map(ToString::to_string)
    }

    let (name, second_line) = match &first_group[..] {
        [name_line, second_line] => Ok((
            clean_name(name_line)?,
            second_line.to_lowercase().replacen('*', "", 2),
        )),
        _ => Err(OutOfBoundsError {
            array: first_group.clone(),
            index: first_group.len() as u32,
            parsing_step: "First group".to_string(),
            problem: Some("Expected array of length 2".to_string()),
        }),
    }?;

    let (size_type, alignment) = match second_line.splitn(2, ", ").collect_vec()[..] {
        [size_type, alignment] => Ok((size_type, Alignment::try_from(alignment)?)),
        _ => Err(ParseError {
            string: second_line.clone(),
            parsing_step: "Separating size and type from alignment".to_string(),
            problem: Some("No `, ` separation found.".to_string()),
        }),
    }?;

    let (size, creature_type) = match size_type.splitn(2, ' ').collect_vec()[..] {
        [size, creature_type] => Ok((
            Size::try_from(size)?,
            CreatureType::try_from(creature_type)?,
        )),
        _ => Err(ParseError {
            string: size_type.to_string(),
            parsing_step: "Separating size and creature type".to_string(),
            problem: Some("Could not separate by ` `".to_string()),
        }),
    }?;

    Ok((name, size, creature_type, alignment))
}

fn parse_second_group(second_group: Vec<String>) -> Result<(ArmorClass, HitPoints, Speed)> {
    match &second_group[..] {
        [ac_line, hp_line, speed_line] => Ok((
            clean_stat_block_line(ac_line)?.1.try_into()?,
            clean_stat_block_line(hp_line)?.1.try_into()?,
            clean_stat_block_line(speed_line)?.1.try_into()?,
        )),
        _ => Err(OutOfBoundsError {
            array: second_group.clone(),
            index: second_group.len() as u32,
            parsing_step: "Second group parsing".to_string(),
            problem: Some("Expected array of length 3".to_string()),
        }
        .into()),
    }
}

fn parse_third_group(third_group: Vec<String>) -> Result<AbilityScores> {
    fn strip_prefix_suffix(line: &String) -> Result<&str> {
        line.strip_prefix('|')
            .ok_or_else(|| ParseError {
                string: line.clone(),
                parsing_step: "Ability scores".to_string(),
                problem: Some("No leading `|` found".to_string()),
            })?
            .strip_suffix('|')
            .ok_or_else(|| {
                ParseError {
                    string: line.clone(),
                    parsing_step: "Ability scores".to_string(),
                    problem: Some("No trailing `|` found".to_string()),
                }
                .into()
            })
    }

    match &third_group[..] {
        [abilities_line, _, scores_line] => {
            let stripped_abilites = strip_prefix_suffix(abilities_line)?.to_lowercase();
            let abilities = stripped_abilites.split('|');
            let scores = strip_prefix_suffix(scores_line)?.split('|').map(|score| {
                score
                    .split_once(' ')
                    .unzip()
                    .0
                    .ok_or_else(|| ParseError {
                        string: score.to_string(),
                        parsing_step: "Ability scores".to_string(),
                        problem: Some(
                            "Score should be formatted as `<score> (<modifier>)`".to_string(),
                        ),
                    })?
                    .parse::<u8>()
                    .map_err(|_| {
                        ParseError {
                            string: score.to_string(),
                            parsing_step: "Ability scores".to_string(),
                            problem: Some("Score could not be parsed as u8".to_string()),
                        }
                        .into()
                    })
            });

            abilities
                .zip(scores)
                // (&str, Result<u8>) -> Result<(&str, u8)>
                .map(|(ability, score_res)| score_res.map(|score| (ability, score)))
                .collect::<Result<HashMap<&str, u8>>>()?
                .try_into()
        }
        _ => Err(OutOfBoundsError {
            index: third_group.len() as u32,
            array: third_group,
            parsing_step: "Ability scores".to_string(),
            problem: Some("Expected array of length 3".to_string()),
        }
        .into()),
    }
}

fn parse_fourth_group(
    fourth_group: Vec<String>,
) -> Result<(
    Option<SavingThrows>,
    Option<Skills>,
    Option<DamageResistances>,
    Option<DamageImmunities>,
    Option<DamageVulnerabilities>,
    Option<ConditionImmunities>,
    Senses,
    PassivePerception,
    Languages,
    ChallengeRating,
)> {
    use DamageModifierType::{Immunity, Resistance, Vulnerability};
    fn parse_line<T>(
        line_type: &str,
        parser: fn(&str) -> Result<T>,
        map: &HashMap<String, &str>,
    ) -> Result<Option<T>> {
        map.get(line_type).map(|line| parser(line)).transpose()
    }

    let lines: HashMap<String, &str> = fourth_group
        .iter()
        .map(clean_stat_block_line)
        .try_collect()?;

    let (passive_perception, senses) =
        parse_line("senses", parse_senses, &lines)?.ok_or_else(|| OutOfBoundsError {
            array: fourth_group.clone(),
            index: 0,
            parsing_step: "Fourth group".to_string(),
            problem: Some("Senses line not found".to_string()),
        })?;

    Ok((
        parse_line("saving throws", parse_saving_throws, &lines)?,
        parse_line("skills", parse_skills, &lines)?,
        parse_line(
            "damage resistances",
            |line| parse_damage_modifier(Resistance, line),
            &lines,
        )?,
        parse_line(
            "damage immunities",
            |line| parse_damage_modifier(Immunity, line),
            &lines,
        )?,
        parse_line(
            "damage vulnerabilities",
            |line| parse_damage_modifier(Vulnerability, line),
            &lines,
        )?,
        parse_line("condition immunities", parse_condition_immunities, &lines)?,
        senses,
        passive_perception,
        parse_line("languages", parse_languages, &lines)?.ok_or_else(|| OutOfBoundsError {
            array: fourth_group.clone(),
            index: 0,
            parsing_step: "Fourth group".to_string(),
            problem: Some("Languages line not found".to_string()),
        })?,
        parse_line("challenge", parse_challenge_rating, &lines)?.ok_or_else(|| {
            OutOfBoundsError {
                array: fourth_group.clone(),
                index: 0,
                parsing_step: "Fourth group".to_string(),
                problem: Some("Challenge rating line not found".to_string()),
            }
        })?,
    ))
}

fn parse_fifth_group(
    fifth_group: Vec<String>,
) -> Result<(
    Option<Traits>,
    Option<Actions>,
    Option<BonusActions>,
    Option<Reactions>,
    Option<LegendaryActions>,
    Option<MythicHeader>,
    Option<MythicActions>,
)> {
    let sub_groups_str = fifth_group
        .iter()
        .filter(|line| !line.is_empty())
        .join("\n");

    let mut sub_groups = sub_groups_str.split("\n### ");

    let Some(traits_str) = sub_groups.next() else { 
        unreachable!("Split always returns at least one element")
    };

    let traits = match &traits_str
        .split("\n***")
        .map(parse_named_entry)
        .collect::<Result<Traits>>()?[..]
    {
        [] => None,
        traits_arr => Some(traits_arr.to_vec()),
    };

    let group_map: HashMap<String, &str> = sub_groups
        .map(|group| {
            group
                .split_once('\n')
                .ok_or_else(|| {
                    ParseError::new_with_problem(
                        group,
                        "Fifth group",
                        "Group does not have multiple lines",
                    )
                })
                .map(|(key, value): (&str, &str)| (key.trim().to_lowercase(), value))
        })
        .try_collect()?;

    let get_entry_type = |entry_type: &str, lines_to_skip: u8| -> Result<Option<Vec<NamedEntry>>> {
        group_map
            .get(entry_type)
            .map(|entries_str| {
                entries_str
                    .split("\n***")
                    .skip(lines_to_skip.into())
                    .map(parse_named_entry)
                    .collect::<Result<Vec<NamedEntry>>>()
            })
            .transpose()
    };

    let actions = get_entry_type("actions", 0)?;
    let bonus_actions = get_entry_type("bonus actions", 0)?;
    let reactions = get_entry_type("reactions", 0)?;
    let legendary_actions = get_entry_type("legendary actions", 1)?;
    let mythic_actions = get_entry_type("mythic actions", 1)?;
    let mythic_header = group_map.get("mythic actions").map(|entries_str| {
        entries_str
            .chars()
            .take_while(|char_| *char_ != '\n')
            .collect()
    });

    Ok((
        traits,
        actions,
        bonus_actions,
        reactions,
        legendary_actions,
        mythic_header,
        mythic_actions,
    ))
}

fn parse_saving_throws(saving_throws_line: &str) -> Result<SavingThrows> {
    saving_throws_line
        .to_lowercase()
        .split(", ")
        .map(|save| {
            if let Some((ability, save_mod)) = save.split_once(' ') {
                Ok((
                    ability.try_into()?,
                    save_mod
                        .parse::<i8>()
                        .map_err(ParseError::from_intparse_error(
                            save_mod.to_string(),
                            "Saving_throws".to_string(),
                        ))?,
                ))
            } else {
                Err(ParseError::new(save, "Saving throws").into())
            }
        })
        .collect()
}

fn parse_skills(skills_line: &str) -> Result<Skills> {
    skills_line
        .to_lowercase()
        .split(", ")
        .map(|skill| {
            if let Some((skill, skill_mod)) = skill.split_once(' ') {
                Ok((
                    skill.try_into()?,
                    skill_mod
                        .parse::<i8>()
                        .map_err(ParseError::from_intparse_error(
                            skill_mod.to_string(),
                            "Skills".to_string(),
                        ))?,
                ))
            } else {
                Err(ParseError::new(skill, "Skills").into())
            }
        })
        .collect()
}

fn parse_damage_modifier(
    modifier_type: DamageModifierType,
    damage_modifier_line: &str,
) -> Result<Vec<DamageModifier>> {
    use DamageModifier::{Conditional, Unconditional};

    let parse_conditional = |conditional: &str| -> Result<ConditionalDamageModifier> {
        let conditional_and_removed = conditional.replacen("and ", "", 1);
        let (conditional_damage_types, condition) = {
            if conditional_and_removed.contains(", ") {
                let (damage_types_str, condition_with_last_type) =
                    conditional_and_removed.rsplit_once(", ").ok_or_else(|| {
                        ParseError::new_with_problem(
                            conditional_and_removed.as_str(),
                            "Damage modifier",
                            "No `, ` found",
                        )
                    })?;

                let (last_damage_type, condition) =
                    condition_with_last_type.split_once(" ").ok_or_else(|| {
                        ParseError::new_with_problem(
                            condition_with_last_type,
                            "Damage modifier",
                            "Conditional without condition",
                        )
                    })?;

                let damage_types = damage_types_str
                    .split(", ")
                    .chain([last_damage_type])
                    .map(str::trim)
                    .map(DamageType::try_from)
                    .try_collect()?;

                Result::Ok((damage_types, condition.to_string()))
            } else {
                conditional_and_removed
                    .split_once(' ')
                    .map(|(damage_type_str, condition)| {
                        // If no damage type can be parsed, we assume that all damage types are
                        // modified, and the whole string `conditional` is the condition.
                        if let Ok(damage_type) = damage_type_str.try_into() {
                            Ok((vec![damage_type], condition.to_string()))
                        } else {
                            Ok((ALL_DAMAGE_TYPES.into(), conditional.to_string()))
                        }
                    })
                    .ok_or_else(|| {
                        ParseError::new_with_problem(
                            conditional,
                            "Damage modifier",
                            "Conditional without condition",
                        )
                    })?
            }
        }?;

        Ok(ConditionalDamageModifier {
            modifier_type: modifier_type.clone(),
            damage_types: conditional_damage_types,
            condition,
        })
    };

    match damage_modifier_line.to_lowercase().split(';').collect_vec()[..] {
        [single_modifier_type] => {
            if single_modifier_type.contains("from") || single_modifier_type.contains("attack") {
                Ok(vec![Conditional(parse_conditional(single_modifier_type)?)])
            } else {
                single_modifier_type
                    .split(", ")
                    .map(str::trim)
                    .map(DamageType::try_from)
                    .map_ok(Unconditional)
                    .try_collect()
            }
        }
        [unconditional, conditional] => {
            let unconditional_modifiers = unconditional
                .split(", ")
                .map(str::trim)
                .map(DamageType::try_from)
                .map_ok(Unconditional);

            let conditional_modifiers = parse_conditional(conditional)?;

            unconditional_modifiers
                .chain([Ok(Conditional(conditional_modifiers))])
                .try_collect()
        }
        _ => Err(ParseError::new_with_problem(
            damage_modifier_line,
            "Damage Modifier",
            "More than 2 types of modifiers",
        )
        .into()),
    }
}

fn parse_condition_immunities(condition_immunities_line: &str) -> Result<ConditionImmunities> {
    condition_immunities_line
        .to_lowercase()
        .split(", ")
        .map(StatusCondition::try_from)
        .collect()
}

fn parse_senses(senses_line: &str) -> Result<(PassivePerception, Senses)> {
    fn parse_passive_perception(passive_perception_str: &str) -> Option<PassivePerception> {
        passive_perception_str
            .to_lowercase()
            .strip_prefix("passive perception ")
            .map(|number| number.parse().ok())
            .flatten()
    }

    let mut passive_perception: Option<u8> = None;
    let mut senses = vec![];
    senses_line
        // Passive perception is usually last, so for performance reasons we start at the back.
        .rsplit(", ")
        .for_each(|sense| {
            if passive_perception.is_some() {
                senses.push(sense.to_string())
            } else {
                // Try to parse passive perception
                passive_perception = parse_passive_perception(sense);
                if passive_perception.is_none() {
                    // passive perception could not be parsed, so the sense is added to the list.
                    senses.push(sense.to_string())
                }
            }
        });

    passive_perception
        .ok_or_else(|| {
            { ParseError::new_with_problem(senses_line, "Senses", "No passive perception found") }
                .into()
        })
        .map(|passive| (passive, senses))
}

fn parse_languages(languages_line: &str) -> Result<Languages> {
    Ok(languages_line
        .split(", ")
        .map(ToString::to_string)
        .collect())
}

fn parse_challenge_rating(challenge_rating_line: &str) -> Result<ChallengeRating> {
    challenge_rating_line
        .split_once(' ')
        .unzip()
        .0
        .ok_or_else(|| {
            {
                ParseError::new_with_problem(
                    challenge_rating_line,
                    "Challenge Rating",
                    "No separating ` ` found",
                )
            }
        })?
        .try_into()
}

fn parse_named_entry(entry: &str) -> Result<NamedEntry> {
    let (name, entries) = entry
        .strip_prefix("***")
        .unwrap_or(entry)
        .split_once("*** ")
        .ok_or_else(|| {
            ParseError::new_with_problem(entry, "Named Entry", "No second `***` found")
        })?;
    let mut split_entries = entries.split("\n* ").peekable();
    let main_entry = split_entries
        .next()
        .ok_or_else(|| ParseError::new_with_problem(entries, "Named Entry", "Empty entry"))?;
    let sub_entries = if split_entries.peek().is_some() {
        split_entries
            .map(|entry_line| {
                let (name, entry) = entry_line
                    .strip_prefix("**")
                    .ok_or_else(|| {
                        ParseError::new_with_problem(
                            entry_line,
                            "Named Entry (sub-entry)",
                            "No leading `**` found",
                        )
                    })?
                    .split_once("** ")
                    .ok_or_else(|| {
                        ParseError::new_with_problem(
                            entry_line,
                            "Named Entry (sub-entry)",
                            "No second `**` found",
                        )
                    })?;
                Result::Ok(Some(NamedEntry {
                    name: name.to_string(),
                    entry: entry.to_string(),
                    sub_entries: None,
                }))
            })
            .try_collect()?
    } else {
        None
    };
    Ok(NamedEntry {
        name: name.to_string(),
        entry: main_entry.to_string(),
        sub_entries,
    })
}

impl TryFrom<&str> for Size {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self> {
        use Size::*;
        match value {
            "tiny" => Ok(Tiny),
            "small" => Ok(Small),
            "medium" => Ok(Medium),
            "large" => Ok(Large),
            "huge" => Ok(Huge),
            "gargantuan" => Ok(Gargantuan),
            _ => Err(ParseError {
                string: value.to_string(),
                parsing_step: "Size".to_string(),
                problem: None,
            }
            .into()),
        }
    }
}

impl TryFrom<&str> for CreatureTypeEnum {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self> {
        use CreatureTypeEnum::*;
        match value {
            "aberration" => Ok(Aberration),
            "beast" => Ok(Beast),
            "celestial" => Ok(Celestial),
            "construct" => Ok(Construct),
            "dragon" => Ok(Dragon),
            "elemental" => Ok(Elemental),
            "fey" => Ok(Fey),
            "fiend" => Ok(Fiend),
            "giant" => Ok(Giant),
            "humanoid" => Ok(Humanoid),
            "monstrosity" => Ok(Monstrosity),
            "ooze" => Ok(Ooze),
            "plant" => Ok(Plant),
            "undead" => Ok(Undead),
            _ => Err(ParseError {
                string: value.to_string(),
                parsing_step: "Main creature type".to_string(),
                problem: None,
            }
            .into()),
        }
    }
}

impl TryFrom<&str> for CreatureType {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self> {
        match value.replace('/', " ").splitn(2, ' ').collect_vec()[..] {
            [main_type] => Ok(CreatureType {
                main_type: main_type.try_into()?,
                subtypes: None,
            }),
            [main_type, subtypes] => Ok(CreatureType {
                main_type: main_type.try_into()?,
                subtypes: Some(
                    subtypes
                        .replace('(', "")
                        .replace(')', "")
                        .split(", ")
                        .map(ToString::to_string)
                        .collect_vec(),
                ),
            }),
            _ => Err(ParseError {
                string: value.to_string(),
                parsing_step: "Creature type".to_string(),
                problem: None,
            }
            .into()),
        }
    }
}

impl TryFrom<&str> for AlignmentAxisMoral {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self> {
        use AlignmentAxisMoral::{Evil, Good, Neutral};
        match value {
            "good" => Ok(Good),
            "neutral" => Ok(Neutral),
            "evil" => Ok(Evil),
            _ => Err(ParseError {
                string: value.to_string(),
                parsing_step: "AlignmentAxisMoral".to_string(),
                problem: None,
            }
            .into()),
        }
    }
}

impl TryFrom<&str> for AlignmentAxisOrder {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self> {
        use AlignmentAxisOrder::{Chaotic, Lawful, Neutral};
        match value {
            "lawful" => Ok(Lawful),
            "neutral" => Ok(Neutral),
            "chaotic" => Ok(Chaotic),
            _ => Err(ParseError {
                string: value.to_string(),
                parsing_step: "AlignmentAxisOrder".to_string(),
                problem: None,
            }
            .into()),
        }
    }
}

impl TryFrom<&str> for AlignmentAxis {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self> {
        use AlignmentAxis::{Moral, Order};
        value
            .try_into()
            .map(Order)
            .or_else(|_| value.try_into().map(Moral))
    }
}

impl TryFrom<&str> for Alignment {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self> {
        match value
            .splitn(3, ' ')
            .filter(|word| !word.contains("alignment"))
            .collect_vec()[..]
        {
            ["any"] => Ok(Self::Any),
            ["unaligned"] => Ok(Self::Unaligned),
            ["neutral"] => Ok(Self::TwoAxes {
                order: AlignmentAxisOrder::Neutral,
                moral: AlignmentAxisMoral::Neutral,
            }),
            ["any", single_axis] => Ok(Self::OneAxis(single_axis.try_into()?)),
            [order_axis, moral_axis] => Ok(Self::TwoAxes {
                order: order_axis.try_into()?,
                moral: moral_axis.try_into()?,
            }),
            _ => Err(ParseError {
                string: value.to_string(),
                parsing_step: "Alignment".to_string(),
                problem: None,
            }
            .into()),
        }
    }
}

impl TryFrom<&str> for ArmorClass {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self> {
        let error = |problem: &str| -> ParseError {
            ParseError {
                string: value.to_string(),
                parsing_step: "Armor class".to_string(),
                problem: Some(problem.to_string()),
            }
        };

        value.split_once(' ').map_or_else(
            // No whitespace found, assume everything is AC.
            || {
                Ok(Self {
                    ac: value.parse().map_err(|_| error("Could not parse as u8"))?,
                    armor_type: None,
                })
            },
            // Whitespace found, so there is both AC and armor type.
            |(ac, armor_types)| {
                Ok(Self {
                    ac: ac.parse().map_err(|_| error("Could not parse AC as u8"))?,
                    armor_type: armor_types
                        .strip_prefix('(')
                        .ok_or_else(|| error("No leading `(` found for armor type"))?
                        .strip_suffix(')')
                        .ok_or_else(|| error("No trailing `)` found for armor type"))
                        .map(|armor_types| {
                            Some(
                                armor_types
                                    .split(", ")
                                    .map(ToString::to_string)
                                    .collect_vec(),
                            )
                        })?,
                })
            },
        )
    }
}

impl TryFrom<&str> for HitPointsFormula {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self> {
        let error = |problem: &str| -> ParseError {
            ParseError {
                string: value.to_string(),
                parsing_step: "Hit Points Formula".to_string(),
                problem: Some(problem.to_string()),
            }
        };

        let parse_dice = |die_formula: &str| -> Result<(u8, u8)> {
            die_formula
                .split_once('d')
                .ok_or_else(|| error("No `d` found in die formula"))
                .map(|(number_of_dice, die_size)| {
                    Ok((
                        number_of_dice
                            .parse()
                            .map_err(|_| error("Number of dice could not be parsed as u8"))?,
                        die_size
                            .parse()
                            .map_err(|_| error("Die size could not be parsed as u8"))?,
                    ))
                })?
        };

        value
            .strip_prefix('(')
            .ok_or_else(|| error("No leading `(` found for hit points formula"))?
            .strip_suffix(')')
            .ok_or_else(|| error("No trailing `)` found for hit points formula"))
            .map(|formula| {
                formula
                    .split_once('+')
                    .or_else(|| formula.split_once('-'))
                    .map_or_else(
                        || {
                            let (number_of_dice, die_size) = parse_dice(formula)?;
                            Ok(Self {
                                number_of_dice,
                                die_size,
                                modifier: 0,
                            })
                        },
                        |(die_formula, modifier)| {
                            let (number_of_dice, die_size) = parse_dice(die_formula.trim())?;
                            let modifier: i16 = modifier
                                .trim()
                                .parse()
                                .map_err(|_| error("Modifier could not be parsed as u8"))?;
                            Ok(Self {
                                number_of_dice,
                                die_size,
                                modifier: if formula.contains('+') {
                                    modifier
                                } else {
                                    -modifier
                                },
                            })
                        },
                    )
            })?
    }
}

impl TryFrom<&str> for HitPoints {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self> {
        let error = |problem: &str| -> ParseError {
            ParseError {
                string: value.to_string(),
                parsing_step: "Hit Points".to_string(),
                problem: Some(problem.to_string()),
            }
        };

        value
            .split_once(' ')
            .ok_or_else(|| error("No separating ` ` found"))
            .map(|(average, formula)| {
                Ok(Self {
                    average: average
                        .parse()
                        .map_err(|_| error("Could not parse average as u16"))?,
                    formula: formula.try_into()?,
                })
            })?
    }
}

impl TryFrom<&str> for Speed {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self> {
        let cleaned_speeds = value.replace(" ft.", "");
        let mut speeds = cleaned_speeds.split(", ").map(str::trim).into_iter();
        let walk = speeds
            .next()
            .ok_or_else(|| OutOfBoundsError {
                array: value
                    .replace(" ft.", "")
                    .split(", ")
                    .map(ToString::to_string)
                    .collect_vec(),
                index: 0,
                parsing_step: "Speed".to_string(),
                problem: None,
            })?
            .parse()
            .map_err(|_| ParseError {
                string: value.to_string(),
                parsing_step: "Walking speed".to_string(),
                problem: None,
            })?;

        let mut speed_map = speeds
            .map(|speed| {
                speed.split_once(' ').ok_or_else(|| {
                    ParseError {
                        string: speed.to_string(),
                        parsing_step: "Speed types".to_string(),
                        problem: Some("No separating ` ` found".to_string()),
                    }
                    .into()
                })
            })
            .collect::<Result<HashMap<&str, &str>>>()?;

        let mut pop_speed = |speed_type: &str| -> Result<Option<u16>> {
            speed_map
                .remove(speed_type)
                .map(|speed| {
                    speed.trim().parse().map_err(|_| {
                        ParseError {
                            string: speed.to_string(),
                            parsing_step: format!("{speed_type} speed"),
                            problem: Some("Could not parse as u16".to_string()),
                        }
                        .into()
                    })
                })
                .transpose()
        };

        let fly = pop_speed("fly").map_or_else(
            |err| {
                if let Error::Parse(ParseError { string, .. }) = &err {
                    if let Some((speed, hover)) = string.split_once(' ') {
                        if hover.contains("hover") {
                            Ok(Some(FlySpeed {
                                speed: speed.parse().map_err(|_| ParseError {
                                    string: speed.to_string(),
                                    parsing_step: "flying speed".to_string(),
                                    problem: Some("Could not parse as u16".to_string()),
                                })?,
                                hover: true,
                            }))
                        } else {
                            Err(ParseError {
                                string: string.clone(),
                                parsing_step: "flying speed".to_string(),
                                problem: Some("Multiple words with no `hover`".to_string()),
                            }
                            .into())
                        }
                    } else {
                        Err(err)
                    }
                } else {
                    Err(err)
                }
            },
            |maybe_speed| {
                Ok(maybe_speed.map(|speed| FlySpeed {
                    speed,
                    hover: false,
                }))
            },
        )?;

        let speed = Self {
            walk,
            burrow: pop_speed("burrow")?,
            climb: pop_speed("climb")?,
            crawl: pop_speed("crawl")?,
            fly,
            swim: pop_speed("swim")?,
        };

        Ok(speed)
    }
}

impl TryFrom<HashMap<&str, u8>> for AbilityScores {
    type Error = Error;
    fn try_from(value: HashMap<&str, u8>) -> Result<Self> {
        let error = |ability_score: &str| -> OutOfBoundsError {
            OutOfBoundsError {
                index: value.len() as u32,
                array: value
                    .iter()
                    .map(|(key, value)| format!("{key} -> {value:?}"))
                    .collect(),
                parsing_step: "Ability scores".to_string(),
                problem: Some(format!(
                    "Ability score '{ability_score}' not found in mapping"
                )),
            }
        };
        let get_score = |ability_score: &str| -> Result<u8> {
            value
                .get(ability_score.chars().take(3).collect::<String>().as_str())
                .ok_or_else(|| error(ability_score).into())
                .map(|score| *score)
        };
        Ok(Self {
            strength: get_score("strength")?,
            dexterity: get_score("dexterity")?,
            constitution: get_score("constitution")?,
            intelligence: get_score("intelligence")?,
            wisdom: get_score("wisdom")?,
            charisma: get_score("charisma")?,
        })
    }
}

impl TryFrom<&str> for AbilityScore {
    type Error = Error;
    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        use AbilityScore::*;
        match value {
            "str" => Ok(Strength),
            "con" => Ok(Constitution),
            "dex" => Ok(Dexterity),
            "wis" => Ok(Wisdom),
            "int" => Ok(Intelligence),
            "cha" => Ok(Charisma),
            _ => Err(ParseError {
                string: value.to_string(),
                parsing_step: "Ability score".to_string(),
                problem: None,
            }
            .into()),
        }
    }
}

impl TryFrom<&str> for Skill {
    type Error = Error;
    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        use Skill::*;
        match value {
            "acrobatics" => Ok(Acrobatics),
            "animal handling" => Ok(AnimalHandling),
            "arcana" => Ok(Arcana),
            "athletics" => Ok(Athletics),
            "deception" => Ok(Deception),
            "history" => Ok(History),
            "insight" => Ok(Insight),
            "intimidation" => Ok(Intimidation),
            "investigation" => Ok(Investigation),
            "medicine" => Ok(Medicine),
            "nature" => Ok(Nature),
            "perception" => Ok(Perception),
            "performance" => Ok(Performance),
            "persuasion" => Ok(Persuasion),
            "religion" => Ok(Religion),
            "sleight of hand" => Ok(SleightOfHand),
            "stealth" => Ok(Stealth),
            "survival" => Ok(Survival),
            _ => Err(ParseError {
                string: value.to_string(),
                parsing_step: "Skill".to_string(),
                problem: None,
            }
            .into()),
        }
    }
}

impl TryFrom<&str> for StatusCondition {
    type Error = Error;
    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        use StatusCondition::*;
        match value {
            "blinded" => Ok(Blinded),
            "charmed" => Ok(Charmed),
            "deafened" => Ok(Deafened),
            "exhaustion" => Ok(Exhaustion),
            "frightened" => Ok(Frightened),
            "grappled" => Ok(Grappled),
            "incapacitated" => Ok(Incapacitated),
            "invisible" => Ok(Invisible),
            "paralyzed" => Ok(Paralyzed),
            "petrified" => Ok(Petrified),
            "poisoned" => Ok(Poisoned),
            "prone" => Ok(Prone),
            "restrained" => Ok(Restrained),
            "stunned" => Ok(Stunned),
            _ => Err(ParseError {
                string: value.to_string(),
                parsing_step: "Status Condition".to_string(),
                problem: None,
            }
            .into()),
        }
    }
}

impl TryFrom<&str> for ChallengeRating {
    type Error = Error;
    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        use ChallengeRating::*;
        match value {
            "1/8" => Ok(Eighth),
            "1/4" => Ok(Quarter),
            "1/2" => Ok(Half),
            whole => whole
                .parse()
                .map(WholeNumber)
                .map_err(ParseError::from_intparse_error(
                    value.to_string(),
                    "Challenge Rating".to_string(),
                ))
                .map_err(ParseError::into),
        }
    }
}
