use dnd_document_parser::models::common::Source;
use dnd_document_parser::parsers::spells::parse_gm_binder;
use itertools::Itertools;

fn main() {
    let source_file = format!(
        "{}/resources/test/spells/gm_binder_input_multiple.html",
        env!("CARGO_MANIFEST_DIR")
    );
    let source_book = Source {
        source_book: "book",
        page: 0,
    };
    let source = std::fs::read_to_string(source_file.clone())
        .unwrap_or_else(|_| panic!("Failed to read {source_file}"));
    let parsed_spells = source
        .split("\n\n")
        .flat_map(|spell_str| parse_gm_binder(spell_str.to_owned(), source_book.clone()))
        .map(|spell| spell.name)
        .collect_vec();
    println!("{:?}", parsed_spells);
}
