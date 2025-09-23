use std::collections::HashMap;

use cairo_lang_semantic::items::function_with_body::{
    FunctionWithBodySemantic, SemanticExprLookup,
};
use cairo_lang_semantic::lookup_item::LookupItemEx;
use cairo_lang_syntax::node::{TypedSyntaxNode, ast};
use lsp_types::{CodeAction, CodeActionKind, TextEdit, Url, WorkspaceEdit};

use crate::lang::analysis_context::AnalysisContext;
use crate::lang::db::AnalysisDatabase;
use crate::lang::lsp::ToLsp;
use crate::lang::members::find_members_for_type;
use crate::lang::text_matching::text_matches;

pub fn suggest_similar_member<'db>(
    db: &'db AnalysisDatabase,
    ctx: &AnalysisContext<'db>,
    uri: Url,
) -> Option<Vec<CodeAction>> {
    let expr_binary = ctx.node.ancestor_of_type::<ast::ExprBinary>(db)?;
    let lhs_stable_ptr = expr_binary.lhs(db).stable_ptr(db);
    // Get its semantic model.
    let function_with_body = ctx.lookup_item_id?.function_with_body()?;

    let expr_id = db.lookup_expr_by_ptr(function_with_body, lhs_stable_ptr).ok()?;
    let expr = db.expr_semantic(function_with_body, expr_id);

    let ty = expr.ty();
    if ty.is_missing(db) {
        return None;
    }

    let member_candidates = find_members_for_type(db, function_with_body, ty)?;

    let typed_member_name = ctx.node.get_text_without_trivia(db);

    // Filter member candidates by similarity to the typed member name
    let suggestions: Vec<String> = member_candidates
        .into_iter()
        .map(|name| name.to_string(db))
        .filter(|name| text_matches(name, typed_member_name.to_string(db)))
        .collect();

    if suggestions.is_empty() {
        return None;
    }

    let bad_member_span = ctx
        .node
        .span_without_trivia(db)
        .position_in_file(db, ctx.module_file_id.file_id(db).ok()?)?;

    let code_actions = suggestions
        .into_iter()
        .map(|member_name| {
            let edits =
                vec![TextEdit { range: bad_member_span.to_lsp(), new_text: member_name.clone() }];

            CodeAction {
                title: format!("Use {member_name} instead"),
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
}
