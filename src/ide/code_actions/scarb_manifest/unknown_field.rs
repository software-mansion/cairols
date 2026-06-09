use lsp_types::CodeAction;

use super::{ManifestActionContext, remove_key_path, replace_manifest_action, unknown_field_path};

pub fn build(ctx: &ManifestActionContext<'_>) -> Vec<CodeAction> {
    let Some(path) = unknown_field_path(&ctx.diagnostic.message) else {
        return vec![];
    };
    let field = path.join(".");
    let Some(new_text) = remove_key_path(ctx.raw_toml, &path) else {
        return vec![];
    };
    vec![replace_manifest_action(
        ctx,
        new_text,
        format!("Remove unknown manifest field `{field}`"),
        ctx.diagnostic.clone(),
    )]
}
