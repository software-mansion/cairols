use cairo_lang_defs::db::DefsGroup;
use cairo_lang_defs::ids::NamedLanguageElementId;
use cairo_lang_filesystem::db::FilesGroup;
use cairo_lang_semantic::Pattern;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::lookup_item::LookupItemEx;
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

    // Local variables and params.
    let Some(lookup_item_id) = ctx.lookup_item_id else {
        return completions;
    };
    let Some(function_id) = lookup_item_id.function_with_body() else {
        return completions;
    };
    let Ok(signature) = db.function_with_body_signature(function_id) else {
        return completions;
    };
    for param in &signature.params {
        completions.push(CompletionItem {
            label: param.name.clone().into(),
            kind: Some(CompletionItemKind::VARIABLE),
            ..CompletionItem::default()
        });
    }

    let Ok(body) = db.function_body(function_id) else {
        return completions;
    };

    let inline_plugins = db.inline_macro_plugins();

    let inline_macros = inline_plugins.iter().map(|plugin| CompletionItem {
        label: format!("{}!", plugin.0),
        kind: Some(CompletionItemKind::FUNCTION),
        ..CompletionItem::default()
    });

    completions.extend(inline_macros);

    for (_id, pat) in &body.arenas.patterns {
        if let Pattern::Variable(var) = pat {
            completions.push(CompletionItem {
                label: var.name.clone().into(),
                kind: Some(CompletionItemKind::VARIABLE),
                ..CompletionItem::default()
            });
        }
    }
    completions
}
