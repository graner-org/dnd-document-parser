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
    /// Path to file that will be written
    #[arg(short, long = "output", default_value = "output.json")]
    output_path: PathBuf,
}

// TODO: Better error messages
fn find_html_files(path: PathBuf) -> Result<Vec<PathBuf>, Error> {
    if path.is_file() {
        return match path.extension().and_then(|ext| ext.to_str()) {
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
    let parsed_spells = sources
        .into_iter()
        .map(fs::read_to_string)
        .map_ok(|source| {
            source
                // TODO: split by something smarter to allow empty lines within spells.
                .split("\n\n")
                .map(|spell_str| parse_gm_binder(spell_str.to_owned(), source_book.clone()))
                .collect_vec()
        })
        .flatten()
        .flatten()
        .filter(|spell_res| match spell_res {
            // Filter out errors that correspond to non-spells
            Err(Error::OutOfBounds(oob_error)) => !vec!["First group parsing", "Level and School"]
                .contains(&oob_error.parsing_step.as_str()),
            Err(Error::Parse(parse_error)) => {
                !(parse_error.parsing_step.starts_with("Name")
                    || parse_error.parsing_step.starts_with("School of Magic"))
            }
            _ => true,
        })
        .collect_vec();
    for spell_res in parsed_spells {
        println!("\t{:?}", spell_res.map(|spell| spell.name));
    }
    Ok(())
}
