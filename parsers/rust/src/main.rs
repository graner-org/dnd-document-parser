use std::path::PathBuf;

use clap::Parser;
use dnd_document_parser::models::common::Source;
use dnd_document_parser::parsers::spells::parse_gm_binder;
use itertools::Itertools;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(help_template = "{name} {version}
{author}
{about-section}
{usage-heading} {usage}

{all-args} {tab}")]
struct Cli {
    /// Path to file that will be parsed
    input_file: PathBuf,
}

fn main() {
    let args = Cli::parse();
    let source_file = args.input_file;
    let source_book = Source {
        source_book: "book",
        page: 0,
    };
    let source = std::fs::read_to_string(source_file.clone())
        .unwrap_or_else(|_| panic!("Failed to read {source_file:?}"));
    let parsed_spells = source
        .split("\n\n")
        .flat_map(|spell_str| parse_gm_binder(spell_str.to_owned(), source_book.clone()))
        .map(|spell| spell.name)
        .collect_vec();
    println!("{:?}", parsed_spells);
}
