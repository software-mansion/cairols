use std::fmt::Display;

use cairo_lang_filesystem::ids::{CodeMapping, CodeOrigin};
use cairo_lang_filesystem::span::{
    TextOffset as CairoTextOffset, TextSpan as CairoTextSpan, TextWidth,
};
use cairo_lang_macro::{
    AllocationContext, TextSpan, Token, TokenStream, TokenStreamMetadata, TokenTree,
};
use cairo_lang_macro::{Diagnostic, TextOffset};
use cairo_lang_syntax::node::{SyntaxNode, TypedSyntaxNode, db::SyntaxGroup};

use crate::lang::db::SyntaxNodeExt;
use crate::lang::proc_macros::plugins::scarb::conversion::SpanSource;
use crate::lang::proc_macros::plugins::scarb::regular::AttrExpansionFound;

/// Helps creating TokenStream based on multiple SyntaxNodes,
/// which aren't descendants or ascendants of each other inside the SyntaxTree.
pub struct TokenStreamBuilder<'db> {
    db: &'db dyn SyntaxGroup,
    nodes: Vec<SyntaxNode<'db>>,
    metadata: Option<TokenStreamMetadata>,
}

impl<'db> TokenStreamBuilder<'db> {
    pub fn new(db: &'db dyn SyntaxGroup) -> Self {
        Self { db, nodes: Vec::default(), metadata: None }
    }

    pub fn add_node(&mut self, node: SyntaxNode<'db>) {
        self.nodes.push(node);
    }

    pub fn with_metadata(&mut self, metadata: TokenStreamMetadata) {
        self.metadata = Some(metadata);
    }

    pub fn build(&self, ctx: &AllocationContext) -> TokenStream {
        let mut result: Vec<TokenTree> = Default::default();

        for node in &self.nodes {
            node.for_each_terminal(self.db, |terminal| {
                self.token_from_syntax_node(*terminal, ctx, &mut result)
            });
        }

        match self.metadata.as_ref() {
            Some(metadata) => TokenStream::new(result).with_metadata(metadata.clone()),
            None => TokenStream::new(result),
        }
    }

    pub fn token_from_syntax_node(
        &self,
        node: SyntaxNode,
        ctx: &AllocationContext,
        result: &mut Vec<TokenTree>,
    ) {
        let span_without_trivia = node.span_without_trivia(self.db);
        let span_with_trivia = node.span(self.db);
        let text = node.get_text(self.db);
        let prefix_len = span_without_trivia.start - span_with_trivia.start;
        let (prefix, rest) = text.split_at(prefix_len.as_u32() as usize);
        if prefix_len > TextWidth::ZERO {
            result.push(TokenTree::Ident(Token::new_in(
                prefix,
                TextSpan {
                    start: span_with_trivia.start.as_u32(),
                    end: span_without_trivia.start.as_u32(),
                },
                ctx,
            )))
        }
        let suffix_len = span_with_trivia.end - span_without_trivia.end;
        let (content, suffix) = rest.split_at(rest.len() - suffix_len.as_u32() as usize);
        if !content.is_empty() {
            result.push(TokenTree::Ident(Token::new_in(
                content,
                TextSpan {
                    start: span_without_trivia.start.as_u32(),
                    end: span_without_trivia.end.as_u32(),
                },
                ctx,
            )));
        }
        if suffix_len > TextWidth::ZERO {
            result.push(TokenTree::Ident(Token::new_in(
                suffix,
                TextSpan {
                    start: span_without_trivia.end.as_u32(),
                    end: span_with_trivia.end.as_u32(),
                },
                ctx,
            )));
        }
    }
}

impl<'db> Extend<SyntaxNode<'db>> for TokenStreamBuilder<'db> {
    fn extend<T: IntoIterator<Item = SyntaxNode<'db>>>(&mut self, iter: T) {
        for node in iter {
            self.add_node(node);
        }
    }
}

/// [`TokenStream`] with token spans adapted for expansion input.
#[derive(Clone, Debug)]
pub struct AdaptedTokenStream(TokenStream);

impl AdaptedTokenStream {
    pub fn with_metadata(self, metadata: TokenStreamMetadata) -> Self {
        Self(self.0.with_metadata(metadata))
    }
}

impl From<AdaptedTokenStream> for TokenStream {
    fn from(value: AdaptedTokenStream) -> Self {
        value.0
    }
}

impl Display for AdaptedTokenStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct AdaptedCodeMapping(CodeMapping);

impl From<AdaptedCodeMapping> for CodeMapping {
    fn from(value: AdaptedCodeMapping) -> Self {
        value.0
    }
}

pub struct AdaptedDiagnostic(Diagnostic);

impl From<AdaptedDiagnostic> for Diagnostic {
    fn from(value: AdaptedDiagnostic) -> Self {
        value.0
    }
}

#[derive(Debug)]
pub struct AdaptedTextSpan(TextSpan);

impl From<AdaptedTextSpan> for TextSpan {
    fn from(value: AdaptedTextSpan) -> Self {
        value.0
    }
}

/// This struct represents the location of the attribute expansion call site (expandable attribute).
///
/// It contains both the original location of the attribute in the source code file and the adapted
/// location, i.e. as if the attribute was the first attribute in the attributes list of that token
/// stream.
pub struct ExpandableAttrLocation {
    span_with_trivia: TextSpan,
    span_without_trivia: TextSpan,
    item_start_offset: TextOffset,
}

impl ExpandableAttrLocation {
    pub fn new<'db, T: TypedSyntaxNode<'db>>(
        node: &T,
        item_start_offset: CairoTextOffset,
        db: &'db dyn SyntaxGroup,
    ) -> Self {
        let span_without_trivia = node.text_span(db);
        let span_with_trivia = node.as_syntax_node().span(db);
        Self {
            span_with_trivia: TextSpan {
                start: span_with_trivia.start.as_u32(),
                end: span_with_trivia.end.as_u32(),
            },
            span_without_trivia,
            item_start_offset: item_start_offset.as_u32(),
        }
    }

    fn start_offset_with_trivia(&self) -> TextOffset {
        self.span_with_trivia.start
    }

    fn end_offset_with_trivia(&self) -> TextOffset {
        self.span_with_trivia.end
    }

    fn width_with_trivia(&self) -> u32 {
        self.span_with_trivia.end - self.span_with_trivia.start
    }

    fn width_without_trivia(&self) -> u32 {
        self.span_without_trivia.end - self.span_without_trivia.start
    }

    pub fn adapted_call_site(&self) -> AdaptedTextSpan {
        let start =
            self.item_start_offset + self.span_without_trivia.start - self.span_with_trivia.start;
        AdaptedTextSpan(TextSpan { start, end: start + self.width_without_trivia() })
    }

    /// Move spans in the `TokenStream` for macro expansion input.
    pub fn adapt_token_stream(&self, token_stream: TokenStream) -> AdaptedTokenStream {
        let attr_start = self.start_offset_with_trivia();
        let attr_end = self.end_offset_with_trivia();
        let attr_width = self.width_with_trivia();
        let token_stream = TokenStream::new(
            token_stream
                .into_iter()
                .map(|tree| match tree {
                    TokenTree::Ident(mut token) => {
                        if token.span.start < attr_start {
                            token.span.start += attr_width;
                            token.span.end += attr_width;
                        } else if token.span.end < attr_end {
                            token.span.start -= attr_start - self.item_start_offset;
                            token.span.end -= attr_start - self.item_start_offset;
                        }
                        TokenTree::Ident(token)
                    }
                })
                .collect(),
        );
        AdaptedTokenStream(token_stream)
    }

    /// Move code mappings to account for the removed expandable attribute for the expansion output.
    pub fn adapt_code_mappings(&self, code_mappings: Vec<CodeMapping>) -> Vec<AdaptedCodeMapping> {
        let attr_start = self.start_offset_with_trivia();
        let attr_width = self.width_with_trivia();
        let attr_end = self.end_offset_with_trivia();
        code_mappings
            .into_iter()
            .map(|code_mapping| {
                let origin = match code_mapping.origin {
                    CodeOrigin::Span(span) => {
                        let span = if span.start.as_u32() < self.item_start_offset + attr_width {
                            CairoTextSpan {
                                start: span.start.add_width(TextWidth::new_for_testing(
                                    attr_start - self.item_start_offset,
                                )),
                                end: span.end.add_width(TextWidth::new_for_testing(
                                    attr_start - self.item_start_offset,
                                )),
                            }
                        } else if span.start.as_u32() < attr_end {
                            CairoTextSpan {
                                start: span.start.sub_width(TextWidth::new_for_testing(attr_width)),
                                end: span.end.sub_width(TextWidth::new_for_testing(attr_width)),
                            }
                        } else {
                            span
                        };
                        CodeOrigin::Span(span)
                    }
                    origin => origin,
                };
                CodeMapping { span: code_mapping.span, origin }
            })
            .map(AdaptedCodeMapping)
            .collect()
    }

    /// Move spans in diagnostics to account for the removed expandable attribute for the expansion output.
    pub fn adapt_diagnostics(&self, diagnostics: Vec<Diagnostic>) -> Vec<AdaptedDiagnostic> {
        let attr_start = self.start_offset_with_trivia();
        let attr_end = self.end_offset_with_trivia();
        let attr_width = self.width_with_trivia();
        diagnostics
            .into_iter()
            .map(|diagnostic| {
                if let Some(mut span) = diagnostic.span() {
                    if span.start < self.item_start_offset + attr_width {
                        span.start += attr_start - self.item_start_offset;
                        span.end += attr_start - self.item_start_offset;
                    } else if span.start < attr_end {
                        span.start -= attr_width;
                        span.end -= attr_width;
                    }
                    Diagnostic::spanned(span, diagnostic.severity(), diagnostic.message())
                } else {
                    diagnostic
                }
            })
            .map(AdaptedDiagnostic)
            .collect()
    }
}

impl<'db> AttrExpansionFound<'db> {
    /// Move spans in the `TokenStream` for macro expansion input.
    pub fn adapt_token_stream(&self, token_stream: TokenStream) -> AdaptedTokenStream {
        match self {
            AttrExpansionFound::Some(args) | AttrExpansionFound::Last(args) => {
                args.attribute_location.adapt_token_stream(token_stream)
            }
            AttrExpansionFound::None => AdaptedTokenStream(token_stream),
        }
    }
}
