use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::hash::Hash;

use attribute::attribute_completions;
use attribute::derive::derive_completions;
use cairo_lang_defs::ids::ImportableId;
use cairo_lang_diagnostics::ToOption;
use cairo_lang_filesystem::ids::CrateId;
use cairo_lang_parser::db::ParserGroup;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{SyntaxNode, TypedSyntaxNode};
use cairo_lang_syntax::node::{TypedStablePtr, ast};
use cairo_lang_utils::ordered_hash_set::OrderedHashSet;
use expr::macro_call::macro_call_completions;
use function::params::params_completions;
use function::variables::variables_completions;
use helpers::binary_expr::dot_rhs::dot_expr_rhs;
use lsp_types::{CompletionItem, CompletionParams, CompletionResponse, CompletionTriggerKind};
use path::path_suffix_completions;
use pattern::{enum_pattern_completions, struct_pattern_completions};
use self_completions::self_completions;
use serde::Serialize;
use struct_constructor::struct_constructor_completions;

use self::dot_completions::dot_completions;
use crate::ide::completion::mod_item::mod_completions;
use crate::ide::completion::use_statement::use_completions;
use crate::lang::analysis_context::AnalysisContext;
use crate::lang::db::{AnalysisDatabase, LsSemanticGroup, LsSyntaxGroup};
use crate::lang::lsp::{LsProtoGroup, ToCairo};

mod attribute;
mod dot_completions;
mod expr;
mod function;
mod helpers;
mod mod_item;
mod path;
mod pattern;
mod self_completions;
mod struct_constructor;
mod use_statement;

/// Compute completion items at a given cursor position.
pub fn complete(params: CompletionParams, db: &AnalysisDatabase) -> Option<CompletionResponse> {
    let text_document_position = params.text_document_position;
    let file_id = db.file_for_url(&text_document_position.text_document.uri)?;
    let mut position = text_document_position.position;
    let base_position_node = db.find_syntax_node_at_position(file_id, position.to_cairo())?;

    // Try searching one character to the left to account for cases where caret is at the end
    // of a text e.g. `my::path::to::ite<caret>`.
    position.character = position.character.saturating_sub(1);
    let mut node = db.find_syntax_node_at_position(file_id, position.to_cairo())?;
    let was_node_corrected = base_position_node == node;

    // There is no completions for these.
    if matches!(
        node.kind(db),
        SyntaxKind::TokenSkipped
            | SyntaxKind::TriviumSkippedNode
            | SyntaxKind::TokenSingleLineComment
    ) {
        return None;
    }

    if matches!(
        node.kind(db),
        SyntaxKind::TokenSingleLineDocComment | SyntaxKind::TokenSingleLineInnerComment
    ) {
        // TODO(#290) doc completions.
        return None;
    }

    // In case we are on eof go back to last non-trivia non-missing node.
    if node.kind(db) == SyntaxKind::SyntaxFile
        || node.ancestor_of_kind(db, SyntaxKind::TerminalEndOfFile).is_some()
    {
        let syntax = db.file_module_syntax(file_id).to_option()?;

        let last_item = syntax.items(db).elements(db).last()?.as_syntax_node();

        node = find_last_meaning_node(db, last_item);
    }

    // Skip trivia.
    while ast::Trivium::is_variant(node.kind(db))
        || node.kind(db) == SyntaxKind::Trivia
        || node.kind(db).is_token()
    {
        node = node.parent(db).unwrap_or(node);
    }

    let trigger_kind =
        params.context.map(|it| it.trigger_kind).unwrap_or(CompletionTriggerKind::INVOKED);

    let deduplicated_items: Vec<_> = db
        .get_node_resultants(node)?
        .into_iter()
        .filter_map(|resultant| complete_ex(resultant, trigger_kind, was_node_corrected, db))
        .flatten()
        .map(CompletionItemHashable)
        .collect::<OrderedHashSet<_>>()
        .into_iter()
        .map(|item| item.0)
        .collect();

    // Need to also deduplicate items with different relevance and leave the one with the highest relevance.
    let mut result = unique_completion_items_with_highest_relevance(deduplicated_items);
    result.sort_by(|a, b| match (&a.relevance, &b.relevance) {
        (Some(ra), Some(rb)) => rb.cmp(ra),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    });

    Some(CompletionResponse::Array(result.into_iter().map(|item| item.item).collect()))
}

fn complete_ex<'db>(
    node: SyntaxNode<'db>,
    trigger_kind: CompletionTriggerKind,
    was_node_corrected: bool,
    db: &'db AnalysisDatabase,
) -> Option<Vec<CompletionItemOrderable>> {
    let ctx = AnalysisContext::from_node(db, node)?;
    let crate_id = ctx.module_file_id.0.owning_crate(db);

    let mut completions = vec![];

    completions.extend(dot_completions(db, &ctx, was_node_corrected));
    completions.extend(struct_constructor_completions(db, &ctx));
    completions.extend(use_completions(db, &ctx));
    completions.extend(self_completions(db, &ctx));
    completions.extend(attribute_completions(db, node, crate_id));
    completions.extend(derive_completions(db, node, crate_id));
    completions.extend(mod_completions(db, node));
    completions.extend(params_completions(db, &ctx, was_node_corrected));
    completions.extend(variables_completions(db, &ctx, was_node_corrected));
    completions.extend(struct_pattern_completions(db, &ctx));
    completions.extend(enum_pattern_completions(db, &ctx));

    if dot_expr_rhs(db, &node, was_node_corrected).is_none() {
        completions.extend(macro_call_completions(db, &ctx));

        if trigger_kind == CompletionTriggerKind::INVOKED {
            completions.extend(path_suffix_completions(db, &ctx))
        }
    }

    Some(completions)
}

fn find_last_meaning_node<'db>(
    db: &'db AnalysisDatabase,
    node: SyntaxNode<'db>,
) -> SyntaxNode<'db> {
    for child in node.get_children(db).iter().rev() {
        if child.kind(db) == SyntaxKind::Trivia {
            continue;
        }

        if child
            .get_children(db)
            .iter()
            .find(|grand_child| grand_child.kind(db) != SyntaxKind::Trivia)
            .is_some_and(|grand_child| grand_child.kind(db) == SyntaxKind::TokenMissing)
        {
            continue;
        }

        return find_last_meaning_node(db, *child);
    }

    node
}

// Specifies how relevant a completion is relative to the scope of the current cursor position.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Debug)]
enum CompletionRelevance {
    Low = 0,
    Medium = 1,
    High = 2,
    Highest = 3,
}

/// Internal representation of a [`CompletionItem`].
#[derive(Clone, Serialize, Debug)]
struct CompletionItemOrderable {
    item: CompletionItem,
    // Relevance tells us in what order we should be showing completions.
    // If the relevance is None, it means that the item can be put in any order.
    relevance: Option<CompletionRelevance>,
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
pub struct CompletionItemHashable(CompletionItemOrderable);

impl Eq for CompletionItemHashable {}

impl Hash for CompletionItemHashable {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        serde_json::to_string(&self.0).expect("serialization should not fail").hash(state);
    }
}

fn get_item_relevance(
    is_in_scope: bool,
    is_current_crate: bool,
    is_corelib: bool,
) -> Option<CompletionRelevance> {
    match (is_in_scope, is_current_crate, is_corelib) {
        (true, _, _) => Some(CompletionRelevance::High),
        (false, true, _) => Some(CompletionRelevance::Medium),
        (false, false, false) => Some(CompletionRelevance::Low),
        _ => None,
    }
}

fn importable_crate_id<'db>(
    db: &'db AnalysisDatabase,
    importable: ImportableId<'db>,
) -> CrateId<'db> {
    match importable {
        ImportableId::Crate(crate_id) => crate_id,
        _ => {
            let importable_node = importable_syntax_node(db, importable)
                .expect("Importable should have a syntax node.");
            let module = db
                .find_module_containing_node(importable_node)
                .expect("A node should be contained in a module");
            module.owning_crate(db)
        }
    }
}

// TODO: Upstream this function to compiler.
fn importable_syntax_node<'db>(
    db: &'db AnalysisDatabase,
    importable: ImportableId<'db>,
) -> Option<SyntaxNode<'db>> {
    match importable {
        ImportableId::Constant(id) => Some(id.stable_ptr(db).lookup(db).as_syntax_node()),
        ImportableId::Submodule(id) => Some(id.stable_ptr(db).lookup(db).as_syntax_node()),
        ImportableId::ExternFunction(id) => Some(id.stable_ptr(db).lookup(db).as_syntax_node()),
        ImportableId::FreeFunction(id) => Some(id.stable_ptr(db).lookup(db).as_syntax_node()),
        ImportableId::ExternType(id) => Some(id.stable_ptr(db).lookup(db).as_syntax_node()),
        ImportableId::TypeAlias(id) => Some(id.stable_ptr(db).lookup(db).as_syntax_node()),
        ImportableId::Impl(id) => Some(id.stable_ptr(db).lookup(db).as_syntax_node()),
        ImportableId::ImplAlias(id) => Some(id.stable_ptr(db).lookup(db).as_syntax_node()),
        ImportableId::Struct(id) => Some(id.stable_ptr(db).lookup(db).as_syntax_node()),
        ImportableId::Variant(id) => Some(id.stable_ptr(db).lookup(db).as_syntax_node()),
        ImportableId::Trait(id) => Some(id.stable_ptr(db).lookup(db).as_syntax_node()),
        ImportableId::Enum(id) => Some(id.stable_ptr(db).lookup(db).as_syntax_node()),
        ImportableId::MacroDeclaration(id) => Some(id.stable_ptr(db).lookup(db).as_syntax_node()),
        ImportableId::Crate(_) => None,
    }
}

/// Given a list of completion items, returns a list with unique items keeping the one with the highest relevance.
fn unique_completion_items_with_highest_relevance(
    relevance_items: Vec<CompletionItemOrderable>,
) -> Vec<CompletionItemOrderable> {
    let mut unique_items: HashMap<String, CompletionItemOrderable> = HashMap::new();

    for relevance_item in relevance_items {
        let key =
            serde_json::to_string(&relevance_item.item).expect("serialization should not fail");
        match unique_items.entry(key) {
            Entry::Occupied(mut occupied) => {
                if relevance_item.relevance > occupied.get().relevance {
                    *occupied.get_mut() = relevance_item;
                }
            }
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(relevance_item);
            }
        };
    }

    unique_items.into_values().collect()
}
