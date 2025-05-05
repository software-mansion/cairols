use std::collections::HashSet;
use std::path::PathBuf;

use cairo_lang_defs::db::DefsGroup;
use cairo_lang_defs::ids::ModuleId;
use cairo_lang_filesystem::db::{
    CORELIB_CRATE_NAME, CrateConfiguration, CrateSettings, FilesGroup, FilesGroupEx,
};
use cairo_lang_filesystem::ids::{CrateId, CrateLongId, Directory};
use cairo_lang_semantic::db::PluginSuiteInput;
use cairo_lang_semantic::inline_macros::get_default_plugin_suite;
use cairo_lang_semantic::plugin::PluginSuite;
use cairo_lang_utils::Intern;
use cairo_lang_utils::smol_str::SmolStr;
use cairo_lint::CairoLintToolMetadata;
use cairo_lint::plugin::{
    cairo_lint_allow_plugin_suite, cairo_lint_plugin_suite_without_metadata_validation,
};

use super::builtin_plugins::BuiltinPlugin;
use crate::TRICKS;
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
    pub name: SmolStr,

    /// Globally unique crate ID used for differentiating between crates with the same name.
    ///
    /// `None` is reserved for the core crate.
    pub discriminator: Option<SmolStr>,

    /// The root directory of the crate.
    ///
    /// This path **must** be absolute,
    /// so it can be safely used as a `FileId` in the analysis database.
    pub root: PathBuf,

    /// Custom stems of crate main files, if it is not `lib.cairo`.
    ///
    /// This is used to generate a virtual lib file for crates without a root `lib.cairo`.
    pub custom_main_file_stems: Option<Vec<SmolStr>>,

    /// Crate settings.
    pub settings: CrateSettings,

    /// Built-in plugins required by the crate.
    pub builtin_plugins: HashSet<BuiltinPlugin>,
}

impl Crate {
    /// Applies this crate to the [`AnalysisDatabase`].
    pub fn apply(
        &self,
        db: &mut AnalysisDatabase,
        lint_config: Option<CairoLintToolMetadata>,
        proc_macro_plugin_suite: Option<PluginSuite>,
    ) {
        assert!(
            (self.name == CORELIB_CRATE_NAME) ^ self.discriminator.is_some(),
            "invariant violation: only the `core` crate should have no discriminator"
        );

        let crate_id = CrateLongId::Real {
            name: self.name.clone(),
            discriminator: self.discriminator.clone(),
        }
        .intern(db);

        let crate_configuration = CrateConfiguration {
            root: Directory::Real(self.root.clone()),
            settings: self.settings.clone(),
            cache_file: None,
        };
        db.set_crate_config(crate_id, Some(crate_configuration));

        if let Some(file_stems) = &self.custom_main_file_stems {
            inject_virtual_wrapper_lib(db, crate_id, file_stems);
        }

        let plugins = self
            .builtin_plugins
            .iter()
            .map(BuiltinPlugin::suite)
            // All crates should receive Tricks.
            .chain(tricks())
            // All crates should receive CairoLintAllow.
            .chain(Some(cairo_lint_allow_plugin_suite()))
            .chain(lint_config.map(cairo_lint_plugin_suite_without_metadata_validation))
            .chain(proc_macro_plugin_suite)
            .fold(get_default_plugin_suite(), |mut acc, suite| {
                acc.add(suite);
                acc
            });

        let interned_plugins = db.intern_plugin_suite(plugins);
        db.set_override_crate_plugins_from_suite(crate_id, interned_plugins);
    }

    pub fn long_id(&self) -> CrateLongId {
        CrateLongId::Real { name: self.name.clone(), discriminator: self.discriminator.clone() }
    }
}

/// Generate a wrapper lib file for a compilation unit without a root `lib.cairo`.
///
/// This approach allows compiling crates that do not define `lib.cairo` file. For example, single
/// file crates can be created this way. The actual single file module is defined as `mod` item in
/// created lib file.
fn inject_virtual_wrapper_lib(
    db: &mut AnalysisDatabase,
    crate_id: CrateId,
    file_stems: &[SmolStr],
) {
    let module_id = ModuleId::CrateRoot(crate_id);
    let file_id = db.module_main_file(module_id).unwrap();

    let file_content =
        file_stems.iter().map(|stem| format!("mod {stem};")).collect::<Vec<_>>().join("\n");

    // Inject virtual lib file wrapper.
    db.override_file_content(file_id, Some(file_content.into()));
}

/// The inverse of [`inject_virtual_wrapper_lib`],
/// tries to infer root module name from crate if it does not have real `lib.cairo`.
pub fn extract_custom_file_stems(db: &AnalysisDatabase, crate_id: CrateId) -> Option<Vec<SmolStr>> {
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

/// Returns the extra [`PluginSuite`]s injected as [`Tricks`].
fn tricks() -> Vec<PluginSuite> {
    TRICKS
        .get_or_init(Default::default)
        .extra_plugin_suites
        .map(|provider| provider())
        .unwrap_or_default()
}
