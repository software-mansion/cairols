use std::collections::HashMap;

use cairo_lang_defs::ids::ModuleId;
use cairo_lang_filesystem::span::TextOffset;
use cairo_lang_syntax::node::{TypedStablePtr, TypedSyntaxNode, ast};
use if_chain::if_chain;
use lsp_types::{CodeAction, CodeActionKind, Range, TextEdit, Url, WorkspaceEdit};

use super::missing_import::is_preferred;
use crate::lang::analysis_context::AnalysisContext;
use crate::lang::db::AnalysisDatabase;
use crate::lang::lsp::{LsProtoGroup, ToLsp};
use crate::lang::methods::available_traits_for_method;

/// Create a Quick Fix code action to add a missing trait given a `CannotCallMethod` diagnostic.
pub fn add_missing_trait(
    db: &AnalysisDatabase,
    ctx: &AnalysisContext<'_>,
    uri: Url,
) -> Option<Vec<CodeAction>> {
    let trait_paths = available_traits_for_method(db, ctx)?;

    let is_preferred = is_preferred(&trait_paths);

    let module_start_offset = if_chain! {
        if let ModuleId::Submodule(submodule_id) = ctx.module_id;
        if let ast::MaybeModuleBody::Some(body) = submodule_id.stable_ptr(db).lookup(db).body(db);

        then {
            body.items(db).as_syntax_node().span_start_without_trivia(db)
        } else {
            TextOffset::default()
        }
    };

    let file_id = db.file_for_url(&uri)?;
    let module_start_position = module_start_offset.position_in_file(db, file_id)?.to_lsp();
    let range = Range::new(module_start_position, module_start_position);

    let code_actions = trait_paths
        .into_iter()
        .map(|trait_path| CodeAction {
            title: format!("Import {}", trait_path),
            kind: Some(CodeActionKind::QUICKFIX),
            is_preferred,
            edit: Some(WorkspaceEdit {
                changes: Some(HashMap::from_iter([(
                    uri.clone(),
                    vec![TextEdit { range, new_text: format!("use {};\n", trait_path) }],
                )])),
                ..Default::default()
            }),
            ..Default::default()
        })
        .collect();

    Some(code_actions)
}
