use std::collections::HashMap;
use std::path::PathBuf;

use cairo_lang_defs::db::DefsGroup;
use cairo_lang_defs::ids::{InlineMacroExprPluginLongId, MacroPluginLongId};
use cairo_lang_filesystem::db::{CrateConfiguration, CrateSettings, FilesGroup};
use cairo_lang_filesystem::ids::{CrateId, CrateLongId, Directory};
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::ids::AnalyzerPluginLongId;
use cairo_lang_utils::{LookupIntern, smol_str::SmolStr};
use itertools::{Itertools, chain};
use serde::Serialize;

use crate::lang::db::AnalysisDatabase;
use crate::lang::plugins::AnalyzerPluginType;
use crate::project::builtin_plugins::BuiltinPlugin;
use crate::project::extract_custom_file_stems;

/// Generates a Markdown text describing all crates in the database.
pub fn inspect_analyzed_crates(db: &AnalysisDatabase) -> String {
    let crates = db
        .crates()
        .into_iter()
        .filter_map(|id| CrateView::for_crate(db, id))
        .sorted()
        .map(|cr| serde_json::to_string_pretty(&cr))
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_else(|err| vec![format!(r#"{{"error": {}}}"#, err.to_string())])
        .join("\n\n");

    format!("# Analyzed Crates\n---\n```json\n{crates}\n```")
}

#[derive(Debug, Serialize, PartialEq, Eq)]
struct CrateView {
    name: SmolStr,
    source_paths: Vec<PathBuf>,
    settings: CrateSettings,
    linter_configuration: LinterConfiguration,
    plugins: Plugins,
}

impl PartialOrd for CrateView {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some((&self.name, &self.source_paths).cmp(&(&other.name, &other.source_paths)))
    }
}

impl Ord for CrateView {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).expect("PartialOrd should not fail")
    }
}

impl CrateView {
    fn for_crate(db: &AnalysisDatabase, crate_id: CrateId) -> Option<Self> {
        let CrateLongId::Real { name, .. } = crate_id.lookup_intern(db) else {
            return None;
        };

        let Some(CrateConfiguration { root: Directory::Real(root), settings, .. }) =
            db.crate_config(crate_id)
        else {
            return None;
        };

        let source_paths = extract_custom_file_stems(db, crate_id)
            .map(|stems| stems.iter().map(|stem| root.join(format!("{stem}.cairo"))).collect_vec())
            .unwrap_or_else(|| vec![root.join("lib.cairo")]);

        let linter_configuration = LinterConfiguration::for_crate(db, crate_id);
        let plugins = Plugins::for_crate(db, crate_id);

        Some(Self { name, source_paths, settings, linter_configuration, plugins })
    }
}

#[derive(Debug, Serialize, PartialEq, Eq)]
enum LinterConfiguration {
    Off,
    #[serde(untagged)]
    On(HashMap<String, bool>),
}

impl LinterConfiguration {
    fn for_crate(db: &AnalysisDatabase, crate_id: CrateId) -> Self {
        let Some(_plugin) = db
            .crate_analyzer_plugins(crate_id)
            .iter()
            .map(|id| id.lookup_intern(db))
            .find(|id| id.is_cairo_lint_plugin())
        else {
            return Self::Off;
        };

        let config = HashMap::new();

        Self::On(config)
    }
}

#[derive(Debug, Serialize, PartialEq, Eq)]
struct Plugins {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    builtin_plugins: Vec<BuiltinPlugin>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    proc_macros: Vec<ProcMacro>,
}

impl Plugins {
    fn for_crate(db: &AnalysisDatabase, crate_id: CrateId) -> Self {
        let analyzer_plugins = db.crate_analyzer_plugins(crate_id);
        let macro_plugins = db.crate_macro_plugins(crate_id);
        let inline_plugins = db.crate_inline_macro_plugins(crate_id);

        let plugins = chain!(
            analyzer_plugins.iter().filter_map(|id| Plugin::try_from(id.lookup_intern(db)).ok()),
            macro_plugins.iter().filter_map(|id| Plugin::try_from(id.lookup_intern(db)).ok()),
            inline_plugins.iter().filter_map(|(_, id)| Plugin::try_from(id.lookup_intern(db)).ok())
        );

        let mut builtin_plugins = vec![];
        let mut proc_macros = vec![];

        for plugin in plugins {
            match plugin {
                Plugin::Builtin(builtin_plugin) => builtin_plugins.push(builtin_plugin),
                Plugin::ProcMacro(proc_macro) => proc_macros.push(proc_macro),
            }
        }

        let builtin_plugins = builtin_plugins.into_iter().unique().sorted().collect_vec();

        Self { builtin_plugins, proc_macros }
    }
}

enum Plugin {
    Builtin(BuiltinPlugin),
    #[expect(dead_code)] // In the next PR :)))
    ProcMacro(ProcMacro),
}

#[derive(Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
struct ProcMacro {
    source_packages: Vec<String>,
}

impl TryFrom<MacroPluginLongId> for Plugin {
    type Error = ();

    fn try_from(id: MacroPluginLongId) -> Result<Self, Self::Error> {
        let plugin = &*id.0;

        if let Some(builtin_plugin) = BuiltinPlugin::try_from_compiler_macro_plugin(plugin) {
            return Ok(Self::Builtin(builtin_plugin));
        }

        Err(())
    }
}

impl TryFrom<InlineMacroExprPluginLongId> for Plugin {
    type Error = ();

    fn try_from(id: InlineMacroExprPluginLongId) -> Result<Self, Self::Error> {
        let plugin = &*id.0;

        if let Some(builtin_plugin) = BuiltinPlugin::try_from_compiler_inline_macro_plugin(plugin) {
            return Ok(Self::Builtin(builtin_plugin));
        }

        Err(())
    }
}

impl TryFrom<AnalyzerPluginLongId> for Plugin {
    type Error = ();

    fn try_from(plugin: AnalyzerPluginLongId) -> Result<Self, Self::Error> {
        BuiltinPlugin::try_from_compiler_analyzer_plugin(&*plugin.0).map(Self::Builtin).ok_or(())
    }
}
