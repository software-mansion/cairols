use std::cmp::Ordering;
use std::hash::Hash;

use lsp_types::CompletionItem;
use serde::Serialize;

// Specifies how relevant a completion is relative to the scope of the current cursor position.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Debug, Copy)]
pub enum CompletionRelevance {
    Lowest = 0,
    Low = 1,
    Medium = 2,
    High = 3,
    Highest = 4,
}

pub fn get_item_relevance(
    is_in_scope: bool,
    is_current_crate: bool,
    is_corelib: bool,
) -> CompletionRelevance {
    match (is_in_scope, is_current_crate, is_corelib) {
        (true, _, _) => CompletionRelevance::High,
        (false, true, _) => CompletionRelevance::Medium,
        (false, false, false) => CompletionRelevance::Low,
        _ => CompletionRelevance::Lowest,
    }
}

/// Internal representation of a [`CompletionItem`].
#[derive(Clone, Serialize, Debug)]
pub struct CompletionItemOrderable {
    pub item: CompletionItem,
    // Relevance tells us in what order we should be showing completions.
    pub relevance: CompletionRelevance,
}

impl PartialEq for CompletionItemOrderable {
    fn eq(&self, other: &Self) -> bool {
        self.item == other.item
    }
}

impl Eq for CompletionItemOrderable {}

impl PartialOrd for CompletionItemOrderable {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Manually implement `Ord` for `Completion`.
impl Ord for CompletionItemOrderable {
    fn cmp(&self, other: &Self) -> Ordering {
        // We only compare the `relevance` field.
        // This makes the sorting behavior explicit and independent of other fields.
        self.relevance.cmp(&other.relevance)
    }
}

#[derive(PartialEq)]
pub struct CompletionItemHashable(pub CompletionItemOrderable);

impl Eq for CompletionItemHashable {}

impl Hash for CompletionItemHashable {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        serde_json::to_string(&self.0).expect("serialization should not fail").hash(state);
    }
}
