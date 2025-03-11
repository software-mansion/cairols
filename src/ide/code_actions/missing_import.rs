use lsp_types::{CodeAction, CodeActionKind, Url, WorkspaceEdit};

use crate::lang::analysis_context::AnalysisContext;
use crate::lang::db::AnalysisDatabase;
use crate::lang::importer::new_import_edit;
use cairo_lang_defs::ids::{FileIndex, ModuleFileId};
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_syntax::node::ast::ExprPath;
use cairo_lang_syntax::node::helpers::GetIdentifier;
use std::collections::HashMap;

pub fn missing_import(
    db: &AnalysisDatabase,
    ctx: &AnalysisContext<'_>,
    uri: Url,
) -> Option<Vec<CodeAction>> {
    let typed_path_generic = ctx.node.ancestor_of_type::<ExprPath>(db)?;

    // Remove generic args.
    let typed_path_segments: Vec<_> = typed_path_generic
        .elements(db)
        .into_iter()
        .map(|e| e.identifier(db).to_string())
        .rev()
        .collect();

    let items = db.visible_importables_from_module(ModuleFileId(ctx.module_id, FileIndex(0)))?;

    let items: Vec<_> = items
        .iter()
        .filter_map(|(_item, proposed_path)| {
            let mut proposed_path_segments: Vec<_> = proposed_path.split("::").collect();

            // We exclude items that are already in scope (commonly prelude).
            // These items can NOT generate E0006.
            // This prevents cases like derive with same name as trait, that is broken and generates code with E0006 error inside.
            if proposed_path_segments.len() == 1 {
                return None;
            }

            let mut last_path_segment = None;

            for typed_path_segment in &typed_path_segments {
                last_path_segment = proposed_path_segments.pop();

                if typed_path_segment != last_path_segment? {
                    return None;
                }
            }

            proposed_path_segments.extend(last_path_segment);

            Some(proposed_path_segments.join("::"))
        })
        .collect();

    let is_preferred = is_preferred(&items);

    Some(
        items
            .into_iter()
            .map(|path| CodeAction {
                title: format!("Import `{path}`"),
                kind: Some(CodeActionKind::QUICKFIX),
                is_preferred,
                edit: Some(WorkspaceEdit {
                    changes: new_import_edit(db, ctx, path)
                        .map(|edit| HashMap::from_iter([(uri.clone(), vec![edit])])),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .collect(),
    )
}

pub fn is_preferred<T>(items: &[T]) -> Option<bool> {
    let is_unambiguous = match items.len() {
        0 => return None,
        1 => true,
        _ => false,
    };

    // We can propose this for autofix if there is exactly one possible option.
    is_unambiguous.then_some(true)
}
