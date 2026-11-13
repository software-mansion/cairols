use cairo_lang_filesystem::db::{FilesGroup, get_originating_location};
use cairo_lang_filesystem::ids::SpanInFile;
use cairo_lang_filesystem::span::{TextPosition, TextSpan};
use cairo_lang_syntax::node::SyntaxNode;
use cairo_lang_syntax::node::kind::SyntaxKind;
use lsp_types::Range;

use crate::lang::db::AnalysisDatabase;
use crate::lang::lsp::ToLsp;

/// Returns the range and text of `resultant` (without trivia) in the originating file,
/// cut off at the cursor.
pub fn get_range_and_text_before_cursor<'db>(
    db: &'db AnalysisDatabase,
    resultant: SyntaxNode<'db>,
    cursor: TextPosition,
) -> Option<(Range, &'db str)> {
    let resultant_span = SpanInFile {
        file_id: resultant.stable_ptr(db).file_id(db),
        span: resultant.span_without_trivia(db),
    };

    let SpanInFile { file_id, span } = get_originating_location(db, resultant_span, None);
    let cursor_offset = cursor.offset_in_file(db, file_id)?;
    let span = TextSpan::new(span.start, span.end.min(cursor_offset).max(span.start));

    let range = Range {
        start: span.start.position_in_file(db, file_id)?.to_lsp(),
        end: span.end.position_in_file(db, file_id)?.to_lsp(),
    };
    let typed_text = span.take(db.file_content(file_id)?);

    Some((range, typed_text))
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
