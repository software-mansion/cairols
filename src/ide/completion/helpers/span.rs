use cairo_lang_filesystem::db::get_originating_location;
use cairo_lang_filesystem::ids::SpanInFile;
use cairo_lang_syntax::node::SyntaxNode;
use lsp_types::Range;

use crate::lang::db::AnalysisDatabase;
use crate::lang::lsp::ToLsp;

pub fn get_resultant_range<'db>(
    db: &'db AnalysisDatabase,
    resultant: SyntaxNode<'db>,
) -> Option<Range> {
    let resultant_span =
        SpanInFile { file_id: resultant.stable_ptr(db).file_id(db), span: resultant.span(db) };

    let span_in_file = get_originating_location(db, resultant_span, None);
    Some(Range {
        start: span_in_file.span.start.position_in_file(db, span_in_file.file_id)?.to_lsp(),
        end: span_in_file.span.end.position_in_file(db, span_in_file.file_id)?.to_lsp(),
    })
}
