use crate::select::Item;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use ratatui::widgets::ListState;
use std::sync::LazyLock;

static MATCHER: LazyLock<SkimMatcherV2> = LazyLock::new(SkimMatcherV2::default);

pub(super) struct FilteredItem<'a> {
    pub(super) item: &'a Item<'a>,
    pub(super) matching_chars: Vec<usize>,
}

pub(super) fn apply_filter<'a>(
    list: &'a [Item<'a>],
    list_state: &mut ListState,
    filter: &str,
) -> Vec<FilteredItem<'a>> {
    let filtered = get_filtered(list, filter);
    if filtered.is_empty() {
        list_state.select(None)
    } else if let Some(selected) = list_state.selected() {
        let clamped = selected.clamp(0, filtered.len() - 1);
        list_state.select(Some(clamped))
    } else {
        list_state.select(Some(0))
    }
    filtered
}

pub(super) fn get_filtered<'a>(list: &'a [Item<'a>], filter: &str) -> Vec<FilteredItem<'a>> {
    list.iter()
        .map(|i| FilteredItem {
            item: i,
            matching_chars: MATCHER
                .fuzzy_indices(i.content.as_ref(), filter)
                .map(|(_, i)| i)
                .unwrap_or_default(),
        })
        .filter(|i| filter.is_empty() || !i.matching_chars.is_empty())
        .collect()
}
