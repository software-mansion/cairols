use cairo_lang_syntax::node::SyntaxNode;
use lsp_types::{
    CodeAction, CodeActionKind, CodeActionOrCommand, CodeActionParams, CodeActionResponse,
    Diagnostic, NumberOrString, Range, TextEdit, Url, WorkspaceEdit,
};
use tracing::{debug, warn};

use crate::lang::analysis_context::AnalysisContext;
use crate::lang::db::{AnalysisDatabase, LsSyntaxGroup};
use crate::lang::lsp::{LsProtoGroup, ToCairo};
use itertools::Itertools;
use std::collections::HashMap;

mod add_missing_trait;
mod create_module_file;
mod expand_macro;
mod fill_struct_fields;
mod fill_trait_members;
mod missing_import;
mod rename_unused_variable;

/// Compute commands for a given text document and range. These commands are typically code fixes to
/// either fix problems or to beautify/refactor code.
pub fn code_actions(params: CodeActionParams, db: &AnalysisDatabase) -> Option<CodeActionResponse> {
    let mut actions = Vec::with_capacity(params.context.diagnostics.len());

    actions.extend(
        get_code_actions_for_diagnostics(db, &params).into_iter().map(CodeActionOrCommand::from),
    );

    let node = node_on_range_start(db, &params.text_document.uri, &params.range)?;

    actions.extend(expand_macro::expand_macro(db, node).into_iter().map(CodeActionOrCommand::from));

    Some(actions)
}

/// Generate code actions for a given diagnostics in context of [`CodeActionParams`].
///
/// # Arguments
///
/// * `db` - A reference to the Salsa database.
/// * `params` - The parameters for the code action request.
///
/// # Returns
///
/// A vector of [`CodeAction`] objects that can be applied to resolve the diagnostics.
fn get_code_actions_for_diagnostics(
    db: &AnalysisDatabase,
    params: &CodeActionParams,
) -> Vec<CodeAction> {
    let uri = &params.text_document.uri;

    let mut result: Vec<_> = params
        .context
        .diagnostics
        .iter()
        .filter_map(|diagnostic| {
            let code = extract_code(diagnostic)?;

            Some((code, diagnostic))
        })
        // Sometimes diagnostics can be duplicated, for example diagostics from macro generated code have ranges mapped to macro call.
        .dedup_by(|(code1, diagnostic1), (code2, diagnostic2)| {
            code1 == code2 && diagnostic1.range == diagnostic2.range
        })
        .filter_map(|(code, diagnostic)| {
            let node = node_on_range_start(db, uri, &diagnostic.range)?;

            let ctx = AnalysisContext::from_node(db, node)?;

            Some((code, diagnostic, ctx))
        })
        .flat_map(|(code, diagnostic, ctx)| match code {
            "E0001" => rename_unused_variable::rename_unused_variable(
                db,
                &ctx.node,
                diagnostic.clone(),
                uri.clone(),
            )
            .to_vec(),
            "E0002" => {
                add_missing_trait::add_missing_trait(db, &ctx, uri.clone()).unwrap_or_default()
            }
            "E0003" => {
                fill_struct_fields::fill_struct_fields(db, ctx.node.clone(), params).to_vec()
            }
            "E0004" => fill_trait_members::fill_trait_members(db, &ctx, params).to_vec(),
            "E0005" => {
                create_module_file::create_module_file(db, ctx.node.clone(), uri.clone()).to_vec()
            }
            "E0006" => missing_import::missing_import(db, &ctx, uri.clone()).unwrap_or_default(),
            _ => {
                debug!("no code actions for diagnostic code: {code}");
                vec![]
            }
        })
        .collect();

    let changes = result
        .iter()
        .filter(|action| action.is_preferred.is_some_and(|a| a))
        .flat_map(|action| action.edit.as_ref())
        .flat_map(|edit| edit.changes.as_ref())
        .cloned()
        .fold(HashMap::<Url, Vec<TextEdit>>::new(), |mut acc, changes| {
            for (url, edits) in changes {
                let entry = acc.entry(url).or_default();

                entry.extend(edits);
            }
            acc
        });

    if !changes.is_empty() {
        result.push(CodeAction {
            title: "Fix All".to_string(),
            kind: Some(CodeActionKind::SOURCE_FIX_ALL),
            edit: Some(WorkspaceEdit { changes: Some(changes), ..Default::default() }),
            ..Default::default()
        });
    }

    result
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

fn node_on_range_start(db: &AnalysisDatabase, uri: &Url, range: &Range) -> Option<SyntaxNode> {
    let file_id = db.file_for_url(uri)?;

    db.find_syntax_node_at_position(file_id, range.start.to_cairo())
}
