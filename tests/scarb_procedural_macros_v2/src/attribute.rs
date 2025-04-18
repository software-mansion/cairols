use cairo_lang_macro::{
    Diagnostic, ProcMacroResult, TextSpan, Token, TokenStream, TokenTree, attribute_macro, quote,
};
use indoc::indoc;

#[attribute_macro]
pub fn simple_attribute_macro_v2(_args: TokenStream, _item: TokenStream) -> ProcMacroResult {
    let result = String::from("fn generated_function_v2() {}");
    let span = TextSpan::new(0, result.len() as u32);
    ProcMacroResult::new(TokenStream::new(vec![TokenTree::Ident(Token::new(result, span))]))
}

#[attribute_macro]
pub fn complex_attribute_macro_v2(_args: TokenStream, _item: TokenStream) -> ProcMacroResult {
    let result = String::from(indoc!(
        r#"
        #[simple_attribute_macro_v2]
        fn generated_function_with_other_attribute_v2() {}
        "#
    ));
    let span = TextSpan::new(0, result.len() as u32);

    ProcMacroResult::new(TokenStream::new(vec![TokenTree::Ident(Token::new(result, span))]))
}

#[attribute_macro]
pub fn improper_attribute_macro_v2(_args: TokenStream, item: TokenStream) -> ProcMacroResult {
    let result = format!("{} fn added_fun_v2() {{ a = b; }}", item); // Syntax error
    let span = TextSpan::new(0, result.len() as u32);

    ProcMacroResult::new(TokenStream::new(vec![TokenTree::Ident(Token::new(result, span))]))
}

#[attribute_macro]
pub fn error_attribute_macro_v2(_args: TokenStream, _item: TokenStream) -> ProcMacroResult {
    ProcMacroResult::new(TokenStream::empty())
        .with_diagnostics(Diagnostic::error("Error from procedural macro").into())
}

#[attribute_macro]
pub fn wrap_with_module(_args: TokenStream, item: TokenStream) -> ProcMacroResult {
    let ts = quote! {
        fn essa() {
            let ABC: felt252 = 123;
            #item
        }
    };
    ProcMacroResult::new(ts)
}
