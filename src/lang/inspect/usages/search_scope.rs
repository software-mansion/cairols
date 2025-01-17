use std::collections::HashMap;
use std::sync::Arc;

use cairo_lang_defs::db::DefsGroup;
use cairo_lang_filesystem::db::FilesGroup;
use cairo_lang_filesystem::ids::FileId;
use cairo_lang_filesystem::span::TextSpan;

use crate::lang::db::AnalysisDatabase;

#[derive(Clone, Default)]
pub struct SearchScope {
    /// A collection of all files constituting this search scope, with optional text spans to
    /// narrow down searching ranges.
    entries: HashMap<FileId, Option<TextSpan>>,
}

impl SearchScope {
    /// Builds a new empty search scope.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Builds a search scope spanning an entire set of analysed crates.
    #[tracing::instrument(skip_all)]
    pub fn everything(db: &AnalysisDatabase) -> Self {
        let mut this = Self::empty();
        for crate_id in db.crates() {
            for &module_id in db.crate_modules(crate_id).iter() {
                if let Ok(file_id) = db.module_main_file(module_id) {
                    this.entries.insert(file_id, None);
                }
            }
        }
        this
    }

    /// Creates an iterator over all files and the optional search scope text spans.
    pub fn files_and_spans(&self) -> impl Iterator<Item = (FileId, Option<TextSpan>)> + use<'_> {
        self.entries.iter().map(|(&file, &span)| (file, span))
    }

    /// Creates an iterator over all files, their contents and the optional search scope text spans.
    pub fn files_contents_and_spans<'a, 'b>(
        &'a self,
        db: &'b AnalysisDatabase,
    ) -> impl Iterator<Item = (FileId, Arc<str>, Option<TextSpan>)> + use<'a, 'b> {
        self.files_and_spans().filter_map(move |(file, span)| {
            let text = db.file_content(file)?;
            Some((file, text, span))
        })
    }
}
