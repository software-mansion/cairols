use cairo_lang_filesystem::ids::{FileKind, FileLongId, VirtualFile};
use cairo_lang_utils::Intern;
use lsp_types::Url;

use crate::lang::db::AnalysisDatabase;
use crate::lang::lsp::{file_for_url, url_for_file};

#[test]
fn file_url() {
    let db = AnalysisDatabase::new();

    let check = |expected_url: &str, expected_file_long: FileLongId| {
        let expected_url = Url::parse(expected_url).unwrap();
        let expected_file = expected_file_long.intern(&db);

        assert_eq!(file_for_url(&db, &expected_url), Some(expected_file));
        assert_eq!(url_for_file(&db, expected_file), Some(expected_url));
    };

    check("file:///foo/bar", FileLongId::OnDisk("/foo/bar".into()));
    check("file:///", FileLongId::OnDisk("/".into()));

    // NOTE: We expect that Salsa is assigning sequential numeric ids to files,
    //   hence numbers 12290 and 12291 appear further down.
    check(
        "vfs://12290/foo.cairo",
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
        "vfs://12291/foo%2Fbar.cairo",
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
