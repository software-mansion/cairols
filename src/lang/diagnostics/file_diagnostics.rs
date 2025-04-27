use std::collections::{HashMap, HashSet};
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

use cairo_lang_defs::db::DefsGroup;
use cairo_lang_defs::ids::ModuleId;
use cairo_lang_diagnostics::Diagnostics;
use cairo_lang_filesystem::db::FilesGroup;
use cairo_lang_filesystem::ids::FileId;
use cairo_lang_lowering::db::LoweringGroup;
use cairo_lang_lowering::diagnostic::LoweringDiagnostic;
use cairo_lang_parser::ParserDiagnostic;
use cairo_lang_parser::db::ParserGroup;
use cairo_lang_semantic::SemanticDiagnostic;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_utils::LookupIntern;
use lsp_types::{Diagnostic, Url};
use tracing::{error, info_span};

use crate::lang::db::AnalysisDatabase;
use crate::lang::diagnostics::lsp::map_cairo_diagnostics_to_lsp;
use crate::lang::lsp::LsProtoGroup;
use crate::server::panic::is_cancelled;

/// Result of processing a single file in search for diagnostics.
///
/// ## Comparisons
///
/// Diagnostics in this structure are stored as Arcs that directly come from Salsa caches.
/// This means that equality comparisons of `FileDiagnostics` are efficient.
///
/// ## Virtual files
///
/// When collecting diagnostics using [`FileDiagnostics::collect`], all virtual files related
/// to the given `file` will also be visited and their diagnostics collected.
#[derive(Clone, PartialEq, Eq)]
pub struct FileDiagnostics {
    pub processed_file_url: Url,
    pub parser: Diagnostics<ParserDiagnostic>,
    pub semantic: Diagnostics<SemanticDiagnostic>,
    pub lowering: Diagnostics<LoweringDiagnostic>,
}

impl FileDiagnostics {
    /// Collects all diagnostics kinds by processing `file` and constructs a new `FileDiagnostics`.
    ///
    /// The `processed_modules` in/out parameter is used to avoid constructing two overlapping
    /// `FileDiagnostics` in a single batch.
    /// Any [`ModuleId`] present in this collection will be skipped (leaving empty diagnostics in
    /// constructed `FileDiagnostics`), and new module IDs will be added.
    pub fn collect(
        db: &AnalysisDatabase,
        processed_file: FileId,
        processed_modules: &mut HashSet<ModuleId>,
    ) -> Option<Self> {
        macro_rules! query {
            ($query:expr) => {
                info_span!(stringify!($query)).in_scope(|| {
                    catch_unwind(AssertUnwindSafe(|| $query)).map_err(|err| {
                        if is_cancelled(err.as_ref()) {
                            resume_unwind(err);
                        } else {
                            error!(
                                "caught panic when computing diagnostics for file: {:?}",
                                processed_file.lookup_intern(db)
                            );
                            err
                        }
                    })
                })
            };
        }

        let processed_file_url = query!(db.url_for_file(processed_file)).ok()??;
        let module_ids = query!(db.file_modules(processed_file)).ok()?.ok()?;

        let mut semantic_file_diagnostics: Vec<SemanticDiagnostic> = vec![];
        let mut lowering_file_diagnostics: Vec<LoweringDiagnostic> = vec![];

        for &module_id in module_ids.iter() {
            if !processed_modules.contains(&module_id) {
                semantic_file_diagnostics.extend(
                    query!(db.module_semantic_diagnostics(module_id))
                        .map(Result::unwrap_or_default)
                        .unwrap_or_default()
                        .get_all(),
                );
                lowering_file_diagnostics.extend(
                    query!(db.module_lowering_diagnostics(module_id))
                        .map(Result::unwrap_or_default)
                        .unwrap_or_default()
                        .get_all(),
                );

                processed_modules.insert(module_id);
            }
        }

        // TODO: collect parser diags for virtual files - check diags for generated files
        let parser_file_diagnostics =
            query!(db.file_syntax_diagnostics(processed_file)).unwrap_or_default();

        Some(FileDiagnostics {
            processed_file_url,
            parser: parser_file_diagnostics,
            semantic: Diagnostics::from_iter(semantic_file_diagnostics),
            lowering: Diagnostics::from_iter(lowering_file_diagnostics),
        })
    }

    /// Converts all diagnostics from this [`FileDiagnostics`] to mapping from [`Url`] to
    /// [`Diagnostic`] .
    pub fn to_lsp(
        &self,
        db: &AnalysisDatabase,
        trace_macro_diagnostics: bool,
    ) -> (Url, HashMap<Url, Vec<Diagnostic>>) {
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

        // In our tests, we often await diagnostics for a given non-virtual file,
        // even when they are supposed to be empty.
        // Processed file is the only non-virtual file in here - ensure we send
        // empty diagnostics when appropriate.
        if !diagnostics.contains_key(&self.processed_file_url) {
            diagnostics.insert(self.processed_file_url.clone(), Vec::new());
        }

        (self.processed_file_url.clone(), diagnostics)
    }
}
