use cairo_lang_semantic::FunctionBody;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::lookup_item::LookupItemEx;
use if_chain::if_chain;
use lsp_types::{CompletionItem, CompletionItemKind};

use crate::lang::analysis_context::AnalysisContext;
use crate::lang::db::AnalysisDatabase;
use cairo_lang_syntax::node::TypedSyntaxNode;
use cairo_lang_syntax::node::ast::{StatementLet, UsePathSingle};
use cairo_lang_syntax::node::kind::SyntaxKind;

pub fn variables_completions(
    db: &AnalysisDatabase,
    ctx: &AnalysisContext<'_>,
) -> Vec<CompletionItem> {
    if_chain!(
        // TODO remove this check when we have expression completions.
        if ctx.node.ancestor_of_type::<UsePathSingle>(db).is_none()
        && ctx.node.ancestor_of_kind(db, SyntaxKind::ExprStructCtorCall).is_none()
        && ctx.node.ancestor_of_kind(db, SyntaxKind::ExprBinary).is_none();

        if let Some(lookup_item_id) = ctx.lookup_item_id;
        if let Some(function_id) = lookup_item_id.function_with_body();
        if let Ok(body) = db.function_body(function_id);

        then {
            patterns(&body, db, ctx)
        } else {
            Default::default()
        }

    )
}

fn patterns(
    body: &FunctionBody,
    db: &AnalysisDatabase,
    ctx: &AnalysisContext<'_>,
) -> Vec<CompletionItem> {
    let cursor = ctx.node.offset();

    let mut completions = vec![];

    for (_id, pat) in &body.arenas.patterns {
        for var in pat.variables(&body.arenas.patterns) {
            let pattern_node = var.stable_ptr.0.lookup(db);
            // Take only already declared variables.
            if cursor < pattern_node.offset() {
                continue;
            }

            if let Some(let_statement) = ctx.node.ancestor_of_type::<StatementLet>(db) {
                if let_statement.pattern(db).stable_ptr().0.lookup(db) == pattern_node {
                    // Disallow recursive variables.
                    // let abc = {
                    //     // do something
                    //     a<caret>
                    // }
                    // `abc` is defined before caret, but we still want to skip it.
                    continue;
                }
            }

            completions.push(CompletionItem {
                label: var.name.clone().into(),
                kind: Some(CompletionItemKind::VARIABLE),
                ..CompletionItem::default()
            });
        }
    }

    completions
}
