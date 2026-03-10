use std::collections::HashSet;
use std::num::NonZero;
use std::path::PathBuf;

use cairo_lang_defs::db::DefsGroup;
use cairo_lang_filesystem::db::FilesGroup;
use cairo_lang_filesystem::ids::{FileId, FileLongId};
use lsp_types::Url;

use crate::lang::db::AnalysisDatabase;
use crate::lang::lsp::LsProtoGroup;
use crate::toolchain::scarb::SCARB_TOML;

/// Finds all analyzable on disk files in `db` that are open and need to be analysed ASAP,
/// thus _primary_.
#[tracing::instrument(skip_all)]
pub fn find_primary_files<'db>(
    db: &'db AnalysisDatabase,
    open_files: &HashSet<Url>,
    tracked_scarb_manifests: &HashSet<PathBuf>,
) -> HashSet<FileId<'db>> {
    open_files
        .iter()
        .cloned()
        .chain(tracked_scarb_manifests.iter().filter_map(|path| Url::from_file_path(path).ok()))
        .filter_map(|uri| db.file_for_url(&uri))
        // We only want to process on disk files.
        // Relevant virtual files will be processed as a result of processing on disk files.
        .filter(|file_id| {
            let FileLongId::OnDisk(path) = file_id.long(db) else { return false };

            // 1. Process Cairo source files that belong to any crate, excluding removed modules.
            // 2. Process open Scarb manifests through the dedicated Scarb manifest diagnostics path.
            db.file_modules(*file_id).is_ok()
                || path.file_name().and_then(|name| name.to_str()) == Some(SCARB_TOML)
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
        for module_id in db.crate_modules(*crate_id).iter() {
            // Schedule only on disk module main files for refreshing.
            // All other related files will be refreshed along with it in a single job.
            if let Ok(file) = db.module_main_file(*module_id)
                && matches!(file.long(db), FileLongId::OnDisk(_))
                && !primary_files.contains(&file)
            {
                result.insert(file);
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

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::fs;

    use tempfile::tempdir;

    use super::find_primary_files;
    use crate::lang::{db::AnalysisDatabase, lsp::LsProtoGroup};

    #[test]
    fn includes_tracked_scarb_manifests_even_when_not_open() {
        let dir = tempdir().unwrap();
        let manifest_path = dir.path().join("Scarb.toml");

        fs::write(&manifest_path, "[package]\nname = \"test\"\nversion = \"0.1.0\"\n").unwrap();

        let db = AnalysisDatabase::new();
        let manifest_uri = lsp_types::Url::from_file_path(&manifest_path).unwrap();
        let manifest_file = db.file_for_url(&manifest_uri).unwrap();

        let primary_files =
            find_primary_files(&db, &HashSet::new(), &HashSet::from([manifest_path]));

        assert!(primary_files.contains(&manifest_file));
    }
}
