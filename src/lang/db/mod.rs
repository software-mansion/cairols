use std::ops::Not;
use std::sync::Arc;

use cairo_lang_defs::db::{
    DefsDatabase, DefsGroup, DefsGroupEx, init_defs_group, try_ext_as_virtual_impl,
};
use cairo_lang_defs::ids::{InlineMacroExprPluginId, MacroPluginId};
use cairo_lang_defs::plugin::MacroPlugin;
use cairo_lang_doc::db::DocDatabase;
use cairo_lang_executable::plugin::executable_plugin_suite;
use cairo_lang_filesystem::cfg::{Cfg, CfgSet};
use cairo_lang_filesystem::db::{ExternalFiles, FilesDatabase, FilesGroup, init_files_group};
use cairo_lang_filesystem::ids::{CrateId, VirtualFile};
use cairo_lang_lowering::db::{
    ExternalCodeSizeEstimator, LoweringDatabase, LoweringGroup, init_lowering_group,
};
use cairo_lang_lowering::utils::InliningStrategy;
use cairo_lang_parser::db::{ParserDatabase, ParserGroup};
use cairo_lang_plugins::plugins::ConfigPlugin;
use cairo_lang_semantic::db::{
    PluginSuiteInput, SemanticDatabase, SemanticGroup, SemanticGroupEx, init_semantic_group,
};
use cairo_lang_semantic::ids::AnalyzerPluginId;
use cairo_lang_semantic::inline_macros::get_default_plugin_suite;
use cairo_lang_semantic::plugin::{InternedPluginSuite, PluginSuite};
use cairo_lang_starknet::starknet_plugin_suite;
use cairo_lang_syntax::node::db::{SyntaxDatabase, SyntaxGroup};
use cairo_lang_test_plugin::test_plugin_suite;
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;
use cairo_lang_utils::{LookupIntern, Upcast};
use cairo_lint::plugin::cairo_lint_allow_plugin_suite;
use itertools::Itertools;
use salsa::{Database, Durability};

pub use self::semantic::*;
pub use self::swapper::*;
pub use self::syntax::*;
use super::proc_macros::db::{ProcMacroDatabase, init_proc_macro_group};
use crate::TRICKS;
use cairo_lint::{LinterDatabase, LinterGroup};

mod semantic;
mod swapper;
mod syntax;

/// The Cairo compiler Salsa database tailored for language server usage.
#[salsa::database(
    DefsDatabase,
    FilesDatabase,
    LoweringDatabase,
    ParserDatabase,
    SemanticDatabase,
    SyntaxDatabase,
    DocDatabase,
    ProcMacroDatabase,
    LsSyntaxDatabase,
    LsSemanticDatabase,
    LinterDatabase
)]
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
        let default_plugin_suite = db.intern_plugin_suite(default_plugin_suite);
        db.set_default_plugins_from_suite(default_plugin_suite);

        // Set default plugins for core to make sure starknet plugin is not applied to it.
        let core_plugin_suite = Self::default_corelib_plugin_suite();
        let core_plugin_suite = db.intern_plugin_suite(core_plugin_suite);
        db.set_override_crate_plugins_from_suite(CrateId::core(&db), core_plugin_suite);

        db
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
        self.salsa_runtime_mut().synthetic_write(Durability::LOW);
    }

    /// Removes the plugins from [`InternedPluginSuite`] for a crate with [`CrateId`].
    pub fn remove_crate_plugin_suite(&mut self, crate_id: CrateId, plugins: &InternedPluginSuite) {
        self.with_plugins_mut(crate_id, |macro_plugins, analyzer_plugins, inline_macro_plugins| {
            remove_plugin_suite(plugins, macro_plugins, analyzer_plugins, inline_macro_plugins)
        })
    }

    /// Adds proc macro plugin suite to the database for a crate with [`CrateInput`] if this
    /// crate exists in the crate configs.
    ///
    /// It *prepends* (with the exception of macro plugins, see the code below) the plugins from
    /// the proc macro plugin suite to appropriate salsa inputs.
    /// It is done to make sure proc macros are resolved first, just like in
    /// [`crate::project::Crate::apply`].
    pub fn add_proc_macro_plugin_suite(&mut self, crate_id: CrateId, plugins: InternedPluginSuite) {
        self.with_plugins_mut(
            crate_id,
            move |macro_plugins, analyzer_plugins, inline_macro_plugins| {
                let maybe_cfg_plugin =
                    macro_plugins.is_empty().not().then(|| macro_plugins.remove(0));
                *macro_plugins = maybe_cfg_plugin
                    .into_iter()
                    .chain(plugins.macro_plugins.iter().cloned())
                    .chain(macro_plugins.iter().cloned())
                    .collect();

                *analyzer_plugins = plugins
                    .analyzer_plugins
                    .iter()
                    .cloned()
                    .chain(analyzer_plugins.iter().cloned())
                    .collect();

                *inline_macro_plugins = plugins
                    .inline_macro_plugins
                    .iter()
                    .map(|(key, id)| (key.clone(), *id))
                    .chain(inline_macro_plugins.iter().map(|(s, id)| (s.clone(), *id)))
                    .collect();
            },
        )
    }

    fn with_plugins_mut(
        &mut self,
        crate_id: CrateId,
        action: impl FnOnce(
            &mut Vec<MacroPluginId>,
            &mut Vec<AnalyzerPluginId>,
            &mut OrderedHashMap<String, InlineMacroExprPluginId>,
        ),
    ) {
        if !self.crate_configs().keys().contains(&crate_id) {
            return;
        }
        let mut macro_plugins = self.crate_macro_plugins(crate_id).to_vec();
        let mut analyzer_plugins = self.crate_analyzer_plugins(crate_id).to_vec();
        let mut inline_macro_plugins =
            Arc::unwrap_or_clone(self.crate_inline_macro_plugins(crate_id));

        action(&mut macro_plugins, &mut analyzer_plugins, &mut inline_macro_plugins);

        assert!(
            macro_plugins.first().is_none_or(|id| id.lookup_intern(self).plugin_type_id()
                == ConfigPlugin::default().plugin_type_id()),
            "cfg plugin must be the first macro plugin"
        );

        self.set_override_crate_macro_plugins(crate_id, macro_plugins.into_iter().collect());
        self.set_override_crate_analyzer_plugins(crate_id, analyzer_plugins.into_iter().collect());
        self.set_override_crate_inline_macro_plugins(crate_id, Arc::new(inline_macro_plugins));
    }

    fn default_global_plugin_suite() -> PluginSuite {
        let tricks = TRICKS.get_or_init(Default::default);

        [
            get_default_plugin_suite(),
            starknet_plugin_suite(),
            test_plugin_suite(),
            executable_plugin_suite(),
            cairo_lint_allow_plugin_suite(),
        ]
        .into_iter()
        .chain(tricks.extra_plugin_suites.iter().flat_map(|f| f()))
        .fold(PluginSuite::default(), |mut acc, suite| {
            acc.add(suite);
            acc
        })
    }

    fn default_corelib_plugin_suite() -> PluginSuite {
        let tricks = TRICKS.get_or_init(Default::default);

        [
            get_default_plugin_suite(),
            test_plugin_suite(),
            executable_plugin_suite(),
            cairo_lint_allow_plugin_suite(),
        ]
        .into_iter()
        .chain(tricks.extra_plugin_suites.iter().flat_map(|f| f()))
        .fold(PluginSuite::default(), |mut acc, suite| {
            acc.add(suite);
            acc
        })
    }
}

fn remove_plugin_suite(
    plugins: &InternedPluginSuite,
    macro_plugins: &mut Vec<MacroPluginId>,
    analyzer_plugins: &mut Vec<AnalyzerPluginId>,
    inline_macro_plugins: &mut OrderedHashMap<String, InlineMacroExprPluginId>,
) {
    macro_plugins.retain(|plugin| !plugins.macro_plugins.contains(plugin));
    analyzer_plugins.retain(|plugin| !plugins.analyzer_plugins.contains(plugin));
    inline_macro_plugins
        .retain(|_, plugin| !plugins.inline_macro_plugins.values().contains(plugin));
}

impl salsa::Database for AnalysisDatabase {}
impl ExternalFiles for AnalysisDatabase {
    fn try_ext_as_virtual(&self, external_id: salsa::InternId) -> Option<VirtualFile> {
        try_ext_as_virtual_impl(self, external_id)
    }
}

impl salsa::ParallelDatabase for AnalysisDatabase {
    fn snapshot(&self) -> salsa::Snapshot<Self> {
        salsa::Snapshot::new(AnalysisDatabase { storage: self.storage.snapshot() })
    }
}

impl Upcast<dyn FilesGroup> for AnalysisDatabase {
    fn upcast(&self) -> &(dyn FilesGroup + 'static) {
        self
    }
}

impl Upcast<dyn SyntaxGroup> for AnalysisDatabase {
    fn upcast(&self) -> &(dyn SyntaxGroup + 'static) {
        self
    }
}

impl Upcast<dyn DefsGroup> for AnalysisDatabase {
    fn upcast(&self) -> &(dyn DefsGroup + 'static) {
        self
    }
}

impl Upcast<dyn SemanticGroup> for AnalysisDatabase {
    fn upcast(&self) -> &(dyn SemanticGroup + 'static) {
        self
    }
}

impl Upcast<dyn LoweringGroup> for AnalysisDatabase {
    fn upcast(&self) -> &(dyn LoweringGroup + 'static) {
        self
    }
}

impl Upcast<dyn ParserGroup> for AnalysisDatabase {
    fn upcast(&self) -> &(dyn ParserGroup + 'static) {
        self
    }
}

impl Upcast<dyn LinterGroup> for AnalysisDatabase {
    fn upcast(&self) -> &(dyn LinterGroup + 'static) {
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
