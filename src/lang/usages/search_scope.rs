use std::collections::HashMap;

use cairo_lang_defs::db::DefsGroup;
use cairo_lang_filesystem::db::FilesGroup;
use cairo_lang_filesystem::ids::FileId;
use cairo_lang_filesystem::span::TextSpan;

use crate::lang::db::{AnalysisDatabase, LsSemanticGroup};

#[derive(Clone, Default)]
pub struct SearchScope<'db> {
    /// A collection of all files constituting this search scope, with optional text spans to
    /// narrow down searching ranges.
    entries: HashMap<FileId<'db>, Option<TextSpan>>,
}

impl<'db> SearchScope<'db> {
    /// Builds a new empty search scope.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Builds a search scope spanning an entire set of analysed crates.
    #[tracing::instrument(skip_all)]
    pub fn everything(db: &'db AnalysisDatabase) -> Self {
        let mut this = Self::empty();
        for crate_id in db.crates() {
            for &module_id in db.crate_modules(*crate_id).iter() {
                if let Ok(file_id) = db.module_main_file(module_id)
                    && let Some((files, _)) =
                        db.file_and_subfiles_with_corresponding_modules(file_id)
                {
                    this.entries.extend(files.iter().copied().map(|f| (f, None)));
                }
            }
        }
        this
    }

    /// Builds a search scope spanning an entire single file.
    pub fn file(file: FileId<'db>) -> Self {
        Self { entries: [(file, None)].into() }
    }

    /// Builds a search scope spanning a slice of a single file.
    pub fn file_span(file: FileId<'db>, span: TextSpan) -> Self {
        Self { entries: [(file, Some(span))].into() }
    }

    /// Builds a search scope spanning an entire single file and files generated from it.
    pub fn file_with_subfiles(db: &'db AnalysisDatabase, file: FileId<'db>) -> Self {
        let mut this = Self { entries: [(file, None)].into() };

        if let Some((files, _)) = db.file_and_subfiles_with_corresponding_modules(file) {
            this.entries.extend(files.iter().copied().map(|f| (f, None)));
        }

        this
    }

    /// Builds a search scope spanning slices of files.
    pub fn files_spans(files: HashMap<FileId<'db>, Option<TextSpan>>) -> Self {
        Self { entries: files }
    }

    /// Creates an iterator over all files, their contents and the optional search scope text spans.
    pub fn files_contents_and_spans<'a>(
        &'a self,
        db: &'db AnalysisDatabase,
    ) -> impl Iterator<Item = (FileId<'db>, &'db str, Option<TextSpan>)> + use<'a, 'db> {
        self.entries.iter().map(|(&file, &span)| (file, span)).filter_map(move |(file, span)| {
            let text = db.file_content(file)?;
            Some((file, text, span))
        })
    }
}
