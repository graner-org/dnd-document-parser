use dnd_document_parser::models::common::Source;
use serde_json::Value;
use dnd_document_parser::parsers::spells::parse_gm_binder;
use dnd_document_parser::utils::{compare::json_compare, traits::To5etools};
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
    let gmbinder_source = format!("{}/gm_binder_input.html", resource_dir,);
    let expected_source = format!("{}/gm_binder_output.json", resource_dir,);
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
        .unwrap()
        .to_owned();
    let comparison = json_compare(parsed_spell, expected_json);
    assert!(
        comparison.is_ok(),
        "Parsed does not match expected:\n{comparison:#?}"
    )
}
}
