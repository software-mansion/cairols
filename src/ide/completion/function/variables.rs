use cairo_lang_semantic::FunctionBody;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::lookup_item::LookupItemEx;
use if_chain::if_chain;
use lsp_types::{CompletionItem, CompletionItemKind};

use crate::ide::completion::expr::selector::expr_selector;
use crate::ide::completion::helpers::binary_expr::dot_rhs::dot_expr_rhs;
use crate::lang::analysis_context::AnalysisContext;
use crate::lang::db::AnalysisDatabase;
use crate::lang::text_matching::text_matches;
use cairo_lang_filesystem::ids::FileLongId;
use cairo_lang_syntax::node::ast::{PathSegment, StatementLet};
use cairo_lang_syntax::node::{Token, TypedSyntaxNode};
use cairo_lang_utils::LookupIntern;

pub fn variables_completions(
    db: &AnalysisDatabase,
    ctx: &AnalysisContext<'_>,
) -> Vec<CompletionItem> {
    if_chain!(
        if let Some(path) = expr_selector(db, &ctx.node);
        if dot_expr_rhs(db, &ctx.node).is_none();
        if let [PathSegment::Simple(segment)] = path.elements(db).as_slice();

        if let Some(lookup_item_id) = ctx.lookup_item_id;
        if let Some(function_id) = lookup_item_id.function_with_body();
        if let Ok(body) = db.function_body(function_id);

        then {
            patterns(&body, db, ctx, &segment.ident(db).token(db).text(db))
        } else {
            Default::default()
        }

    )
}

fn patterns(
    body: &FunctionBody,
    db: &AnalysisDatabase,
    ctx: &AnalysisContext<'_>,
    typed_text: &str,
) -> Vec<CompletionItem> {
    let cursor = ctx.node.offset();

    let mut completions = vec![];

    for (_id, pat) in &body.arenas.patterns {
        for var in pat.variables(&body.arenas.patterns) {
            let pattern_node = var.stable_ptr.0.lookup(db);

            // Skip vars from macros.
            if !matches!(var.stable_ptr.0.file_id(db).lookup_intern(db), FileLongId::OnDisk(_)) {
                continue;
            }

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

            if !text_matches(&var.name, typed_text) {
                continue;
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
