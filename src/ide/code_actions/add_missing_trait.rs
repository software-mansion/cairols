use std::collections::HashMap;

use lsp_types::{CodeAction, CodeActionKind, Url, WorkspaceEdit};

use super::missing_import::is_preferred;
use crate::lang::analysis_context::AnalysisContext;
use crate::lang::db::AnalysisDatabase;
use crate::lang::importer::new_import_edit;
use crate::lang::methods::available_traits_for_method;

/// Create a Quick Fix code action to add a missing trait given a `CannotCallMethod` diagnostic.
pub fn add_missing_trait(
    db: &AnalysisDatabase,
    ctx: &AnalysisContext<'_>,
    uri: Url,
) -> Option<Vec<CodeAction>> {
    let trait_paths = available_traits_for_method(db, ctx)?;

    let is_preferred = is_preferred(&trait_paths);

    let code_actions = trait_paths
        .into_iter()
        .map(|trait_path| CodeAction {
            title: format!("Import {trait_path}"),
            kind: Some(CodeActionKind::QUICKFIX),
            is_preferred,
            edit: Some(WorkspaceEdit {
                changes: new_import_edit(db, ctx, trait_path)
                    .map(|edit| HashMap::from_iter([(uri.clone(), vec![edit])])),
                ..Default::default()
            }),
            ..Default::default()
        })
        .collect();

    Some(code_actions)
}
