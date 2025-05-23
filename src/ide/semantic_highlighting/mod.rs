use cairo_lang_filesystem::span::TextOffset;
use cairo_lang_parser::db::ParserGroup;
use cairo_lang_syntax as syntax;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{SyntaxNode, TypedSyntaxNode, ast};
use cairo_lang_utils::unordered_hash_map::UnorderedHashMap;
use lsp_types::{SemanticToken, SemanticTokens, SemanticTokensParams, SemanticTokensResult};
use tracing::error;

use self::encoder::{EncodedToken, TokenEncoder};
pub use self::token_kind::SemanticTokenKind;
use crate::lang::db::AnalysisDatabase;
use crate::lang::lsp::LsProtoGroup;

mod encoder;
pub mod token_kind;

/// Resolve the semantic tokens of a given file.
pub fn semantic_highlight_full(
    params: SemanticTokensParams,
    db: &AnalysisDatabase,
) -> Option<SemanticTokensResult> {
    let file_uri = params.text_document.uri;
    let file = db.file_for_url(&file_uri)?;
    let Ok(node) = db.file_syntax(file) else {
        error!("semantic analysis failed: file '{file_uri}' does not exist");
        return None;
    };

    let mut data: Vec<SemanticToken> = Vec::new();
    SemanticTokensTraverser::default().find_semantic_tokens(db, &mut data, node);
    Some(SemanticTokensResult::Tokens(SemanticTokens { result_id: None, data }))
}

#[derive(Default)]
struct SemanticTokensTraverser {
    encoder: TokenEncoder,
    /// A map from an offset in the file to semantic token kind.
    /// This map is used to override future tokens based on the context.
    /// For example: when we see the "fn" keyword, the name token is added
    /// to the map, so that instead of marking it as an identifier, we will mark it
    /// as a function name.
    offset_to_kind_lookahead: UnorderedHashMap<TextOffset, SemanticTokenKind>,
}
impl SemanticTokensTraverser {
    pub fn find_semantic_tokens(
        &mut self,
        db: &AnalysisDatabase,
        data: &mut Vec<SemanticToken>,
        node: SyntaxNode,
    ) {
        let green_node = node.green_node(db);
        match &green_node.details {
            syntax::node::green::GreenNodeDetails::Token(text) => {
                if green_node.kind == SyntaxKind::TokenNewline {
                    self.encoder.next_line();
                    return;
                }

                let width = text.len() as u32;
                let maybe_semantic_kind = self
                    .offset_to_kind_lookahead
                    .remove(&node.offset(db))
                    .or_else(|| SemanticTokenKind::from_syntax_node(db, node));

                if let Some(semantic_kind) = maybe_semantic_kind {
                    let text = node.text(db).unwrap();

                    if text.contains('\n') {
                        // Split multiline token into multiple single line tokens.
                        for line in text.split_inclusive('\n') {
                            self.push_semantic_token(line.len() as u32, &semantic_kind, data);

                            if line.ends_with('\n') {
                                self.encoder.next_line();
                            }
                        }
                    } else {
                        self.push_semantic_token(width, &semantic_kind, data);
                    }
                } else {
                    self.encoder.skip(width);
                }
            }
            syntax::node::green::GreenNodeDetails::Node { .. } => {
                let children = node.get_children(db);
                match green_node.kind {
                    SyntaxKind::Param => {
                        self.mark_future_token(
                            ast::Param::from_syntax_node(db, node)
                                .name(db)
                                .as_syntax_node()
                                .offset(db),
                            SemanticTokenKind::Parameter,
                        );
                    }
                    SyntaxKind::FunctionWithBody => {
                        self.mark_future_token(
                            ast::FunctionWithBody::from_syntax_node(db, node)
                                .declaration(db)
                                .name(db)
                                .as_syntax_node()
                                .offset(db),
                            SemanticTokenKind::Function,
                        );
                    }
                    SyntaxKind::ItemStruct => self.mark_future_token(
                        ast::ItemStruct::from_syntax_node(db, node)
                            .name(db)
                            .as_syntax_node()
                            .offset(db),
                        SemanticTokenKind::Struct,
                    ),
                    SyntaxKind::ItemEnum => self.mark_future_token(
                        ast::ItemEnum::from_syntax_node(db, node)
                            .name(db)
                            .as_syntax_node()
                            .offset(db),
                        SemanticTokenKind::Enum,
                    ),
                    _ => {}
                }
                for child in children.iter() {
                    self.find_semantic_tokens(db, data, *child);
                }
            }
        }
    }

    fn push_semantic_token(
        &mut self,
        width: u32,
        semantic_kind: &SemanticTokenKind,
        data: &mut Vec<SemanticToken>,
    ) {
        let EncodedToken { delta_line, delta_start } = self.encoder.encode(width);

        data.push(SemanticToken {
            delta_line,
            delta_start,
            length: width,
            token_type: semantic_kind.as_u32(),
            token_modifiers_bitset: 0,
        });
    }

    fn mark_future_token(&mut self, offset: TextOffset, semantic_kind: SemanticTokenKind) {
        self.offset_to_kind_lookahead.insert(offset, semantic_kind);
    }
}
