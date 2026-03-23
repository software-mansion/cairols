use cairo_lang_semantic::items::function_with_body::FunctionWithBodySemantic;
use cairo_lang_semantic::lookup_item::LookupItemEx;
use cairo_lang_syntax::node::Token;
use cairo_lang_syntax::node::ast::{self, PathSegment};
use cairo_lang_syntax::node::kind::SyntaxKind;
use itertools::Itertools;
use lsp_types::{CompletionItem, CompletionItemKind, CompletionItemLabelDetails};

use crate::ide::completion::expr::selector::expr_selector;
use crate::ide::completion::helpers::binary_expr::dot_rhs::dot_expr_rhs;
use crate::ide::completion::helpers::formatting::format_type_in_node_context;
use crate::ide::completion::{CompletionItemOrderable, CompletionRelevance};
use crate::lang::analysis_context::AnalysisContext;
use crate::lang::db::AnalysisDatabase;
use crate::lang::text_matching::text_matches;

pub fn params_completions<'db>(
    db: &'db AnalysisDatabase,
    ctx: &AnalysisContext<'db>,
    was_node_corrected: bool,
) -> Vec<CompletionItemOrderable> {
    if dot_expr_rhs(db, &ctx.node, was_node_corrected).is_some() {
        return Default::default();
    }

    let Some(lookup_item_id) = ctx.lookup_item_id else { return Default::default() };
    let Some(function_id) = lookup_item_id.function_with_body() else { return Default::default() };
    let Ok(signature) = db.function_with_body_signature(function_id) else {
        return Default::default();
    };

    let typed_text = if let Some(path) = expr_selector(db, &ctx.node)
        && let [PathSegment::Simple(segment)] =
            path.segments(db).elements(db).take(2).collect_vec().as_slice()
    {
        segment.ident(db).token(db).text(db).to_string(db)
    } else if ctx.node.kind(db) == SyntaxKind::ExprBlock
        || ast::Statement::is_variant(ctx.node.kind(db))
        || ctx.node.parent(db).is_some_and(|p| p.kind(db) == SyntaxKind::ExprBlock)
        || ctx.node.parent(db).is_some_and(|p| ast::Statement::is_variant(p.kind(db)))
    {
        String::new()
    } else {
        return Default::default();
    };

    signature
        .params
        .iter()
        .filter_map(|param| {
            let param_name = param.name.to_string(db);
            if !text_matches(param_name.clone(), &typed_text) {
                return None;
            }

            Some(CompletionItemOrderable {
                item: CompletionItem {
                    label: param_name,
                    label_details: Some(CompletionItemLabelDetails {
                        description: Some(format_type_in_node_context(
                            db,
                            param.stable_ptr.0.lookup(db),
                            &param.ty,
                        )),
                        detail: None,
                    }),
                    kind: Some(CompletionItemKind::VARIABLE),
                    ..CompletionItem::default()
                },
                relevance: CompletionRelevance::Highest,
            })
        })
        .collect()
}
