use lsp_types::CodeAction;
use toml_edit::value;

use super::toml::replace_key_path_value;
use super::{
    ManifestActionContext, remove_key_at_range_action, replace_manifest_action, sibling_key_path,
};
use crate::lang::lsp::ToCairo;

pub fn build(ctx: &ManifestActionContext<'_>) -> Vec<CodeAction> {
    let Some(inlining_strategy_path) = inlining_strategy_path(ctx) else {
        return Vec::new();
    };

    let mut actions = Vec::new();

    if let Some(action) = remove_inlining_strategy_action(ctx) {
        actions.push(action);
    }

    if let Some(action) = set_inlining_strategy_to_avoid_action(ctx, &inlining_strategy_path) {
        actions.push(action);
    }

    if let Some(action) = set_skip_optimizations_to_false_action(ctx, &inlining_strategy_path) {
        actions.push(action);
    }

    actions
}

fn inlining_strategy_path(ctx: &ManifestActionContext<'_>) -> Option<Vec<String>> {
    let offset = ctx
        .diagnostic
        .range
        .start
        .to_cairo()
        .offset_in_file(ctx.db, ctx.file_id)
        .map(|offset| offset.as_u32() as usize)?;

    super::find_key_path_at_offset(ctx.raw_toml, offset)
}

/// Generates the quick fix that removes the conflicting `inlining-strategy` field.
fn remove_inlining_strategy_action(ctx: &ManifestActionContext<'_>) -> Option<CodeAction> {
    remove_key_at_range_action(ctx, "Remove conflicting")
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
    inlining_strategy_path: &[String],
) -> Option<CodeAction> {
    let skip_optimizations_path = sibling_key_path(inlining_strategy_path, "skip-optimizations")?;

    let new_text = replace_key_path_value(
        ctx.raw_toml,
        &skip_optimizations_path,
        value(false).into_value().expect("bool literal should build a TOML value"),
    )?;

    Some(replace_manifest_action(
        ctx,
        new_text,
        "Set `skip-optimizations` to `false`".to_string(),
        ctx.diagnostic.clone(),
    ))
}
