use std::path::PathBuf;

use cairo_lang_filesystem::db::FilesGroup;
use lsp_types::{
    CodeAction, CodeActionKind, DocumentChanges, OneOf, OptionalVersionedTextDocumentIdentifier,
    TextDocumentEdit, TextEdit, Url, WorkspaceEdit,
};

use super::toml::move_patch_to_workspace_root;
use super::{ManifestActionContext, full_document_range};
use crate::lang::lsp::LsProtoGroup;

pub fn build(ctx: &ManifestActionContext<'_>) -> Vec<CodeAction> {
    let Some(member_manifest_path) =
        ctx.uri.to_file_path().ok().and_then(|path| path.canonicalize().ok())
    else {
        return vec![];
    };
    let Some((diagnostic_manifest_path, workspace_root_manifest_path)) =
        diagnostic_manifest_paths(ctx)
    else {
        return vec![];
    };
    if diagnostic_manifest_path != member_manifest_path
        || workspace_root_manifest_path == member_manifest_path
    {
        return vec![];
    }

    let Some(workspace_root_uri) = Url::from_file_path(&workspace_root_manifest_path).ok() else {
        return vec![];
    };
    let Some(workspace_root_file_id) = ctx.db.file_for_url(&workspace_root_uri) else {
        return vec![];
    };
    let Some(workspace_root_raw_toml) = ctx.db.file_content(workspace_root_file_id) else {
        return vec![];
    };

    move_patch_to_workspace_root_action(
        ctx,
        workspace_root_uri,
        workspace_root_file_id,
        workspace_root_raw_toml,
    )
    .into_iter()
    .collect()
}

fn move_patch_to_workspace_root_action(
    ctx: &ManifestActionContext<'_>,
    workspace_root_uri: Url,
    workspace_root_file_id: cairo_lang_filesystem::ids::FileId<'_>,
    workspace_root_raw_toml: &str,
) -> Option<CodeAction> {
    let (member_new_text, workspace_root_new_text) =
        move_patch_to_workspace_root(ctx.raw_toml, workspace_root_raw_toml)?;

    let member_edit = TextDocumentEdit {
        text_document: OptionalVersionedTextDocumentIdentifier {
            uri: ctx.uri.clone(),
            version: None,
        },
        edits: vec![OneOf::Left(TextEdit {
            range: full_document_range(ctx.db, ctx.file_id, ctx.raw_toml),
            new_text: member_new_text,
        })],
    };
    let workspace_root_edit = TextDocumentEdit {
        text_document: OptionalVersionedTextDocumentIdentifier {
            uri: workspace_root_uri,
            version: None,
        },
        edits: vec![OneOf::Left(TextEdit {
            range: full_document_range(ctx.db, workspace_root_file_id, workspace_root_raw_toml),
            new_text: workspace_root_new_text,
        })],
    };

    Some(CodeAction {
        title: "Move `[patch]` to workspace root manifest".to_string(),
        kind: Some(CodeActionKind::QUICKFIX),
        is_preferred: Some(true),
        edit: Some(WorkspaceEdit {
            changes: None,
            document_changes: Some(DocumentChanges::Edits(vec![member_edit, workspace_root_edit])),
            change_annotations: None,
        }),
        diagnostics: Some(vec![ctx.diagnostic.clone()]),
        ..Default::default()
    })
}

/// Extracts and canonicalizes the manifest paths from the diagnostic data.
///
/// Returns the member manifest path first and the workspace root manifest path second.
fn diagnostic_manifest_paths(ctx: &ManifestActionContext<'_>) -> Option<(PathBuf, PathBuf)> {
    let data = ctx.diagnostic.data.as_ref()?;
    let manifest_path = data.get("manifest_path")?.as_str()?;
    let workspace_manifest_path = data.get("workspace_manifest_path")?.as_str()?;

    Some((
        PathBuf::from(manifest_path).canonicalize().ok()?,
        PathBuf::from(workspace_manifest_path).canonicalize().ok()?,
    ))
}
