use std::sync::Arc;

use cairo_lang_defs::db::{
    DefsDatabase, DefsGroup, DefsGroupEx, init_defs_group, try_ext_as_virtual_impl,
};
use cairo_lang_defs::ids::{InlineMacroExprPluginId, MacroPluginId};
use cairo_lang_doc::db::DocDatabase;
use cairo_lang_executable::plugin::executable_plugin_suite;
use cairo_lang_filesystem::cfg::{Cfg, CfgSet};
use cairo_lang_filesystem::db::{
    AsFilesGroupMut, ExternalFiles, FilesDatabase, FilesGroup, init_files_group,
};
use cairo_lang_filesystem::ids::{CrateId, VirtualFile};
use cairo_lang_lowering::db::{LoweringDatabase, LoweringGroup, init_lowering_group};
use cairo_lang_lowering::utils::InliningStrategy;
use cairo_lang_parser::db::{ParserDatabase, ParserGroup};
use cairo_lang_semantic::db::{
    PluginSuiteInput, SemanticDatabase, SemanticGroup, SemanticGroupEx, init_semantic_group,
};
use cairo_lang_semantic::ids::AnalyzerPluginId;
use cairo_lang_semantic::inline_macros::get_default_plugin_suite;
use cairo_lang_semantic::plugin::{InternedPluginSuite, PluginSuite};
use cairo_lang_starknet::starknet_plugin_suite;
use cairo_lang_syntax::node::db::{SyntaxDatabase, SyntaxGroup};
use cairo_lang_test_plugin::test_plugin_suite;
use cairo_lang_utils::Upcast;
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;
use cairo_lint::plugin::cairo_lint_allow_plugin_suite;
use itertools::Itertools;
use salsa::{Database, Durability};

pub use self::semantic::*;
pub use self::syntax::*;
use super::proc_macros::db::{ProcMacroDatabase, init_proc_macro_group};
use crate::TRICKS;

mod lru;
mod semantic;
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
    ProcMacroDatabase
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

        lru::set_lru_capacities(&mut db);

        // Those pluins are relevant for projects with `cairo_project.toml` (e.g. our tests).
        let default_plugin_suite = Self::default_global_plugin_suite();

        let default_plugin_suite = db.intern_plugin_suite(default_plugin_suite);
        db.set_default_plugins_from_suite(default_plugin_suite);

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
    pub fn cancel_all(&mut self) {
        self.salsa_runtime_mut().synthetic_write(Durability::LOW);
    }

    /// Removes the plugins from [`InternedPluginSuite`] for a crate with [`CrateId`].
    pub fn remove_crate_plugin_suite(&mut self, crate_id: CrateId, plugins: &InternedPluginSuite) {
        self.with_plugins_mut(crate_id, |macro_plugins, analyzer_plugins, inline_macro_plugins| {
            remove_plugin_suite(plugins, macro_plugins, analyzer_plugins, inline_macro_plugins)
        })
    }

    /// Adds plugin suit to database.
    pub fn add_crate_plugin_suite(&mut self, crate_id: CrateId, plugins: InternedPluginSuite) {
        self.with_plugins_mut(
            crate_id,
            move |macro_plugins, analyzer_plugins, inline_macro_plugins| {
                add_plugin_suite(plugins, macro_plugins, analyzer_plugins, inline_macro_plugins)
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
        let mut macro_plugins = self.crate_macro_plugins(crate_id).to_vec();
        let mut analyzer_plugins = self.crate_analyzer_plugins(crate_id).to_vec();
        let mut inline_macro_plugins =
            Arc::unwrap_or_clone(self.crate_inline_macro_plugins(crate_id));

        action(&mut macro_plugins, &mut analyzer_plugins, &mut inline_macro_plugins);

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

fn add_plugin_suite(
    plugins: InternedPluginSuite,
    macro_plugins: &mut Vec<MacroPluginId>,
    analyzer_plugins: &mut Vec<AnalyzerPluginId>,
    inline_macro_plugins: &mut OrderedHashMap<String, InlineMacroExprPluginId>,
) {
    macro_plugins.extend_from_slice(&plugins.macro_plugins);
    analyzer_plugins.extend_from_slice(&plugins.analyzer_plugins);
    inline_macro_plugins.extend(Arc::unwrap_or_clone(plugins.inline_macro_plugins));
}

impl salsa::Database for AnalysisDatabase {}

impl salsa::ParallelDatabase for AnalysisDatabase {
    fn snapshot(&self) -> salsa::Snapshot<Self> {
        salsa::Snapshot::new(AnalysisDatabase { storage: self.storage.snapshot() })
    }
}

impl ExternalFiles for AnalysisDatabase {
    fn try_ext_as_virtual(&self, external_id: salsa::InternId) -> Option<VirtualFile> {
        try_ext_as_virtual_impl(self.upcast(), external_id)
    }
}

impl AsFilesGroupMut for AnalysisDatabase {
    fn as_files_group_mut(&mut self) -> &mut (dyn FilesGroup + 'static) {
        self
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
