use lsp_types::CodeAction;
use toml_edit::value;

use super::toml::{remove_key_path, replace_key_path_value};
use super::{ManifestActionContext, diagnostic_string_array, replace_manifest_action};

pub fn build(ctx: &ManifestActionContext<'_>) -> Vec<CodeAction> {
    let Some(inherits_path) = diagnostic_string_array(ctx.diagnostic, "field_path") else {
        return Vec::new();
    };
    let Some(valid_profiles) = diagnostic_string_array(ctx.diagnostic, "valid_values") else {
        return Vec::new();
    };

    let mut actions = Vec::new();

    if let Some(action) = remove_inherits_action(ctx, &inherits_path) {
        actions.push(action);
    }

    for profile in valid_profiles {
        if let Some(action) = set_inherits_to_action(ctx, &inherits_path, profile) {
            actions.push(action);
        }
    }

    actions
}

/// Generates the quick fix that removes the invalid `inherits` field.
fn remove_inherits_action(
    ctx: &ManifestActionContext<'_>,
    inherits_path: &[String],
) -> Option<CodeAction> {
    let new_text = remove_key_path(ctx.raw_toml, inherits_path)?;

    Some(replace_manifest_action(
        ctx,
        new_text,
        "Remove invalid `inherits` field".to_string(),
        ctx.diagnostic.clone(),
    ))
}

/// Generates the quick fix that rewrites `inherits` to a valid profile value.
fn set_inherits_to_action(
    ctx: &ManifestActionContext<'_>,
    inherits_path: &[String],
    profile: String,
) -> Option<CodeAction> {
    let new_text = replace_key_path_value(
        ctx.raw_toml,
        inherits_path,
        value(profile.as_str()).into_value().expect("string literal should build a TOML value"),
    )?;

    Some(replace_manifest_action(
        ctx,
        new_text,
        format!("Set `inherits` to `\"{profile}\"`"),
        ctx.diagnostic.clone(),
    ))
}
