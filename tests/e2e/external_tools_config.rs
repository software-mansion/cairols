use indoc::indoc;
use lsp_types::notification::DidChangeWatchedFiles;
use lsp_types::request::Formatting;
use lsp_types::{
    DidChangeWatchedFilesParams, DocumentFormattingParams, FileChangeType, FileEvent, Position,
    Range, TextEdit,
};
use serde_json::json;

use crate::support::sandbox;

#[test]
fn lint_config_is_respected() {
    let mut ls = sandbox! {
        files {
            "Scarb.toml" => indoc! (r#"
                [package]
                name = "a"
                version = "0.1.0"
                edition = "2024_07"
            "#),
            "src/lib.cairo" => indoc! {r#"
                fn main() {
                    ((5));  // This should generate a diagnostic.
                }
            "#
            },
        }
        workspace_configuration = json!({
            "cairo1": {
                "enableLinter": true,
            }
        });
    };

    let diags = ls.open_and_wait_for_diagnostics("src/lib.cairo");
    assert_eq!(diags.len(), 1);
    assert!(diags[0].message.contains("double parentheses"));

    ls.edit_file(
        "Scarb.toml",
        indoc!(
            r#"
                [package]
                name = "a"
                version = "0.1.0"
                edition = "2024_07"

                [tool.cairo-lint]
                double_parens = false
            "#,
        ),
    );
    ls.send_notification::<DidChangeWatchedFiles>(DidChangeWatchedFilesParams {
        changes: vec![FileEvent { uri: ls.doc_id("Scarb.toml").uri, typ: FileChangeType::CHANGED }],
    });

    ls.wait_for_diagnostics_generation();

    assert!(ls.get_diagnostics_for_file("src/lib.cairo").is_empty());
}

#[test]
fn fmt_config_is_respected() {
    let mut ls = sandbox! {
        files {
            "Scarb.toml" => indoc! (r#"
                [package]
                name = "a"
                version = "0.1.0"
                edition = "2024_07"

                [dependencies]
                snforge_std = "0.38.0"

                [tool.fmt]
                tab-size = 2
            "#),
            "src/lib.cairo" => indoc! {r#"
                fn main() {
                    5;
                }
            "#
            },
        }
    };

    ls.open_and_wait_for_diagnostics_generation("src/lib.cairo");
    let text_edits = ls
        .send_request::<Formatting>(DocumentFormattingParams {
            text_document: ls.doc_id("src/lib.cairo"),
            options: Default::default(),
            work_done_progress_params: Default::default(),
        })
        .unwrap();

    assert_eq!(
        text_edits,
        vec![TextEdit {
            range: Range {
                start: Position { line: 0, character: 0 },
                end: Position { line: 3, character: 0 }
            },
            new_text: indoc! {r#"
                fn main() {
                  5;
                }
            "# }
            .to_string()
        }]
    );
}
