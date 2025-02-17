use cairo_lang_semantic::db::SemanticGroup;
use cairo_lint_core::plugin::cairo_lint_plugin_suite;

use super::db::AnalysisDatabase;
use super::plugins::AnalyzerPluginType;
use crate::config::Config;

pub struct LinterController;

impl LinterController {
    /// Updates the necessary inputs of the [`AnalysisDatabase`] according to the new [`Config`].
    pub fn on_config_change(db: &mut AnalysisDatabase, config: &Config) {
        if config.enable_linter {
            if db.analyzer_plugins().iter().any(|plugin| plugin.is_cairo_lint_plugin()) {
                return;
            }

            db.add_plugin_suite(cairo_lint_plugin_suite());
        } else {
            let mut analyzer_plugins = db.analyzer_plugins();
            analyzer_plugins.retain(|plugin| !plugin.is_cairo_lint_plugin());
            db.set_analyzer_plugins(analyzer_plugins);
        }
    }
}
