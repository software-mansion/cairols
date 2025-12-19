use cairo_lang_filesystem::db::get_originating_location;
use cairo_lang_filesystem::ids::SpanInFile;
use cairo_lang_filesystem::span::TextSpan;
use cairo_lang_syntax::node::SyntaxNode;
use cairo_lang_syntax::node::kind::SyntaxKind;
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

pub fn get_empty_arglist_range<'db>(
    db: &'db AnalysisDatabase,
    resultant: SyntaxNode<'db>,
) -> Option<Range> {
    let arg_list = resultant.ancestor_of_kind(db, SyntaxKind::ArgList)?;
    let arg_list_span = arg_list.span(db);
    let empty_span = TextSpan::new(arg_list_span.end, arg_list_span.end);

    let arg_list_span =
        SpanInFile { file_id: arg_list.stable_ptr(db).file_id(db), span: empty_span };

    let span_in_file = get_originating_location(db, arg_list_span, None);

    Some(Range {
        start: span_in_file.span.start.position_in_file(db, span_in_file.file_id)?.to_lsp(),
        end: span_in_file.span.end.position_in_file(db, span_in_file.file_id)?.to_lsp(),
    })
}
