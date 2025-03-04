mod modules;

use crate::support::cairo_project_toml::CAIRO_PROJECT_TOML_2024_07;
use crate::support::cursor::render_text_edits;
use crate::support::{MockClient, cursors, sandbox};
use itertools::Itertools;
use lsp_types::{
    ClientCapabilities, DocumentChangeOperation, DocumentChanges, OneOf, RenameClientCapabilities,
    RenameFile, RenameParams, ResourceOp, ResourceOperationKind, TextDocumentClientCapabilities,
    TextDocumentEdit, TextDocumentIdentifier, TextDocumentPositionParams, TextEdit,
    WorkspaceClientCapabilities, WorkspaceEdit, WorkspaceEditClientCapabilities, lsp_request,
};
use std::collections::HashMap;

fn caps(base: ClientCapabilities) -> ClientCapabilities {
    ClientCapabilities {
        text_document: base.text_document.or_else(Default::default).map(|it| {
            TextDocumentClientCapabilities {
                rename: Some(RenameClientCapabilities {
                    dynamic_registration: Some(false),
                    prepare_support: None,
                    prepare_support_default_behavior: None,
                    honors_change_annotations: None,
                }),
                ..it
            }
        }),
        workspace: base.workspace.or_else(Default::default).map(|it| WorkspaceClientCapabilities {
            workspace_edit: Some(WorkspaceEditClientCapabilities {
                resource_operations: Some(vec![ResourceOperationKind::Rename]),
                ..Default::default()
            }),
            ..it
        }),
        ..base
    }
}

#[expect(dead_code)]
fn rename(cairo_code: &str) -> String {
    rename_with_additional_modules(cairo_code, HashMap::new())
}

fn rename_with_additional_modules(cairo_code: &str, module_files: HashMap<&str, &str>) -> String {
    let (cairo, cursors) = cursors(cairo_code);

    let mut ls = sandbox! {
        files {
            "cairo_project.toml" => CAIRO_PROJECT_TOML_2024_07,
            "src/lib.cairo" => cairo.clone(),
        }
        client_capabilities = caps;
    };

    for (file, code) in module_files {
        ls.fixture.add_file(file, code);
    }

    ls.open_all_cairo_files_and_wait_for_project_update();
    let text_doc_id = ls.doc_id("src/lib.cairo");

    assert_ne!(cursors.carets().len(), 0);

    let responses: Vec<_> = cursors
        .carets()
        .into_iter()
        .map(|position| {
            let params = RenameParams {
                text_document_position: TextDocumentPositionParams {
                    text_document: text_doc_id.clone(),
                    position,
                },
                new_name: "RENAMED".to_string(),
                work_done_progress_params: Default::default(),
            };
            ls.send_request::<lsp_request!("textDocument/rename")>(params)
                .expect("non-empty response was expected, got empty one instead")
        })
        .collect();

    assert!(responses.iter().all_equal());

    let WorkspaceEdit { changes, document_changes, .. } = responses.into_iter().next().unwrap();

    let mut result = String::new();
    let (text_edits, file_renames) = match (changes, document_changes) {
        (None, None) => panic!("non-empty changes were expected, got empty ones instead"),
        (Some(_), Some(_)) => {
            panic!(
                "both `changes` abd `document_changes` were present, but according to LSP only one should be"
            )
        }
        (Some(mut changes), None) => {
            assert_eq!(changes.len(), 1);
            let (_, text_edits) = changes.remove_entry(&text_doc_id.uri).unwrap();
            (text_edits, Vec::new())
        }
        (None, Some(doc_changes)) => extract_text_edits_and_file_renames(doc_changes, text_doc_id),
    };

    result += &render_file_renames(file_renames, ls);
    result += &render_text_edits(text_edits, cairo);

    result
}

fn extract_text_edits_and_file_renames(
    doc_changes: DocumentChanges,
    text_doc_id: TextDocumentIdentifier,
) -> (Vec<TextEdit>, Vec<RenameFile>) {
    match doc_changes {
        DocumentChanges::Edits(text_doc_edits) => {
            assert_eq!(text_doc_edits.len(), 1);
            let TextDocumentEdit { text_document, edits } =
                text_doc_edits.into_iter().next().unwrap();
            assert_eq!(text_document.uri, text_doc_id.uri, "invalid uri");
            assert!(text_document.version.is_none(), "versioned edits are not supported");
            let text_edits = edits
                .into_iter()
                .map(|e| match e {
                    OneOf::Left(text_edit) => text_edit,
                    OneOf::Right(_) => panic!("annotated text edits are not supported"),
                })
                .collect();
            (text_edits, Vec::new())
        }
        DocumentChanges::Operations(operations) => {
            let mut text_edit_encountered = false;
            operations.into_iter().fold(
                (Vec::new(), Vec::new()),
                |(mut text_edits, mut file_renames), doc_op| {
                    match doc_op {
                        DocumentChangeOperation::Op(op) => match op {
                            ResourceOp::Rename(rename_file) => file_renames.push(rename_file),
                            ResourceOp::Delete(_) | ResourceOp::Create(_) => {
                                panic!("unexpected non-rename resource op")
                            }
                        },
                        DocumentChangeOperation::Edit(text_doc_edit) => {
                            if text_edit_encountered {
                                panic!("expected one text edit, got more")
                            }
                            text_edit_encountered = true;

                            assert_eq!(
                                text_doc_edit.text_document.uri, text_doc_id.uri,
                                "invalid uri"
                            );
                            assert!(
                                text_doc_edit.text_document.version.is_none(),
                                "versioned edits are not supported"
                            );

                            text_edits.extend(text_doc_edit.edits.into_iter().map(|e| match e {
                                OneOf::Left(text_edit) => text_edit,
                                OneOf::Right(_) => {
                                    panic!("annotated text edits are not supported")
                                }
                            }));
                        }
                    }
                    (text_edits, file_renames)
                },
            )
        }
    }
}

fn render_file_renames(file_renames: Vec<RenameFile>, ls: MockClient) -> String {
    let mut result = String::new();
    let non_empty = !file_renames.is_empty();

    if non_empty {
        result.push_str("File renames:\n");
    }
    for rename in file_renames {
        assert!(rename.options.is_none());
        assert!(rename.annotation_id.is_none(), "annotated text edits are not supported");
        let old_path = ls.fixture.url_path(&rename.old_uri).unwrap();
        let new_path = ls.fixture.url_path(&rename.new_uri).unwrap();
        result.push_str(&format!("- {} -> {}\n", old_path.display(), new_path.display()));
    }
    if non_empty {
        result.push_str("================================\n\n");
    }

    result
}
