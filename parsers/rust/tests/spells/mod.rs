use dnd_document_parser::models::common::Source;
use dnd_document_parser::{parsers::spells::parse_gm_binder, traits::To5etools};
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

fn read_json_file(filename: String) -> Value {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}

#[test]
fn gmbinder_integration_test() {
    let resource_dir = format!("{}/resources/test/spells", env!("CARGO_MANIFEST_DIR"));
    let gmbinder_source = format!("{}/gm_binder_input.html", resource_dir,);
    let expected_source = format!("{}/gm_binder_output.json", resource_dir,);
    let source_book = Source {
        source_book: "test-source",
        page: 0,
    };
    let parsed_spell = parse_gm_binder(gmbinder_source, source_book)
        .map(|spell| spell.to_5etools_spell())
        .unwrap();
    let expected_json = read_json_file(expected_source);
    let expected_json = expected_json.get("spell").map(|val| val.get(0)).flatten();
    assert_eq!(Some(&parsed_spell), expected_json)
}
