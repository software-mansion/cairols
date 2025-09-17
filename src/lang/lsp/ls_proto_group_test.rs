use cairo_lang_filesystem::ids::{FileKind, FileLongId, VirtualFile};
use cairo_lang_utils::Intern;
use lsp_types::Url;

use crate::lang::{db::AnalysisDatabase, lsp::LsProtoGroup};
use salsa::AsDynDatabase;

#[test]
fn file_url() {
    let analysis_database = AnalysisDatabase::new();
    let db = analysis_database.as_dyn_database();

    let check = |expected_url: &str, expected_file_long: FileLongId| {
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

    check("file:///foo/bar", FileLongId::OnDisk("/foo/bar".into()));
    check("file:///", FileLongId::OnDisk("/".into()));

    // NOTE: We expect that Salsa is assigning sequential numeric ids to files,
    //   hence numbers 8194 and 8195 appear further down.
    check(
        "vfs://8194/foo.cairo",
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
        "vfs://8195/foo%2Fbar.cairo",
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
