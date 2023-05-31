use dnd_document_parser::models::common::Source;
use dnd_document_parser::parsers::spells::parse_gm_binder;
use dnd_document_parser::utils::{compare::json_compare, traits::To5etools};
use itertools::Itertools;
use serde_json::{json, Value};
use std::fs::{read_to_string, File};
use std::io::BufReader;

fn read_json_file(filename: String) -> Value {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}

#[test]
fn gmbinder_parse_single_spell() {
    let resource_dir = format!("{}/resources/test/spells", env!("CARGO_MANIFEST_DIR"));
    let gmbinder_source = format!("{resource_dir}/gm_binder_input.html",);
    let expected_source = format!("{resource_dir}/gm_binder_output.json",);
    let source_book = Source {
        source_book: "test-source",
        page: 0,
    };
    let spell = read_to_string(gmbinder_source.clone())
        .unwrap_or_else(|_| panic!("Failed to read {gmbinder_source}"));
    let parsed_spell = parse_gm_binder(spell, source_book)
        .map(|spell| spell.to_5etools_spell())
        .unwrap();
    let expected_json = read_json_file(expected_source);
    let expected_json = expected_json
        .get("spell")
        .and_then(|val| val.get(0))
        .unwrap().clone();
    let comparison = json_compare(parsed_spell, expected_json);
    assert!(
        comparison.is_ok(),
        "Parsed does not match expected:\n{comparison:#?}"
    );
}

#[test]
fn gmbinder_parse_multiple_spells() {
    let resource_dir = format!("{}/resources/test/spells", env!("CARGO_MANIFEST_DIR"));
    let gmbinder_source = format!("{resource_dir}/gm_binder_input_multiple.html",);
    let expected_source = format!("{resource_dir}/gm_binder_output_multiple.json",);
    let source_book = Source {
        source_book: "test-source",
        page: 0,
    };
    let spells = read_to_string(gmbinder_source.clone())
        .unwrap_or_else(|_| panic!("Failed to read {gmbinder_source}"));
    let parsed_spells = json!(spells
        .split("\n\n")
        .flat_map(|spell_str| parse_gm_binder(spell_str.to_owned(), source_book.clone()))
        .map(|spell| spell.to_5etools_spell())
        .collect_vec());
    let expected_json = read_json_file(expected_source);
    let expected_json = expected_json.get("spell").unwrap().clone();
    let comparison = json_compare(parsed_spells, expected_json);
    assert!(
        comparison.is_ok(),
        "Parsed does not match expected:\n{comparison:#?}"
    );
}
