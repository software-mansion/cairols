use std::collections::HashSet;
use std::ops::Not;
use std::sync::Arc;

use cairo_lang_defs::db::{DefsGroup, defs_group_input, init_defs_group, init_external_files};
use cairo_lang_defs::ids::{InlineMacroExprPluginLongId, MacroPluginLongId};
use cairo_lang_defs::plugin::MacroPlugin;
use cairo_lang_executable_plugin::executable_plugin_suite;
use cairo_lang_filesystem::cfg::{Cfg, CfgSet};
use cairo_lang_filesystem::db::{
    FileContentStorage, FileContentView, FileContents, FilesGroup, files_group_input,
    init_files_group, new_file_content_storage, register_files_group_view,
};
use cairo_lang_filesystem::ids::{ArcStr, CrateInput, CrateLongId, FileId, FileInput};
use cairo_lang_lowering::db::init_lowering_group;
use cairo_lang_lowering::optimizations::config::Optimizations;
use cairo_lang_lowering::utils::InliningStrategy;
use cairo_lang_plugins::plugins::ConfigPlugin;
use cairo_lang_semantic::db::{
    PluginSuiteInput, SemanticGroup, init_semantic_group, semantic_group_input,
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

mod semantic;
mod swapper;
mod syntax;

/// The Cairo compiler Salsa database tailored for language server usage.
#[salsa::db]
#[derive(Clone)]
pub struct AnalysisDatabase {
    storage: salsa::Storage<Self>,
    file_contents: FileContentStorage,
}

impl Default for AnalysisDatabase {
    fn default() -> Self {
        Self::new()
    }
}

impl AnalysisDatabase {
    /// Creates a new instance of the database.
    pub fn new() -> Self {
        let mut db =
            Self { storage: Default::default(), file_contents: new_file_content_storage() };

        register_files_group_view(&db);
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

    fn file_contents_handle(&self, file_input: &FileInput) -> Option<FileContents> {
        self.file_contents.read().unwrap().get(file_input).copied()
    }

    fn file_contents_handle_for_file<'db>(&'db self, file_id: FileId<'db>) -> Option<FileContents> {
        let file_input = self.file_input(file_id).clone();
        self.file_contents_handle(&file_input)
    }

    fn bump_file_contents_revision(&mut self) {
        let next_revision = files_group_input(self).file_contents_revision(self).saturating_add(1);
        files_group_input(self)
            .set_file_contents_revision(self)
            .with_durability(Durability::HIGH)
            .to(next_revision);
    }
    fn ensure_file_contents_handle_for_input(&mut self, file_input: FileInput) -> FileContents {
        if let Some(handle) = self.file_contents_handle(&file_input) {
            return handle;
        }

        let handle = FileContents::new(self, None, None);
        self.file_contents.write().unwrap().insert(file_input, handle);
        self.bump_file_contents_revision();
        handle
    }

    pub fn set_editor_file_content<'db>(
        &'db mut self,
        file_id: FileId<'db>,
        content: Option<Arc<str>>,
    ) {
        let file_input = self.file_input(file_id).clone();
        self.set_on_disk_file_content_for_input(file_input, content);
    }

    pub fn set_on_disk_file_content_for_input(
        &mut self,
        file_input: FileInput,
        content: Option<Arc<str>>,
    ) {
        let handle = self.ensure_file_contents_handle_for_input(file_input);
        handle
            .set_on_disk_content(self)
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
        let handle = self.ensure_file_contents_handle_for_input(file_input);
        handle
            .set_generated_content(self)
            .with_durability(Durability::HIGH)
            .to(content.map(ArcStr::new));
    }

    pub fn clear_generated_file_contents(&mut self) {
        let handles = self.file_contents.read().unwrap().values().copied().collect::<Vec<_>>();
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
                let handle = self.file_contents_handle(&file_input)?;
                let content = handle.on_disk_content(self).as_ref()?;
                Some((file_input, (**content).clone()))
            })
            .collect()
    }

    pub fn restore_open_file_overrides(&mut self, overrides: OrderedHashMap<FileInput, Arc<str>>) {
        for (file_input, content) in overrides {
            self.set_on_disk_file_content_for_input(file_input, Some(content));
        }
    }

    pub fn set_override_crate_plugins_from_suite(
        &mut self,
        crate_input: CrateInput,
        plugins: PluginSuite,
    ) {
        let mut overrides = self.macro_plugin_overrides_input().clone();
        overrides.insert(
            crate_input.clone(),
            plugins.plugins.into_iter().map(MacroPluginLongId).collect(),
        );
        defs_group_input(self).set_macro_plugin_overrides(self).to(Some(overrides));

        let mut overrides = self.analyzer_plugin_overrides_input().clone();
        overrides.insert(
            crate_input.clone(),
            plugins.analyzer_plugins.into_iter().map(AnalyzerPluginLongId).collect(),
        );

        semantic_group_input(self).set_analyzer_plugin_overrides(self).to(Some(overrides));

        let mut overrides = self.inline_macro_plugin_overrides_input().clone();
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
        defs_group_input(self).set_inline_macro_plugin_overrides(self).to(Some(overrides));
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

    /// Adds proc macro plugin suite to the database for a crate with [`CrateInput`] if this
    /// crate exists in the crate configs.
    ///
    /// It *prepends* (with the exception of macro plugins, see the code below) the plugins from
    /// the proc macro plugin suite to appropriate salsa inputs.
    /// It is done to make sure proc macros are resolved first.
    pub fn add_proc_macro_plugin_suite(&mut self, crate_input: CrateInput, plugins: PluginSuite) {
        self.with_plugins_mut(
            crate_input,
            move |macro_plugins, analyzer_plugins, inline_macro_plugins| {
                let maybe_cfg_plugin =
                    macro_plugins.is_empty().not().then(|| macro_plugins.remove(0));
                *macro_plugins = maybe_cfg_plugin
                    .into_iter()
                    .chain(plugins.plugins.into_iter().map(MacroPluginLongId))
                    .chain(macro_plugins.iter().cloned())
                    .collect();

                *analyzer_plugins = plugins
                    .analyzer_plugins
                    .into_iter()
                    .map(AnalyzerPluginLongId)
                    .chain(analyzer_plugins.iter().cloned())
                    .collect();

                *inline_macro_plugins = plugins
                    .inline_macro_plugins
                    .into_iter()
                    .map(|(key, arc)| (key, InlineMacroExprPluginLongId(arc)))
                    .chain(inline_macro_plugins.iter().map(|(s, id)| (s.clone(), id.clone())))
                    .collect();
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
        if !self
            .crate_configs()
            .keys()
            .contains(&crate_input.clone().into_crate_long_id(self).intern(self))
        {
            return;
        }

        let mut macro_plugin_overrides_input = self.macro_plugin_overrides_input().clone();
        let mut macro_plugins =
            macro_plugin_overrides_input.get(&crate_input).map(|a| a.to_vec()).unwrap_or_default();

        let mut analyzer_plugin_overrides_input = self.analyzer_plugin_overrides_input().clone();
        let mut analyzer_plugins = analyzer_plugin_overrides_input
            .get(&crate_input)
            .map(|a| a.to_vec())
            .unwrap_or_default();

        let mut inline_macro_plugin_overrides_input =
            self.inline_macro_plugin_overrides_input().clone();
        let mut inline_macro_plugins = inline_macro_plugin_overrides_input
            .get(&crate_input)
            .map(|a| (**a).clone())
            .unwrap_or_default();

        action(&mut macro_plugins, &mut analyzer_plugins, &mut inline_macro_plugins);

        assert!(
            macro_plugins
                .first()
                .is_none_or(|id| id.plugin_type_id() == ConfigPlugin::default().plugin_type_id()),
            "cfg plugin must be the first macro plugin"
        );

        macro_plugin_overrides_input.insert(crate_input.clone(), macro_plugins.into());
        analyzer_plugin_overrides_input.insert(crate_input.clone(), analyzer_plugins.into());
        inline_macro_plugin_overrides_input
            .insert(crate_input.clone(), inline_macro_plugins.into());

        defs_group_input(self)
            .set_macro_plugin_overrides(self)
            .to(Some(macro_plugin_overrides_input));
        defs_group_input(self)
            .set_inline_macro_plugin_overrides(self)
            .to(Some(inline_macro_plugin_overrides_input));
        semantic_group_input(self)
            .set_analyzer_plugin_overrides(self)
            .to(Some(analyzer_plugin_overrides_input));
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

impl FileContentView for AnalysisDatabase {
    fn file_contents<'db>(&'db self, file_id: FileId<'db>) -> Option<FileContents> {
        self.file_contents_handle_for_file(file_id)
    }

    fn on_disk_file_content<'db>(&'db self, file_id: FileId<'db>) -> Option<&'db ArcStr> {
        self.file_contents_handle_for_file(file_id)?.on_disk_content(self).as_ref()
    }

    fn generated_file_content<'db>(&'db self, file_id: FileId<'db>) -> Option<&'db ArcStr> {
        self.file_contents_handle_for_file(file_id)?.generated_content(self).as_ref()
    }
}

impl salsa::Database for AnalysisDatabase {}
