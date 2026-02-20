use cairo_lang_filesystem::ids::{FileKind, FileLongId, SmolStrId, VirtualFile};
use cairo_lang_filesystem::span::{TextOffset, TextSpan, TextWidth};
use cairo_lang_parser::db::ParserGroup;
use cairo_lang_syntax::node::TypedSyntaxNode;
use cairo_lang_syntax::node::ast::{self, ExprPath, PathSegment};
use cairo_lang_utils::Intern;

use crate::lang::db::AnalysisDatabase;

/// Parse a markdown doc-link destination into an expression path.
pub fn parse_doc_link_path<'db>(
    db: &'db AnalysisDatabase,
    dest_text: &str,
) -> Option<ExprPath<'db>> {
    let virtual_file = FileLongId::Virtual(VirtualFile {
        parent: None,
        name: SmolStrId::from(db, "doc-link"),
        content: SmolStrId::from(db, dest_text),
        code_mappings: Default::default(),
        kind: FileKind::Expr,
        original_item_removed: false,
    })
    .intern(db);

    let expr = db.file_expr_syntax(virtual_file).ok()?;
    let ast::Expr::Path(expr_path) = expr else {
        return None;
    };

    Some(expr_path)
}

pub struct DocLinkCursorContext<'db> {
    segments: Vec<PathSegment<'db>>,
    dest_text: &'db str,
    relative_cursor_offset: usize,
}

impl<'db> DocLinkCursorContext<'db> {
    pub fn new(
        db: &'db AnalysisDatabase,
        expr_path: &ExprPath<'db>,
        dest_span: TextSpan,
        dest_text: &'db str,
        cursor_offset: TextOffset,
    ) -> Option<Self> {
        let absolute_cursor = TextSpan::cursor(cursor_offset);
        if !dest_span.contains(absolute_cursor) {
            return None;
        }

        let relative_cursor_offset = (cursor_offset - dest_span.start).as_u32() as usize;

        Some(Self {
            segments: expr_path.segments(db).elements_vec(db),
            relative_cursor_offset,
            dest_text,
        })
    }

    pub fn relative_cursor(&self) -> TextOffset {
        TextOffset::START.add_width(TextWidth::new_for_testing(self.relative_cursor_offset as u32))
    }

    pub fn cursor_on_path_separator(&self, dest_text: &str) -> bool {
        dest_text.as_bytes().get(self.relative_cursor_offset) == Some(&b':')
    }

    pub fn segments_up_to_cursor(
        &self,
        db: &'db AnalysisDatabase,
    ) -> Option<Vec<PathSegment<'db>>> {
        if self.cursor_on_path_separator(self.dest_text) {
            return None;
        }
        let relative_cursor = self.relative_cursor();
        let relative_cursor_span = TextSpan::cursor(relative_cursor);

        let current_segment_index = self
            .segments
            .iter()
            .position(|segment| segment.as_syntax_node().span(db).contains(relative_cursor_span))?;

        Some(self.segments.iter().take(current_segment_index + 1).cloned().collect())
    }
}
