use cairo_lang_defs::db::DefsGroup;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::lookup_item::LookupItemEx;
use cairo_lang_syntax::node::Token;
use cairo_lang_syntax::node::ast::PathSegment;
use if_chain::if_chain;
use lsp_types::{CompletionItem, InsertTextFormat};

use crate::ide::completion::expr::selector::expr_selector;
use crate::lang::analysis_context::AnalysisContext;
use crate::lang::db::AnalysisDatabase;
use crate::lang::text_matching::text_matches;

pub fn macro_call_completions(
    db: &AnalysisDatabase,
    ctx: &AnalysisContext<'_>,
) -> Vec<CompletionItem> {
    if_chain!(
        if let Some(lookup_item_id) = ctx.lookup_item_id;
        if let Some(function_id) = lookup_item_id.function_with_body();
        if db.function_body(function_id).is_ok();

        if let Some(path) = expr_selector(db, &ctx.node);
        // Currently inline macros can not be imported/exported
        if let [PathSegment::Simple(path_segment)] = path.segments(db).elements(db).as_slice();

        then {
            let crate_id = ctx.module_file_id.0.owning_crate(db);

            let inline_plugins = db.crate_inline_macro_plugins(crate_id);

            let typed = path_segment.ident(db).token(db).text(db).to_string();

            inline_plugins
                .iter()
                .filter(|(name,_)| text_matches(name, &typed))
                .map(|(plugin_name, _)| snippet_completions_for_inline_plugins(plugin_name))
                .collect()
        } else {
            Default::default()
        }
    )
}

fn snippet_completions_for_inline_plugins(inline_macro_name: &str) -> CompletionItem {
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

    CompletionItem {
        label: format!("{inline_macro_name}!"),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        insert_text: Some(insert_text),
        ..CompletionItem::default()
    }
}
