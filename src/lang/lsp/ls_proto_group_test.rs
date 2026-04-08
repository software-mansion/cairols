use cairo_lang_filesystem::ids::{FileKind, FileLongId, SmolStrId, VirtualFile};
use cairo_lang_utils::Intern;
use lsp_types::Url;

use crate::lang::{db::AnalysisDatabase, lsp::LsProtoGroup};

#[test]
fn file_url() {
    let db = &AnalysisDatabase::new();

    let check_on_disk = |expected_url: &str, expected_file_long: FileLongId| {
        let expected_url = Url::parse(expected_url).unwrap();
        let expected_file = expected_file_long.intern(db);

        assert_eq!(
            db.file_for_url(&expected_url),
            Some(expected_file),
            "just use {}",
            db.url_for_file(expected_file).unwrap()
        );
        assert_eq!(
            db.url_for_file(expected_file),
            Some(expected_url.clone()),
            "just use {:?}",
            db.file_for_url(&expected_url).unwrap()
        );
    };

    let check_virtual = |expected_path: &str, expected_file_long: FileLongId| {
        let expected_file = expected_file_long.intern(db);
        let actual_url = db.url_for_file(expected_file).unwrap();
        let expected_host = expected_file.as_intern_id().index().to_string();

        assert_eq!(db.file_for_url(&actual_url), Some(expected_file), "just use {actual_url}");
        assert_eq!(actual_url.scheme(), "vfs");
        assert_eq!(
            actual_url.host_str(),
            Some(expected_host.as_str()),
            "unexpected host in {actual_url}"
        );
        assert_eq!(actual_url.path(), expected_path, "unexpected path in {actual_url}");
    };

    check_on_disk("file:///foo/bar", FileLongId::OnDisk("/foo/bar".into()));
    check_on_disk("file:///", FileLongId::OnDisk("/".into()));

    check_virtual(
        "/foo.cairo",
        FileLongId::Virtual(VirtualFile {
            parent: None,
            name: SmolStrId::from(db, "foo"),
            content: SmolStrId::from(db, ""),
            code_mappings: [].into(),
            kind: FileKind::Module,
            original_item_removed: false,
        }),
    );
    check_virtual(
        "/foo%2Fbar.cairo",
        FileLongId::Virtual(VirtualFile {
            parent: None,
            name: SmolStrId::from(db, "foo/bar"),
            content: SmolStrId::from(db, ""),
            code_mappings: [].into(),
            kind: FileKind::Module,
            original_item_removed: false,
        }),
    );
}
