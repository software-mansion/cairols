use lsp_types::CodeAction;

use super::{ManifestActionContext, remove_key_from_diagnostic_data_action};

pub fn build(ctx: &ManifestActionContext<'_>) -> Vec<CodeAction> {
    remove_key_from_diagnostic_data_action(ctx, "Remove unsupported").into_iter().collect()
}
