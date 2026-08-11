use nucleo_matcher::{
    pattern::{CaseMatching, Normalization, Pattern},
    Config, Matcher, Utf32Str,
};

use crate::PaletteItem;

/// Returns matching catalog indexes ordered by fuzzy-match quality.
pub fn rank(query: &str, items: &[PaletteItem]) -> Vec<usize> {
    if query.trim().is_empty() {
        return (0..items.len()).collect();
    }

    let pattern = Pattern::parse(query, CaseMatching::Ignore, Normalization::Smart);
    let mut matcher = Matcher::new(Config::DEFAULT);
    let mut character_buffer = Vec::new();
    let mut matches = items
        .iter()
        .enumerate()
        .filter_map(|(index, item)| {
            let searchable = format!(
                "{} {} {} {}",
                item.title,
                item.description,
                item.aliases.join(" "),
                item.shortcuts.join(" ")
            );
            pattern
                .score(
                    Utf32Str::new(&searchable, &mut character_buffer),
                    &mut matcher,
                )
                .map(|score| (index, score))
        })
        .collect::<Vec<_>>();

    matches.sort_unstable_by(|(left_index, left_score), (right_index, right_score)| {
        right_score
            .cmp(left_score)
            .then_with(|| left_index.cmp(right_index))
    });
    matches.into_iter().map(|(index, _)| index).collect()
}
