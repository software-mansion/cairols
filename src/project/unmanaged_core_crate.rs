use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::{fs, path};

use anyhow::Context;
use cairo_lang_filesystem::db::{
    CORELIB_CRATE_NAME, CORELIB_VERSION, FilesGroup, init_dev_corelib,
};
use cairo_lang_filesystem::ids::{CrateId, CrateLongId};
use cairo_lang_utils::Intern;
use indoc::indoc;
use semver::Version;
use tempfile::tempdir;
use tracing::{error, warn};

use crate::config::Config;
use crate::lang::db::AnalysisDatabase;
use crate::toolchain::scarb::{SCARB_TOML, ScarbToolchain};
use cairo_lang_filesystem::set_crate_config;

/// Try to find a Cairo `core` crate (see [`find_unmanaged_core`]) and initialize it in the
/// provided database.
pub fn try_to_init_unmanaged_core_if_not_present(
    db: &mut AnalysisDatabase,
    config: &Config,
    scarb: &ScarbToolchain,
) {
    if db.crates().contains(&CrateId::core(db)) {
        return;
    }

    if let Some(UnmanagedCore { path, version }) = find_unmanaged_core(config, scarb) {
        // Initialize with default config.
        init_dev_corelib(db, path);

        let core_id = CrateLongId::core().intern(db);

        // Override the config with the correct version.
        let mut crate_configs = db.crate_config(core_id).unwrap().clone();

        crate_configs.settings.version = Some(version);

        set_crate_config!(db, core_id, Some(crate_configs));
    } else {
        warn!("failed to find unmanaged core crate")
    }
}

#[derive(Clone)]
struct UnmanagedCore {
    path: PathBuf,
    version: Version,
}

/// Try to find a Cairo `core` crate in various well-known places, for use in project backends that
/// do not manage the `core` crate (i.e. anything non-Scarb).
///
/// The path is guaranteed to be absolute, so it can be safely used as a `FileId` in LS Salsa DB.
fn find_unmanaged_core(config: &Config, scarb: &ScarbToolchain) -> Option<UnmanagedCore> {
    find_core_at_config_path(config).or_else(|| find_scarb_managed_core(scarb)).and_then(
        |UnmanagedCore { path, version }| {
            Some(UnmanagedCore { path: ensure_absolute(path)?, version })
        },
    )
}

/// Attempts to find the `core` crate source root at the path provided in the configuration.
fn find_core_at_config_path(config: &Config) -> Option<UnmanagedCore> {
    find_core_at_path(config.unmanaged_core_path.as_ref()?.as_path())
        .map(|path| UnmanagedCore { path, version: Version::parse(CORELIB_VERSION).ok().unwrap() })
}

/// Attempts to find the `core` crate source root at a given path.
///
/// In the [starkware-libs/cairo] repository, the `core` crate sits in `./corelib/src`.
/// This is the first place looked for.
/// The `core` crate is a regular Scarb package, so it sounds obvious that the user may provide a
/// path to the directory containing the manifest file, hence next this function looks for `./src`.
/// Finally, the input path is considered as a candidate and is just checked for existence.
///
/// [starkware-libs/cairo]: https://github.com/starkware-libs/cairo
fn find_core_at_path(root_path: &Path) -> Option<PathBuf> {
    let mut path = root_path.to_owned();
    path.push("corelib");
    path.push("src");
    if path.exists() {
        return Some(path);
    }

    let mut path = root_path.to_owned();
    path.push("src");
    if path.exists() {
        return Some(path);
    }

    if root_path.exists() {
        return Some(root_path.to_owned());
    }

    None
}

/// Try to find a Scarb-managed `core` package if we have Scarb toolchain.
///
/// The easiest way to do this is to create an empty Scarb package and run `scarb metadata` on it.
/// The `core` package will be a component of this empty package.
/// For minimal packages, `scarb metadata` should be pretty fast.
///
/// Because CairoLS is tightly bound to Scarb (due to hard compiler version dependency),
/// we can safely make this lookup once and keep it cached for the entire LS lifetime.
#[tracing::instrument(skip_all)]
fn find_scarb_managed_core(scarb: &ScarbToolchain) -> Option<UnmanagedCore> {
    let lookup = || {
        let workspace = tempdir()
            .context("failed to create temporary directory")
            .inspect_err(|e| warn!("{e:?}"))
            .ok()?;

        let scarb_toml = workspace.path().join(SCARB_TOML);
        fs::write(
            &scarb_toml,
            indoc! {r#"
                [package]
                name = "cairols_unmanaged_core_lookup"
                version = "1.0.0"
            "#},
        )
        .context("failed to write Scarb.toml")
        .inspect_err(|e| warn!("{e:?}"))
        .ok()?;

        let metadata = scarb.silent().metadata(&scarb_toml).inspect_err(|e| warn!("{e:?}")).ok()?;

        // Ensure the workspace directory is deleted after running Scarb.
        // We are ignoring the error, leaving doing proper clean-up to the OS.
        let _ = workspace
            .close()
            .context("failed to wipe temporary directory")
            .inspect_err(|e| warn!("{e:?}"));

        // Scarb is expected to generate only one compilation unit (for our stub package)
        // that will consist of this package and the `core` crate.
        // Therefore, we allow ourselves to liberally just look for any first usage of a package
        // named `core` in all compilation units components we got.
        let path = metadata.compilation_units.into_iter().find_map(|compilation_unit| {
            compilation_unit
                .components
                .iter()
                .find(|component| component.name == CORELIB_CRATE_NAME)
                .map(|component| component.source_root().to_path_buf().into_std_path_buf())
        })?;
        let version = metadata
            .packages
            .into_iter()
            .find(|package| package.name == CORELIB_CRATE_NAME)
            .map(|package| package.version)?;

        Some(UnmanagedCore { path, version })
    };

    static CACHE: OnceLock<Option<UnmanagedCore>> = OnceLock::new();
    CACHE.get_or_init(lookup).clone()
}

/// Makes a path absolute, or logs an error.
fn ensure_absolute(path: PathBuf) -> Option<PathBuf> {
    path::absolute(&path)
        .with_context(|| {
            format!("failed to make `core` crate path absolute: {path}", path = path.display())
        })
        .inspect_err(|e| error!("{e:?}"))
        .ok()
}
