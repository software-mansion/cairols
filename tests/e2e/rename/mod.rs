use cairo_lang_utils::ordered_hash_map::OrderedHashMap;
use itertools::Itertools;
use lsp_types::{
    ClientCapabilities, DocumentChangeOperation, DocumentChanges, OneOf, RenameClientCapabilities,
    RenameFile, RenameParams, ResourceOp, ResourceOperationKind, TextDocumentClientCapabilities,
    TextDocumentPositionParams, TextEdit, Url, WorkspaceClientCapabilities, WorkspaceEdit,
    WorkspaceEditClientCapabilities, lsp_request,
};
use std::collections::HashMap;

use crate::support::cairo_project_toml::CAIRO_PROJECT_TOML_2024_07;
use crate::support::cursor::render_text_edits_and_file_renames;
use crate::support::{cursors, sandbox};

mod modules;

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
    rename_with_additional_modules(HashMap::from([("src/lib.cairo", cairo_code)]))
}

type FileContents<'a> = HashMap<&'a str, &'a str>;

fn rename_with_additional_modules(file_contents: FileContents) -> String {
    let mut ls = sandbox! {
        files {
            "cairo_project.toml" => CAIRO_PROJECT_TOML_2024_07,
        }
        client_capabilities = caps;
    };

    let file_contents: Vec<_> = file_contents
        .into_iter()
        .map(|(path, code_with_carets)| {
            let (cairo, cursors) = cursors(code_with_carets);
            (path, cairo, cursors.carets())
        })
        .collect();

    for (path, cairo, _) in &file_contents {
        ls.fixture.add_file(path, cairo);
    }

    ls.open_all_cairo_files_and_wait_for_project_update();

    let responses: Vec<_> = file_contents
        .iter()
        .flat_map(|(path, _, carets)| {
            carets
                .iter()
                .map(|position| {
                    let params = RenameParams {
                        text_document_position: TextDocumentPositionParams {
                            text_document: ls.doc_id(path),
                            position: *position,
                        },
                        new_name: "RENAMED".to_string(),
                        work_done_progress_params: Default::default(),
                    };
                    let request = ls
                        .send_request::<lsp_request!("textDocument/rename")>(params)
                        .expect("non-empty response was expected, got empty one instead");

                    extract_sorted_text_edits_and_file_renames(request)
                })
                .collect::<Vec<_>>()
        })
        .collect();

    assert!(responses.iter().all_equal());

    let (text_edits, file_renames) = responses.into_iter().next().unwrap();

    let file_contents = file_contents
        .into_iter()
        .map(|(file, content, _)| (ls.fixture.file_url(file), (file, content)))
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
                        let x = a.range.end.cmp(&b.range.end);
                        if x.is_eq() { a.range.start.cmp(&b.range.start) } else { x }
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect();

    let file_renames: Vec<_> =
        file_renames.into_iter().sorted_by(|a, b| a.old_uri.cmp(&b.old_uri)).collect();

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
