use lsp_types::CodeAction;
use toml_edit::value;

use super::toml::replace_key_path_value;
use super::{ManifestActionContext, remove_key_at_range_action, replace_manifest_action};
use crate::lang::lsp::ToCairo;

/// Valid `inherits` profile values, see Scarb diagnostic `SE0004`.
const VALID_PROFILES: [&str; 2] = ["dev", "release"];

pub fn build(ctx: &ManifestActionContext<'_>) -> Vec<CodeAction> {
    let Some(inherits_path) = inherits_path(ctx) else {
        return Vec::new();
    };

    let mut actions = Vec::new();

    if let Some(action) = remove_inherits_action(ctx) {
        actions.push(action);
    }

    for profile in VALID_PROFILES {
        if let Some(action) = set_inherits_to_action(ctx, &inherits_path, profile) {
            actions.push(action);
        }
    }

    actions
}

/// Relies on the fact that the diagnostic is pinned to `inherits` key directly
fn inherits_path(ctx: &ManifestActionContext<'_>) -> Option<Vec<String>> {
    let offset = ctx
        .diagnostic
        .range
        .start
        .to_cairo()
        .offset_in_file(ctx.db, ctx.file_id)
        .map(|offset| offset.as_u32() as usize)?;

    super::find_key_path_at_offset(ctx.raw_toml, offset)
}

/// Generates the quick fix that removes the invalid `inherits` field.
fn remove_inherits_action(ctx: &ManifestActionContext<'_>) -> Option<CodeAction> {
    remove_key_at_range_action(ctx, "Remove invalid")
}

/// Generates the quick fix that rewrites `inherits` to a valid profile value.
fn set_inherits_to_action(
    ctx: &ManifestActionContext<'_>,
    inherits_path: &[String],
    profile: &str,
) -> Option<CodeAction> {
    let new_text = replace_key_path_value(
        ctx.raw_toml,
        inherits_path,
        value(profile).into_value().expect("string literal should build a TOML value"),
    )?;

    Some(replace_manifest_action(
        ctx,
        new_text,
        format!("Set `inherits` to `\"{profile}\"`"),
        ctx.diagnostic.clone(),
    ))
}
