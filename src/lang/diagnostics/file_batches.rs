use std::collections::HashSet;
use std::num::NonZero;

use cairo_lang_defs::db::DefsGroup;
use cairo_lang_filesystem::db::FilesGroup;
use cairo_lang_filesystem::ids::{FileId, FileLongId};
use lsp_types::Url;

use crate::lang::db::AnalysisDatabase;
use crate::lang::lsp::LsProtoGroup;

/// Finds all analyzable on disk files in `db` that are open and need to be analysed ASAP,
/// thus _primary_.
#[tracing::instrument(skip_all)]
pub fn find_primary_files<'db>(
    db: &'db AnalysisDatabase,
    open_files: &HashSet<Url>,
) -> HashSet<FileId<'db>> {
    open_files
        .iter()
        .filter_map(|uri| db.file_for_url(uri))
        // 1. Filter out files that don't belong to any crate, e.g. removed modules.
        // 2. We only want to process on disk files.
        //    Relevant virtual files will be processed as a result of processing on disk files.
        .filter(|file_id| {
            db.file_modules(*file_id).is_ok() && matches!(file_id.long(db), FileLongId::OnDisk(_))
        })
        .collect()
}

/// Finds all analyzable on disk files in `db` that are **not** primary.
#[tracing::instrument(skip_all)]
pub fn find_secondary_files<'db>(
    db: &'db AnalysisDatabase,
    primary_files: &HashSet<FileId<'db>>,
) -> Vec<FileId<'db>> {
    let mut result = HashSet::new();
    for crate_id in db.crates() {
        for module_id in db.crate_modules(crate_id).iter() {
            // Schedule only on disk module main files for refreshing.
            // All other related files will be refreshed along with it in a single job.
            if let Ok(file) = db.module_main_file(*module_id) {
                if matches!(file.long(db), FileLongId::OnDisk(_)) && !primary_files.contains(&file)
                {
                    result.insert(file);
                }
            }
        }
    }
    result.into_iter().collect()
}

/// Returns `n` optimally distributed batches of the input.
pub fn batches<'db>(input: &[FileId<'db>], n: NonZero<usize>) -> Vec<Vec<FileId<'db>>> {
    let n = n.get();
    let batches = (1..=n)
        .map(|offset| input.iter().copied().skip(offset - 1).step_by(n).collect())
        .collect::<Vec<_>>();
    debug_assert_eq!(batches.len(), n);
    batches
}
