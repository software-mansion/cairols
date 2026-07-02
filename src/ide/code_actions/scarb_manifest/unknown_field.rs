use lsp_types::CodeAction;

use super::{
    ManifestActionContext, diagnostic_string_array, remove_key_path, replace_manifest_action,
};

pub fn build(ctx: &ManifestActionContext<'_>) -> Vec<CodeAction> {
    let Some(path) = diagnostic_string_array(ctx.diagnostic, "field_path") else {
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
