use cairo_lang_filesystem::db::FilesGroup;
use cairo_lang_semantic::db::{PluginSuiteInput, SemanticGroup, SemanticGroupEx};
use cairo_lint_core::plugin::cairo_lint_plugin_suite;

use super::db::AnalysisDatabase;
use super::plugins::AnalyzerPluginType;
use crate::config::Config;

pub struct LinterController;

impl LinterController {
    /// Updates the necessary inputs of the [`AnalysisDatabase`] according to the new [`Config`].
    pub fn on_config_change(db: &mut AnalysisDatabase, config: &Config) {
        if config.enable_linter {
            enable_cairo_lint_plugin_for_all_crates(db);
        } else {
            disable_cairo_lint_plugin_for_all_crates(db);
        }
    }
}

fn enable_cairo_lint_plugin_for_all_crates(db: &mut AnalysisDatabase) {
    let cairo_lint_analyzer_plugins =
        db.intern_plugin_suite(cairo_lint_plugin_suite()).analyzer_plugins;

    let default_analyzer_plugins = db.default_analyzer_plugins();

    if !default_analyzer_plugins
        .iter()
        .any(|plugin_id| db.lookup_intern_analyzer_plugin(*plugin_id).0.is_cairo_lint_plugin())
    {
        db.set_default_analyzer_plugins(
            default_analyzer_plugins
                .iter()
                .chain(cairo_lint_analyzer_plugins.iter())
                .cloned()
                .collect(),
        );
    }

    for crate_id in db.crates() {
        let crate_analyzer_plugins = db.crate_analyzer_plugins(crate_id);

        if crate_analyzer_plugins
            .iter()
            .any(|plugin_id| db.lookup_intern_analyzer_plugin(*plugin_id).0.is_cairo_lint_plugin())
        {
            continue;
        }

        let new_analyzer_plugins = crate_analyzer_plugins
            .iter()
            .chain(cairo_lint_analyzer_plugins.iter())
            .cloned()
            .collect();

        db.set_override_crate_analyzer_plugins(crate_id, new_analyzer_plugins);
    }
}

fn disable_cairo_lint_plugin_for_all_crates(db: &mut AnalysisDatabase) {
    let default_analyzer_plugins = db
        .default_analyzer_plugins()
        .iter()
        .filter(|&&plugin_id| !db.lookup_intern_analyzer_plugin(plugin_id).is_cairo_lint_plugin())
        .cloned()
        .collect();

    db.set_default_analyzer_plugins(default_analyzer_plugins);

    for crate_id in db.crates() {
        let new_crate_analyzer_plugins = db
            .crate_analyzer_plugins(crate_id)
            .iter()
            .filter(|&&plugin_id| {
                !db.lookup_intern_analyzer_plugin(plugin_id).is_cairo_lint_plugin()
            })
            .cloned()
            .collect();

        db.set_override_crate_analyzer_plugins(crate_id, new_crate_analyzer_plugins);
    }
}
