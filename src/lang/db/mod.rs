use std::collections::HashSet;
use std::sync::Arc;

use cairo_lang_defs::db::{DefsGroup, init_defs_group, try_ext_as_virtual_impl};
use cairo_lang_defs::ids::{InlineMacroExprPluginLongId, MacroPluginLongId};
use cairo_lang_executable::plugin::executable_plugin_suite;
use cairo_lang_filesystem::cfg::{Cfg, CfgSet};
use cairo_lang_filesystem::db::{ExternalFiles, FilesGroup, init_files_group};
use cairo_lang_filesystem::ids::{CrateInput, CrateLongId, VirtualFile};
use cairo_lang_lowering::db::{ExternalCodeSizeEstimator, LoweringGroup, init_lowering_group};
use cairo_lang_lowering::utils::InliningStrategy;
use cairo_lang_parser::db::ParserGroup;
use cairo_lang_semantic::db::{Elongate, PluginSuiteInput, SemanticGroup, init_semantic_group};
use cairo_lang_semantic::ids::AnalyzerPluginLongId;
use cairo_lang_semantic::inline_macros::get_default_plugin_suite;
use cairo_lang_semantic::plugin::PluginSuite;
use cairo_lang_starknet::starknet_plugin_suite;
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_test_plugin::test_plugin_suite;
use cairo_lang_utils::Upcast;
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;
use cairo_lint::LinterGroup;
use cairo_lint::plugin::cairo_lint_allow_plugin_suite;
use salsa::{Database, Durability};

pub use self::semantic::*;
pub use self::swapper::*;
pub use self::syntax::*;
use super::proc_macros::db::init_proc_macro_group;

mod semantic;
mod swapper;
mod syntax;

/// The Cairo compiler Salsa database tailored for language server usage.
#[salsa::db]
#[derive(Clone)]
pub struct AnalysisDatabase {
    storage: salsa::Storage<Self>,
}

impl AnalysisDatabase {
    /// Creates a new instance of the database.
    #[expect(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut db = Self { storage: Default::default() };

        init_files_group(&mut db);
        init_defs_group(&mut db);
        init_semantic_group(&mut db);
        init_lowering_group(&mut db, InliningStrategy::Default);
        // proc-macro-server can be restarted many times but we want to keep these data across
        // multiple server starts, so init it once per database, not per server.
        init_proc_macro_group(&mut db);

        db.set_cfg_set(Self::initial_cfg_set().into());

        // Those plugins are relevant for projects with `cairo_project.toml` (e.g. our tests).
        let default_plugin_suite = Self::default_global_plugin_suite();
        db.set_default_plugins_from_suite(default_plugin_suite);

        // Set default plugins for core to make sure starknet plugin is not applied to it.
        let core_plugin_suite = Self::default_corelib_plugin_suite();
        db.set_override_crate_plugins_from_suite(
            CrateLongId::core().into_crate_input(&db),
            core_plugin_suite,
        );

        db
    }

    pub fn set_override_crate_plugins_from_suite(
        &mut self,
        crate_input: CrateInput,
        plugins: PluginSuite,
    ) {
        let mut overrides = self.macro_plugin_overrides_input().as_ref().clone();
        overrides.insert(
            crate_input.clone(),
            plugins.plugins.into_iter().map(MacroPluginLongId).collect(),
        );
        self.set_macro_plugin_overrides_input(overrides.into());

        let mut overrides = self.analyzer_plugin_overrides_input().as_ref().clone();
        overrides.insert(
            crate_input.clone(),
            plugins.analyzer_plugins.into_iter().map(AnalyzerPluginLongId).collect(),
        );
        self.set_analyzer_plugin_overrides_input(overrides.into());

        let mut overrides = self.inline_macro_plugin_overrides_input().as_ref().clone();
        overrides.insert(
            crate_input,
            Arc::new(
                plugins
                    .inline_macro_plugins
                    .into_iter()
                    .map(|(key, value)| (key, InlineMacroExprPluginLongId(value)))
                    .collect(),
            ),
        );
        self.set_inline_macro_plugin_overrides_input(overrides.into());
    }

    /// Returns the [`CfgSet`] that should be assumed in the initial database state
    /// and in [`CfgSet`] for workspace members.
    /// This enables code fragments tagged with `#[cfg(test)]` and `#[cfg(target: 'test')]` to be
    /// included in analysis by Language Server.
    pub(crate) fn initial_cfg_set() -> CfgSet {
        CfgSet::from_iter([Cfg::name("test"), Cfg::kv("target", "test")])
    }

    /// Returns the [`CfgSet`] that should be assumed for dependencies.
    /// This enables code fragments tagged with `#[cfg(target: 'test')]` to be
    /// included in analysis by Language Server.
    pub(crate) fn initial_cfg_set_for_deps() -> CfgSet {
        CfgSet::from_iter([Cfg::kv("target", "test")])
    }

    /// Trigger cancellation in any background tasks that might still be running.
    /// This method will block until all db snapshots are dropped.
    pub fn cancel_all(&mut self) {
        self.synthetic_write(Durability::LOW);
    }

    /// Removes the plugins from [`InternedPluginSuite`] for a crate with [`CrateId`].
    pub fn remove_crate_plugin_suite(&mut self, crate_input: CrateInput, plugins: PluginSuite) {
        self.with_plugins_mut(
            crate_input,
            |macro_plugins, analyzer_plugins, inline_macro_plugins| {
                let macro_plugins_set: HashSet<_> =
                    plugins.plugins.into_iter().map(MacroPluginLongId).collect();
                let analyzer_plugins_set: HashSet<_> =
                    plugins.analyzer_plugins.into_iter().map(AnalyzerPluginLongId).collect();
                let inline_macro_plugin_set: HashSet<_> = plugins
                    .inline_macro_plugins
                    .into_iter()
                    .map(|(_, arc)| InlineMacroExprPluginLongId(arc))
                    .collect();

                macro_plugins.retain(|plugin| !macro_plugins_set.contains(plugin));
                analyzer_plugins.retain(|plugin| !analyzer_plugins_set.contains(plugin));
                inline_macro_plugins.retain(|_, plugin| !inline_macro_plugin_set.contains(plugin));
            },
        )
    }

    /// Adds plugin suit to database.
    pub fn add_crate_plugin_suite(&mut self, crate_input: CrateInput, plugins: PluginSuite) {
        self.with_plugins_mut(
            crate_input,
            move |macro_plugins, analyzer_plugins, inline_macro_plugins| {
                macro_plugins.extend(plugins.plugins.into_iter().map(MacroPluginLongId));
                analyzer_plugins
                    .extend(plugins.analyzer_plugins.into_iter().map(AnalyzerPluginLongId));
                inline_macro_plugins.extend(
                    plugins
                        .inline_macro_plugins
                        .into_iter()
                        .map(|(key, arc)| (key, InlineMacroExprPluginLongId(arc))),
                );
            },
        )
    }

    fn with_plugins_mut(
        &mut self,
        crate_input: CrateInput,
        action: impl FnOnce(
            &mut Vec<MacroPluginLongId>,
            &mut Vec<AnalyzerPluginLongId>,
            &mut OrderedHashMap<String, InlineMacroExprPluginLongId>,
        ),
    ) {
        let mut macro_plugin_overrides_input =
            Arc::unwrap_or_clone(self.macro_plugin_overrides_input());
        let mut macro_plugins =
            macro_plugin_overrides_input.get(&crate_input).map(|a| a.to_vec()).unwrap_or_default();

        let mut analyzer_plugin_overrides_input =
            Arc::unwrap_or_clone(self.analyzer_plugin_overrides_input());
        let mut analyzer_plugins = self
            .analyzer_plugin_overrides_input()
            .get(&crate_input)
            .map(|a| a.to_vec())
            .unwrap_or_default();

        let mut inline_macro_plugin_overrides_input =
            Arc::unwrap_or_clone(self.inline_macro_plugin_overrides_input());
        let mut inline_macro_plugins = self
            .inline_macro_plugin_overrides_input()
            .get(&crate_input)
            .map(|a| (**a).clone())
            .unwrap_or_default();

        action(&mut macro_plugins, &mut analyzer_plugins, &mut inline_macro_plugins);

        macro_plugin_overrides_input.insert(crate_input.clone(), macro_plugins.into());
        analyzer_plugin_overrides_input.insert(crate_input.clone(), analyzer_plugins.into());
        inline_macro_plugin_overrides_input
            .insert(crate_input.clone(), inline_macro_plugins.into());

        self.set_macro_plugin_overrides_input(macro_plugin_overrides_input.into());
        self.set_analyzer_plugin_overrides_input(analyzer_plugin_overrides_input.into());
        self.set_inline_macro_plugin_overrides_input(inline_macro_plugin_overrides_input.into());
    }

    fn default_global_plugin_suite() -> PluginSuite {
        [
            get_default_plugin_suite(),
            starknet_plugin_suite(),
            test_plugin_suite(),
            executable_plugin_suite(),
            cairo_lint_allow_plugin_suite(),
        ]
        .into_iter()
        .fold(PluginSuite::default(), |mut acc, suite| {
            acc.add(suite);
            acc
        })
    }

    fn default_corelib_plugin_suite() -> PluginSuite {
        [
            get_default_plugin_suite(),
            test_plugin_suite(),
            executable_plugin_suite(),
            cairo_lint_allow_plugin_suite(),
        ]
        .into_iter()
        .fold(PluginSuite::default(), |mut acc, suite| {
            acc.add(suite);
            acc
        })
    }
}

impl salsa::Database for AnalysisDatabase {}
impl ExternalFiles for AnalysisDatabase {
    fn try_ext_as_virtual(&self, external_id: salsa::Id) -> Option<VirtualFile<'_>> {
        try_ext_as_virtual_impl(self, external_id)
    }
}

impl<'db> Upcast<'db, dyn FilesGroup> for AnalysisDatabase {
    fn upcast(&self) -> &(dyn FilesGroup + 'static) {
        self
    }
}

impl<'db> Upcast<'db, dyn SyntaxGroup> for AnalysisDatabase {
    fn upcast(&self) -> &(dyn SyntaxGroup + 'static) {
        self
    }
}

impl<'db> Upcast<'db, dyn DefsGroup> for AnalysisDatabase {
    fn upcast(&self) -> &(dyn DefsGroup + 'static) {
        self
    }
}

impl<'db> Upcast<'db, dyn SemanticGroup> for AnalysisDatabase {
    fn upcast(&self) -> &(dyn SemanticGroup + 'static) {
        self
    }
}

impl<'db> Upcast<'db, dyn LoweringGroup> for AnalysisDatabase {
    fn upcast(&self) -> &(dyn LoweringGroup + 'static) {
        self
    }
}

impl<'db> Upcast<'db, dyn ParserGroup> for AnalysisDatabase {
    fn upcast(&self) -> &(dyn ParserGroup + 'static) {
        self
    }
}

impl<'db> Upcast<'db, dyn LinterGroup> for AnalysisDatabase {
    fn upcast(&self) -> &(dyn LinterGroup + 'static) {
        self
    }
}

impl Elongate for AnalysisDatabase {
    fn elongate(&self) -> &(dyn SemanticGroup + 'static) {
        self
    }
}

// We don't need this implementation at the moment but it's required by `LoweringGroup`.
impl ExternalCodeSizeEstimator for AnalysisDatabase {
    fn estimate_size(
        &self,
        _function_id: cairo_lang_lowering::ids::ConcreteFunctionWithBodyId,
    ) -> cairo_lang_diagnostics::Maybe<isize> {
        cairo_lang_diagnostics::Maybe::Ok(0)
    }
}
