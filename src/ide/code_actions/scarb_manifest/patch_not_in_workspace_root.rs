use std::path::PathBuf;

use cairo_lang_filesystem::db::FilesGroup;
use lsp_types::{CodeAction, Url};

use super::{ManifestActionContext, move_patch_to_workspace_root_action};
use crate::lang::lsp::LsProtoGroup;

pub fn build(ctx: &ManifestActionContext<'_>) -> Vec<CodeAction> {
    let Some(member_manifest_path) = ctx.uri.to_file_path().ok() else {
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

fn diagnostic_manifest_paths(ctx: &ManifestActionContext<'_>) -> Option<(PathBuf, PathBuf)> {
    let data = ctx.diagnostic.data.as_ref()?;
    let manifest_path = data.get("manifest_path")?.as_str()?;
    let workspace_manifest_path = data.get("workspace_manifest_path")?.as_str()?;

    Some((PathBuf::from(manifest_path), PathBuf::from(workspace_manifest_path)))
}
