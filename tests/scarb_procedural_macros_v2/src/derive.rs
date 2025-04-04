use cairo_lang_macro::{
    Diagnostic, ProcMacroResult, TextSpan, Token, TokenStream, TokenTree, derive_macro,
};
use indoc::indoc;

#[derive_macro]
pub fn simple_derive_macro_v2(_item: TokenStream) -> ProcMacroResult {
    let result = indoc!(
        r#"
        trait MyTrait<T> {
            fn foo(t: T);
        }

        impl MyTraitImpl of MyTrait<felt252> {
            fn foo(t: felt252) {}
        }
        "#
    );
    let span = TextSpan::new(0, result.len() as u32);

    ProcMacroResult::new(TokenStream::new(vec![TokenTree::Ident(Token::new(result, span))]))
}

#[derive_macro]
pub fn complex_derive_macro_v2(_item: TokenStream) -> ProcMacroResult {
    let result = indoc!(
        r#"
        #[simple_attribute_macro_v2]
        fn generated_function_v2() {}

        trait MyTraitV2<T> {
            fn bar(t: T);
        }

        impl MyTraitImpl of MyTraitV2<felt252> {
            fn bar(t: felt252) {}
        }
        "#
    );
    let span = TextSpan::new(0, result.len() as u32);

    ProcMacroResult::new(TokenStream::new(vec![TokenTree::Ident(Token::new(result, span))]))
}

#[derive_macro]
pub fn improper_derive_macro_v2(_item: TokenStream) -> ProcMacroResult {
    let result = indoc!(
        r#"
        fn generated_function_v2() {
            some <*> haskell <$> syntax
        }
        "#
    );
    let span = TextSpan::new(0, result.len() as u32);

    ProcMacroResult::new(TokenStream::new(vec![TokenTree::Ident(Token::new(result, span))]))
}

#[derive_macro]
pub fn error_derive_macro_v2(_item: TokenStream) -> ProcMacroResult {
    ProcMacroResult::new(TokenStream::empty())
        .with_diagnostics(Diagnostic::error("Error from procedural macro").into())
}
