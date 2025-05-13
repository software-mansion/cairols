use std::collections::HashMap;

use cairo_lang_diagnostics::Diagnostics;
use cairo_lang_filesystem::db::FilesGroup;
use cairo_lang_filesystem::ids::FileId;
use cairo_lang_lowering::db::LoweringGroup;
use cairo_lang_lowering::diagnostic::LoweringDiagnostic;
use cairo_lang_parser::ParserDiagnostic;
use cairo_lang_parser::db::ParserGroup;
use cairo_lang_semantic::SemanticDiagnostic;
use cairo_lang_semantic::db::SemanticGroup;
use lsp_types::{Diagnostic, Url};

use crate::lang::db::{AnalysisDatabase, LsSemanticGroup};
use crate::lang::diagnostics::lsp::map_cairo_diagnostics_to_lsp;
use crate::lang::lsp::LsProtoGroup;

/// Result of processing a single on disk file `root_on_disk_file` and virtual files that are its
/// descendants in search for diagnostics.
///
/// ## Comparisons
///
/// Diagnostics in this structure are stored as Arcs that directly come from Salsa caches.
/// This means that equality comparisons of `FileDiagnostics` are efficient.
///
/// ## Virtual files
///
/// When collecting diagnostics using [`FilesDiagnostics::collect`], all virtual files related
/// to the given `file` will also be visited and their diagnostics collected.
#[derive(Clone, PartialEq, Eq)]
pub struct FilesDiagnostics {
    pub root_on_disk_file: (Url, FileId),
    pub parser: Diagnostics<ParserDiagnostic>,
    pub semantic: Diagnostics<SemanticDiagnostic>,
    pub lowering: Diagnostics<LoweringDiagnostic>,
}

impl FilesDiagnostics {
    /// Collects all diagnostics kinds by processing an on disk `root_on_disk_file` together with
    /// virtual files that are its descendants.
    pub fn collect(db: &AnalysisDatabase, root_on_disk_file: FileId) -> Option<Self> {
        let root_on_disk_file_url = db.url_for_file(root_on_disk_file)?;

        let mut semantic_file_diagnostics: Vec<SemanticDiagnostic> = vec![];
        let mut lowering_file_diagnostics: Vec<LoweringDiagnostic> = vec![];
        let mut parser_file_diagnostics: Vec<ParserDiagnostic> = vec![];

        let (files_to_process, modules_to_process) =
            db.file_and_subfiles_with_corresponding_modules(root_on_disk_file)?;

        for module_id in modules_to_process.into_iter() {
            semantic_file_diagnostics
                .extend(db.module_semantic_diagnostics(module_id).unwrap_or_default().get_all());
            lowering_file_diagnostics
                .extend(db.module_lowering_diagnostics(module_id).unwrap_or_default().get_all());
        }

        for file_id in files_to_process.into_iter() {
            parser_file_diagnostics.extend(db.file_syntax_diagnostics(file_id).get_all());
        }

        Some(FilesDiagnostics {
            root_on_disk_file: (root_on_disk_file_url, root_on_disk_file),
            parser: Diagnostics::from_iter(parser_file_diagnostics),
            semantic: Diagnostics::from_iter(semantic_file_diagnostics),
            lowering: Diagnostics::from_iter(lowering_file_diagnostics),
        })
    }

    /// Converts all diagnostics from this [`FilesDiagnostics`] to mapping from [`Url`] and
    /// [`FileId`] to [`Diagnostic`].
    ///
    /// The key in the mapping refers to either the processed on disk file or
    /// any of the virtual files originating from the processed file.
    pub fn to_lsp(
        &self,
        db: &AnalysisDatabase,
        trace_macro_diagnostics: bool,
    ) -> (Url, HashMap<(Url, FileId), Vec<Diagnostic>>) {
        let mut diagnostics = HashMap::new();
        map_cairo_diagnostics_to_lsp(
            db as &dyn FilesGroup,
            &mut diagnostics,
            &self.parser,
            trace_macro_diagnostics,
        );
        map_cairo_diagnostics_to_lsp(
            db as &dyn SemanticGroup,
            &mut diagnostics,
            &self.semantic,
            trace_macro_diagnostics,
        );
        map_cairo_diagnostics_to_lsp(
            db as &dyn SemanticGroup,
            &mut diagnostics,
            &self.lowering,
            trace_macro_diagnostics,
        );

        // In our tests, we often await diagnostics for an on disk file,
        // even when they are supposed to be empty.
        // Processed file is the only on disk file in here - ensure we send
        // empty diagnostics when appropriate.
        if !diagnostics.contains_key(&self.root_on_disk_file) {
            diagnostics.insert(self.root_on_disk_file.clone(), Vec::new());
        }

        (self.root_on_disk_file.0.clone(), diagnostics)
    }
}
