use crate::models::spells::*;
use itertools::Itertools;
use regex::Regex;
use std::convert::TryFrom;
use std::fs;

type Name = String;
type SpellLevel = u8;

pub fn parse_gm_binder(source: &str) -> Spell {
    let spell = fs::read_to_string(source).expect(format!("Failed to read {source}").as_str());
    let spell_groups: Vec<Vec<&str>> = split_spell_into_groups(spell.as_str());
    println!("{spell_groups:?}");
    let mut spell_groups_iter = spell_groups.iter();
    let (name, level, school) = spell_groups_iter
        .next()
        .map(|group| parse_first_group(group).ok())
        .unwrap()
        .unwrap();
    println!("{:?}", (name, level, school));
    println!("{:?}", spell_groups_iter.next());
    todo!()
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

fn parse_first_group(group: &Vec<&str>) -> Result<(Name, SpellLevel, MagicSchool), ()> {
    fn clean_name(raw_name: &&str) -> String {
        raw_name.replace("#### ", "")
    }
    fn word_is_school(word: &str) -> Option<MagicSchool> {
        word.try_into().ok()
    }
    fn char_is_level(c: char) -> Option<SpellLevel> {
        c.to_digit(10).map(|level| level as u8)
    }
    // The name is the first line of the group.
    let name = group.get(0).map(clean_name).ok_or(())?;
    // The second line contains spell level and school.
    let level_and_school = group.get(1).ok_or(())?.replace("*", "");

    let school: MagicSchool = level_and_school
        .split(" ")
        .find_map(word_is_school)
        .ok_or(())?;
    let level: SpellLevel = level_and_school
        .chars()
        .find_map(char_is_level)
        .unwrap_or(0);
    Ok((name, level, school))
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
