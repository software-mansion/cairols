use cairo_lang_defs::db::DefsGroup;
use cairo_lang_semantic::items::function_with_body::FunctionWithBodySemantic;
use cairo_lang_semantic::lookup_item::LookupItemEx;
use cairo_lang_syntax::node::Token;
use cairo_lang_syntax::node::ast::PathSegment;
use itertools::Itertools;
use lsp_types::{CompletionItem, InsertTextFormat};

use crate::ide::completion::expr::selector::expr_selector;
use crate::ide::completion::helpers::binary_expr::dot_rhs::dot_expr_rhs;
use crate::ide::completion::{CompletionItemOrderable, CompletionRelevance};
use crate::lang::analysis_context::AnalysisContext;
use crate::lang::db::AnalysisDatabase;
use crate::lang::text_matching::text_matches;

pub fn macro_call_completions<'db>(
    db: &'db AnalysisDatabase,
    ctx: &AnalysisContext<'db>,
    was_node_corrected: bool,
) -> Vec<CompletionItemOrderable> {
    if dot_expr_rhs(db, &ctx.node, was_node_corrected).is_none()
        && let Some(lookup_item_id) = ctx.lookup_item_id
        && let Some(function_id) = lookup_item_id.function_with_body()
        && db.function_body(function_id).is_ok()
        && let Some(path) = expr_selector(db, &ctx.node)
        // Currently inline macros can not be imported/exported
        && let [PathSegment::Simple(path_segment)] =
            path.segments(db).elements(db).take(2).collect_vec().as_slice()
    {
        let crate_id = ctx.module_file_id.0.owning_crate(db);

        let inline_plugins = db.crate_inline_macro_plugins(crate_id);

        let typed = path_segment.ident(db).token(db).text(db).to_string();

        inline_plugins
            .iter()
            .filter(|(name, _)| text_matches(name, &typed))
            .map(|(plugin_name, _)| snippet_completions_for_inline_plugins(plugin_name))
            .collect()
    } else {
        Default::default()
    }
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
