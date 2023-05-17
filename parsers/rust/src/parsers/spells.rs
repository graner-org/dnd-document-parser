use crate::models::common::*;
use crate::models::items::{Currency, ItemValue};
use crate::models::spells::*;
use itertools::Itertools;
use regex::Regex;
use std::convert::TryFrom;
use std::fs;

#[cfg(test)]
mod tests;

type Name = String;
type SpellLevel = u8;
type Ritual = bool;

pub fn parse_gm_binder(source: &str) -> Result<Spell, ()> {
    let spell = fs::read_to_string(source).expect(format!("Failed to read {source}").as_str());
    let spell_groups: Vec<Vec<&str>> = split_spell_into_groups(spell.as_str());
    let mut spell_groups_iter = spell_groups.iter();
    //TODO: Parse rituals here.
    let (name, level, school, ritual) = spell_groups_iter
        .next()
        .map(|group| parse_first_group(group).ok())
        .flatten()
        .ok_or(())?;
    let (casting_time, range, components, duration, classes) = spell_groups_iter
        .next()
        .map(|group| parse_second_group(group).ok())
        .flatten()
        .ok_or(())?;
    let (damage_types, description, at_higher_levels) =
        parse_entries(spell_groups_iter).map_err(|_| ())?;
    Ok(Spell {
        source: Source {
            source_book: "book",
            page: 0,
        },
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
        .split("\n")
        .filter(|line| !excluder.is_match(line))
        .group_by(|line| *line == divider)
        .into_iter()
        // Collect groups into vectors, remove divider lines.
        .filter_map(|(key, group)| match key {
            false => Some(group.collect()),
            _ => None,
        })
        .collect()
}

fn try_parse_word<'a, T: TryFrom<&'a str>>(word: &'a str) -> Option<T> {
    word.try_into().ok()
}

fn strip_str(s: &&str) -> String {
    // Match everything before `:`, and any symbols after
    let symbol_regex = Regex::new(r"(.*:|[^a-zA-Z\d])+").unwrap();
    let symbols_removed = symbol_regex.replace_all(s, " ").to_lowercase();
    let prefix_removed = symbols_removed
        .strip_prefix(" ")
        .map(|s| s.to_owned())
        .unwrap_or(symbols_removed);
    prefix_removed
        .strip_suffix(" ")
        .map(|s| s.to_owned())
        .unwrap_or(prefix_removed)
}

fn parse_casting_time(casting_time_str: &String) -> Result<CastingTime, ()> {
    let mut words = casting_time_str.split(" ");
    let number: u8 = words
        .next()
        .map(|word| word.parse::<u8>().map_err(|_| ()))
        .ok_or(())??;
    let unit = words
        .next()
        .map(try_parse_word::<CastingTimeUnit>)
        .ok_or(())?
        .ok_or(())?;
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

fn parse_range(range_str: &String) -> Result<Range, ()> {
    use Range::*;
    let mut words = range_str.split(" ");
    // First word is range type
    match words.next() {
        Some("touch") => Ok(Touch),
        Some("special") => Ok(Special),
        Some("self") => match words.next() {
            // {range} {unit} {radius|cone}
            Some(number) => Ok(Ranged {
                range: number.parse::<u16>().map_err(|_| ())?,
                unit: words
                    .next()
                    .map(try_parse_word::<RangeUnit>)
                    .ok_or(())?
                    .ok_or(())?,
                type_: words
                    .next()
                    .map(try_parse_word::<TargetType>)
                    .ok_or(())?
                    .ok_or(())?,
            }),
            None => Ok(Self_),
        },
        Some(number) => Ok(Ranged {
            range: number.parse::<u16>().map_err(|_| ())?,
            unit: words.next().ok_or(())?.try_into().map_err(|_| ())?,
            type_: TargetType::Point,
        }),
        None => Err(()),
    }
}

fn parse_components(components_str: String) -> Result<Components, ()> {
    let stripped_components_str = strip_str(&components_str.as_str());
    fn parse_components_helper(
        stripped_str: String,
        original_str: String,
    ) -> Result<Components, ()> {
        let mut stripped_words = stripped_str.split(" ");
        match stripped_words.next() {
            Some("v") => {
                let other_components =
                    parse_components_helper(stripped_words.join(" "), original_str)?;
                Ok(Components {
                    verbal: true,
                    somatic: other_components.somatic,
                    material: other_components.material,
                })
            }
            Some("s") => {
                let other_components =
                    parse_components_helper(stripped_words.join(" "), original_str)?;
                Ok(Components {
                    verbal: false,
                    somatic: true,
                    material: other_components.material,
                })
            }
            Some("m") => {
                let words_vec = original_str
                    .split(" ")
                    .skip_while(|word| word.to_lowercase() != "m")
                    .skip(1)
                    .map(|word| word.replace("(", "").replace(")", ""))
                    .collect_vec();
                let component = words_vec.join(" ");
                let consumed = words_vec
                    .iter()
                    .any(|word| word.to_lowercase().starts_with("consume"));
                let value = match words_vec.iter().contains(&"worth".to_owned()) {
                    true => {
                        let mut words = stripped_str.split(" ");
                        let value = words
                            .find(|word| word.parse::<u32>().is_ok())
                            .ok_or(())?
                            .parse::<u32>()
                            .unwrap();
                        let unit = words
                            .next()
                            .map(|word| word)
                            .map(try_parse_word::<Currency>)
                            .ok_or(())?
                            .ok_or(())?;
                        Some(ItemValue { value, unit })
                    }
                    false => None,
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
    parse_components_helper(stripped_components_str, components_str)
}

fn parse_duration(duration_str: String) -> Result<Duration, ()> {
    let mut words = duration_str.split(" ");
    match words.next() {
        Some("instantaneous") => Ok(Duration::Instantaneous),
        Some("concentration") => {
            // Skip all words up to a number.
            let number = words.find_map(|word| word.parse::<u8>().ok()).ok_or(())?;
            let unit = words
                .next()
                .map(try_parse_word::<TimeUnit>)
                .ok_or(())?
                .ok_or(())?;
            Ok(Duration::Timed(TimedDuration {
                number,
                unit,
                concentration: true,
            }))
        }
        Some(word) => {
            let number = word.parse::<u8>().map_err(|_| ())?;
            let unit = words
                .next()
                .map(try_parse_word::<TimeUnit>)
                .ok_or(())?
                .ok_or(())?;
            Ok(Duration::Timed(TimedDuration {
                number,
                unit,
                concentration: false,
            }))
        }
        None => Err(()),
    }
}

fn parse_classes(classes_str: String) -> Result<Vec<Classes>, ()> {
    let found_classes = classes_str
        .split(" ")
        .filter_map(try_parse_word::<Classes>)
        .collect_vec();
    if found_classes.is_empty() {
        Err(())
    } else {
        Ok(found_classes)
    }
}

fn parse_entries<'a, I>(
    all_entries: I,
) -> Result<(Option<Vec<DamageType>>, Vec<String>, Option<String>), ()>
where
    I: Iterator<Item = &'a Vec<&'a str>>,
{
    // Normal entries don't start with **, but "at higher level"-entries do
    let entries_by_type = all_entries
        .filter_map(|group| group.get(0))
        .group_by(|entry| entry.starts_with("**"));
    let main_entries = entries_by_type
        .into_iter()
        .map(
            // Collapse normal entries into one group.
            |(key, entry)| {
                (
                    key,
                    entry.cloned().map(|entry| entry.to_owned()).collect_vec(),
                )
            },
        )
        .find(|(key, _)| !*key) // Get the first group (which we just collapsed)
        .unzip() // We are not interested in the key
        .1
        .ok_or(())?;
    let damage_types = main_entries
        .clone()
        .into_iter()
        .flat_map(|entry| {
            entry
                .split(" ")
                .filter_map(try_parse_word::<DamageType>)
                .collect_vec()
        })
        .collect_vec();
    let damage_types = if damage_types.is_empty() {
        None
    } else {
        Some(damage_types)
    };
    // Coerce into needed format.
    let at_higher_levels = entries_by_type
        .into_iter()
        .next()
        .map(|(_, group)| group.into_iter().cloned().next())
        .flatten()
        .map(|entry| {
            entry
                .split(" ")
                .filter(|group| !group.is_empty()) // Collapse multiple whitespaces
                .dropping(3)
                .join(" ")
        });
    Ok((damage_types, main_entries, at_higher_levels))
}

fn parse_second_group<'a>(
    group: &Vec<&str>,
) -> Result<(CastingTime, Range, Components, Duration, Vec<Classes>), ()> {
    let group_stripped = group.iter().map(strip_str).collect_vec();
    let casting_time: CastingTime = group_stripped.get(0).map(parse_casting_time).ok_or(())??;
    let range = group_stripped.get(1).map(parse_range).ok_or(())??;
    let components = group
        .get(2)
        .map(|s| parse_components(s.to_owned().to_owned()))
        .ok_or(());
    let duration = group_stripped
        .get(3)
        .map(|s| parse_duration(s.to_owned()))
        .ok_or(())??;
    let classes = group_stripped
        .get(4)
        .map(|s| parse_classes(s.to_owned()))
        .ok_or(())??;
    Ok((casting_time, range, components??, duration, classes))
}

fn parse_first_group(group: &Vec<&str>) -> Result<(Name, SpellLevel, MagicSchool, Ritual), ()> {
    fn clean_name(raw_name: &&str) -> String {
        raw_name.replace("#### ", "")
    }
    fn char_is_level(c: char) -> Option<SpellLevel> {
        c.to_digit(10).map(|level| level as u8)
    }
    // The name is the first line of the group.
    let name = group.get(0).map(clean_name).ok_or(())?;
    // The second line contains spell level and school, as well as whether the spell is a ritual.
    let level_and_school = strip_str(&group.get(1).ok_or(())?);

    let school: MagicSchool = level_and_school
        .split(" ")
        .find_map(try_parse_word::<MagicSchool>)
        .ok_or(())?;
    let level: SpellLevel = level_and_school
        .chars()
        .find_map(char_is_level)
        .unwrap_or(0);
    let ritual: Ritual = level_and_school.split(" ").contains(&"ritual");
    Ok((name, level, school, ritual))
}

impl TryFrom<&str> for MagicSchool {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use MagicSchool::*;
        match value.to_lowercase().as_str() {
            "abjuration" => Ok(Abjuration),
            "conjuration" => Ok(Conjuration),
            "divination" => Ok(Divination),
            "enchantment" => Ok(Enchantment),
            "evocation" => Ok(Evocation),
            "illusion" => Ok(Illusion),
            "necromancy" => Ok(Necromancy),
            "transmutation" => Ok(Transmutation),
            _ => Err(()),
        }
    }
}

impl TryFrom<&str> for TimeUnit {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use TimeUnit::*;
        match value.to_lowercase().as_str() {
            "round" => Ok(Round),
            "rounds" => Ok(Round),
            "minute" => Ok(Minute),
            "minutes" => Ok(Minute),
            "hour" => Ok(Hour),
            "hours" => Ok(Hour),
            "day" => Ok(Day),
            "days" => Ok(Day),
            "year" => Ok(Year),
            "years" => Ok(Year),
            _ => Err(()),
        }
    }
}

impl TryFrom<&str> for ActionType {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use ActionType::*;
        match value.to_lowercase().as_str() {
            "bonus action" => Ok(BonusAction),
            "bonus" => Ok(BonusAction),
            "action" => Ok(Action),
            "reaction" => Ok(Reaction {
                condition: "".to_owned(),
            }),
            _ => Err(()),
        }
    }
}

impl TryFrom<&str> for CastingTimeUnit {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use CastingTimeUnit::*;
        let maybe_action = value.try_into().map(Action);
        maybe_action.or(value.try_into().map(Time))
    }
}

impl TryFrom<&str> for RangeUnit {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use RangeUnit::*;
        match value.to_lowercase().as_str() {
            "feet" => Ok(Feet),
            "foot" => Ok(Feet),
            "mile" => Ok(Miles),
            "miles" => Ok(Miles),
            _ => Err(()),
        }
    }
}

impl TryFrom<&str> for TargetType {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use TargetType::*;
        match value.to_lowercase().as_str() {
            "point" => Ok(Point),
            "radius" => Ok(Radius),
            "cone" => Ok(Cone),
            _ => Err(()),
        }
    }
}

impl TryFrom<&str> for Currency {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use Currency::*;
        match value.to_lowercase().as_str() {
            "cp" => Ok(Copper),
            "copper" => Ok(Copper),
            "sp" => Ok(Silver),
            "silver" => Ok(Silver),
            "ep" => Ok(Electrum),
            "electrum" => Ok(Electrum),
            "gp" => Ok(Gold),
            "gold" => Ok(Gold),
            "pp" => Ok(Platinum),
            "platinume" => Ok(Platinum),
            _ => Err(()),
        }
    }
}

impl TryFrom<&str> for Classes {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use Classes::*;
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
            _ => Err(()),
        }
    }
}

impl TryFrom<&str> for DamageType {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use DamageType::*;
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
            _ => Err(()),
        }
    }
}
