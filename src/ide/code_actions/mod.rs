use cairo_lang_syntax::node::SyntaxNode;
use itertools::Itertools;
use lsp_types::{
    CodeAction, CodeActionOrCommand, CodeActionParams, CodeActionResponse, Diagnostic,
    NumberOrString,
};
use tracing::{debug, warn};

use crate::lang::analysis_context::AnalysisContext;
use crate::lang::db::{AnalysisDatabase, LsSyntaxGroup};
use crate::lang::lsp::{LsProtoGroup, ToCairo};

mod add_missing_trait;
mod create_module_file;
mod expand_macro;
mod fill_struct_fields;
mod fill_trait_members;
mod rename_unused_variable;

/// Compute commands for a given text document and range. These commands are typically code fixes to
/// either fix problems or to beautify/refactor code.
pub fn code_actions(params: CodeActionParams, db: &AnalysisDatabase) -> Option<CodeActionResponse> {
    let mut actions = Vec::with_capacity(params.context.diagnostics.len());
    let file_id = db.file_for_url(&params.text_document.uri)?;
    let node = db.find_syntax_node_at_position(file_id, params.range.start.to_cairo())?;

    actions.extend(
        get_code_actions_for_diagnostics(db, &node, &params)
            .into_iter()
            .map(CodeActionOrCommand::from),
    );

    actions.extend(
        expand_macro::expand_macro(db, node.clone()).into_iter().map(CodeActionOrCommand::from),
    );

    Some(actions)
}

/// Generate code actions for a given diagnostics in context of [`CodeActionParams`].
///
/// # Arguments
///
/// * `db` - A reference to the Salsa database.
/// * `node` - The syntax node where the diagnostic is located.
/// * `params` - The parameters for the code action request.
///
/// # Returns
///
/// A vector of [`CodeAction`] objects that can be applied to resolve the diagnostics.
fn get_code_actions_for_diagnostics(
    db: &AnalysisDatabase,
    node: &SyntaxNode,
    params: &CodeActionParams,
) -> Vec<CodeAction> {
    let diagnostic_groups_by_codes = params
        .context
        .diagnostics
        .iter()
        .filter_map(|diagnostic| extract_code(diagnostic).map(|code| (code, diagnostic)))
        .into_group_map();

    let Some(ctx) = AnalysisContext::from_node(db, node.clone()) else { return Default::default() };

    diagnostic_groups_by_codes
        .into_iter()
        .flat_map(|(code, diagnostics)| match code {
            "E0001" => diagnostics
                .into_iter()
                // There should be exactly one diagnostic.
                .next()
                .and_then(|diagnostic| {
                    rename_unused_variable::rename_unused_variable(
                        db,
                        node,
                        diagnostic.clone(),
                        params.text_document.uri.clone(),
                    )
                })
                .to_vec(),
            "E0002" => {
                add_missing_trait::add_missing_trait(db, &ctx, params.text_document.uri.clone())
                    .unwrap_or_default()
            }
            "E0003" => fill_struct_fields::fill_struct_fields(db, node.clone(), params).to_vec(),
            "E0004" => fill_trait_members::fill_trait_members(db, &ctx, params).to_vec(),
            "E0005" => create_module_file::create_module_file(
                db,
                node.clone(),
                params.text_document.uri.clone(),
            )
            .to_vec(),
            _ => {
                debug!("no code actions for diagnostic code: {code}");
                vec![]
            }
        })
        .collect()
}

trait VecExt<T> {
    fn to_vec(self) -> Vec<T>;
}

impl<T> VecExt<T> for Option<T> {
    fn to_vec(self) -> Vec<T> {
        self.map(|result| vec![result]).unwrap_or_default()
    }
}

/// Extracts [`Diagnostic`] code if it's given as a string, returns None otherwise.
fn extract_code(diagnostic: &Diagnostic) -> Option<&str> {
    match &diagnostic.code {
        Some(NumberOrString::String(code)) => Some(code),
        Some(NumberOrString::Number(code)) => {
            warn!("diagnostic code is not a string: `{code}`");
            None
        }
        None => None,
    }
}
