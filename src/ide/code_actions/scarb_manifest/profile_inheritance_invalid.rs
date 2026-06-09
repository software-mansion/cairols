use lsp_types::CodeAction;

use super::{ManifestActionContext, remove_key_at_range_action};

pub fn build(ctx: &ManifestActionContext<'_>) -> Vec<CodeAction> {
    remove_key_at_range_action(ctx, "Remove invalid").into_iter().collect()
}
