use cairo_lang_filesystem::span::TextOffset;
use cairo_lang_syntax::node::{
    SyntaxNode, TypedSyntaxNode, ast, green::GreenNodeDetails, kind::SyntaxKind,
};
use cairo_lang_utils::unordered_hash_map::UnorderedHashMap;
use lsp_types::SemanticToken;

use super::token_kind::SemanticTokenKind;
use crate::{
    ide::semantic_highlighting::encoder::{EncodedToken, TokenEncoder},
    lang::db::AnalysisDatabase,
};

#[derive(Default)]
pub(crate) struct SemanticTokensTraverser {
    encoder: TokenEncoder,
    /// A map from an offset in the file to semantic token kind.
    /// This map is used to override future tokens based on the context.
    /// For example: when we see the "fn" keyword, the name token is added
    /// to the map, so that instead of marking it as an identifier, we will mark it
    /// as a function name.
    offset_to_kind_lookahead: UnorderedHashMap<TextOffset, SemanticTokenKind>,
}
impl SemanticTokensTraverser {
    /// Gets all the SemanticTokens for the given node.
    /// Traverses the syntax tree and encodes the tokens based on their semantic kind.
    pub fn get_semantic_tokens<'db>(
        &mut self,
        db: &'db AnalysisDatabase,
        node: SyntaxNode<'db>,
    ) -> Vec<SemanticToken> {
        let green_node = node.green_node(db);
        match &green_node.details {
            GreenNodeDetails::Token(text) => {
                self.find_semantic_tokens_for_syntax_token(db, node, text, green_node.kind)
            }
            GreenNodeDetails::Node { .. } => {
                let mut semantic_tokens = vec![];
                let children = node.get_children(db);
                self.mark_future_tokens_for_node(db, node, green_node.kind);
                for child in children.iter() {
                    semantic_tokens.extend(self.get_semantic_tokens(db, *child));
                }
                semantic_tokens
            }
        }
    }

    /// Finds the corresponding semantic tokens for a syntax token.
    fn find_semantic_tokens_for_syntax_token<'db>(
        &mut self,
        db: &'db AnalysisDatabase,
        node: SyntaxNode<'db>,
        token_text: &&str,
        green_node_kind: SyntaxKind,
    ) -> Vec<SemanticToken> {
        if green_node_kind == SyntaxKind::TokenNewline {
            self.encoder.next_line();
            return vec![];
        }

        let width = token_text.len() as u32;
        let maybe_semantic_kind = self
            .offset_to_kind_lookahead
            .remove(&node.offset(db))
            .or_else(|| SemanticTokenKind::from_syntax_node(db, node));

        if let Some(semantic_kind) = maybe_semantic_kind {
            let text = node.text(db).expect("Node text should be available");

            // Case where a token spans multiple lines.
            if text.contains('\n') {
                self.get_tokens_from_multiline_syntax_node(semantic_kind, text)
            } else {
                vec![self.get_semantic_token(width, &semantic_kind)]
            }
        } else {
            self.encoder.skip(width);
            vec![]
        }
    }

    /// In case of a multiline token, we need to split it into multiple tokens,
    /// each representing a single line.
    fn get_tokens_from_multiline_syntax_node(
        &mut self,
        node_semantic_kind: SemanticTokenKind,
        node_text: &str,
    ) -> Vec<SemanticToken> {
        let mut tokens = vec![];
        // Split multiline token into multiple single line tokens.
        for line in node_text.split_inclusive('\n') {
            tokens.push(self.get_semantic_token(line.len() as u32, &node_semantic_kind));

            if line.ends_with('\n') {
                self.encoder.next_line();
            }
        }
        tokens
    }

    /// Marks future tokens for the given node based on its kind.
    /// This is used to ensure that the next tokens are correctly classified based on the context.
    fn mark_future_tokens_for_node<'db>(
        &mut self,
        db: &'db AnalysisDatabase,
        node: SyntaxNode<'db>,
        green_node_kind: SyntaxKind,
    ) {
        match green_node_kind {
            SyntaxKind::Param => {
                self.mark_future_token(
                    ast::Param::from_syntax_node(db, node).name(db).as_syntax_node().offset(db),
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
                ast::ItemStruct::from_syntax_node(db, node).name(db).as_syntax_node().offset(db),
                SemanticTokenKind::Struct,
            ),
            SyntaxKind::ItemEnum => self.mark_future_token(
                ast::ItemEnum::from_syntax_node(db, node).name(db).as_syntax_node().offset(db),
                SemanticTokenKind::Enum,
            ),
            _ => {}
        }
    }

    /// Retrieves the semantic token for the current node based on its width and assumed SemanticTokenKind.
    fn get_semantic_token(
        &mut self,
        width: u32,
        assumed_semantic_kind: &SemanticTokenKind,
    ) -> SemanticToken {
        let EncodedToken { delta_line, delta_start } = self.encoder.encode(width);

        SemanticToken {
            delta_line,
            delta_start,
            length: width,
            token_type: assumed_semantic_kind.as_u32(),
            token_modifiers_bitset: 0,
        }
    }

    fn mark_future_token(&mut self, offset: TextOffset, semantic_kind: SemanticTokenKind) {
        self.offset_to_kind_lookahead.insert(offset, semantic_kind);
    }
}
