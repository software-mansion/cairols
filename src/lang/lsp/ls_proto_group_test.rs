use assert_fs::TempDir;
use assert_fs::prelude::*;
use cairo_lang_filesystem::db::{CrateSettings, ExperimentalFeaturesConfig, init_dev_corelib};
use cairo_lang_filesystem::ids::{FileId, FileLongId};
use cairo_lang_utils::Intern;
use lsp_types::Url;

use crate::lang::{db::AnalysisDatabase, lsp::LsProtoGroup};
use crate::project::Crate;

/// Shares the Scarb-managed `core` lookup with the e2e tests.
#[path = "../../../tests/e2e/support/scarb.rs"]
#[allow(dead_code)]
mod scarb;
use scarb::scarb_core_path;

/// Asserts that `file` and `url` convert into each other.
fn check(db: &AnalysisDatabase, url: &str, file: FileId<'_>) {
    let url = Url::parse(url).unwrap();
    assert_eq!(db.url_for_file(file), Some(url.clone()));
    assert_eq!(db.file_for_url(&url), Some(file));
}

/// Asserts that `vfs:///<location>/<name>.cairo` resolves to the generated file the compiler
/// names `<location>[<name>]` in [`FileLongId::full_path`], and that the file converts back.
fn check_generated(db: &AnalysisDatabase, location: &str, name: &str) {
    let url = Url::parse(&format!("vfs://{location}/{name}.cairo")).unwrap();
    let file = db.file_for_url(&url).unwrap_or_else(|| panic!("{url} should resolve"));
    assert_eq!(file.full_path(db), format!("{location}[{name}]"));
    assert_eq!(db.url_for_file(file), Some(url));
}

/// Builds a database with a single crate whose `lib.cairo` has the given content.
fn db_with_crate(lib_cairo: &str, settings: CrateSettings) -> (TempDir, AnalysisDatabase) {
    let root = TempDir::new().unwrap();
    root.child("lib.cairo").write_str(lib_cairo).unwrap();
    let crate_ = Crate {
        name: "test".into(),
        discriminator: Some("test".into()),
        root: root.path().to_path_buf(),
        custom_main_file_stems: None,
        settings,
        builtin_plugins: Default::default(),
    };
    let mut db = AnalysisDatabase::new();
    // Resolving a vfs URL semantically analyses the crate, which needs `core`. Take Scarb's.
    init_dev_corelib(&mut db, scarb_core_path().to_path_buf());
    crate_.apply(&mut db, None);
    (root, db)
}

#[test]
fn on_disk_file_url() {
    let db = &AnalysisDatabase::new();

    check(db, "file:///foo/bar", FileLongId::OnDisk("/foo/bar".into()).intern(db));
    check(db, "file:///", FileLongId::OnDisk("/".into()).intern(db));
}

/// Generated files are addressed by their origin, the way the compiler prints it in
/// [`FileLongId::full_path`]: the parent's location (`<path>:<line>:<col>: <line>:<col>`)
/// followed by the file name, so nothing in the URL depends on Salsa ids.
#[test]
fn generated_file_url() {
    // A single derive, which the builtin derive plugin expands into a generated file named `impls`.
    let (root, db) = db_with_crate("#[derive(Drop)]\nstruct S {}\n", CrateSettings::default());

    // The origin of the generated file is the whole `struct` item, attribute included.
    let lib = format!("{}/lib.cairo:1:1: 2:12", root.path().display());
    check_generated(&db, &lib, "impls");
}

/// Every level of macro expansion adds a `[<name>]:<line>:<col>: <line>:<col>` segment: the
/// location of the origin inside the parent's generated file.
#[test]
fn nested_generated_file_url() {
    // A derive inside a user-defined macro expansion: the derive's `impls` file is generated from
    // the macro's file `m`, which in turn is generated from `lib.cairo`.
    let (root, db) = db_with_crate(
        "macro m {\n    () => {\n        #[derive(Drop)]\n        struct S {}\n    };\n}\nm!();\n",
        CrateSettings {
            experimental_features: ExperimentalFeaturesConfig {
                user_defined_inline_macros: true,
                ..Default::default()
            },
            ..Default::default()
        },
    );

    // The origin of `m` is the `m!();` item; the origin of `impls` is the `struct` inside `m`.
    let m: String = format!("{}/lib.cairo:7:1: 7:6", root.path().display());
    check_generated(&db, &m, "m");
    check_generated(&db, &format!("{m}[m]:1:9: 2:20"), "impls");
}
