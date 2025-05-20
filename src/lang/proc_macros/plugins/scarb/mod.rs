use itertools::Itertools;

use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_filesystem::span::{TextOffset as CairoTextOffset, TextWidth as CairoTextWidth};
use cairo_lang_macro::{Diagnostic as DiagnosticV2, Severity as SeverityV2, TextSpan};
use cairo_lang_syntax::node::SyntaxNode;
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::ids::SyntaxStablePtrId;
use cairo_lang_syntax::node::stable_ptr::SyntaxStablePtr;
use cairo_lang_utils::LookupIntern;

mod conversion;
mod expansion;
pub mod inline;
pub mod regular;
mod types;

// <https://github.com/software-mansion/scarb/blob/741336e8abdcc2e13f852b65c5ce37ae2dd83a5a/scarb/src/compiler/plugin/proc_macro/v2/host/conversion.rs#L37-L123>
pub fn into_cairo_diagnostics(
    db: &dyn SyntaxGroup,
    diagnostics: Vec<DiagnosticV2>,
    call_site_stable_ptr: SyntaxStablePtrId,
) -> Vec<PluginDiagnostic> {
    let root_stable_ptr = get_root_ptr(db, call_site_stable_ptr);
    let root_syntax_node = root_stable_ptr.lookup(db);
    diagnostics
        .into_iter()
        .map(|diag| {
            // Resolve the best possible diagnostic location.
            // If the diagnostic span is provided, find the encompassing node and compute the span relative to that node.
            // Fall back to the call-site stable pointer, if diagnostic span is not provided or if the encompassing node cannot be found.
            let (node_stable_ptr, relative_span) = if let Some(span) = diag.span {
                if let Some(node) = find_encompassing_node(&root_syntax_node, db, &span) {
                    let relative_span = compute_relative_span(&node, db, &span);
                    (node.stable_ptr(db), Some(relative_span))
                } else {
                    (call_site_stable_ptr, None)
                }
            } else {
                (call_site_stable_ptr, None)
            };

            PluginDiagnostic {
                stable_ptr: node_stable_ptr,
                relative_span,
                message: diag.message,
                severity: match diag.severity {
                    SeverityV2::Error => cairo_lang_diagnostics::Severity::Error,
                    SeverityV2::Warning => cairo_lang_diagnostics::Severity::Warning,
                },
            }
        })
        .collect_vec()
}

fn get_root_ptr(db: &dyn SyntaxGroup, stable_ptr: SyntaxStablePtrId) -> SyntaxStablePtrId {
    let mut current_ptr = stable_ptr;

    while let SyntaxStablePtr::Child { parent: parent_ptr, kind: _, key_fields: _, index: _ } =
        current_ptr.lookup_intern(db)
    {
        current_ptr = parent_ptr;
    }
    current_ptr
}

/// Finds the most specific node that fully encompasses the given text span.
/// Returns `None` if unable to find such node.
pub fn find_encompassing_node(
    root_syntax_node: &SyntaxNode,
    db: &dyn SyntaxGroup,
    span: &TextSpan,
) -> Option<SyntaxNode> {
    let start_offset =
        CairoTextOffset::default().add_width(CairoTextWidth::new_for_testing(span.start));
    let end_offset =
        CairoTextOffset::default().add_width(CairoTextWidth::new_for_testing(span.end));

    let mut current_node = root_syntax_node.lookup_offset(db, start_offset);
    while current_node.span(db).end < end_offset {
        if let Some(parent) = current_node.parent(db) {
            current_node = parent;
        } else {
            return None;
        }
    }
    Some(current_node)
}

/// Computes a span relative to `node` from an `absolute_span`.
fn compute_relative_span(
    node: &SyntaxNode,
    db: &dyn SyntaxGroup,
    absolute_span: &TextSpan,
) -> cairo_lang_filesystem::span::TextSpan {
    let offset = node.offset(db).as_u32();
    let relative_start = absolute_span.start.saturating_sub(offset);
    let relative_end = absolute_span.end.saturating_sub(offset);
    cairo_lang_filesystem::span::TextSpan {
        start: CairoTextOffset::default()
            .add_width(CairoTextWidth::new_for_testing(relative_start)),
        end: CairoTextOffset::default().add_width(CairoTextWidth::new_for_testing(relative_end)),
    }
}
