use cairo_lang_macro::{
    AllocationContext, TextSpan, Token, TokenStream, TokenStreamMetadata, TokenTree,
};
use cairo_lang_syntax::node::{SyntaxNode, db::SyntaxGroup};

/// Helps creating TokenStream based on multiple SyntaxNodes,
/// which aren't descendants or ascendants of each other inside the SyntaxTree.
pub struct TokenStreamBuilder<'a> {
    db: &'a dyn SyntaxGroup,
    nodes: Vec<SyntaxNode>,
    metadata: Option<TokenStreamMetadata>,
}

impl<'a> TokenStreamBuilder<'a> {
    pub fn new(db: &'a dyn SyntaxGroup) -> Self {
        Self { db, nodes: Vec::default(), metadata: None }
    }

    pub fn add_node(&mut self, node: SyntaxNode) {
        self.nodes.push(node);
    }

    pub fn build(&self, ctx: &AllocationContext) -> TokenStream {
        let result: Vec<TokenTree> = self
            .nodes
            .iter()
            .flat_map(|node| {
                let leaves = node.tokens(self.db);
                leaves.map(|node| TokenTree::Ident(self.token_from_syntax_node(node.clone(), ctx)))
            })
            .collect();

        match self.metadata.as_ref() {
            Some(metadata) => TokenStream::new(result).with_metadata(metadata.clone()),
            None => TokenStream::new(result),
        }
    }

    pub fn token_from_syntax_node(&self, node: SyntaxNode, ctx: &AllocationContext) -> Token {
        let span = node.span(self.db);
        let text = node.get_text(self.db);
        // We skip the whitespace prefix, so that diagnostics start where the actual token contents is.
        let start = span.start.as_u32() + whitespace_prefix_len(&text);
        // Then we also skip the whitespace suffix, for the same reason.
        let end = span.end.as_u32() - whitespace_suffix_len(&text);
        // This handles the case of a whitespace only string.
        let end = if end < start { start } else { end };
        let span = TextSpan { start, end };
        Token::new_in(text, span, ctx)
    }
}

fn whitespace_prefix_len(s: &str) -> u32 {
    s.chars().take_while(|c| c.is_whitespace()).count() as u32
}

fn whitespace_suffix_len(s: &str) -> u32 {
    s.chars().rev().take_while(|c| c.is_whitespace()).count() as u32
}
