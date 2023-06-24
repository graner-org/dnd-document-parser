use std::{
    assert_eq,
    fs::File,
    io::{read_to_string, BufReader},
};

use crate::parsers::creatures::extract_stat_blocks;

#[test]
fn extract_stat_blocks_test() {
    let filename = "resources/test/creatures/unit_input.md";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let document = read_to_string(reader).unwrap();
    let extracted_stat_blocks = extract_stat_blocks(document);

    assert!(
        extracted_stat_blocks.len() == 3,
        "Incorrect number of stat blocks extracted.",
    );

    let first_extracted_block = extracted_stat_blocks[0].clone();
    let expected_first_block = vec!["Unparsable entity", "With multiple lines"];
    assert_eq!(
        first_extracted_block, expected_first_block,
        "Stat block not parsed correctly.",
    )
}
