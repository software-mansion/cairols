use lsp_types::CodeAction;

use super::toml::remove_key_path;
use super::{
    ManifestActionContext, diagnostic_string_array, replace_manifest_action_with_preference,
};

pub fn build(ctx: &ManifestActionContext<'_>) -> Vec<CodeAction> {
    let Some(sources) = diagnostic_string_array(ctx.diagnostic, "sources") else {
        return Vec::new();
    };

    sources.iter().filter_map(|source| remove_patch_source_action(ctx, source)).collect()
}

/// Generates the quick fix that removes one of the two conflicting `[patch.<source>]` tables.
///
/// Both fixes are non-preferred: neither source is inherently the right one to drop.
fn remove_patch_source_action(ctx: &ManifestActionContext<'_>, source: &str) -> Option<CodeAction> {
    let path = vec!["patch".to_string(), source.to_string()];
    let new_text = remove_key_path(ctx.raw_toml, &path)?;

    Some(replace_manifest_action_with_preference(
        ctx,
        new_text,
        format!("Remove `{source}` patch source"),
        ctx.diagnostic.clone(),
        false,
    ))
}
