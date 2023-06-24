use itertools::Itertools;

#[cfg(test)]
mod tests;

/// Extract stat blocks from a document containing multiple stat blocks.
///
/// * `document` - The document to extract stat blocks from
/// Returns: Vector of raw stat blocks.
fn extract_stat_blocks(document: String) -> Vec<Vec<String>> {
    document
        .split('\n')
        // Stat blocks always start with `>`
        .group_by(|line| line.starts_with('>'))
        .into_iter()
        .flat_map(|(is_stat_block, line_group)| {
            if is_stat_block {
                Some(
                    line_group
                        // Remove `>` and potential leading spaces. Equivalent to s/^>\s*//
                        .map(|line| line.replacen('>', "", 1).trim().to_string())
                        .collect_vec(),
                )
            } else {
                None
            }
        })
        .collect_vec()
}
