use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_macro::{Diagnostic as DiagnosticV2, Severity as SeverityV2};
use cairo_lang_syntax::node::ids::SyntaxStablePtrId;

mod conversion;
mod expansion;
pub mod inline;
pub mod regular;
mod types;

// <https://github.com/software-mansion/scarb/blob/4e81d1c4498137f80e211c6e2c6a5a6de01c66f2/scarb/src/compiler/plugin/proc_macro/host.rs#L1068-L1083>
fn into_cairo_diagnostics(
    diagnostics: Vec<DiagnosticV2>,
    stable_ptr: SyntaxStablePtrId,
) -> Vec<PluginDiagnostic> {
    diagnostics
        .into_iter()
        .map(|diag| PluginDiagnostic {
            stable_ptr,
            relative_span: None,
            message: diag.message,
            severity: match diag.severity {
                SeverityV2::Error => cairo_lang_diagnostics::Severity::Error,
                SeverityV2::Warning => cairo_lang_diagnostics::Severity::Warning,
            },
        })
        .collect()
}
