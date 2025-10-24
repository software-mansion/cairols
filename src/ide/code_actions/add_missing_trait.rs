use std::collections::HashMap;

use lsp_types::{CodeAction, CodeActionKind, Url, WorkspaceEdit};

use super::missing_import::is_preferred;
use crate::lang::analysis_context::AnalysisContext;
use crate::lang::db::{AnalysisDatabase, LsSemanticGroup};
use crate::lang::importer::new_import_edit;
use crate::lang::methods::available_traits_for_method;

/// Create a Quick Fix code action to add a missing trait given a `CannotCallMethod` diagnostic.
pub fn add_missing_trait<'db>(
    db: &'db AnalysisDatabase,
    ctx: &AnalysisContext<'db>,
    uri: Url,
) -> Option<Vec<CodeAction>> {
    db.get_node_resultants(ctx.node)?.iter().find_map(|resultant_node| {
        let resultant_ctx = AnalysisContext::from_node(db, *resultant_node)?;
        let trait_paths =
            available_traits_for_method(db, resultant_node, &mut resultant_ctx.resolver(db))?;
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
    })
}
