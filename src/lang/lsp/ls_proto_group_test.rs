use cairo_lang_filesystem::db::{ExternalFiles, FilesGroup, init_files_group};
use cairo_lang_filesystem::ids::{FileKind, FileLongId, VirtualFile};
use cairo_lang_utils::Upcast;
use lsp_types::Url;

use super::LsProtoGroup;

// Test salsa database.
#[salsa::db]
#[derive(Clone)]
pub struct FilesDatabaseForTesting {
    storage: salsa::Storage<FilesDatabaseForTesting>,
}

#[salsa::db]
impl salsa::Database for FilesDatabaseForTesting {}

impl ExternalFiles for FilesDatabaseForTesting {}
impl Default for FilesDatabaseForTesting {
    fn default() -> Self {
        let mut res = Self { storage: Default::default() };
        init_files_group(&mut res);
        res
    }
}

// TODO(#869) This impl is missing in compiler, upstream
impl<'db> Upcast<'db, dyn FilesGroup> for FilesDatabaseForTesting {
    fn upcast(&'db self) -> &'db dyn FilesGroup {
        self
    }
}

#[test]
fn file_url() {
    let db = FilesDatabaseForTesting::default();

    let check = |expected_url: &str, expected_file_long: FileLongId| {
        let expected_url = Url::parse(expected_url).unwrap();
        let expected_file = db.intern_file(expected_file_long);

        assert_eq!(db.file_for_url(&expected_url), Some(expected_file));
        assert_eq!(db.url_for_file(expected_file), Some(expected_url));
    };

    check("file:///foo/bar", FileLongId::OnDisk("/foo/bar".into()));
    check("file:///", FileLongId::OnDisk("/".into()));

    // NOTE: We expect that Salsa is assigning sequential numeric ids to files,
    //   hence numbers 2050 and 2051 appear further down.
    check(
        "vfs://2050/foo.cairo",
        FileLongId::Virtual(VirtualFile {
            parent: None,
            name: "foo".into(),
            content: "".into(),
            code_mappings: [].into(),
            kind: FileKind::Module,
            original_item_removed: false,
        }),
    );
    check(
        "vfs://2051/foo%2Fbar.cairo",
        FileLongId::Virtual(VirtualFile {
            parent: None,
            name: "foo/bar".into(),
            content: "".into(),
            code_mappings: [].into(),
            kind: FileKind::Module,
            original_item_removed: false,
        }),
    );
}
