use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_macro::{
    Diagnostic as DiagnosticV2, Severity as SeverityV2, TokenStream as TokenStreamV2,
};
use cairo_lang_primitive_token::{PrimitiveSpan, PrimitiveToken};
use cairo_lang_syntax::node::TypedSyntaxNode;
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::ids::SyntaxStablePtrId;

mod conversion;
mod expansion;
pub mod inline;
pub mod regular;
mod types;

// <https://github.com/software-mansion/scarb/blob/4e81d1c4498137f80e211c6e2c6a5a6de01c66f2/scarb/src/compiler/plugin/proc_macro/ffi.rs#L30-L40>
trait FromSyntaxNode {
    fn from_syntax_node(db: &dyn SyntaxGroup, node: &impl TypedSyntaxNode) -> Self;
}

impl FromSyntaxNode for TokenStreamV2 {
    fn from_syntax_node(db: &dyn SyntaxGroup, node: &impl TypedSyntaxNode) -> Self {
        Self::from_primitive_token_stream(node.as_syntax_node().tokens(db).map(|syntax_node| {
            let content = syntax_node.get_text(db);
            let span = syntax_node.span(db);
            PrimitiveToken {
                content,
                span: Some(PrimitiveSpan {
                    start: span.start.as_u32() as usize,
                    end: span.end.as_u32() as usize,
                }),
            }
        }))
    }
}

// <https://github.com/software-mansion/scarb/blob/4e81d1c4498137f80e211c6e2c6a5a6de01c66f2/scarb/src/compiler/plugin/proc_macro/host.rs#L1068-L1083>
fn into_cairo_diagnostics(
    diagnostics: Vec<DiagnosticV2>,
    stable_ptr: SyntaxStablePtrId,
) -> Vec<PluginDiagnostic> {
    diagnostics
        .into_iter()
        .map(|diag| PluginDiagnostic {
            stable_ptr,
            message: diag.message,
            severity: match diag.severity {
                SeverityV2::Error => cairo_lang_diagnostics::Severity::Error,
                SeverityV2::Warning => cairo_lang_diagnostics::Severity::Warning,
            },
        })
        .collect()
}
