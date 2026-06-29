use lsp_types::CodeAction;
use toml_edit::value;

use super::toml::replace_key_path_value;
use super::{
    ManifestActionContext, diagnostic_string_array, remove_key_path, replace_manifest_action,
};

pub fn build(ctx: &ManifestActionContext<'_>) -> Vec<CodeAction> {
    let Some(inlining_strategy_path) =
        diagnostic_string_array(ctx.diagnostic, "inlining_strategy_path")
    else {
        return Vec::new();
    };
    let Some(skip_optimizations_path) =
        diagnostic_string_array(ctx.diagnostic, "skip_optimizations_path")
    else {
        return Vec::new();
    };

    let mut actions = Vec::new();

    if let Some(action) = remove_inlining_strategy_action(ctx, &inlining_strategy_path) {
        actions.push(action);
    }

    if let Some(action) = set_inlining_strategy_to_avoid_action(ctx, &inlining_strategy_path) {
        actions.push(action);
    }

    if let Some(action) = set_skip_optimizations_to_false_action(ctx, &skip_optimizations_path) {
        actions.push(action);
    }

    actions
}

/// Generates the quick fix that removes the conflicting `inlining-strategy` field.
fn remove_inlining_strategy_action(
    ctx: &ManifestActionContext<'_>,
    inlining_strategy_path: &[String],
) -> Option<CodeAction> {
    let new_text = remove_key_path(ctx.raw_toml, inlining_strategy_path)?;

    Some(replace_manifest_action(
        ctx,
        new_text,
        "Remove conflicting `inlining-strategy` field".to_string(),
        ctx.diagnostic.clone(),
    ))
}

/// Generates the quick fix that rewrites `inlining-strategy` to `"avoid"`.
fn set_inlining_strategy_to_avoid_action(
    ctx: &ManifestActionContext<'_>,
    inlining_strategy_path: &[String],
) -> Option<CodeAction> {
    let new_text = replace_key_path_value(
        ctx.raw_toml,
        inlining_strategy_path,
        value("avoid").into_value().expect("string literal should build a TOML value"),
    )?;

    Some(replace_manifest_action(
        ctx,
        new_text,
        "Set `inlining-strategy` to `\"avoid\"`".to_string(),
        ctx.diagnostic.clone(),
    ))
}

/// Generates the quick fix that rewrites `skip-optimizations` to `false`.
fn set_skip_optimizations_to_false_action(
    ctx: &ManifestActionContext<'_>,
    skip_optimizations_path: &[String],
) -> Option<CodeAction> {
    let new_text = replace_key_path_value(
        ctx.raw_toml,
        skip_optimizations_path,
        value(false).into_value().expect("bool literal should build a TOML value"),
    )?;

    Some(replace_manifest_action(
        ctx,
        new_text,
        "Set `skip-optimizations` to `false`".to_string(),
        ctx.diagnostic.clone(),
    ))
}
