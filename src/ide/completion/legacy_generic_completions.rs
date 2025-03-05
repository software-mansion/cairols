use cairo_lang_defs::db::DefsGroup;
use cairo_lang_defs::ids::NamedLanguageElementId;
use cairo_lang_filesystem::db::FilesGroup;
use cairo_lang_semantic::resolve::ResolvedGenericItem;
use cairo_lang_utils::{LookupIntern, Upcast};
use lsp_types::{CompletionItem, CompletionItemKind};

use super::helpers::completion_kind::resolved_generic_item_completion_kind;
use crate::lang::analysis_context::AnalysisContext;
use crate::lang::db::AnalysisDatabase;

pub fn generic_completions(
    db: &AnalysisDatabase,
    ctx: &AnalysisContext<'_>,
) -> Vec<CompletionItem> {
    let mut completions = vec![];

    // Crates.
    completions.extend(db.crate_configs().keys().map(|crate_id| CompletionItem {
        label: crate_id.lookup_intern(db).name().into(),
        kind: Some(CompletionItemKind::MODULE),
        ..CompletionItem::default()
    }));

    // Module completions.
    if let Ok(module_items) = db.module_items(ctx.module_id) {
        completions.extend(module_items.iter().map(|item| {
            CompletionItem {
                label: item.name(db.upcast()).to_string(),
                kind: ResolvedGenericItem::from_module_item(db, *item)
                    .ok()
                    .map(resolved_generic_item_completion_kind),
                ..CompletionItem::default()
            }
        }));
    }

    completions
}
