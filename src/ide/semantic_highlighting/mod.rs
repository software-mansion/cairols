pub use self::token_kind::SemanticTokenKind;
use crate::ide::semantic_highlighting::token_traverser::SemanticTokensTraverser;
use crate::lang::db::AnalysisDatabase;
use crate::lang::lsp::LsProtoGroup;
use crate::state::MetaState;
use cairo_lang_parser::db::ParserGroup;
use cairo_lang_syntax::node::SyntaxNode;
use cairo_lang_syntax::node::kind::SyntaxKind;
use lsp_types::{SemanticTokens, SemanticTokensParams, SemanticTokensResult};
use tracing::error;

mod encoder;
pub mod token_kind;
mod token_traverser;

/// Resolves the semantic tokens of a given file.
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

/// Checks whether the given node is an inline macro invocation and not just the simple path segment.
fn is_inline_macro<'db>(db: &'db AnalysisDatabase, node: SyntaxNode<'db>) -> bool {
    if matches!(node.kind(db), SyntaxKind::ExprInlineMacro) {
        return true;
    }
    if let Some(path_node) = node.ancestor_of_kind(db, SyntaxKind::ExprPath)
        && let Some(maybe_macro) = path_node.parent(db)
    {
        let kind = maybe_macro.kind(db);
        return kind == SyntaxKind::ExprInlineMacro || kind == SyntaxKind::ItemInlineMacro;
    }
    false
}
