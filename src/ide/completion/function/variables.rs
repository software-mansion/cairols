use std::collections::HashSet;

use cairo_lang_filesystem::ids::FileLongId;
use cairo_lang_semantic::FunctionBody;
use cairo_lang_semantic::items::function_with_body::FunctionWithBodySemantic;
use cairo_lang_semantic::lookup_item::LookupItemEx;
use cairo_lang_syntax::node::ast::{PathSegment, StatementLet};
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{Token, TypedSyntaxNode};
use itertools::Itertools;
use lsp_types::{CompletionItem, CompletionItemKind};

use crate::ide::completion::expr::selector::expr_selector;
use crate::ide::completion::helpers::binary_expr::dot_rhs::dot_expr_rhs;
use crate::ide::completion::helpers::formatting::format_type_in_node_context;
use crate::ide::completion::{CompletionItemOrderable, CompletionRelevance};
use crate::lang::analysis_context::AnalysisContext;
use crate::lang::db::AnalysisDatabase;
use crate::lang::text_matching::text_matches;

pub fn variables_completions<'db>(
    db: &'db AnalysisDatabase,
    ctx: &AnalysisContext<'db>,
    was_node_corrected: bool,
) -> Vec<CompletionItemOrderable> {
    if let Some(path) = expr_selector(db, &ctx.node)
        && dot_expr_rhs(db, &ctx.node, was_node_corrected).is_none()
        && let [PathSegment::Simple(segment)] =
            path.segments(db).elements(db).take(2).collect_vec().as_slice()
        && let Some(lookup_item_id) = ctx.lookup_item_id
        && let Some(function_id) = lookup_item_id.function_with_body()
        && let Ok(body) = db.function_body(function_id)
    {
        patterns(body, db, ctx, segment.ident(db).token(db).text(db).to_string(db).as_str())
    } else {
        Default::default()
    }
}

fn patterns<'db>(
    body: &FunctionBody<'db>,
    db: &'db AnalysisDatabase,
    ctx: &AnalysisContext<'db>,
    typed_text: &str,
) -> Vec<CompletionItemOrderable> {
    let cursor = ctx.node.offset(db);

    let mut completions = vec![];

    for (_id, pat) in &body.arenas.patterns {
        for var in pat.variables(&body.arenas.patterns) {
            let pattern_node = var.stable_ptr.0.lookup(db);

            // Skip vars from macros.
            if !matches!(var.stable_ptr.0.file_id(db).long(db), FileLongId::OnDisk(_)) {
                continue;
            }

            // Take only already declared variables.
            if cursor < pattern_node.offset(db) {
                continue;
            }

            // Find all ancestor let statements and check if we are on pattern created with one of these.
            let is_recursive = ctx
                .node
                .ancestors_with_self(db)
                .filter_map(|node| StatementLet::cast(db, node))
                .any(|let_statement| {
                    let_statement.pattern(db).stable_ptr(db).0.lookup(db) == pattern_node
                });

            if is_recursive {
                // Disallow recursive variables.
                // let abc = {
                //     // do something
                //     a<caret>
                // }
                // `abc` is defined before caret, but we still want to skip it.
                continue;
            }

            let ancestors: HashSet<_> = ctx.node.ancestors_with_self(db).collect();

            let Some(common_ancestor) =
                pattern_node.ancestors_with_self(db).find(|node| ancestors.contains(node))
            else {
                continue;
            };

            let blocks_to_common_ancestor = pattern_node
                .ancestors_with_self(db)
                .take_while(|node| node != &common_ancestor)
                .filter(|node| node.kind(db) == SyntaxKind::ExprBlock)
                .count();

            match blocks_to_common_ancestor {
                0 => {}
                // This is allowed only if common ancestor is block.
                1 if common_ancestor.kind(db) == SyntaxKind::ExprBlock => {}
                _ => continue,
            }

            if !text_matches(var.name.to_string(db), typed_text) {
                continue;
            }

            completions.push(CompletionItemOrderable {
                item: CompletionItem {
                    label: var.name.to_string(db),
                    detail: Some(format_type_in_node_context(
                        db,
                        var.stable_ptr.0.lookup(db),
                        &var.var.ty,
                    )),
                    kind: Some(CompletionItemKind::VARIABLE),
                    ..CompletionItem::default()
                },
                relevance: CompletionRelevance::Highest,
            });
        }
    }

    completions
}
