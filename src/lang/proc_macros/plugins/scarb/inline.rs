use cairo_lang_defs::plugin::{InlinePluginResult, PluginGeneratedFile};
use cairo_lang_filesystem::ids::{CodeMapping, CodeOrigin};
use cairo_lang_filesystem::span::TextSpan as CairoTextSpan;
use cairo_lang_macro::AllocationContext;
use cairo_lang_syntax::node::{TypedSyntaxNode, ast};
use salsa::Database;
use scarb_proc_macro_server_types::methods::expand::ExpandInlineMacroParams;
use scarb_proc_macro_server_types::scope::ProcMacroScope;

use super::into_cairo_diagnostics;
use crate::lang::proc_macros::db::get_inline_macros_expansion;
use crate::lang::proc_macros::plugins::scarb::conversion::{
    CallSiteLocation, code_mapping_from_proc_macro_server,
};
use crate::lang::proc_macros::plugins::scarb::types::TokenStreamBuilder;

// <https://github.com/software-mansion/scarb/blob/4e81d1c4498137f80e211c6e2c6a5a6de01c66f2/scarb/src/compiler/plugin/proc_macro/host.rs#L1015-L1059>
pub fn inline_macro_generate_code<'db>(
    db: &'db dyn Database,
    expansion_context: ProcMacroScope,
    syntax: &ast::ExprInlineMacro<'db>,
    fingerprint: u64,
) -> InlinePluginResult<'db> {
    let call_site = CallSiteLocation::new(syntax, db);
    let ctx = AllocationContext::default();
    let arguments = syntax.arguments(db);
    let mut token_stream_builder = TokenStreamBuilder::new(db);
    token_stream_builder.add_node(arguments.as_syntax_node());
    let token_stream = token_stream_builder.build(&ctx);
    // region: Modified scarb code
    let inline_macro_name = syntax.path(db).as_syntax_node().get_text_without_trivia(db);
    let result = get_inline_macros_expansion(
        db,
        ExpandInlineMacroParams {
            context: expansion_context,
            name: inline_macro_name.to_string(db),
            args: token_stream,
            call_site: call_site.span,
        },
        fingerprint,
    );
    // endregion
    // Handle diagnostics.
    let diagnostics = into_cairo_diagnostics(db, result.diagnostics, call_site.stable_ptr);
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
                    // region: Modified scarb code
                    // Scarb returns an empty vector for v1 macros, but our solution is better.
                    .unwrap_or_else(|| {
                        vec![CodeMapping {
                            origin: CodeOrigin::Span(
                                syntax.as_syntax_node().span_without_trivia(db),
                            ),
                            span: CairoTextSpan::from_str(&content),
                        }]
                    }),
                // endregion
                content,
                aux_data: None,
                diagnostics_note: Some(format!(
                    "this error originates in the inline macro: `{}`",
                    inline_macro_name.to_string(db),
                )),
                is_unhygienic: false,
            }),
            diagnostics,
        }
    }
}
