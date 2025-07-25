use cairo_lang_filesystem::db::FilesGroup;
use cairo_lang_filesystem::ids::Directory;
use cairo_lang_semantic::db::{PluginSuiteInput, SemanticGroup, SemanticGroupEx};
use cairo_lint::plugin::cairo_lint_plugin_suite_without_metadata_validation;

use super::db::AnalysisDatabase;
use super::plugins::AnalyzerPluginType;
use crate::config::Config;
use crate::project::ConfigsRegistry;

pub struct LinterController;

impl LinterController {
    /// Updates the necessary inputs of the [`AnalysisDatabase`] according to the new [`Config`].
    pub fn on_config_change(
        db: &mut AnalysisDatabase,
        config: &Config,
        configs_registry: &ConfigsRegistry,
    ) {
        if config.enable_linter {
            enable_cairo_lint_plugin_for_all_crates(db, configs_registry);
        } else {
            disable_cairo_lint_plugin_for_all_crates(db);
        }
    }
}

fn enable_cairo_lint_plugin_for_all_crates(
    db: &mut AnalysisDatabase,
    configs_registry: &ConfigsRegistry,
) {
    let default_cairo_lint_analyzer_plugins = db
        .intern_plugin_suite(
            cairo_lint_plugin_suite_without_metadata_validation(Default::default()),
        )
        .analyzer_plugins;
    let default_analyzer_plugins = db.default_analyzer_plugins();

    if !default_analyzer_plugins
        .iter()
        .any(|plugin_id| db.lookup_intern_analyzer_plugin(*plugin_id).0.is_cairo_lint_plugin())
    {
        db.set_default_analyzer_plugins(
            default_analyzer_plugins
                .iter()
                .chain(default_cairo_lint_analyzer_plugins.iter())
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

        let lint_config = db
            .crate_config(crate_id)
            .and_then(
                |config| {
                    if let Directory::Real(root) = config.root { Some(root) } else { None }
                },
            )
            .and_then(|root| configs_registry.config_for_file(&root))
            .map(|member_config| member_config.lint)
            .unwrap_or_default();

        let new_analyzer_plugins = crate_analyzer_plugins
            .iter()
            .chain(
                db.intern_plugin_suite(cairo_lint_plugin_suite_without_metadata_validation(
                    lint_config,
                ))
                .analyzer_plugins
                .iter(),
            )
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
