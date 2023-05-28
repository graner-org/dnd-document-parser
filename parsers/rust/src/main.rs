use std::fs;
use std::path::PathBuf;

use clap::Parser;
use dnd_document_parser::models::common::{merge_json, Source};
use dnd_document_parser::models::spells::Spell;
use dnd_document_parser::parsers::spells::parse_gm_binder;
use dnd_document_parser::utils::error::Error;

use dnd_document_parser::utils::traits::To5etools;
use itertools::Itertools;
use serde_json::Value;

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
    /// Path to metadata json file
    #[arg(short, long = "meta", default_value = "meta.json")]
    meta_path: PathBuf,
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

// TODO: Encode Metadata as a proper struct, not untyped Value.
// TODO: Better error messages
fn read_meta_file(meta_path: PathBuf) -> Result<Value, Error> {
    let metadata_str = fs::read_to_string(meta_path)?;
    serde_json::from_str::<Value>(metadata_str.as_str()).map_err(Into::into)
}

fn parse_gm_binder_spells(sources: Vec<PathBuf>, source_book: Source) -> Vec<Result<Spell, Error>> {
    sources
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
        .collect_vec()
}

fn main() -> Result<(), Error> {
    let args = Cli::parse();
    let sources = find_html_files(args.input_path)?;
    let num_sources = sources.len();
    let meta_path = args.meta_path.clone();
    let meta = read_meta_file(args.meta_path)?;
    let abbrev = &meta["_meta"]["sources"][0]["abbreviation"];
    let source_book = Source {
        source_book: abbrev.as_str().unwrap(),
        page: 0,
    };
    let parsed_spells = parse_gm_binder_spells(sources, source_book);
    let parsed_spells = parsed_spells
        .iter()
        .filter_map(|maybe_spell| match maybe_spell {
            Ok(spell) => Some(spell.to_5etools_spell()),
            Err(err) => {
                eprintln!("{err:?}");
                None
            }
        })
        .collect_vec();
    let num_parsed_spells = parsed_spells.len();
    let meta_with_spells = merge_json(vec![
        meta,
        serde_json::json!({ "spell": Value::Array(parsed_spells) }),
    ]);
    let output_path = args.output_path.clone();
    if args.output_path.exists() {
        fs::remove_file(&args.output_path)?;
    }
    let output_file = fs::File::options()
        .create(true)
        .write(true)
        .open(args.output_path)?;
    serde_json::to_writer_pretty(output_file, &meta_with_spells)?;

    println!(
        "Successfully parsed {} files with a total of {} spells into {}, using metadata from {}",
        num_sources,
        num_parsed_spells,
        output_path.to_str().unwrap(),
        meta_path.to_str().unwrap(),
    );
    Ok(())
}
