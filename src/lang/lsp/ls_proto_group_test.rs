use assert_fs::TempDir;
use assert_fs::prelude::*;
use cairo_lang_defs::db::DefsGroup;
use cairo_lang_defs::ids::ModuleId;
use cairo_lang_filesystem::db::CrateSettings;
use cairo_lang_filesystem::ids::{FileId, FileLongId};
use cairo_lang_utils::Intern;
use lsp_types::Url;

use crate::lang::{db::AnalysisDatabase, lsp::LsProtoGroup};
use crate::project::Crate;

/// Asserts that `file` and `url` convert into each other.
fn check(db: &AnalysisDatabase, url: &str, file: FileId<'_>) {
    let url = Url::parse(url).unwrap();
    assert_eq!(db.url_for_file(file), Some(url.clone()));
    assert_eq!(db.file_for_url(&url), Some(file));
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
    // A crate with a single derive, which the builtin derive plugin expands into a generated file
    // named `impls`.
    let root = TempDir::new().unwrap();
    root.child("lib.cairo").write_str("#[derive(Drop)]\nstruct S {}\n").unwrap();
    let krate = Crate {
        name: "test".into(),
        discriminator: Some("test".into()),
        root: root.path().to_path_buf(),
        custom_main_file_stems: None,
        settings: CrateSettings::default(),
        builtin_plugins: Default::default(),
    };
    let mut db = AnalysisDatabase::new();
    krate.apply(&mut db, None);
    let db = &db;

    let module = ModuleId::CrateRoot(krate.input().into_crate_long_id(db).intern(db));
    let generated = db
        .module_files(module)
        .unwrap()
        .iter()
        .copied()
        .find(|file| !matches!(file.long(db), FileLongId::OnDisk(_)))
        .expect("the derive should generate a file");

    // The origin of the generated file is the whole `struct` item, attribute included.
    let lib = format!("{}/lib.cairo:1:1: 2:12", root.path().display());
    check(db, &format!("vfs://{lib}/impls.cairo"), generated);
}
