use cairo_lang_defs::plugin::{InlinePluginResult, PluginGeneratedFile};
use cairo_lang_filesystem::ids::{CodeMapping, CodeOrigin};
use cairo_lang_filesystem::span::TextSpan as CairoTextSpan;
use cairo_lang_macro::TokenStream;
use cairo_lang_syntax::node::{TypedStablePtr, TypedSyntaxNode, ast};
use scarb_proc_macro_server_types::methods::expand::ExpandInlineMacroParams;
use scarb_proc_macro_server_types::scope::ProcMacroScope;

use super::{FromSyntaxNode, into_cairo_diagnostics};
use crate::lang::db::AnalysisDatabase;
use crate::lang::proc_macros::db::get_inline_macros_expansion;
use crate::lang::proc_macros::plugins::scarb::conversion::{
    CallSiteLocation, code_mapping_from_proc_macro_server,
};

// <https://github.com/software-mansion/scarb/blob/4e81d1c4498137f80e211c6e2c6a5a6de01c66f2/scarb/src/compiler/plugin/proc_macro/host.rs#L1015-L1059>
pub fn inline_macro_generate_code(
    db: &AnalysisDatabase,
    expansion_context: ProcMacroScope,
    syntax: &ast::ExprInlineMacro,
) -> InlinePluginResult {
    let origin = CodeOrigin::Span(syntax.as_syntax_node().span(db));
    let stable_ptr = syntax.clone().stable_ptr(db).untyped();
    let arguments = syntax.arguments(db);
    let token_stream = TokenStream::from_syntax_node(db, &arguments);
    // region: Modified scarb code
    let result = get_inline_macros_expansion(
        db,
        ExpandInlineMacroParams {
            context: expansion_context,
            name: syntax.path(db).as_syntax_node().get_text_without_trivia(db),
            args: token_stream,
            call_site: CallSiteLocation::new(syntax, db).span,
        },
    );
    // endregion
    // Handle diagnostics.
    let diagnostics = into_cairo_diagnostics(result.diagnostics, stable_ptr);
    let token_stream = result.token_stream.clone();

    if token_stream.is_empty() {
        // Remove original code
        InlinePluginResult { code: None, diagnostics }
    } else {
        let content = token_stream.to_string();
        InlinePluginResult {
            code: Some(PluginGeneratedFile {
                name: "inline_proc_macro".into(),
                code_mappings: result
                    .code_mappings
                    .map(|x| x.into_iter().map(code_mapping_from_proc_macro_server).collect())
                    .unwrap_or_else(|| {
                        vec![CodeMapping { origin, span: CairoTextSpan::from_str(&content) }]
                    }),
                content,
                aux_data: None,
                diagnostics_note: Default::default(),
            }),
            diagnostics,
        }
    }
}
