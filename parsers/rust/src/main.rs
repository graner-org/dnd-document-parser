use dnd_document_parser::parsers::spells::*;
use dnd_document_parser::traits::To5etools;

fn main() {
    let source = format!(
        "{}/resources/test/spells/gm_binder_input.html",
        env!("CARGO_MANIFEST_DIR")
    );
    let parsed_spell = parse_gm_binder(source.as_str()).map(|spell| spell.to_5etools_spell());
    println!("{:?}", parsed_spell);
}
