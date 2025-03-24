use cairo_lang_filesystem::ids::CrateLongId;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use crate::lang::db::AnalysisDatabase;
use crate::lang::proc_macros::controller::ProcMacroClientController;
use crate::project::{Crate, ManifestRegistry};

#[derive(Default)]
pub struct ProjectModel {
    // The two fields below keep exactly the same information;
    // therefore, their contents should be kept synchronised.
    // We keep both of them for efficiency and ease of use.
    /// Mapping from a workspace root to crates contained in the dependency graph of that workspace.
    loaded_workspaces: HashMap<PathBuf, HashMap<CrateLongId, Crate>>,
    /// Mapping from a crate to roots of workspaces that contained this crate in their dependency graphs.
    loaded_crates: HashMap<CrateLongId, HashSet<PathBuf>>,
}

impl ProjectModel {
    pub fn clear_loaded_workspaces(&mut self) {
        self.loaded_workspaces.clear();
        self.loaded_crates.clear();
    }

    pub fn load_workspace(
        &mut self,
        db: &mut AnalysisDatabase,
        workspace_crates: Vec<Crate>,
        workspace_dir: PathBuf,
        proc_macro_controller: &ProcMacroClientController,
        manifest_registry: &ManifestRegistry,
        enable_linter: bool,
    ) {
        let workspace_crates = workspace_crates.into_iter().map(|cr| (cr.long_id(), cr)).collect();
        if let Some(old_crates) = self.loaded_workspaces.get(&workspace_dir) {
            if old_crates == &workspace_crates {
                return;
            }

            // Static because the borrow checker.
            ProjectModel::remove_crates(&mut self.loaded_crates, &workspace_dir, old_crates);
        };

        self.add_crates(workspace_crates, &workspace_dir);

        self.apply_changes_to_db(db, proc_macro_controller, manifest_registry, enable_linter);
    }

    fn remove_crates(
        loaded_crates: &mut HashMap<CrateLongId, HashSet<PathBuf>>,
        workspace_dir: &Path,
        old_crates: &HashMap<CrateLongId, Crate>,
    ) {
        for old_cr in old_crates.keys() {
            loaded_crates.entry(old_cr.clone()).and_modify(|paths| {
                paths.remove(workspace_dir);
            });
        }
    }

    fn add_crates(&mut self, workspace_crates: HashMap<CrateLongId, Crate>, workspace_dir: &Path) {
        for cr in workspace_crates.keys() {
            self.loaded_crates.entry(cr.clone()).or_default().insert(workspace_dir.to_path_buf());
        }

        self.loaded_workspaces.insert(workspace_dir.to_path_buf(), workspace_crates);
    }

    fn apply_changes_to_db(
        &mut self,
        db: &mut AnalysisDatabase,
        proc_macro_controller: &ProcMacroClientController,
        manifest_registry: &ManifestRegistry,
        enable_linter: bool,
    ) {
        for (cr, workspaces) in &self.loaded_crates {
            let same_crates: Vec<_> = workspaces
                .iter()
                .map(|ws| {
                    self.loaded_workspaces
                        .get(ws)
                        .expect("loaded_crates and loaded_workspaces are expected to be synchronised at this point")
                        .get(cr)
                        .expect("loaded_crates and loaded_workspaces are expected to be synchronised at this point")
                })
                .collect();

            let merged_builtin_plugins = same_crates
                .iter()
                .map(|cr| cr.builtin_plugins.clone())
                .reduce(|mut x, y| {
                    x.extend(y);
                    x
                })
                .expect("same_crates cannot be empty")
                .clone();
            let merged_settings = same_crates
                .iter()
                .map(|cr| cr.settings.clone())
                .reduce(|mut x, y| {
                    x.cfg_set =
                        x.cfg_set.map(|cfg_set| cfg_set.union(&y.cfg_set.unwrap_or_default()));
                    x.dependencies.extend(y.dependencies);
                    x
                })
                .expect("same_crates cannot be empty");
            let cr = Crate {
                settings: merged_settings,
                builtin_plugins: merged_builtin_plugins,
                ..same_crates.into_iter().next().expect("same_crates cannot be empty").clone()
            };

            let cr_long_id = cr.long_id();

            let proc_macro_plugin_suite =
                proc_macro_controller.proc_macro_plugin_suite_for_crate(&cr_long_id);
            let lint_config = manifest_registry
                .config_for_file(&cr.root)
                .filter(|_| enable_linter)
                .map(|member_config| member_config.lint);

            cr.apply(db, lint_config, proc_macro_plugin_suite.cloned());
        }
    }
}
