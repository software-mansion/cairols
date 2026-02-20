use std::cmp::Ordering;
use std::hash::Hash;

use cairo_lang_defs::ids::{ImportableId, NamedLanguageElementId};
use cairo_lang_filesystem::ids::{CrateId, CrateLongId, SmolStrId};
use cairo_lang_semantic::items::extern_function::ExternFunctionSemantic;
use cairo_lang_semantic::items::free_function::FreeFunctionSemantic;
use cairo_lang_semantic::lsp_helpers::LspHelpers;
use cairo_lang_syntax::node::TypedSyntaxNode;
use cairo_lang_syntax::node::ast::ItemStruct;
use lsp_types::{CompletionItem, CompletionItemLabelDetails, InsertTextFormat};
use serde::Serialize;

use crate::ide::completion::helpers::completion_kind::importable_completion_kind;
use crate::ide::completion::helpers::snippets::TypedSnippet;
use crate::lang::analysis_context::AnalysisContext;
use crate::lang::db::AnalysisDatabase;
use crate::lang::importable::{importable_crate_id, importable_syntax_node};
use crate::lang::importer::new_import_edit;
use crate::lang::text_matching::text_matches;

pub struct FirstSegmentCompletionCandidate<'a> {
    pub path: String,
    pub completion: ImportableCompletionItem<'a>,
}

impl FirstSegmentCompletionCandidate<'_> {
    pub fn into_path_completion(self) -> CompletionItemOrderable {
        let description = self
            .completion
            .item
            .item
            .label_details
            .as_ref()
            .and_then(|details| details.description.clone());

        CompletionItemOrderable {
            item: CompletionItem {
                label: self.path,
                kind: self.completion.item.item.kind,
                label_details: description.map(|description| CompletionItemLabelDetails {
                    detail: None,
                    description: Some(description),
                }),
                ..CompletionItem::default()
            },
            relevance: self.completion.item.relevance,
        }
    }
}

pub fn first_segment_completion_candidates<'db>(
    db: &'db AnalysisDatabase,
    ctx: &'db AnalysisContext<'db>,
    typed: &str,
) -> Vec<FirstSegmentCompletionCandidate<'db>> {
    let Some(importables) = db.visible_importables_from_module(ctx.module_id) else {
        return Default::default();
    };
    let current_crate = ctx.module_id.owning_crate(db);
    let typed_segment = SmolStrId::from(db, typed);

    importables
        .iter()
        .filter_map(|(importable, path)| {
            if path.split("::").count() != 1 || !text_matches(path, typed) {
                return None;
            }

            let completion = ImportableCompletionItem::get_completion_item_for_importable(
                db,
                ctx,
                importable,
                current_crate,
                path,
                vec![],
                typed_segment,
            )?;

            Some(FirstSegmentCompletionCandidate { path: path.clone(), completion })
        })
        .collect()
}

// Specifies how relevant a completion is relative to the scope of the current cursor position.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Debug, Copy)]
pub enum CompletionRelevance {
    Lowest = 0,
    Low = 1,
    Medium = 2,
    MediumHigh = 3,
    High = 4,
    Highest = 5,
}

pub fn get_item_relevance(
    is_in_scope: bool,
    is_current_crate: bool,
    is_corelib: bool,
) -> CompletionRelevance {
    match (is_in_scope, is_current_crate, is_corelib) {
        (true, _, false) => CompletionRelevance::High,
        // This one ensures that prelude items are below items from the current scope, but still high enough.
        (true, _, _) => CompletionRelevance::MediumHigh,
        (false, true, _) => CompletionRelevance::Medium,
        (false, false, false) => CompletionRelevance::Low,
        _ => CompletionRelevance::Lowest,
    }
}

/// A completion item associated with an `ImportableId`.
/// We need this to track which importable was used to create the completion item,
/// so later we can filter out certain types of importables.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImportableCompletionItem<'a> {
    pub item: CompletionItemOrderable,
    pub importable_id: ImportableId<'a>,
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

#[derive(PartialEq)]
pub struct ImportableCompletionItemHashable<'a>(pub ImportableCompletionItem<'a>);

impl Eq for ImportableCompletionItemHashable<'_> {}

impl Hash for ImportableCompletionItemHashable<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        serde_json::to_string(&self.0.item).expect("serialization should not fail").hash(state);
    }
}

impl ImportableCompletionItem<'_> {
    pub fn get_completion_item_for_importable<'db>(
        db: &'db AnalysisDatabase,
        ctx: &'db AnalysisContext<'db>,
        importable: &ImportableId<'db>,
        current_crate: CrateId,
        path_str: &str,
        typed_text: Vec<SmolStrId<'db>>,
        last_typed_segment: SmolStrId<'db>,
    ) -> Option<ImportableCompletionItem<'db>> {
        let mut path_segments: Vec<_> = path_str.split("::").collect();

        let is_not_in_scope = path_segments.len() != 1;

        let last_segment = path_segments.pop().expect("path to import should not be empty");

        let mut last_popped = None;

        let previous_segment_matches = typed_text.iter().rev().all(|typed_segment| {
            last_popped = path_segments.pop();
            last_popped.map(|s| s == typed_segment.to_string(db).as_str()).unwrap_or(false)
        });

        // Import path and typed path must have single overlapping element.
        // use foo::bar;
        //          bar::baz(12345);
        // If path was *not* empty we should *not* add use statement at all.
        if !path_segments.is_empty() {
            path_segments.push(last_popped.unwrap_or(last_segment));
        }

        if !previous_segment_matches
            || !text_matches(last_segment, last_typed_segment.to_string(db).as_str())
        {
            return None;
        }

        let additional_text_edits = if is_not_in_scope
            && !path_segments.is_empty()
            && let Some(import_edit) = new_import_edit(db, ctx, path_segments.join("::"))
        {
            Some(vec![import_edit])
        } else {
            None
        };

        let does_require_import = additional_text_edits.is_some();
        let importable_crate = importable_crate_id(db, *importable);
        let is_current_crate = importable_crate == current_crate;
        let is_core = *importable_crate.long(db) == CrateLongId::core(db);

        let CompletionParams { label, snippet: snippet_text } =
            CompletionParams::for_importable(db, ctx, importable, last_segment);

        Some(ImportableCompletionItem {
            item: CompletionItemOrderable {
                item: CompletionItem {
                    label: label.clone(),
                    insert_text: snippet_text.clone().map(|snippet| snippet.lsp_snippet),
                    insert_text_format: snippet_text.clone().map(|_| InsertTextFormat::SNIPPET),
                    kind: Some(importable_completion_kind(*importable)),
                    label_details: Some(CompletionItemLabelDetails {
                        detail: (path_str != label).then(|| format!("(use {path_str})")),
                        description: snippet_text
                            .map(|typed_snippet| typed_snippet.type_hint)
                            .unwrap_or_default(),
                    }),
                    additional_text_edits,
                    ..CompletionItem::default()
                },
                relevance: get_item_relevance(!does_require_import, is_current_crate, is_core),
            },
            importable_id: *importable,
        })
    }
}

struct CompletionParams {
    label: String,
    snippet: Option<TypedSnippet>,
}

impl CompletionParams {
    fn for_importable(
        db: &AnalysisDatabase,
        ctx: &AnalysisContext<'_>,
        importable: &ImportableId<'_>,
        last_segment: &str,
    ) -> CompletionParams {
        let (snippet, label) = match importable {
            ImportableId::Struct(_) => {
                let snippet = importable_syntax_node(db, *importable).and_then(|struct_node| {
                    TypedSnippet::struct_initialization(
                        db,
                        ctx,
                        ItemStruct::from_syntax_node(db, struct_node),
                    )
                });
                let label = if snippet.is_some() {
                    format!("{} {{...}}", last_segment)
                } else {
                    last_segment.to_string()
                };

                (snippet, label)
            }
            ImportableId::FreeFunction(id) => {
                let snippet = db.free_function_signature(*id).ok().map(|signature| {
                    TypedSnippet::function_call(db, &id.name(db).to_string(db), signature, None)
                });

                let label = if snippet.is_some() {
                    format!("{}(...)", last_segment)
                } else {
                    last_segment.to_string()
                };

                (snippet, label)
            }
            ImportableId::ExternFunction(id) => {
                let snippet = db.extern_function_signature(*id).ok().map(|signature| {
                    TypedSnippet::function_call(db, &id.name(db).to_string(db), signature, None)
                });

                let label = if snippet.is_some() {
                    format!("{}(...)", last_segment)
                } else {
                    last_segment.to_string()
                };

                (snippet, label)
            }
            ImportableId::MacroDeclaration(_) => {
                (Some(TypedSnippet::macro_call(last_segment)), format!("{}!", last_segment))
            }
            _ => (None, last_segment.to_string()),
        };

        CompletionParams { label, snippet }
    }
}
