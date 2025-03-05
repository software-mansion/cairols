use cairo_lang_macro::{Diagnostic, ProcMacroResult, TokenStream, derive_macro};
use indoc::indoc;

#[derive_macro]
pub fn simple_derive_macro(_item: TokenStream) -> ProcMacroResult {
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

    ProcMacroResult::new(TokenStream::new(result.to_string()))
}

#[derive_macro]
pub fn complex_derive_macro(_item: TokenStream) -> ProcMacroResult {
    let result = indoc!(
        r#"
        #[simple_attribute_macro]
        fn generated_function() {}

        trait MyTrait<T> {
            fn foo(t: T);
        }

        impl MyTraitImpl of MyTrait<felt252> {
            fn foo(t: felt252) {}
        }
        "#
    );

    ProcMacroResult::new(TokenStream::new(result.to_string()))
}

#[derive_macro]
pub fn improper_derive_macro(_item: TokenStream) -> ProcMacroResult {
    let result = indoc!(
        r#"
        fn generated_function() {
            some <*> haskell <$> syntax
        }
        "#
    );

    ProcMacroResult::new(TokenStream::new(result.to_string()))
}

#[derive_macro]
pub fn error_derive_macro(_item: TokenStream) -> ProcMacroResult {
    ProcMacroResult::new(TokenStream::empty())
        .with_diagnostics(Diagnostic::error("Error from procedural macro").into())
}
