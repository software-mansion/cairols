use cairo_lang_macro::{
    Diagnostic, ProcMacroResult, TextSpan, Token, TokenStream, TokenTree, inline_macro,
};

#[inline_macro]
pub fn simple_inline_macro_v2(item: TokenStream) -> ProcMacroResult {
    let result = item.to_string();
    let span = TextSpan::new(0, result.len() as u32);
    ProcMacroResult::new(TokenStream::new(vec![TokenTree::Ident(Token::new(result, span))]))
}

#[inline_macro]
pub fn complex_inline_macro_v2(item: TokenStream) -> ProcMacroResult {
    let result =
        format!("simple_inline_macro_v2!({0}) + simple_inline_macro_v2!({0})", item.to_string());
    let span = TextSpan::new(0, result.len() as u32);
    ProcMacroResult::new(TokenStream::new(vec![TokenTree::Ident(Token::new(result, span))]))
}

#[inline_macro]
pub fn improper_inline_macro_v2(_item: TokenStream) -> ProcMacroResult {
    let result = String::from("unbound_identifier_v2");
    let span = TextSpan::new(0, result.len() as u32);
    ProcMacroResult::new(TokenStream::new(vec![TokenTree::Ident(Token::new(result, span))]))
}

#[inline_macro]
pub fn error_inline_macro_v2(_item: TokenStream) -> ProcMacroResult {
    ProcMacroResult::new(TokenStream::empty())
        .with_diagnostics(Diagnostic::error("Error from procedural macro").into())
}
