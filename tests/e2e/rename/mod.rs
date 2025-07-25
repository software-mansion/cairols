use std::collections::HashMap;
use std::ffi::OsStr;

use cairo_lang_utils::ordered_hash_map::OrderedHashMap;
use itertools::Itertools;
use lsp_types::request::Rename;
use lsp_types::{
    ClientCapabilities, DocumentChangeOperation, DocumentChanges, OneOf, RenameClientCapabilities,
    RenameFile, RenameParams, ResourceOp, ResourceOperationKind, TextDocumentClientCapabilities,
    TextDocumentPositionParams, TextEdit, Url, WorkspaceClientCapabilities, WorkspaceEdit,
    WorkspaceEditClientCapabilities, lsp_request,
};

use crate::support::MockClient;
use crate::support::cursor::{Cursors, render_text_edits_and_file_renames};
use crate::support::transform::Transformer;

mod consts;
mod enums;
mod fns;
mod invalid_new_name;
mod macros;
mod modules;
mod structs;
mod traits;
mod types;
mod vars;

const DEFAULT_NEW_NAME: &'_ str = "RENAMED";

impl Transformer for Rename {
    fn capabilities(base: ClientCapabilities) -> ClientCapabilities {
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
            workspace: base.workspace.or_else(Default::default).map(|it| {
                WorkspaceClientCapabilities {
                    workspace_edit: Some(WorkspaceEditClientCapabilities {
                        resource_operations: Some(vec![ResourceOperationKind::Rename]),
                        ..Default::default()
                    }),
                    ..it
                }
            }),
            ..base
        }
    }

    fn transform(
        ls: MockClient,
        cursors: Cursors,
        additional_data: Option<serde_json::Value>,
    ) -> String {
        let new_name = if let Some(data) = additional_data {
            data.get("new_name").unwrap().as_str().unwrap().to_string()
        } else {
            DEFAULT_NEW_NAME.to_string()
        };

        rename(ls, cursors, new_name)
    }
}

fn rename(mut ls: MockClient, cursors: Cursors, new_name: String) -> String {
    let caret = cursors.assert_single_caret();

    let file_contents: Vec<_> = ls
        .fixture
        .files()
        .iter()
        .filter(|path| path.extension() == Some(OsStr::new("cairo")))
        .map(|path| (path.to_string_lossy().to_string(), ls.fixture.read_file(path)))
        .collect();

    let params = RenameParams {
        text_document_position: TextDocumentPositionParams {
            text_document: ls.doc_id("src/lib.cairo"),
            position: caret,
        },
        new_name,
        work_done_progress_params: Default::default(),
    };
    let Some(request) = ls.send_request::<lsp_request!("textDocument/rename")>(params) else {
        return "none response".to_string();
    };

    let (text_edits, file_renames) = extract_sorted_text_edits_and_file_renames(request);

    let file_contents = file_contents
        .into_iter()
        .map(|(path, content)| (ls.fixture.file_url(&path), (path, content)))
        .collect();

    let file_renames: HashMap<_, _> = file_renames
        .into_iter()
        .map(|r| (r.old_uri, ls.fixture.url_path(&r.new_uri).unwrap()))
        .collect();

    render_text_edits_and_file_renames(text_edits, file_renames, &file_contents)
}

fn extract_sorted_text_edits_and_file_renames(
    request: WorkspaceEdit,
) -> (OrderedHashMap<Url, Vec<TextEdit>>, Vec<RenameFile>) {
    let (text_edits, file_renames) = match (request.changes, request.document_changes) {
        (None, None) => panic!("non-empty changes were expected, got empty ones instead"),
        (Some(_), Some(_)) => {
            panic!(
                "both `changes` abd `document_changes` were present, but according to LSP only one should be"
            )
        }
        (Some(changes), None) => (changes, Vec::new()),
        (None, Some(doc_changes)) => extract_relevant_data(doc_changes),
    };

    let text_edits: OrderedHashMap<_, _> = text_edits
        .into_iter()
        .sorted_by(|a, b| a.0.cmp(&b.0))
        .map(|(url, text_edit)| {
            (
                url,
                text_edit
                    .into_iter()
                    .sorted_by(|a, b| {
                        // Sorting from bottom to top of the text. Important when applying edits.
                        let x = b.range.end.cmp(&a.range.end);
                        if x.is_eq() { b.range.start.cmp(&a.range.start) } else { x }
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect();

    (text_edits, file_renames)
}

fn extract_relevant_data(
    doc_changes: DocumentChanges,
) -> (HashMap<Url, Vec<TextEdit>>, Vec<RenameFile>) {
    match doc_changes {
        DocumentChanges::Edits(text_doc_edits) => {
            let text_edits = text_doc_edits
                .into_iter()
                .map(|edit| {
                    assert!(
                        edit.text_document.version.is_none(),
                        "versioned edits are not supported"
                    );

                    let edits = edit
                        .edits
                        .into_iter()
                        .map(|e| match e {
                            OneOf::Left(text_edit) => text_edit,
                            OneOf::Right(_) => panic!("annotated text edits are not supported"),
                        })
                        .collect();
                    (edit.text_document.uri, edits)
                })
                .collect();

            (text_edits, Vec::new())
        }
        DocumentChanges::Operations(operations) => operations.into_iter().fold(
            (HashMap::new(), Vec::new()),
            |(mut text_edits, mut file_renames), doc_op| {
                match doc_op {
                    DocumentChangeOperation::Op(op) => match op {
                        ResourceOp::Rename(rename_file) => {
                            assert!(rename_file.options.is_none());
                            assert!(
                                rename_file.annotation_id.is_none(),
                                "annotated text edits are not supported"
                            );
                            file_renames.push(rename_file)
                        }
                        ResourceOp::Delete(_) | ResourceOp::Create(_) => {
                            panic!("unexpected non-rename resource op")
                        }
                    },
                    DocumentChangeOperation::Edit(text_doc_edit) => {
                        assert!(
                            text_doc_edit.text_document.version.is_none(),
                            "versioned edits are not supported"
                        );

                        let uri = text_doc_edit.text_document.uri;

                        let edits = text_doc_edit
                            .edits
                            .into_iter()
                            .map(|e| match e {
                                OneOf::Left(text_edit) => text_edit,
                                OneOf::Right(_) => {
                                    panic!("annotated text edits are not supported")
                                }
                            })
                            .collect();

                        text_edits.insert(uri, edits);
                    }
                }
                (text_edits, file_renames)
            },
        ),
    }
}
