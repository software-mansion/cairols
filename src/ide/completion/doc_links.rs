use cairo_lang_doc::db::DocGroup;
use cairo_lang_filesystem::span::{TextOffset, TextPosition, TextSpan};
use cairo_lang_syntax::node::TypedSyntaxNode;
use cairo_lang_syntax::node::ast::{ExprPath, PathSegment};

use super::helpers::item::{CompletionItemOrderable, first_segment_completion_candidates};
use super::path::path_prefix_completions;
use crate::ide::doc_links::{DocLinkCursorContext, parse_doc_link_path};
use crate::lang::analysis_context::AnalysisContext;
use crate::lang::db::AnalysisDatabase;
use crate::lang::text_matching::text_matches;

pub fn doc_link_completions<'db>(
    db: &'db AnalysisDatabase,
    ctx: &AnalysisContext<'db>,
    cursor_position: TextPosition,
) -> Vec<CompletionItemOrderable> {
    doc_link_completions_ex(db, ctx, cursor_position).unwrap_or_default()
}

fn doc_link_completions_ex<'db>(
    db: &'db AnalysisDatabase,
    ctx: &AnalysisContext<'db>,
    cursor_position: TextPosition,
) -> Option<Vec<CompletionItemOrderable>> {
    let doc_token = ctx.node;

    let file_id = doc_token.stable_ptr(db).file_id(db);
    let cursor_offset = cursor_position.offset_in_file(db, file_id)?;
    let cursor_span = TextSpan::cursor(cursor_offset);
    let link = db
        .get_embedded_markdown_links(doc_token)
        .into_iter()
        .find(|link| link.dest_span.is_some_and(|span| span.contains(cursor_span)))?;

    let dest_text = link.dest_text?.to_string();
    let dest_span = link.dest_span?;
    let expr_path = parse_doc_link_path(db, dest_text.as_str())?;
    let (prefix_segments, typed_segment) =
        resolve_prefix_and_typed_segment(db, &expr_path, &dest_text, dest_span, cursor_offset)?;

    if prefix_segments.is_empty() {
        return Some(
            first_segment_completion_candidates(db, ctx, typed_segment.as_str())
                .into_iter()
                .map(|candidate| candidate.into_path_completion())
                .collect(),
        );
    }

    let completions: Vec<CompletionItemOrderable> =
        path_prefix_completions(db, ctx, prefix_segments)?
            .into_iter()
            .filter(|completion| {
                text_matches(completion.item.label.clone(), typed_segment.as_str())
            })
            .collect();

    Some(completions)
}

fn resolve_prefix_and_typed_segment<'db>(
    db: &'db AnalysisDatabase,
    expr_path: &ExprPath<'db>,
    dest_text: &'db str,
    dest_span: TextSpan,
    cursor_offset: TextOffset,
) -> Option<(Vec<PathSegment<'db>>, String)> {
    let cursor_ctx = DocLinkCursorContext::new(db, expr_path, dest_span, dest_text, cursor_offset)?;

    let mut segments = cursor_ctx.segments_up_to_cursor(db)?;
    let last_seg = segments.pop()?;

    Some((segments, get_typed_segment_text(db, &last_seg)?))
}

fn get_typed_segment_text<'db>(
    db: &'db AnalysisDatabase,
    segment: &PathSegment<'db>,
) -> Option<String> {
    match segment {
        PathSegment::Simple(simple) => Some(simple.as_syntax_node().get_text(db).to_string()),
        PathSegment::WithGenericArgs(_) => None,
        PathSegment::Missing(_) => Some(String::new()),
    }
}
