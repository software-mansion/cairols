use std::collections::{HashMap, HashSet};
use std::ops::Not;
use std::sync::{Arc, RwLock};

use cairo_lang_defs::db::{
    GranularInlineMacroPluginOverride, GranularInlineMacroPluginOverrideStorage,
    GranularInlineMacroPluginOverrideView, GranularMacroPluginOverride,
    GranularMacroPluginOverrideStorage, GranularMacroPluginOverrideView, defs_group_input,
    init_defs_group, init_external_files, register_granular_inline_macro_plugin_override_view,
    register_granular_macro_plugin_override_view,
};
use cairo_lang_defs::ids::{InlineMacroExprPluginLongId, MacroPluginLongId};
use cairo_lang_defs::plugin::MacroPlugin;
use cairo_lang_executable_plugin::executable_plugin_suite;
use cairo_lang_filesystem::cfg::{Cfg, CfgSet};
use cairo_lang_filesystem::db::{
    CrateConfigurationInput, FilesGroup, GranularCrateConfig, GranularCrateConfigView,
    GranularCrateConfigStorage, GranularFileContentView, GranularFileContents, files_group_input, init_files_group,
    register_files_group_view, register_granular_crate_config_view,
};
use cairo_lang_filesystem::ids::{ArcStr, CrateId, CrateInput, CrateLongId, FileId, FileInput};
use cairo_lang_lowering::db::init_lowering_group;
use cairo_lang_lowering::optimizations::config::Optimizations;
use cairo_lang_lowering::utils::InliningStrategy;
use cairo_lang_plugins::plugins::ConfigPlugin;
use cairo_lang_semantic::db::{
    GranularAnalyzerPluginOverride, GranularAnalyzerPluginOverrideStorage,
    GranularAnalyzerPluginOverrideView, PluginSuiteInput, init_semantic_group,
    register_granular_analyzer_plugin_override_view, semantic_group_input,
};
use cairo_lang_semantic::ids::AnalyzerPluginLongId;
use cairo_lang_semantic::inline_macros::get_default_plugin_suite;
use cairo_lang_semantic::plugin::PluginSuite;
use cairo_lang_starknet::starknet_plugin_suite;
use cairo_lang_test_plugin::test_plugin_suite;
use cairo_lang_utils::Intern;
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;
use cairo_lint::plugin::cairo_lint_allow_plugin_suite;
use itertools::Itertools;
use salsa::{Database, Durability, Setter};

pub use self::semantic::*;
pub use self::swapper::*;
pub use self::syntax::*;

mod memory_report;
mod semantic;
mod swapper;
mod syntax;

pub(crate) use self::memory_report::build_memory_usage_report;

/// The Cairo compiler Salsa database tailored for language server usage.
#[salsa::db]
#[derive(Clone)]
pub struct AnalysisDatabase {
    storage: salsa::Storage<Self>,
    current_granular_crate_configs: Arc<RwLock<OrderedHashMap<CrateInput, CrateConfigurationInput>>>,
    current_granular_crate_plugin_suites:
        Arc<RwLock<OrderedHashMap<CrateInput, CratePluginSuiteInputs>>>,
    current_granular_crate_plugin_suite_fingerprints:
        Arc<RwLock<OrderedHashMap<CrateInput, CratePluginSuiteFingerprint>>>,
    granular_crate_configs: Arc<RwLock<HashMap<CrateInput, GranularCrateConfig>>>,
    granular_file_contents: Arc<RwLock<HashMap<FileInput, GranularFileContents>>>,
    granular_macro_plugin_overrides: GranularMacroPluginOverrideStorage,
    granular_inline_macro_plugin_overrides: GranularInlineMacroPluginOverrideStorage,
    granular_analyzer_plugin_overrides: GranularAnalyzerPluginOverrideStorage,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct CratePluginSuiteInputs {
    macro_plugins: Arc<[MacroPluginLongId]>,
    analyzer_plugins: Arc<[AnalyzerPluginLongId]>,
    inline_macro_plugins: Arc<OrderedHashMap<String, InlineMacroExprPluginLongId>>,
}

impl From<PluginSuite> for CratePluginSuiteInputs {
    fn from(value: PluginSuite) -> Self {
        Self {
            macro_plugins: Arc::from(
                value.plugins.into_iter().map(MacroPluginLongId).collect::<Vec<_>>(),
            ),
            analyzer_plugins: Arc::from(
                value.analyzer_plugins.into_iter().map(AnalyzerPluginLongId).collect::<Vec<_>>(),
            ),
            inline_macro_plugins: Arc::new(
                value
                    .inline_macro_plugins
                    .into_iter()
                    .map(|(key, value)| (key, InlineMacroExprPluginLongId(value)))
                    .collect(),
            ),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct CratePluginSuiteFingerprint {
    macro_plugins: Arc<[usize]>,
    analyzer_plugins: Arc<[usize]>,
    inline_macro_plugins: Arc<OrderedHashMap<String, usize>>,
}

impl CratePluginSuiteFingerprint {
    fn from_plugin_suite(value: &PluginSuite) -> Self {
        Self {
            macro_plugins: Arc::from(
                value.plugins.iter().map(|plugin| plugin_identity(plugin)).collect::<Vec<_>>(),
            ),
            analyzer_plugins: Arc::from(
                value
                    .analyzer_plugins
                    .iter()
                    .map(|plugin| plugin_identity(plugin))
                    .collect::<Vec<_>>(),
            ),
            inline_macro_plugins: Arc::new(
                value
                    .inline_macro_plugins
                    .iter()
                    .map(|(name, plugin)| (name.clone(), plugin_identity(plugin)))
                    .collect(),
            ),
        }
    }

    fn from_inputs(value: &CratePluginSuiteInputs) -> Self {
        Self {
            macro_plugins: Arc::from(
                value
                    .macro_plugins
                    .iter()
                    .map(|plugin| plugin_identity(&plugin.0))
                    .collect::<Vec<_>>(),
            ),
            analyzer_plugins: Arc::from(
                value
                    .analyzer_plugins
                    .iter()
                    .map(|plugin| plugin_identity(&plugin.0))
                    .collect::<Vec<_>>(),
            ),
            inline_macro_plugins: Arc::new(
                value
                    .inline_macro_plugins
                    .iter()
                    .map(|(name, plugin)| (name.clone(), plugin_identity(&plugin.0)))
                    .collect(),
            ),
        }
    }
}

fn plugin_identity<T: ?Sized>(plugin: &Arc<T>) -> usize {
    Arc::as_ptr(plugin) as *const () as usize
}

impl AnalysisDatabase {
    /// Creates a new instance of the database.
    pub fn new() -> Self {
        let mut db = Self {
            storage: Default::default(),
            current_granular_crate_configs: Default::default(),
            current_granular_crate_plugin_suites: Default::default(),
            current_granular_crate_plugin_suite_fingerprints: Default::default(),
            granular_crate_configs: Default::default(),
            granular_file_contents: Default::default(),
            granular_macro_plugin_overrides: Default::default(),
            granular_inline_macro_plugin_overrides: Default::default(),
            granular_analyzer_plugin_overrides: Default::default(),
        };

        register_files_group_view(&db);
        register_granular_crate_config_view(&db);
        register_granular_macro_plugin_override_view(&db);
        register_granular_inline_macro_plugin_override_view(&db);
        register_granular_analyzer_plugin_override_view(&db);
        init_external_files(&mut db);
        init_files_group(&mut db);
        init_defs_group(&mut db);
        init_semantic_group(&mut db);
        init_lowering_group(
            &mut db,
            Optimizations::enabled_with_default_movable_functions(InliningStrategy::Default),
            None,
        );
        files_group_input(&db).set_cfg_set(&mut db).to(Some(Self::initial_cfg_set()));

        // Those plugins are relevant for projects with `cairo_project.toml` (e.g. our tests).
        let default_plugin_suite = Self::default_global_plugin_suite();
        db.set_default_plugins_from_suite(default_plugin_suite);

        // Set default plugins for core to make sure starknet plugin is not applied to it.
        let core_plugin_suite = Self::default_corelib_plugin_suite();
        db.set_override_crate_plugins_from_suite(
            CrateLongId::core(&db).into_crate_input(&db),
            core_plugin_suite,
        );

        db
    }

    fn granular_file_contents_handle(
        &self,
        file_input: &FileInput,
    ) -> Option<GranularFileContents> {
        self.granular_file_contents.read().unwrap().get(file_input).copied()
    }

    fn granular_crate_config_handle(
        &self,
        crate_input: &CrateInput,
    ) -> Option<GranularCrateConfig> {
        self.granular_crate_configs.read().unwrap().get(crate_input).copied()
    }

    fn granular_file_contents_handle_for_file<'db>(
        &'db self,
        file_id: FileId<'db>,
    ) -> Option<GranularFileContents> {
        let file_input = self.file_input(file_id).clone();
        self.granular_file_contents_handle(&file_input)
    }

    fn ensure_granular_crate_config_handle_for_input_impl(
        &mut self,
        crate_input: CrateInput,
    ) -> (GranularCrateConfig, bool) {
        if let Some(handle) = self.granular_crate_config_handle(&crate_input) {
            return (handle, false);
        }

        let handle = GranularCrateConfig::new(self, None);
        self.granular_crate_configs.write().unwrap().insert(crate_input, handle);
        (handle, true)
    }

    pub fn ensure_granular_crate_config_handle_for_input(
        &mut self,
        crate_input: CrateInput,
    ) -> GranularCrateConfig {
        let (handle, inserted) =
            self.ensure_granular_crate_config_handle_for_input_impl(crate_input);
        if inserted {
            self.bump_granular_crate_configs_revision();
        }
        handle
    }

    fn bump_granular_crate_configs_revision(&mut self) {
        let next_revision =
            files_group_input(self).granular_crate_configs_revision(self).saturating_add(1);
        files_group_input(self)
            .set_granular_crate_configs_revision(self)
            .with_durability(Durability::HIGH)
            .to(next_revision);
    }

    fn bump_granular_file_contents_revision(&mut self) {
        let next_revision =
            files_group_input(self).granular_file_contents_revision(self).saturating_add(1);
        files_group_input(self)
            .set_granular_file_contents_revision(self)
            .with_durability(Durability::HIGH)
            .to(next_revision);
    }

    pub fn set_granular_crate_config_for_input(
        &mut self,
        crate_input: CrateInput,
        config: Option<CrateConfigurationInput>,
    ) {
        let was_inserted = {
            let mut current = self.current_granular_crate_configs.write().unwrap();
            if current.get(&crate_input) == config.as_ref() {
                return;
            }

            let inserted = !current.contains_key(&crate_input) && config.is_some();
            match config.as_ref() {
                Some(config) => {
                    current.insert(crate_input.clone(), config.clone());
                }
                None => {
                    current.swap_remove(&crate_input);
                }
            }
            inserted
        };

        if was_inserted {
            self.bump_granular_crate_configs_revision();
        }

        match config {
            Some(config) => {
                let handle = self.ensure_granular_crate_config_handle_for_input(crate_input);
                handle.set_config(self).with_durability(Durability::HIGH).to(Some(config));
            }
            None => {
                if let Some(handle) = self.granular_crate_config_handle(&crate_input) {
                    handle.set_config(self).with_durability(Durability::HIGH).to(None);
                }
            }
        }
    }

    pub fn sync_granular_crate_configs(
        &mut self,
        crate_configs: OrderedHashMap<CrateInput, CrateConfigurationInput>,
    ) {
        let (inserted_inputs, changed_inputs, removed_inputs) = {
            let mut current = self.current_granular_crate_configs.write().unwrap();
            if *current == crate_configs {
                return;
            }

            let inserted_inputs = crate_configs
                .keys()
                .filter(|crate_input| !current.contains_key(*crate_input))
                .cloned()
                .collect_vec();
            let changed_inputs = crate_configs
                .iter()
                .filter(|(crate_input, config)| current.get(*crate_input) != Some(*config))
                .map(|(crate_input, config)| (crate_input.clone(), config.clone()))
                .collect_vec();
            let removed_inputs = current
                .keys()
                .filter(|crate_input| !crate_configs.contains_key(*crate_input))
                .cloned()
                .collect_vec();

            *current = crate_configs;

            (inserted_inputs, changed_inputs, removed_inputs)
        };

        if !inserted_inputs.is_empty() {
            self.bump_granular_crate_configs_revision();
        }

        for (crate_input, config) in changed_inputs {
            let handle = self.ensure_granular_crate_config_handle_for_input(crate_input);
            handle.set_config(self).with_durability(Durability::HIGH).to(Some(config));
        }

        for crate_input in removed_inputs {
            if let Some(handle) = self.granular_crate_config_handle(&crate_input) {
                handle.set_config(self).with_durability(Durability::HIGH).to(None);
            }
        }
    }
    fn ensure_granular_file_contents_handle_for_input(
        &mut self,
        file_input: FileInput,
    ) -> GranularFileContents {
        if let Some(handle) = self.granular_file_contents_handle(&file_input) {
            return handle;
        }

        let handle = GranularFileContents::new(self, None, None);
        self.granular_file_contents.write().unwrap().insert(file_input, handle);
        self.bump_granular_file_contents_revision();
        handle
    }

    pub fn set_editor_file_content<'db>(
        &'db mut self,
        file_id: FileId<'db>,
        content: Option<Arc<str>>,
    ) {
        let file_input = self.file_input(file_id).clone();
        self.set_editor_file_content_for_input(file_input, content);
    }

    pub fn set_editor_file_content_for_input(
        &mut self,
        file_input: FileInput,
        content: Option<Arc<str>>,
    ) {
        let handle = self.ensure_granular_file_contents_handle_for_input(file_input);
        handle
            .set_editor_content(self)
            .with_durability(Durability::LOW)
            .to(content.map(ArcStr::new));
    }

    pub fn set_generated_file_content<'db>(
        &'db mut self,
        file_id: FileId<'db>,
        content: Option<Arc<str>>,
    ) {
        let file_input = self.file_input(file_id).clone();
        self.set_generated_file_content_for_input(file_input, content);
    }

    pub fn set_generated_file_content_for_input(
        &mut self,
        file_input: FileInput,
        content: Option<Arc<str>>,
    ) {
        let handle = self.ensure_granular_file_contents_handle_for_input(file_input);
        handle
            .set_generated_content(self)
            .with_durability(Durability::HIGH)
            .to(content.map(ArcStr::new));
    }

    pub fn clear_generated_file_contents(&mut self) {
        let handles =
            self.granular_file_contents.read().unwrap().values().copied().collect::<Vec<_>>();
        for handle in handles {
            handle.set_generated_content(self).with_durability(Durability::HIGH).to(None);
        }
    }

    pub fn collect_open_file_overrides(
        &self,
        files: impl IntoIterator<Item = FileInput>,
    ) -> OrderedHashMap<FileInput, Arc<str>> {
        files
            .into_iter()
            .filter_map(|file_input| {
                let handle = self.granular_file_contents_handle(&file_input)?;
                let content = handle.editor_content(self).as_ref()?;
                Some((file_input, (**content).clone()))
            })
            .collect()
    }

    pub fn restore_open_file_overrides(&mut self, overrides: OrderedHashMap<FileInput, Arc<str>>) {
        for (file_input, content) in overrides {
            self.set_editor_file_content_for_input(file_input, Some(content));
        }
    }

    fn granular_macro_plugin_override_handle(
        &self,
        crate_input: &CrateInput,
    ) -> Option<GranularMacroPluginOverride> {
        self.granular_macro_plugin_overrides.read().unwrap().get(crate_input).copied()
    }

    fn granular_inline_macro_plugin_override_handle(
        &self,
        crate_input: &CrateInput,
    ) -> Option<GranularInlineMacroPluginOverride> {
        self.granular_inline_macro_plugin_overrides.read().unwrap().get(crate_input).copied()
    }

    fn granular_analyzer_plugin_override_handle(
        &self,
        crate_input: &CrateInput,
    ) -> Option<GranularAnalyzerPluginOverride> {
        self.granular_analyzer_plugin_overrides.read().unwrap().get(crate_input).copied()
    }

    fn ensure_granular_macro_plugin_override_handle(
        &mut self,
        crate_input: CrateInput,
    ) -> GranularMacroPluginOverride {
        if let Some(handle) = self.granular_macro_plugin_override_handle(&crate_input) {
            return handle;
        }

        let handle = GranularMacroPluginOverride::new(self, None);
        self.granular_macro_plugin_overrides.write().unwrap().insert(crate_input, handle);
        let next =
            defs_group_input(self).granular_macro_plugin_overrides_revision(self).saturating_add(1);
        defs_group_input(self)
            .set_granular_macro_plugin_overrides_revision(self)
            .with_durability(Durability::HIGH)
            .to(next);
        handle
    }

    fn ensure_granular_inline_macro_plugin_override_handle(
        &mut self,
        crate_input: CrateInput,
    ) -> GranularInlineMacroPluginOverride {
        if let Some(handle) = self.granular_inline_macro_plugin_override_handle(&crate_input) {
            return handle;
        }

        let handle = GranularInlineMacroPluginOverride::new(self, None);
        self.granular_inline_macro_plugin_overrides.write().unwrap().insert(crate_input, handle);
        let next = defs_group_input(self)
            .granular_inline_macro_plugin_overrides_revision(self)
            .saturating_add(1);
        defs_group_input(self)
            .set_granular_inline_macro_plugin_overrides_revision(self)
            .with_durability(Durability::HIGH)
            .to(next);
        handle
    }

    fn ensure_granular_analyzer_plugin_override_handle(
        &mut self,
        crate_input: CrateInput,
    ) -> GranularAnalyzerPluginOverride {
        if let Some(handle) = self.granular_analyzer_plugin_override_handle(&crate_input) {
            return handle;
        }

        let handle = GranularAnalyzerPluginOverride::new(self, None);
        self.granular_analyzer_plugin_overrides.write().unwrap().insert(crate_input, handle);
        let next = semantic_group_input(self)
            .granular_analyzer_plugin_overrides_revision(self)
            .saturating_add(1);
        semantic_group_input(self)
            .set_granular_analyzer_plugin_overrides_revision(self)
            .with_durability(Durability::HIGH)
            .to(next);
        handle
    }

    fn apply_granular_crate_plugin_suite_for_input(
        &mut self,
        crate_input: CrateInput,
        suite: Option<CratePluginSuiteInputs>,
    ) {
        match suite {
            Some(suite) => {
                assert!(
                    suite.macro_plugins.first().is_none_or(
                        |id| id.plugin_type_id() == ConfigPlugin::default().plugin_type_id()
                    ),
                    "cfg plugin must be the first macro plugin"
                );

                self.ensure_granular_macro_plugin_override_handle(crate_input.clone())
                    .set_plugins(self)
                    .with_durability(Durability::HIGH)
                    .to(Some(suite.macro_plugins.clone()));
                self.ensure_granular_inline_macro_plugin_override_handle(crate_input.clone())
                    .set_plugins(self)
                    .with_durability(Durability::HIGH)
                    .to(Some(suite.inline_macro_plugins.clone()));
                self.ensure_granular_analyzer_plugin_override_handle(crate_input)
                    .set_plugins(self)
                    .with_durability(Durability::HIGH)
                    .to(Some(suite.analyzer_plugins.clone()));
            }
            None => {
                if let Some(handle) = self.granular_macro_plugin_override_handle(&crate_input) {
                    handle.set_plugins(self).with_durability(Durability::HIGH).to(None);
                }
                if let Some(handle) =
                    self.granular_inline_macro_plugin_override_handle(&crate_input)
                {
                    handle.set_plugins(self).with_durability(Durability::HIGH).to(None);
                }
                if let Some(handle) = self.granular_analyzer_plugin_override_handle(&crate_input) {
                    handle.set_plugins(self).with_durability(Durability::HIGH).to(None);
                }
            }
        }
    }

    fn has_loaded_crate(&self, crate_input: &CrateInput) -> bool {
        let crate_id = crate_input.clone().into_crate_long_id(self).intern(self);
        self.crate_configs().contains_key(&crate_id)
    }

    pub fn set_override_crate_plugins_from_suite(
        &mut self,
        crate_input: CrateInput,
        plugins: PluginSuite,
    ) {
        self.set_override_crate_plugins_from_suites(std::iter::once((crate_input, plugins)));
    }

    pub fn set_override_crate_plugins_from_suites(
        &mut self,
        suites: impl IntoIterator<Item = (CrateInput, PluginSuite)>,
    ) {
        for (crate_input, plugins) in suites {
            let suite_inputs = CratePluginSuiteInputs::from(plugins);
            let fingerprint = CratePluginSuiteFingerprint::from_inputs(&suite_inputs);
            if self
                .current_granular_crate_plugin_suite_fingerprints
                .read()
                .unwrap()
                .get(&crate_input)
                == Some(&fingerprint)
            {
                continue;
            }
            self.current_granular_crate_plugin_suites
                .write()
                .unwrap()
                .insert(crate_input.clone(), suite_inputs.clone());
            self.current_granular_crate_plugin_suite_fingerprints
                .write()
                .unwrap()
                .insert(crate_input.clone(), fingerprint);
            self.apply_granular_crate_plugin_suite_for_input(crate_input, Some(suite_inputs));
        }
    }

    pub fn clear_override_crate_plugins(&mut self, crate_input: CrateInput) {
        self.current_granular_crate_plugin_suites.write().unwrap().swap_remove(&crate_input);
        self.current_granular_crate_plugin_suite_fingerprints
            .write()
            .unwrap()
            .swap_remove(&crate_input);
        self.apply_granular_crate_plugin_suite_for_input(crate_input, None);
    }

    pub fn sync_granular_crate_plugin_suites(
        &mut self,
        suites: impl IntoIterator<Item = (CrateInput, PluginSuite)>,
    ) {
        let (changed, removed) = {
            let mut current_fingerprints =
                self.current_granular_crate_plugin_suite_fingerprints.write().unwrap();
            let mut current = self.current_granular_crate_plugin_suites.write().unwrap();

            let mut seen = HashSet::<CrateInput>::default();
            let mut changed = Vec::new();

            for (crate_input, suite) in suites {
                seen.insert(crate_input.clone());
                let fingerprint = CratePluginSuiteFingerprint::from_plugin_suite(&suite);
                if current_fingerprints.get(&crate_input) == Some(&fingerprint) {
                    continue;
                }

                let suite_inputs = CratePluginSuiteInputs::from(suite);
                current_fingerprints.insert(crate_input.clone(), fingerprint);
                current.insert(crate_input.clone(), suite_inputs.clone());
                changed.push((crate_input, suite_inputs));
            }

            let removed = current_fingerprints
                .keys()
                .filter(|crate_input| !seen.contains(*crate_input))
                .cloned()
                .collect_vec();

            if changed.is_empty() && removed.is_empty() {
                return;
            }

            for crate_input in &removed {
                current_fingerprints.swap_remove(crate_input);
                current.swap_remove(crate_input);
            }

            (changed, removed)
        };

        for (crate_input, suite) in changed {
            self.apply_granular_crate_plugin_suite_for_input(crate_input, Some(suite));
        }

        for crate_input in removed {
            self.apply_granular_crate_plugin_suite_for_input(crate_input, None);
        }
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

    /// Removes the plugins from [`PluginSuite`] for a crate with [`CrateInput`] if this
    /// crate exists in the crate configs.
    pub fn remove_crate_plugin_suite(&mut self, crate_input: CrateInput, plugins: PluginSuite) {
        self.remove_crate_plugin_suites(std::iter::once((crate_input, plugins)));
    }

    pub fn remove_crate_plugin_suites(
        &mut self,
        suites: impl IntoIterator<Item = (CrateInput, PluginSuite)>,
    ) {
        for (crate_input, plugins) in suites {
            let Some(current) = self
                .current_granular_crate_plugin_suites
                .read()
                .unwrap()
                .get(&crate_input)
                .cloned()
            else {
                continue;
            };

            let macro_plugins_set: HashSet<_> =
                plugins.plugins.into_iter().map(MacroPluginLongId).collect();
            let analyzer_plugins_set: HashSet<_> =
                plugins.analyzer_plugins.into_iter().map(AnalyzerPluginLongId).collect();
            let inline_macro_plugin_set: HashSet<_> = plugins
                .inline_macro_plugins
                .into_iter()
                .map(|(_, arc)| InlineMacroExprPluginLongId(arc))
                .collect();

            let next_suite = CratePluginSuiteInputs {
                macro_plugins: Arc::from(
                    current
                        .macro_plugins
                        .iter()
                        .filter(|plugin| !macro_plugins_set.contains(*plugin))
                        .cloned()
                        .collect::<Vec<_>>(),
                ),
                analyzer_plugins: Arc::from(
                    current
                        .analyzer_plugins
                        .iter()
                        .filter(|plugin| !analyzer_plugins_set.contains(*plugin))
                        .cloned()
                        .collect::<Vec<_>>(),
                ),
                inline_macro_plugins: Arc::new(
                    current
                        .inline_macro_plugins
                        .iter()
                        .filter(|(_, plugin)| !inline_macro_plugin_set.contains(*plugin))
                        .map(|(name, plugin)| (name.clone(), plugin.clone()))
                        .collect(),
                ),
            };

            self.current_granular_crate_plugin_suites
                .write()
                .unwrap()
                .insert(crate_input.clone(), next_suite.clone());
            self.current_granular_crate_plugin_suite_fingerprints
                .write()
                .unwrap()
                .insert(
                    crate_input.clone(),
                    CratePluginSuiteFingerprint::from_inputs(&next_suite),
                );
            self.apply_granular_crate_plugin_suite_for_input(crate_input, Some(next_suite));
        }
    }

    /// Adds proc macro plugin suite to the database for a crate with [`CrateInput`] if this
    /// crate exists in the crate configs.
    ///
    /// It *prepends* (with the exception of macro plugins, see the code below) the plugins from
    /// the proc macro plugin suite to appropriate salsa inputs.
    /// It is done to make sure proc macros are resolved first, just like in
    /// [`crate::project::Crate::apply`].
    pub fn add_proc_macro_plugin_suite(&mut self, crate_input: CrateInput, plugins: PluginSuite) {
        if !self.has_loaded_crate(&crate_input) {
            return;
        }

        let mut current = self
            .current_granular_crate_plugin_suites
            .read()
            .unwrap()
            .get(&crate_input)
            .cloned()
            .unwrap_or_default();

        let mut macro_plugins = current.macro_plugins.iter().cloned().collect_vec();
        let maybe_cfg_plugin = macro_plugins.is_empty().not().then(|| macro_plugins.remove(0));
        current.macro_plugins = Arc::from(
            maybe_cfg_plugin
                .into_iter()
                .chain(plugins.plugins.into_iter().map(MacroPluginLongId))
                .chain(macro_plugins)
                .collect::<Vec<_>>(),
        );
        current.analyzer_plugins = Arc::from(
            plugins
                .analyzer_plugins
                .into_iter()
                .map(AnalyzerPluginLongId)
                .chain(current.analyzer_plugins.iter().cloned())
                .collect::<Vec<_>>(),
        );
        current.inline_macro_plugins = Arc::new(
            plugins
                .inline_macro_plugins
                .into_iter()
                .map(|(key, arc)| (key, InlineMacroExprPluginLongId(arc)))
                .chain(
                    current
                        .inline_macro_plugins
                        .iter()
                        .map(|(name, id)| (name.clone(), id.clone())),
                )
                .collect(),
        );

        self.current_granular_crate_plugin_suites
            .write()
            .unwrap()
            .insert(crate_input.clone(), current.clone());
        self.current_granular_crate_plugin_suite_fingerprints
            .write()
            .unwrap()
            .insert(
                crate_input.clone(),
                CratePluginSuiteFingerprint::from_inputs(&current),
            );
        self.apply_granular_crate_plugin_suite_for_input(crate_input, Some(current));
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

impl GranularFileContentView for AnalysisDatabase {
    fn granular_file_contents<'db>(
        &'db self,
        file_id: FileId<'db>,
    ) -> Option<GranularFileContents> {
        self.granular_file_contents_handle_for_file(file_id)
    }

    fn editor_file_content<'db>(&'db self, file_id: FileId<'db>) -> Option<&'db ArcStr> {
        self.granular_file_contents_handle_for_file(file_id)?.editor_content(self).as_ref()
    }

    fn generated_file_content<'db>(&'db self, file_id: FileId<'db>) -> Option<&'db ArcStr> {
        self.granular_file_contents_handle_for_file(file_id)?.generated_content(self).as_ref()
    }
}

impl GranularCrateConfigView for AnalysisDatabase {
    fn granular_crate_config_storage(&self) -> Option<&GranularCrateConfigStorage> {
        Some(&self.granular_crate_configs)
    }

    fn crate_config_input_for<'db>(
        &'db self,
        crate_id: CrateId<'db>,
    ) -> Option<&'db CrateConfigurationInput> {
        let crate_input = self.crate_input(crate_id).clone();
        self.crate_config_input_for_input(&crate_input)
    }

    fn crate_config_input_for_input<'db>(
        &'db self,
        crate_input: &CrateInput,
    ) -> Option<&'db CrateConfigurationInput> {
        self.granular_crate_config_handle(crate_input)?.config(self).as_ref()
    }
}

impl GranularMacroPluginOverrideView for AnalysisDatabase {
    fn granular_macro_plugin_override_storage(&self) -> Option<&GranularMacroPluginOverrideStorage> {
        Some(&self.granular_macro_plugin_overrides)
    }
}

impl GranularInlineMacroPluginOverrideView for AnalysisDatabase {
    fn granular_inline_macro_plugin_override_storage(
        &self,
    ) -> Option<&GranularInlineMacroPluginOverrideStorage> {
        Some(&self.granular_inline_macro_plugin_overrides)
    }
}

impl GranularAnalyzerPluginOverrideView for AnalysisDatabase {
    fn granular_analyzer_plugin_override_storage(
        &self,
    ) -> Option<&GranularAnalyzerPluginOverrideStorage> {
        Some(&self.granular_analyzer_plugin_overrides)
    }
}

impl salsa::Database for AnalysisDatabase {}

#[cfg(test)]
mod tests {
    use std::fs;

    use cairo_lang_defs::db::DefsGroup;
    use cairo_lang_defs::ids::ModuleId;
    use cairo_lang_filesystem::db::{CrateSettings, FilesGroup};
    use cairo_lang_filesystem::ids::{FileId, FileInput};
    use cairo_lang_utils::Intern;
    use tempfile::tempdir;

    use super::AnalysisDatabase;
    use crate::project::Crate;

    #[test]
    fn grouped_integration_test_crate_maps_module_file() {
        let workspace = tempdir().expect("failed to create tempdir");
        let tests_root = workspace.path().join("tests");
        fs::create_dir_all(&tests_root).expect("failed to create tests dir");
        fs::write(tests_root.join("test.cairo"), "fn probe() {}")
            .expect("failed to write integration test file");

        let mut db = AnalysisDatabase::new();
        let crate_data = Crate {
            name: "hello_integrationtest".into(),
            discriminator: Some("disc".into()),
            root: tests_root.clone(),
            custom_main_file_stems: Some(vec!["test".into()]),
            settings: CrateSettings {
                cfg_set: Some(AnalysisDatabase::initial_cfg_set()),
                ..Default::default()
            },
            builtin_plugins: Default::default(),
        };
        crate_data.apply(&mut db, None);

        let crate_id = crate_data.input().into_crate_long_id(&db).intern(&db);
        let root_module = ModuleId::CrateRoot(crate_id);
        let submodule_id = *db
            .module_submodules_ids(root_module)
            .expect("failed to read root submodules")
            .first()
            .expect("expected integration test wrapper to expose submodule");
        let submodule = ModuleId::Submodule(submodule_id);
        let test_file = FileId::new_on_disk(&db, tests_root.join("test.cairo"));

        assert_eq!(db.module_main_file(submodule).unwrap(), test_file);
        assert_eq!(db.file_modules(test_file).unwrap(), &vec![submodule]);

        let wrapper_input: FileInput;
        let wrapper_path = tests_root.join("lib.cairo");
        {
            let wrapper_file = FileId::new_on_disk(&db, wrapper_path.clone());
            wrapper_input = db.file_input(wrapper_file).clone();
            let wrapper_text = db.file_content(wrapper_file).expect("wrapper content should exist");
            assert_eq!(wrapper_text, "mod test;");
        }

        db.clear_generated_file_contents();
        let wrapper_file = FileId::new_on_disk(&db, wrapper_path.clone());
        assert!(
            db.file_content(wrapper_file).is_none(),
            "generated content should disappear when cleared"
        );

        db.set_generated_file_content_for_input(wrapper_input, Some("mod test;".into()));
        let wrapper_file = FileId::new_on_disk(&db, wrapper_path);
        assert_eq!(db.file_content(wrapper_file), Some("mod test;"));
    }
}
