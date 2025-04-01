use std::collections::HashSet;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use cairo_lang_defs::db::DefsGroup;
use cairo_lang_filesystem::db::FilesGroup;
use cairo_lang_filesystem::ids::FileId;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;
use cairo_lang_utils::{Intern, LookupIntern};
use lsp_types::Url;
use tracing::{error, warn};

use crate::config::Config;
use crate::env_config;
use crate::lang::db::AnalysisDatabase;
use crate::lang::lsp::LsProtoGroup;
use crate::lang::proc_macros::controller::ProcMacroClientController;
use crate::lang::proc_macros::db::ProcMacroGroup;
use crate::project::ProjectController;

/// Swaps entire [`AnalysisDatabase`] with empty one periodically.
///
/// Salsa does not perform GC, which means that whatever computes in query groups stays in memory
/// forever (with little exception being the LRU mechanism).
/// The usage patterns of Salsa queries in the Cairo compiler cause Salsa to steadily allocate
/// new memory, which is a problem if the user is having a long coding session.
/// This object realises a nuclear GC strategy by wiping the entire analysis database from time to
/// time.
///
/// The swapping period can be configured with environment variables.
/// Consult [`env_config::db_replace_interval`] for more information.
///
/// The new database has a clean state.
/// It is expected that diagnostics will be refreshed on it as quickly as possible, otherwise
/// the entire workspace would be recompiled at an undetermined time leading to bad UX delays.
pub struct AnalysisDatabaseSwapper {
    last_replace: SystemTime,
    db_replace_interval: Duration,
}

impl AnalysisDatabaseSwapper {
    /// Creates a new `AnalysisDatabaseSwapper`.
    pub fn new() -> Self {
        Self {
            last_replace: SystemTime::now(),
            db_replace_interval: env_config::db_replace_interval(),
        }
    }

    /// Checks if enough time has passed since last db swap, and if so, swaps the database.
    pub fn maybe_swap(
        &mut self,
        db: &mut AnalysisDatabase,
        open_files: &HashSet<Url>,
        project_controller: &mut ProjectController,
        proc_macro_client_controller: &ProcMacroClientController,
        config: &Config,
    ) {
        let Ok(elapsed) = self.last_replace.elapsed() else {
            warn!("system time went backwards, skipping db swap");

            // Reset last replace time because in this place the old value will never make sense.
            self.last_replace = SystemTime::now();

            return;
        };

        if elapsed <= self.db_replace_interval {
            // Not enough time passed since the last swap.
            return;
        }

        self.swap(db, open_files, project_controller, proc_macro_client_controller, config)
    }

    /// Swaps the database.
    #[tracing::instrument(skip_all)]
    fn swap(
        &mut self,
        db: &mut AnalysisDatabase,
        open_files: &HashSet<Url>,
        project_controller: &mut ProjectController,
        proc_macro_client_controller: &ProcMacroClientController,
        config: &Config,
    ) {
        let Ok(new_db) = catch_unwind(AssertUnwindSafe(|| {
            let mut new_db = AnalysisDatabase::new();

            self.migrate_default_plugins(&mut new_db, db);
            self.migrate_proc_macro_state(&mut new_db, db);
            self.migrate_file_overrides(&mut new_db, db, open_files);

            project_controller.migrate_crates_to_new_db(
                db,
                proc_macro_client_controller,
                config.enable_linter,
            );

            new_db
        })) else {
            error!("caught panic when preparing new db for swap");
            return;
        };

        *db = new_db;

        self.last_replace = SystemTime::now();
    }

    /// Copies current default macro plugins into new db.
    fn migrate_default_plugins(&self, new_db: &mut AnalysisDatabase, old_db: &AnalysisDatabase) {
        new_db.set_default_macro_plugins(
            old_db
                .default_macro_plugins()
                .iter()
                .map(|&id| new_db.intern_macro_plugin(old_db.lookup_intern_macro_plugin(id)))
                .collect(),
        );

        new_db.set_default_analyzer_plugins(
            old_db
                .default_analyzer_plugins()
                .iter()
                .map(|&id| new_db.intern_analyzer_plugin(old_db.lookup_intern_analyzer_plugin(id)))
                .collect(),
        );

        new_db.set_default_inline_macro_plugins(Arc::new(
            old_db
                .default_inline_macro_plugins()
                .iter()
                .map(|(name, &id)| {
                    (
                        name.clone(),
                        new_db.intern_inline_macro_plugin(
                            old_db.lookup_intern_inline_macro_plugin(id),
                        ),
                    )
                })
                .collect(),
        ));
    }

    /// Copies current proc macro state into new db.
    fn migrate_proc_macro_state(&self, new_db: &mut AnalysisDatabase, old_db: &AnalysisDatabase) {
        new_db.set_proc_macro_server_status(old_db.proc_macro_server_status());

        // TODO(#6646): Probably this should not be part of migration as it will be ever growing,
        // but diagnostics going crazy every 5 minutes are no better.
        new_db.set_attribute_macro_resolution(old_db.attribute_macro_resolution());
        new_db.set_derive_macro_resolution(old_db.derive_macro_resolution());
        new_db.set_inline_macro_resolution(old_db.inline_macro_resolution());
    }

    /// Makes sure that all open files exist in the new db, with their current changes.
    fn migrate_file_overrides(
        &self,
        new_db: &mut AnalysisDatabase,
        old_db: &AnalysisDatabase,
        open_files: &HashSet<Url>,
    ) {
        let overrides = old_db.file_overrides();
        let mut new_overrides: OrderedHashMap<FileId, Arc<str>> = Default::default();
        for uri in open_files {
            let Some(file_id) = old_db.file_for_url(uri) else {
                // This branch is hit for open files that have never been seen by the old db.
                // This is a strange condition, but it is OK to just not think about such files
                // here.
                continue;
            };
            let new_file_id = file_id.lookup_intern(old_db).intern(new_db);
            if let Some(content) = overrides.get(&file_id) {
                new_overrides.insert(new_file_id, content.clone());
            }
        }
        new_db.set_file_overrides(Arc::new(new_overrides));
    }
}
