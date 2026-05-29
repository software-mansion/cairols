use std::collections::HashSet;

use cairo_lang_defs::db::{DefsGroup, defs_group_input};
use cairo_lang_defs::ids::ModuleId;
use cairo_lang_filesystem::db::{FilesGroup, files_group_input};
use cairo_lang_semantic::db::{SemanticGroup, semantic_group_input};
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;
use lsp_types::Url;
use salsa::Setter;

use crate::lang::db::AnalysisDatabase;
use crate::lang::lsp::LsProtoGroup;
use crate::lang::proc_macros::controller::ProcMacroClientController;
use crate::lang::proc_macros::db::ProcMacroGroup;
use crate::project::ProjectController;

/// Builds a fresh analysis database that contains only state required to keep analysis usable.
#[tracing::instrument(skip_all)]
pub fn migrate_to_fresh_database(
    old_db: &AnalysisDatabase,
    open_files: &HashSet<Url>,
    project_controller: &ProjectController,
    proc_macro_client_controller: &ProcMacroClientController,
) -> AnalysisDatabase {
    let mut new_db = AnalysisDatabase::new();

    migrate_default_plugins(&mut new_db, old_db);
    migrate_proc_macro_state(&mut new_db, old_db);
    migrate_file_overrides(&mut new_db, old_db, open_files);
    migrate_crate_model_inputs(&mut new_db, old_db);

    new_db
}

/// Copies current default macro plugins into new db.
fn migrate_default_plugins(new_db: &mut AnalysisDatabase, old_db: &AnalysisDatabase) {
    defs_group_input(new_db).set_default_macro_plugins(new_db).to(Some(
        old_db.default_macro_plugins().iter().map(|&id| id.long(old_db).clone()).collect(),
    ));
    defs_group_input(new_db).set_default_inline_macro_plugins(new_db).to(Some(
        old_db
            .default_inline_macro_plugins()
            .iter()
            .map(|(name, &id)| (name.clone(), id.long(old_db).clone()))
            .collect(),
    ));
    semantic_group_input(new_db).set_default_analyzer_plugins(new_db).to(Some(
        old_db.default_analyzer_plugins().iter().map(|&id| id.long(old_db).clone()).collect(),
    ));
}

/// Copies current proc macro state into new db.
fn migrate_proc_macro_state(new_db: &mut AnalysisDatabase, old_db: &AnalysisDatabase) {
    let old_db_input = old_db.proc_macro_input();
    new_db
        .proc_macro_input()
        .set_proc_macro_server_status(new_db)
        .to(old_db_input.proc_macro_server_status(old_db));

    // TODO(#6646): Probably this should not be part of migration as it will be ever growing,
    // but diagnostics going crazy every 5 minutes are no better.
    new_db
        .proc_macro_input()
        .set_attribute_macro_resolution(new_db)
        .to(old_db_input.attribute_macro_resolution(old_db).clone());
    new_db
        .proc_macro_input()
        .set_derive_macro_resolution(new_db)
        .to(old_db_input.derive_macro_resolution(old_db).clone());
    new_db
        .proc_macro_input()
        .set_inline_macro_resolution(new_db)
        .to(old_db_input.inline_macro_resolution(old_db).clone());
}

/// Makes sure that open files and project-model virtual crate roots exist in the new db.
fn migrate_file_overrides(
    new_db: &mut AnalysisDatabase,
    old_db: &AnalysisDatabase,
    open_files: &HashSet<Url>,
) {
    let overrides = old_db.file_overrides();
    let mut new_overrides: OrderedHashMap<_, _> = Default::default();
    for uri in open_files {
        let Some(file_id) = old_db.file_for_url(uri) else {
            // This branch is hit for open files that have never been seen by the old db.
            // This is a strange condition, but it is OK to just not think about such files
            // here.
            continue;
        };
        let file_input = file_id.long(old_db).into_file_input(old_db);
        if let Some(content) = overrides.get(&file_id) {
            new_overrides.insert(file_input, content.to_string().into());
        }
    }
    for crate_id in old_db.crate_configs().keys() {
        let Ok(file_id) = old_db.module_main_file(ModuleId::CrateRoot(*crate_id)) else {
            continue;
        };
        let file_input = file_id.long(old_db).into_file_input(old_db);
        if let Some(content) = overrides.get(&file_id) {
            new_overrides.insert(file_input, content.to_string().into());
        }
    }
    files_group_input(new_db).set_file_overrides(new_db).to(Some(new_overrides));
}

/// Copies the currently configured project crates without migrating query outputs.
fn migrate_crate_model_inputs(new_db: &mut AnalysisDatabase, old_db: &AnalysisDatabase) {
    files_group_input(new_db)
        .set_crate_configs(new_db)
        .to(files_group_input(old_db).crate_configs(old_db).clone());

    defs_group_input(new_db)
        .set_macro_plugin_overrides(new_db)
        .to(Some(old_db.macro_plugin_overrides_input().clone()));
    defs_group_input(new_db)
        .set_inline_macro_plugin_overrides(new_db)
        .to(Some(old_db.inline_macro_plugin_overrides_input().clone()));
    semantic_group_input(new_db)
        .set_analyzer_plugin_overrides(new_db)
        .to(Some(old_db.analyzer_plugin_overrides_input().clone()));
}
