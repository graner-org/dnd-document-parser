use std::fs;

use dnd_document_parser::models::common::Source;
use dnd_document_parser::parsers::spells::*;
use dnd_document_parser::utils::traits::To5etools;

fn main() {
    let source_file = format!(
        "{}/resources/test/spells/gm_binder_input.html",
        env!("CARGO_MANIFEST_DIR")
    );
    let source_book = Source {
        source_book: "book",
        page: 0,
    };
    let spell = fs::read_to_string(source_file.clone())
        .unwrap_or_else(|_| panic!("Failed to read {source_file}"));
    let parsed_spell = parse_gm_binder(spell, source_book).map(|spell| spell.to_5etools_spell());
    println!("{:?}", parsed_spell);
}
