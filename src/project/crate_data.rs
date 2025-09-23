use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Arc;

use cairo_lang_defs::db::DefsGroup;
use cairo_lang_defs::ids::ModuleId;
use cairo_lang_filesystem::db::{
    CORELIB_CRATE_NAME, CrateConfiguration, CrateSettings, FilesGroup,
};
use cairo_lang_filesystem::ids::{CrateId, CrateInput, Directory};
use cairo_lang_filesystem::{override_file_content, set_crate_config};
use cairo_lang_plugins::plugins::ConfigPlugin;
use cairo_lang_semantic::inline_macros::get_default_plugin_suite;
use cairo_lang_semantic::plugin::PluginSuite;
use cairo_lang_utils::Intern;
use cairo_lint::plugin::cairo_lint_allow_plugin_suite;
use itertools::chain;

use super::builtin_plugins::BuiltinPlugin;
use crate::lang::db::AnalysisDatabase;
use crate::project::model::PackageConfig;

#[derive(Debug)]
pub struct CrateInfo {
    pub cr: Crate,
    pub package_config: PackageConfig,
    /// Path to Scarb.toml.
    pub manifest_path: PathBuf,
    /// If the crate is a workspace member in the context of the loaded workspace.
    pub is_member: bool,
}

impl CrateInfo {
    /// States whether this is the `core` crate.
    pub fn is_core(&self) -> bool {
        self.cr.name == CORELIB_CRATE_NAME
    }
}

/// A complete set of information needed to set up a real crate in the analysis database.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Crate {
    /// Crate name.
    pub name: String,

    /// Globally unique crate ID used for differentiating between crates with the same name.
    ///
    /// `None` is reserved for the core crate.
    pub discriminator: Option<String>,

    /// The root directory of the crate.
    ///
    /// This path **must** be absolute,
    /// so it can be safely used as a `FileId` in the analysis database.
    pub root: PathBuf,

    /// Custom stems of crate main files, if it is not `lib.cairo`.
    ///
    /// This is used to generate a virtual lib file for crates without a root `lib.cairo`.
    pub custom_main_file_stems: Option<Vec<String>>,

    /// Crate settings.
    pub settings: CrateSettings,

    /// Built-in plugins required by the crate.
    pub builtin_plugins: HashSet<BuiltinPlugin>,
}

impl Crate {
    /// Applies this crate to the [`AnalysisDatabase`].
    pub fn apply(&self, db: &mut AnalysisDatabase, proc_macro_plugin_suite: Option<PluginSuite>) {
        assert!(
            (self.name == CORELIB_CRATE_NAME) ^ self.discriminator.is_some(),
            "invariant violation: only the `core` crate should have no discriminator"
        );

        let crate_input =
            CrateInput::Real { name: self.name.clone(), discriminator: self.discriminator.clone() };

        let crate_configuration = CrateConfiguration {
            root: Directory::Real(self.root.clone()),
            settings: self.settings.clone(),
            cache_file: None,
        };

        set_crate_config!(
            db,
            crate_input.clone().into_crate_long_id(db).intern(db),
            Some(crate_configuration)
        );

        if let Some(file_stems) = &self.custom_main_file_stems {
            inject_virtual_wrapper_lib(db, crate_input.clone(), file_stems);
        }

        let config_plugin_suite = Some(PluginSuite {
            plugins: vec![Arc::new(ConfigPlugin::default())],
            ..Default::default()
        });
        let builtin = self.builtin_plugins.iter().map(BuiltinPlugin::suite);
        let base = Some(get_default_plugin_suite());
        let lint_allow = Some(cairo_lint_allow_plugin_suite());
        // Keep the order the same as in Scarb.
        let plugins =
            chain!(config_plugin_suite, proc_macro_plugin_suite, base, builtin, lint_allow).fold(
                PluginSuite::default(),
                |mut acc, suite| {
                    acc.add(suite);
                    acc
                },
            );

        db.set_override_crate_plugins_from_suite(crate_input, plugins);
    }

    pub fn input(&self) -> CrateInput {
        CrateInput::Real { name: self.name.clone(), discriminator: self.discriminator.clone() }
    }
}

/// Generate a wrapper lib file for a compilation unit without a root `lib.cairo`.
///
/// This approach allows compiling crates that do not define `lib.cairo` file. For example, single
/// file crates can be created this way. The actual single file module is defined as `mod` item in
/// created lib file.
fn inject_virtual_wrapper_lib(
    db: &mut AnalysisDatabase,
    crate_input: CrateInput,
    file_stems: &[String],
) {
    let module_id = ModuleId::CrateRoot(crate_input.into_crate_long_id(db).intern(db));
    let file_id = db.module_main_file(module_id).unwrap();

    let file_content =
        file_stems.iter().map(|stem| format!("mod {stem};")).collect::<Vec<_>>().join("\n");

    // Inject virtual lib file wrapper.
    override_file_content!(db, file_id, Some(file_content.into()));
}

/// The inverse of [`inject_virtual_wrapper_lib`],
/// tries to infer root module name from crate if it does not have real `lib.cairo`.
pub fn extract_custom_file_stems<'db>(
    db: &'db AnalysisDatabase,
    crate_id: CrateId<'db>,
) -> Option<Vec<String>> {
    let CrateConfiguration { root: Directory::Real(root), .. } = db.crate_config(crate_id)? else {
        return None;
    };

    if root.join("lib.cairo").exists() {
        return None;
    }

    let module_id = ModuleId::CrateRoot(crate_id);
    let file_id = db.module_main_file(module_id).ok()?;
    let content = db.file_content(file_id)?;

    content
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| Some(line.strip_prefix("mod ")?.strip_suffix(';')?.into()))
        .collect::<Option<Vec<_>>>()
}
