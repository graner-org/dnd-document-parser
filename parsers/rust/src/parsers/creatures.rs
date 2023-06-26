use itertools::Itertools;

use crate::{
    models::{
        common::{Alignment, AlignmentAxis, AlignmentAxisMoral, AlignmentAxisOrder},
        creatures::{ArmorClass, CreatureType, CreatureTypeEnum, HitPoints, Size, Speed},
    },
    utils::error::{Error, OutOfBoundsError, ParseError, Result},
};

type Name = String;

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
