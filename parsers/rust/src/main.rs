use std::fs;
use std::path::PathBuf;

use clap::Parser;
use dnd_document_parser::models::common::Source;
use dnd_document_parser::parsers::spells::parse_gm_binder;
use dnd_document_parser::utils::error::Error;
use itertools::Itertools;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(help_template = "{name} {version}
{author}
{about-section}
{usage-heading} {usage}

{all-args} {tab}")]
struct Cli {
    /// Path to file or directory that will be parsed
    input_path: PathBuf,
}

// TODO: Better error messages
fn find_html_files(path: PathBuf) -> Result<Vec<PathBuf>, Error> {
    if path.is_file() {
        return match path.extension().map(|ext| ext.to_str()).flatten() {
            Some("html") => Ok(vec![path]),
            _ => Ok(vec![]),
        };
    }
    fs::read_dir(path)?
        .map_ok(|path| find_html_files(path.path()))
        .flatten()
        .fold_ok(vec![], |acc, paths| [acc, paths].concat())
}

fn main() -> Result<(), Error> {
    let args = Cli::parse();
    let sources = find_html_files(args.input_path)?;
    let source_book = Source {
        source_book: "book",
        page: 0,
    };
    for source_path in sources {
        let source = fs::read_to_string(source_path.clone())?;
        let parsed_spells = source
            .split("\n\n")
            .flat_map(|spell_str| {
                match parse_gm_binder(spell_str.to_owned(), source_book.clone()) {
                    Ok(spell) => Some(spell.name),
                    Err(Error::Parse(parse_error)) => match parse_error.parsing_step.as_str() {
                        "Name" | "School of Magic" => None,
                        _ => Some(format!("{parse_error:?}")),
                    },
                    _ => None,
                }
            })
            .collect_vec();
        println!("{source_path:?}:");
        for spell in parsed_spells.clone() {
            println!("\t{spell:?}");
        }
        println!("Spells found: {}", parsed_spells.len());
    }
    Ok(())
}
