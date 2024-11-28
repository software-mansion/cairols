use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use scarb_metadata::MetadataCommand;
use tempfile::tempdir;

/// Finds Scarb-managed `core` package.
///
/// This is a stripped-down version of similar logic in `unmanaged_core_crate` with these changes:
/// - instant panicking instead of trying to recover,
/// - using `scarb_metadata` directly instead of `ScarbToolchain`,
/// - no tracing.
pub fn scarb_core_path() -> &'static Path {
    static CACHE: LazyLock<PathBuf> = LazyLock::new(|| {
        let workspace = tempdir().expect("failed to create temporary directory");

        let scarb_toml = workspace.path().join("Scarb.toml");
        fs::write(
            &scarb_toml,
            r#"
            [package]
            name = "cairols_unmanaged_core_lookup"
            version = "1.0.0"
        "#,
        )
        .expect("failed to write Scarb.toml");

        let metadata = MetadataCommand::new()
            .manifest_path(scarb_toml)
            .inherit_stderr()
            .exec()
            .expect("failed to execute: scarb metadata");

        // Ensure the workspace directory is deleted after running Scarb.
        workspace.close().expect("failed to wipe temporary directory");

        // Scarb is expected to generate only one compilation unit (for our stub package)
        // that will consist of this package and the `core` crate.
        // Therefore, we allow ourselves to liberally just look for any first usage of a package
        // named `core` in all compilation units components we got.
        metadata
            .compilation_units
            .into_iter()
            .find_map(|compilation_unit| {
                compilation_unit
                    .components
                    .iter()
                    .find(|component| component.name == "core")
                    .map(|component| component.source_root().to_path_buf().into_std_path_buf())
            })
            .expect("failed to find `core` crate path")
    });

    &CACHE
}

/// Finds a path where Scarb unpacks its `std` source.
pub fn scarb_registry_std_path() -> &'static Path {
    scarb_core_path().parent().unwrap().parent().unwrap()
}
