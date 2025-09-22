use lsp_types::{SemanticTokens, SemanticTokensParams, SemanticTokensResult};
use tracing::error;

pub use self::token_kind::SemanticTokenKind;
use crate::lang::db::AnalysisDatabase;
use crate::lang::lsp::LsProtoGroup;
use crate::state::MetaState;
use crate::{
    ide::semantic_highlighting::token_traverser::SemanticTokensTraverser,
    lang::db::upstream::file_syntax,
};

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
    let Ok(node) = file_syntax(db, file) else {
        error!("semantic analysis failed: file '{file_uri}' does not exist");
        return None;
    };

    Some(SemanticTokensResult::Tokens(SemanticTokens {
        result_id: None,
        data: SemanticTokensTraverser::default().get_semantic_tokens(db, node),
    }))
}
