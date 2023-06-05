use crate::models::common::{
    ActionType, Classes, DamageType, Description, RangeUnit, Source, TimeUnit,
};
use crate::models::items::{Currency, ItemValue};
use crate::models::spells::{
    CastingTime, CastingTimeUnit, Components, Duration, MagicSchool, MaterialComponent, Range,
    Spell, TargetType, TimedDuration,
};
use crate::utils::error::{Error, OutOfBoundsError, ParseError};
use itertools::Itertools;
use regex::Regex;
use std::borrow::ToOwned;
use std::convert::TryFrom;

#[cfg(test)]
mod tests;

type Name = String;
type SpellLevel = u8;
type Ritual = bool;
type MaybeDamageType = Option<Vec<DamageType>>;
type Entries = Vec<Description>;
type AtHigherLevels = Option<String>;

pub fn parse_gm_binder(source_file: String, source_book: Source) -> Result<Spell, Error> {
    let spell_groups: Vec<Vec<&str>> = split_spell_into_groups(source_file.as_str());
    let out_of_bounds_error = |index, parsing_step| OutOfBoundsError {
        array: spell_groups
            .clone()
            .into_iter()
            .map(|vec| vec.join("\n"))
            .collect_vec(),
        index,
        parsing_step,
    };
    let mut spell_groups_iter = spell_groups.iter();
    let (name, level, school, ritual) = spell_groups_iter
        .next()
        .ok_or_else(|| out_of_bounds_error(0, "First group parsing".to_owned()))
        .map(parse_first_group)??;
    let (casting_time, range, components, duration, classes) = spell_groups_iter
        .next()
        .ok_or_else(|| out_of_bounds_error(1, "Second group parsing".to_owned()))
        .map(parse_second_group)??;
    let (damage_types, description, at_higher_levels) = parse_entries(spell_groups_iter)?;
    Ok(Spell {
        source: source_book,
        name,
        level,
        school,
        casting_time,
        ritual,
        duration,
        range,
        components,
        damage_types,
        description,
        at_higher_levels,
        classes,
    })
}

fn split_spell_into_groups(spell: &str) -> Vec<Vec<&str>> {
    // Exclude lines that are empty or start with `<`, indicating an HTML tag.
    let excluder = Regex::new(r"^($|<)").unwrap();
    // String that divides groups such as name + level + school, entries, etc.
    let divider = "___";
    spell
        .split('\n')
        .filter(|line| !excluder.is_match(line))
        .group_by(|line| *line == divider)
        .into_iter()
        // Collect groups into vectors, remove divider lines.
        .filter_map(|(key, group)| if key { None } else { Some(group.collect()) })
        .collect()
}

fn strip_str(s: &&str) -> String {
    // Match everything before `:`, and any symbols after
    let symbol_regex = Regex::new(r"(.*:|[^a-zA-Z\d])+").unwrap();
    let symbols_removed = symbol_regex.replace_all(s, " ").to_lowercase();
    symbols_removed
        .strip_prefix(' ')
        .map_or(symbols_removed.clone(), ToOwned::to_owned)
}

fn parse_casting_time(casting_time_str: &str) -> Result<CastingTime, Error> {
    let mut words = casting_time_str.split(' ');
    let out_of_bounds_error = |index, parsing_step: &str| OutOfBoundsError {
        array: casting_time_str
            .split(' ')
            .map_into::<String>()
            .collect_vec(),
        index,
        parsing_step: parsing_step.to_owned(),
    };
    let number: u8 = words
        .next()
        .ok_or_else(|| out_of_bounds_error(0, "CastingTime: amount"))?
        .parse::<u8>()
        .map_err(|error| ParseError {
            string: casting_time_str.to_owned(),
            parsing_step: "CastingTime: amount".to_owned(),
            problem: Some(error.to_string()),
        })?;
    let unit = words
        .next()
        .ok_or_else(|| out_of_bounds_error(1, "CastingTime: unit"))?
        .try_into()?;
    // If the unit is a reaction, there is an associated condition.
    let unit = match unit {
        CastingTimeUnit::Action(ActionType::Reaction { condition: _ }) => {
            CastingTimeUnit::Action(ActionType::Reaction {
                condition: words.collect_vec().join(" "),
            })
        }
        _ => unit,
    };
    Ok(CastingTime { number, unit })
}

fn parse_range(range_str: &str) -> Result<Range, Error> {
    use Range::{Ranged, Self_, Special, Touch};
    let mut words = range_str.split(' ');
    // First word is range type
    match words.next() {
        Some("touch") => Ok(Touch),
        Some("special") => Ok(Special),
        Some("self") => match words.next() {
            // {range} {unit} {radius|cone}
            Some(number) => Ok(Ranged {
                range: number
                    .parse::<u16>()
                    .map_err(ParseError::from_intparse_error(
                        number.to_owned(),
                        "Range (self): amount".to_owned(),
                    ))?,
                unit: words
                    .next()
                    .ok_or(OutOfBoundsError {
                        array: range_str.split(' ').map_into().collect_vec(),
                        index: 2,
                        parsing_step: "Range (self): unit".to_owned(),
                    })?
                    .try_into()?,
                type_: words
                    .next()
                    .ok_or(OutOfBoundsError {
                        array: range_str.split(' ').map_into().collect_vec(),
                        index: 3,
                        parsing_step: "Range (self): type".to_owned(),
                    })?
                    .try_into()?,
            }),
            None => Ok(Self_),
        },
        Some(number) => Ok(Ranged {
            range: number
                .parse::<u16>()
                .map_err(ParseError::from_intparse_error(
                    number.to_owned(),
                    "Range (point): amount".to_owned(),
                ))?,
            unit: words
                .next()
                .ok_or(OutOfBoundsError {
                    array: range_str.split(' ').map_into().collect_vec(),
                    index: 2,
                    parsing_step: "Range (point): unit".to_owned(),
                })?
                .try_into()?,
            type_: TargetType::Point,
        }),
        None => Err(OutOfBoundsError {
            array: range_str.split(' ').map_into().collect_vec(),
            index: 0,
            parsing_step: "Range".to_owned(),
        }
        .into()),
    }
}

fn parse_components(components_str: String) -> Result<Components, Error> {
    let stripped_components_str = strip_str(&components_str.as_str());
    let components = _parse_components_helper(stripped_components_str, &components_str)?;
    if components.verbal || components.somatic || components.material.is_some() {
        Ok(components)
    } else {
        Err(ParseError {
            string: components_str,
            parsing_step: "Components".to_owned(),
            problem: Some("No components could be parsed.".to_owned()),
        }
        .into())
    }
}

fn _parse_components_helper(
    stripped_str: String,
    original_str: &String,
) -> Result<Components, Error> {
    let mut stripped_words = stripped_str.split(' ');
    match stripped_words.next() {
        Some("v") => {
            let other_components =
                _parse_components_helper(stripped_words.join(" "), original_str)?;
            Ok(Components {
                verbal: true,
                somatic: other_components.somatic,
                material: other_components.material,
            })
        }
        Some("s") => {
            let other_components =
                _parse_components_helper(stripped_words.join(" "), original_str)?;
            Ok(Components {
                verbal: false,
                somatic: true,
                material: other_components.material,
            })
        }
        Some("m") => {
            let words_vec = original_str
                .split(' ')
                .skip_while(|word| word.to_lowercase() != "m")
                .skip(1)
                .map(|word| word.replace(['(', ')'], ""))
                .collect_vec();
            let component = words_vec.join(" ");
            let consumed = words_vec
                .iter()
                .any(|word| word.to_lowercase().starts_with("consume"));
            let value = if words_vec.iter().contains(&"worth".to_owned()) {
                let mut words = stripped_str.split(' ');
                let value = words
                    .find(|word| word.parse::<u32>().is_ok())
                    .ok_or(ParseError {
                        string: stripped_str.clone(),
                        parsing_step: "Components (material): value".to_owned(),
                        problem: Some("No word found that parses as u32.".to_owned()),
                    })?
                    .parse::<u32>()
                    .unwrap();
                let unit = words
                    .next()
                    .ok_or({
                        let array: Vec<String> = stripped_str.split(' ').map_into().collect_vec();
                        OutOfBoundsError {
                            index: array
                                .clone()
                                .into_iter()
                                .find_position(|word| word.parse::<u32>().is_ok())
                                .map(|(i, _)| i as u32 + 1)
                                .expect("A parsable u32 was found above."),
                            array,
                            parsing_step: "Components (material): currency".to_owned(),
                        }
                    })?
                    .try_into()?;
                Some(ItemValue { value, unit })
            } else {
                None
            };
            // Parse cost and consumption
            Ok(Components {
                verbal: false,
                somatic: false,
                material: Some(MaterialComponent {
                    component,
                    value,
                    consumed,
                }),
            })
        }
        _ => Ok(Components {
            verbal: false,
            somatic: false,
            material: None,
        }),
    }
}

fn parse_duration(duration_str: String) -> Result<Duration, Error> {
    let mut words = duration_str.split(' ');
    let out_of_bounds_error = |parsing_step: &str| {
        let array: Vec<String> = duration_str.split(' ').map_into().collect_vec();
        OutOfBoundsError {
            index: array
                .clone()
                .into_iter()
                .find_position(|word| word.parse::<u8>().is_ok())
                .map(|(i, _)| i as u32 + 1)
                .expect("A parsable u8 was found above."),
            array,
            parsing_step: parsing_step.to_owned(),
        }
    };
    match words.next() {
        Some("instantaneous") => Ok(Duration::Instantaneous),
        Some("concentration") => {
            // Skip all words up to a number.
            let number = words
                .find_map(|word| word.parse::<u8>().ok())
                .ok_or(ParseError {
                    string: duration_str.clone(),
                    parsing_step: "Duration (concentration): amount".to_owned(),
                    problem: Some(
                        "No number after 'concentration' can be parsed as u8.".to_owned(),
                    ),
                })?;
            let unit = words
                .next()
                .ok_or_else(|| out_of_bounds_error("Duration (concentration): unit"))?
                .try_into()?;
            Ok(Duration::Timed(TimedDuration {
                number,
                unit,
                concentration: true,
            }))
        }
        Some(word) => {
            let number = word.parse::<u8>().map_or_else(
                |_| {
                    words
                        .find_map(|word| word.parse::<u8>().ok())
                        .ok_or(ParseError {
                            string: duration_str.clone(),
                            parsing_step: "Duration (Timed): amount".to_owned(),
                            problem: Some("No number can be parsed as u8.".to_owned()),
                        })
                },
                Ok,
            )?;
            let unit = words
                .next()
                .ok_or_else(|| out_of_bounds_error("Duration (Timed): unit"))?
                .try_into()?;
            Ok(Duration::Timed(TimedDuration {
                number,
                unit,
                concentration: false,
            }))
        }
        None => Err(ParseError {
            string: duration_str,
            parsing_step: "Duration".to_owned(),
            problem: Some("Nothing parsable found.".to_owned()),
        }
        .into()),
    }
}

fn parse_classes(classes_str: String) -> Result<Vec<Classes>, Error> {
    let found_classes = classes_str
        .split(' ')
        .flat_map(Classes::try_from)
        .collect_vec();
    if found_classes.is_empty() {
        Err(ParseError {
            string: classes_str,
            parsing_step: "Classes".to_owned(),
            problem: Some("No classes could be parsed.".to_owned()),
        }
        .into())
    } else {
        Ok(found_classes)
    }
}

fn parse_entries<'a, I>(all_entries: I) -> Result<(MaybeDamageType, Entries, AtHigherLevels), Error>
where
    I: Iterator<Item = &'a Vec<&'a str>>,
{
    // Normal entries don't start with **, but "at higher level"-entries do
    let entries_by_type = all_entries
        .flatten()
        .group_by(|entry| entry.starts_with("**"));
    let main_entries: Vec<String> = entries_by_type
        .into_iter()
        .map(
            // Collapse normal entries into one group.
            |(key, entry)| (key, entry.copied().map(ToOwned::to_owned).collect()),
        )
        .find(|(key, _)| !*key) // Get the first group (which we just collapsed)
        .ok_or_else(|| ParseError {
            string: entries_by_type
                .into_iter()
                .map(|(_, entry)| {
                    entry
                        .copied()
                        .map(ToOwned::to_owned)
                        .collect_vec()
                        .join("\n")
                })
                .collect_vec()
                .join("\n"),
            parsing_step: "Entries: main entries".to_owned(),
            problem: Some("No entries found.".to_owned()),
        })?
        .1;
    let damage_types = main_entries
        .clone()
        .into_iter()
        .flat_map(|entry| {
            entry
                .split(' ')
                .flat_map(DamageType::try_from)
                .collect_vec()
        })
        .unique()
        .collect_vec();
    let damage_types = if damage_types.is_empty() {
        None
    } else {
        Some(damage_types)
    };
    let main_entries: Vec<Description> = main_entries
        .iter()
        .group_by(|entry| entry.starts_with("- "))
        .into_iter()
        .flat_map(|(is_list_item, entry)| {
            if is_list_item {
                vec![Description::List(
                    entry
                        // We know from the grouping that the line starts with "- "
                        .flat_map(|line| line.strip_prefix("- "))
                        .map(str::to_owned)
                        .map(Description::Entry)
                        .collect(),
                )]
            } else {
                entry.cloned().map(Description::Entry).collect()
            }
        })
        .collect();
    // Coerce into needed format.
    let at_higher_levels = entries_by_type
        .into_iter()
        .next()
        .and_then(|(_, group)| group.into_iter().copied().next())
        .map(|entry| {
            entry
                .split(' ')
                .filter(|group| !group.is_empty()) // Collapse multiple whitespaces
                // Drop the starting 3 words "**At higher levels.**"
                .dropping(3)
                .join(" ")
        });
    Ok((damage_types, main_entries, at_higher_levels))
}

fn parse_second_group(
    #[allow(clippy::ptr_arg)] group: &Vec<&str>,
) -> Result<(CastingTime, Range, Components, Duration, Vec<Classes>), Error> {
    let group_stripped = group.iter().map(strip_str).collect_vec();
    let out_of_bounds_error = |index, parsing_step: &str| OutOfBoundsError {
        array: group_stripped.clone(),
        index,
        parsing_step: parsing_step.to_owned(),
    };
    let casting_time: CastingTime = group_stripped
        .get(0)
        .ok_or_else(|| out_of_bounds_error(0, "CastingTime"))
        .map(String::as_str)
        .map(parse_casting_time)??;
    let range = group_stripped
        .get(1)
        .ok_or_else(|| out_of_bounds_error(1, "Range"))
        .map(String::as_str)
        .map(parse_range)??;
    let components = group
        .get(2)
        .ok_or_else(|| out_of_bounds_error(2, "Components"))
        .map(|s| parse_components(s.to_owned().to_owned()))??;
    let duration = group_stripped
        .get(3)
        .ok_or_else(|| out_of_bounds_error(3, "Duration"))
        .map(|s| parse_duration(s.clone()))??;
    let classes = group_stripped
        .get(4)
        .ok_or_else(|| out_of_bounds_error(4, "Classes"))
        .map(|s| parse_classes(s.clone()))??;
    Ok((casting_time, range, components, duration, classes))
}

fn parse_first_group(
    #[allow(clippy::ptr_arg)] group: &Vec<&str>,
) -> Result<(Name, SpellLevel, MagicSchool, Ritual), Error> {
    fn clean_name(raw_name: &&str) -> String {
        raw_name.replace("#### ", "")
    }
    fn char_is_level(c: char) -> Option<SpellLevel> {
        c.to_digit(10).map(|level| level as u8)
    }
    // The name is the first line of the group.
    let name = group
        .first()
        .ok_or(OutOfBoundsError {
            array: group.iter().map(|s| s.to_owned().to_owned()).collect_vec(),
            index: 0,
            parsing_step: "Name".to_owned(),
        })
        .map(clean_name)?;
    // The second line contains spell level and school, as well as whether the spell is a ritual.
    let level_and_school = strip_str(&group.get(1).ok_or(OutOfBoundsError {
        array: group.iter().map(|s| s.to_owned().to_owned()).collect_vec(),
        index: 1,
        parsing_step: "Level and School".to_owned(),
    })?);

    let school: MagicSchool = level_and_school
        .split(' ')
        .flat_map(MagicSchool::try_from)
        .next()
        .ok_or(ParseError {
            string: level_and_school.clone(),
            parsing_step: "School of Magic".to_owned(),
            problem: None,
        })?;
    let level: SpellLevel = level_and_school
        .chars()
        .find_map(char_is_level)
        .unwrap_or(0);
    let ritual: Ritual = level_and_school.split(' ').contains(&"ritual");
    Ok((name, level, school, ritual))
}

impl TryFrom<&str> for MagicSchool {
    type Error = ParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use MagicSchool::{
            Abjuration, Conjuration, Divination, Enchantment, Evocation, Illusion, Necromancy,
            Transmutation,
        };
        match value.to_lowercase().as_str() {
            "abjuration" => Ok(Abjuration),
            "conjuration" => Ok(Conjuration),
            "divination" => Ok(Divination),
            "enchantment" => Ok(Enchantment),
            "evocation" => Ok(Evocation),
            "illusion" => Ok(Illusion),
            "necromancy" => Ok(Necromancy),
            "transmutation" => Ok(Transmutation),
            _ => Err(ParseError {
                string: value.to_owned(),
                parsing_step: "MagicSchool".to_owned(),
                problem: None,
            }),
        }
    }
}

impl TryFrom<&str> for TimeUnit {
    type Error = ParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use TimeUnit::{Day, Hour, Minute, Round, Year};
        match value.to_lowercase().as_str() {
            "rounds" | "round" => Ok(Round),
            "minute" | "minutes" => Ok(Minute),
            "hour" | "hours" => Ok(Hour),
            "day" | "days" => Ok(Day),
            "year" | "years" => Ok(Year),
            _ => Err(ParseError {
                string: value.to_owned(),
                parsing_step: "TimeUnit".to_owned(),
                problem: None,
            }),
        }
    }
}

impl TryFrom<&str> for ActionType {
    type Error = ParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use ActionType::{Action, BonusAction, Reaction};
        match value.to_lowercase().as_str() {
            "bonus" | "bonus action" => Ok(BonusAction),
            "action" => Ok(Action),
            "reaction" => Ok(Reaction {
                condition: String::new(),
            }),
            _ => Err(ParseError {
                string: value.to_owned(),
                parsing_step: "ActionType".to_owned(),
                problem: None,
            }),
        }
    }
}

impl TryFrom<&str> for CastingTimeUnit {
    type Error = ParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use CastingTimeUnit::{Action, Time};
        let maybe_action = value.try_into().map(Action);
        maybe_action
            .or_else(|_| value.try_into().map(Time))
            .map_err(|error| ParseError {
                string: error.string,
                parsing_step: "CastingTimeUnit".to_owned(),
                problem: Some("Neither ActionType nor TimeUnit".to_owned()),
            })
    }
}

impl TryFrom<&str> for RangeUnit {
    type Error = ParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use RangeUnit::{Feet, Miles};
        match value.to_lowercase().as_str() {
            "foot" | "feet" => Ok(Feet),
            "mile" | "miles" => Ok(Miles),
            _ => Err(ParseError {
                string: value.to_owned(),
                parsing_step: "RangeUnit".to_owned(),
                problem: None,
            }),
        }
    }
}

impl TryFrom<&str> for TargetType {
    type Error = ParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use TargetType::{Cone, Point, Radius};
        match value.to_lowercase().as_str() {
            "point" => Ok(Point),
            "radius" => Ok(Radius),
            "cone" => Ok(Cone),
            _ => Err(ParseError {
                string: value.to_owned(),
                parsing_step: "TargetType".to_owned(),
                problem: None,
            }),
        }
    }
}

impl TryFrom<&str> for Currency {
    type Error = ParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use Currency::{Copper, Electrum, Gold, Platinum, Silver};
        match value.to_lowercase().as_str() {
            "cp" | "copper" => Ok(Copper),
            "sp" | "silver" => Ok(Silver),
            "ep" | "electrum" => Ok(Electrum),
            "gp" | "gold" => Ok(Gold),
            "pp" | "platinum" => Ok(Platinum),
            _ => Err(ParseError {
                string: value.to_owned(),
                parsing_step: "Currency".to_owned(),
                problem: None,
            }),
        }
    }
}

impl TryFrom<&str> for Classes {
    type Error = ParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use Classes::{
            Artificer, Barbarian, Bard, Cleric, Druid, Fighter, Monk, Paladin, Ranger, Rogue,
            Sorcerer, Warlock, Wizard,
        };
        match value.to_lowercase().as_str() {
            "artificer" => Ok(Artificer),
            "barbarian" => Ok(Barbarian),
            "bard" => Ok(Bard),
            "cleric" => Ok(Cleric),
            "druid" => Ok(Druid),
            "fighter" => Ok(Fighter),
            "monk" => Ok(Monk),
            "paladin" => Ok(Paladin),
            "ranger" => Ok(Ranger),
            "rogue" => Ok(Rogue),
            "sorcerer" => Ok(Sorcerer),
            "warlock" => Ok(Warlock),
            "wizard" => Ok(Wizard),
            _ => Err(ParseError {
                string: value.to_owned(),
                parsing_step: "Classes".to_owned(),
                problem: None,
            }),
        }
    }
}

impl TryFrom<&str> for DamageType {
    type Error = ParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use DamageType::{
            Acid, Bludgeoning, Cold, Fire, Force, Lightning, Necrotic, Piercing, Poison, Psychic,
            Radiant, Slashing, Thunder,
        };
        match value.to_lowercase().as_str() {
            "acid" => Ok(Acid),
            "bludgeoning" => Ok(Bludgeoning),
            "cold" => Ok(Cold),
            "fire" => Ok(Fire),
            "force" => Ok(Force),
            "lightning" => Ok(Lightning),
            "necrotic" => Ok(Necrotic),
            "piercing" => Ok(Piercing),
            "poison" => Ok(Poison),
            "psychic" => Ok(Psychic),
            "radiant" => Ok(Radiant),
            "slashing" => Ok(Slashing),
            "thunder" => Ok(Thunder),
            _ => Err(ParseError {
                string: value.to_owned(),
                parsing_step: "DamageType".to_owned(),
                problem: None,
            }),
        }
    }
}
