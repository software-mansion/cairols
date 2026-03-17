use std::ops::Range as StdRange;

use cairo_lang_filesystem::db::FilesGroup;
use cairo_lang_filesystem::ids::FileId;
use cairo_lang_filesystem::span::{TextOffset, TextSpan, TextWidth};
use lsp_types::Range;

use crate::lang::db::AnalysisDatabase;
use crate::lang::lsp::ToLsp;

/// A span expressed as UTF-8 byte offsets within a file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Utf8Span {
    pub start: usize,
    pub end: usize,
}

impl Utf8Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn to_lsp_range<'db>(
        self,
        db: &'db AnalysisDatabase,
        file_id: FileId<'db>,
    ) -> Option<Range> {
        let file_content = db.file_content(file_id)?;
        let start = TextOffset::START.add_width(TextWidth::at(file_content, self.start));
        let end = TextOffset::START.add_width(TextWidth::at(file_content, self.end));
        let span = TextSpan::new(start, end);

        span.position_in_file(db, file_id).map(|span| span.to_lsp())
    }
}

impl From<StdRange<usize>> for Utf8Span {
    fn from(value: StdRange<usize>) -> Self {
        Self::new(value.start, value.end)
    }
}
