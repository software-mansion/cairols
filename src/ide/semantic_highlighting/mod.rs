use cairo_lang_parser::db::ParserGroup;
use cairo_lang_syntax::node::ast::{TerminalIdentifier, TerminalIdentifierPtr};
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{SyntaxNode, TypedSyntaxNode};
use lsp_types::{SemanticTokens, SemanticTokensParams, SemanticTokensResult};
use tracing::error;

pub use self::token_kind::SemanticTokenKind;
use crate::ide::semantic_highlighting::token_traverser::SemanticTokensTraverser;
use crate::lang::db::{AnalysisDatabase, LsSemanticGroup};
use crate::lang::lsp::LsProtoGroup;
use crate::state::MetaState;

mod encoder;
pub mod token_kind;
mod token_traverser;

/// Resolve the semantic tokens of a given file.
pub fn semantic_highlight_full(
    params: SemanticTokensParams,
    db: &AnalysisDatabase,
    _ls_meta_state: MetaState,
) -> Option<SemanticTokensResult> {
    let file_uri = params.text_document.uri;
    let file = db.file_for_url(&file_uri)?;
    let Ok(node) = db.file_syntax(file) else {
        error!("semantic analysis failed: file '{file_uri}' does not exist");
        return None;
    };

    Some(SemanticTokensResult::Tokens(SemanticTokens {
        result_id: None,
        data: SemanticTokensTraverser::default().get_semantic_tokens(db, node),
    }))
}

// Retrieves the most-likely-usable resultant, and the terminal ptr we can use for semantic lookup
fn get_resultants_and_closest_terminals<'db>(
    db: &'db AnalysisDatabase,
    node: SyntaxNode<'db>,
) -> Vec<(SyntaxNode<'db>, TerminalIdentifierPtr<'db>)> {
    let Some(resultants) = db.get_node_resultants(node) else {
        return vec![];
    };

    resultants
        .into_iter()
        .filter_map(|resultant| {
            let terminal = if resultant.kind(db).is_terminal() {
                Some(resultant)
            } else if resultant.kind(db).is_token() {
                resultant.ancestors(db).find(|ancestor| ancestor.kind(db).is_terminal())
            } else {
                None
            }?;

            Some((resultant, TerminalIdentifier::cast(db, terminal)?.stable_ptr(db)))
        })
        .collect()
}

/// Checks whether the given node is an inline macro invocation and not just the simple path segment.
fn is_inline_macro<'db>(db: &'db AnalysisDatabase, node: SyntaxNode<'db>) -> bool {
    if let Some(path_node) = node.ancestor_of_kind(db, SyntaxKind::ExprPath)
        && let Some(maybe_macro) = path_node.parent(db)
    {
        let kind = maybe_macro.kind(db);
        return kind == SyntaxKind::ExprInlineMacro || kind == SyntaxKind::ItemInlineMacro;
    }
    false
}
