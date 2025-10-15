use std::collections::HashMap;

use cairo_lang_defs::ids::NamedLanguageElementLongId;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::items::function_with_body::{
    FunctionWithBodySemantic, SemanticExprLookup,
};
use cairo_lang_semantic::lookup_item::LookupItemEx;
use cairo_lang_syntax::node::{TypedStablePtr, TypedSyntaxNode, ast};
use lsp_types::{CodeAction, CodeActionKind, TextEdit, Url, WorkspaceEdit};

use crate::lang::analysis_context::AnalysisContext;
use crate::lang::db::{AnalysisDatabase, LsSemanticGroup};
use crate::lang::importer::import_edit_for_trait_if_needed;
use crate::lang::lsp::ToLsp;
use crate::lang::methods::find_methods_for_type;
use crate::lang::text_matching::text_matches;

/// Create a Quick Fix code action to substitute this call with a similar one.
pub fn suggest_similar_method<'db>(
    db: &'db AnalysisDatabase,
    ctx: &AnalysisContext<'db>,
    uri: Url,
) -> Option<Vec<CodeAction>> {
    db.get_node_resultants(ctx.node)?.iter().find_map(|resultant_node| {
        let expr_binary = resultant_node.ancestor_of_type::<ast::ExprBinary>(db)?;
        let lhs_stable_ptr = expr_binary.lhs(db).stable_ptr(db);
        // Get its semantic model.
        let resultant_lookup_item = db.find_lookup_item(*resultant_node)?;
        let resultant_lookup_function = resultant_lookup_item.function_with_body()?;

        let expr_id = db.lookup_expr_by_ptr(resultant_lookup_function, lhs_stable_ptr).ok()?;

        let semantic_db: &dyn SemanticGroup = db;
        let ty = semantic_db.expr_semantic(resultant_lookup_function, expr_id).ty();

        if ty.is_missing(db) {
            return None;
        }

        let method_candidates =
            find_methods_for_type(db, &mut ctx.resolver(db), ty, lhs_stable_ptr.untyped());

        let typed_fn_name = ctx.node.get_text(db);

        // Build list of suggestions with optional import edit per suggestion
        let suggestions: Vec<(String, Option<TextEdit>)> = method_candidates
            .into_iter()
            .filter_map(|function_id| {
                let fn_name = function_id.long(db).name(db).to_string(db);
                let matches = text_matches(&fn_name, typed_fn_name);
                if !matches {
                    return None;
                }

                // Determine if the trait providing this method is in scope, and if not, prepare an import edit.
                let trait_id = function_id.trait_id(db);
                let import_edit = import_edit_for_trait_if_needed(db, ctx, trait_id);
                Some((fn_name, import_edit))
            })
            .collect();

        let bad_method_name_span =
            ctx.node.span(db).position_in_file(db, ctx.node.stable_ptr(db).file_id(db))?;

        let code_actions = suggestions
            .into_iter()
            .map(|(method_name, import_edit)| {
                let mut edits = Vec::new();
                // First, add the method name replacement edit
                edits.push(TextEdit {
                    range: bad_method_name_span.to_lsp(),
                    new_text: method_name.to_string(),
                });
                // Then, add the import edit if needed
                if let Some(edit) = import_edit {
                    edits.push(edit);
                }

                CodeAction {
                    title: format!("Use {method_name} instead"),
                    kind: Some(CodeActionKind::QUICKFIX),
                    edit: Some(WorkspaceEdit {
                        changes: Some(HashMap::from([(uri.clone(), edits)])),
                        document_changes: None,
                        change_annotations: None,
                    }),
                    ..Default::default()
                }
            })
            .collect();
        Some(code_actions)
    })
}
