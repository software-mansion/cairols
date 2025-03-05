use std::{path::PathBuf, sync::LazyLock};

pub const SCARB_TEST_MACROS_PACKAGE_NAME: &str = "scarb_procedural_macros";

#[expect(dead_code)] // Removed in the next PR.
pub static SCARB_TEST_MACROS_PACKAGE: LazyLock<PathBuf> = LazyLock::new(|| {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join(SCARB_TEST_MACROS_PACKAGE_NAME)
        .canonicalize()
        .expect("should be able to obtain an absolute path to `scarb_procedural_macros`")
});
