use cairo_lang_defs::db::DefsGroup;
use cairo_lang_defs::plugin::MacroPlugin;
use cairo_lang_plugins::plugins::CompileErrorPlugin;
use cairo_lang_semantic::items::function_with_body::FunctionWithBodySemantic;
use cairo_lang_semantic::lookup_item::LookupItemEx;
use cairo_lang_starknet::plugin::StarknetPlugin;
use cairo_lang_starknet::plugin::consts::CONTRACT_ATTR;
use cairo_lang_syntax::node::Token;
use cairo_lang_syntax::node::ast::{
    ItemModule, ModuleItem, PathSegment, SkippedNode, TriviumSkippedNode,
};
use cairo_lang_syntax::node::helpers::QueryAttrs;
use cairo_lang_syntax::node::kind::SyntaxKind;
use itertools::{Itertools, chain};
use lsp_types::{CompletionItem, InsertTextFormat};

use crate::ide::completion::expr::selector::expr_selector;
use crate::ide::completion::helpers::binary_expr::dot_rhs::dot_expr_rhs;
use crate::ide::completion::{CompletionItemOrderable, CompletionRelevance};
use crate::lang::analysis_context::AnalysisContext;
use crate::lang::db::AnalysisDatabase;
use crate::lang::text_matching::text_matches;

pub fn expr_inline_macro_completions<'db>(
    db: &'db AnalysisDatabase,
    ctx: &AnalysisContext<'db>,
    was_node_corrected: bool,
) -> Vec<CompletionItemOrderable> {
    if dot_expr_rhs(db, &ctx.node, was_node_corrected).is_none()
        && let Some(lookup_item_id) = ctx.lookup_item_id
        && let Some(function_id) = lookup_item_id.function_with_body()
        && db.function_body(function_id).is_ok()
        && let Some(path) = expr_selector(db, &ctx.node)
        // Currently, inline macros cannot be imported/exported.
        && let [PathSegment::Simple(path_segment)] =
        path.segments(db).elements(db).take(2).collect_vec().as_slice()
    {
        let crate_id = ctx.module_id.owning_crate(db);

        let inline_plugins = db.crate_inline_macro_plugins(crate_id);

        let typed = path_segment.ident(db).token(db).text(db).to_string(db);

        inline_plugins
            .iter()
            .filter(|(name, _)| text_matches(name, &typed))
            .map(|(plugin_name, _)| snippet_completions_for_inline_plugins(plugin_name))
            .collect()
    } else {
        Default::default()
    }
}

pub fn top_level_inline_macro_completions<'db>(
    db: &'db AnalysisDatabase,
    ctx: &AnalysisContext<'db>,
) -> Vec<CompletionItemOrderable> {
    // Covers the case when we are not in any module item:
    //
    // ```cairo
    // impl X of Y {
    //     // blabla
    // }
    // compile_er<caret>
    // ```
    let is_on_trivium_skipped_expr_path = ctx
        .node
        .ancestor_of_type::<TriviumSkippedNode>(db)
        .is_some_and(|triv_skipped| matches!(triv_skipped.node(db), SkippedNode::ExprPath(_)));
    let is_descendant_of_module_item = ctx.node.ancestors_with_self(db).any(|node| {
        let kind = node.kind(db);
        ModuleItem::is_variant(kind)
            && ![
                // Exclude these since they can contain top level inline macro calls.
                SyntaxKind::ItemModule,
                SyntaxKind::ItemInlineMacro,
                SyntaxKind::ItemMacroDeclaration,
            ]
            .contains(&kind)
    });

    if (is_on_trivium_skipped_expr_path || !is_descendant_of_module_item)
        && let Some(path) = expr_selector(db, &ctx.node)
        // Currently, inline macros cannot be imported/exported.
        && let [PathSegment::Simple(path_segment)] =
        path.segments(db).elements(db).take(2).collect_vec().as_slice()
    {
        let available_top_level_inline_macros = available_top_level_inline_macro(db, ctx);

        let typed = path_segment.ident(db).token(db).text(db).to_string(db);

        available_top_level_inline_macros
            .into_iter()
            .filter(|name| text_matches(name, &typed))
            .map(snippet_completions_for_inline_plugins)
            .collect()
    } else {
        Default::default()
    }
}

fn available_top_level_inline_macro(
    db: &AnalysisDatabase,
    ctx: &AnalysisContext,
) -> Vec<&'static str> {
    let crate_id = ctx.module_id.owning_crate(db);
    let macro_plugins = db.crate_macro_plugins(crate_id);

    let starknet_plugin_present = macro_plugins
        .iter()
        .any(|id| id.long(db).plugin_type_id() == StarknetPlugin::default().plugin_type_id());
    let should_suggest_component = starknet_plugin_present
        && ctx
            .node
            .ancestor_of_type::<ItemModule>(db)
            .is_some_and(|item_module| item_module.has_attr(db, CONTRACT_ATTR));
    let compile_error_plugin_present = macro_plugins
        .iter()
        .any(|id| id.long(db).plugin_type_id() == CompileErrorPlugin::default().plugin_type_id());

    chain!(
        should_suggest_component.then_some("component"),
        compile_error_plugin_present.then_some("compile_error")
    )
    .collect()
}

fn snippet_completions_for_inline_plugins(inline_macro_name: &str) -> CompletionItemOrderable {
    let insert_text = match inline_macro_name {
        "array" => "array![$1]".to_string(),
        "assert" => "assert!($1, \"$2\")".to_string(),
        macro_name @ ("assert_eq" | "assert_ne" | "assert_lt" | "assert_le" | "assert_gt"
        | "assert_ge") => {
            format!("{macro_name}!($1, $2, \"$3\")")
        }
        macro_name @ ("format" | "print" | "println" | "panic" | "selector") => {
            format!("{macro_name}!(\"$1\")")
        }
        macro_name @ ("write" | "writeln") => {
            format!("{macro_name}!($1, \"$2\")")
        }
        "consteval_int" => "consteval_int!($1)".to_string(),
        "component" => "component!(path: $1, storage: $2, event: $3);".to_string(),
        "compile_error" => "compile_error!(\"$1\");".to_string(),
        rest => format!("{rest}!($1)"),
    };

    CompletionItemOrderable {
        item: CompletionItem {
            label: format!("{inline_macro_name}!"),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            insert_text: Some(insert_text),
            ..CompletionItem::default()
        },
        relevance: CompletionRelevance::High,
    }
}
