use indoc::indoc;

use crate::support::fixture::{Fixture, fixture};

use super::MacroTestFixture;

pub struct SimpleProject;

impl MacroTestFixture for SimpleProject {
    const TEST_PACKAGE: &str = "test_package";

    fn fixture() -> Fixture {
        fixture! {
            "test_package/Scarb.toml" => indoc!(r#"
                [package]
                name = "test_package"
                version = "0.1.0"
                edition = "2024_07"
                [dependencies]
                macros = { path = "../macros" }
            "#),

            "macros/Cargo.toml" => indoc!(r#"
                [package]
                name = "some_macro"
                version = "0.1.0"
                edition = "2021"
                publish = false

                [lib]
                crate-type = ["rlib", "cdylib"]

                [dependencies]
                cairo-lang-macro = "0.1.1"
                cairo-lang-parser = "2.7.0"
                indoc = "2"
            "#),

            "macros/Scarb.toml" => indoc!(r#"
                [package]
                name = "macros"
                version = "0.1.0"

                [cairo-plugin]
            "#),

            "macros/src/lib.rs" => indoc!(r##"
                use indoc::indoc;
                use cairo_lang_macro::{
                    Diagnostic,
                    ProcMacroResult,
                    TokenStream,
                    attribute_macro,
                    inline_macro,
                    derive_macro
                };

                #[attribute_macro]
                pub fn simple_attribute_macro(_args: TokenStream, item: TokenStream) -> ProcMacroResult {
                    let result = String::from("fn generated_function() {}");
                    ProcMacroResult::new(TokenStream::new(result))
                }

                #[attribute_macro]
                pub fn complex_attribute_macro(_args: TokenStream, item: TokenStream) -> ProcMacroResult {
                    let result = String::from(indoc!(r#"
                        #[simple_attribute_macro]
                        fn generated_function_with_other_attribute() {}
                    "#));

                    ProcMacroResult::new(TokenStream::new(result))
                }

                #[attribute_macro]
                pub fn improper_attribute_macro(_args: TokenStream, item: TokenStream) -> ProcMacroResult {
                    let result = format!("{} fn added_fun() {{ a = b; }}", item); // Syntax error
                    ProcMacroResult::new(TokenStream::new(result))
                }

                #[attribute_macro]
                pub fn error_attribute_macro(_args: TokenStream, item: TokenStream) -> ProcMacroResult {
                    ProcMacroResult::new(TokenStream::empty())
                        .with_diagnostics(Diagnostic::error("Error from procedural macro").into())
                }

                #[inline_macro]
                pub fn simple_inline_macro(item: TokenStream) -> ProcMacroResult {
                    ProcMacroResult::new(TokenStream::new(item.to_string().len().to_string()))
                }

                #[inline_macro]
                pub fn complex_inline_macro(item: TokenStream) -> ProcMacroResult {
                    let result = format!("simple_inline_macro!({0}) + simple_inline_macro!({0})", item.to_string());
                    ProcMacroResult::new(TokenStream::new(result))
                }

                #[inline_macro]
                pub fn improper_inline_macro(item: TokenStream) -> ProcMacroResult {
                    ProcMacroResult::new(TokenStream::new(String::from("unbound_identifier")))
                }

                #[inline_macro]
                pub fn error_inline_macro(item: TokenStream) -> ProcMacroResult {
                    ProcMacroResult::new(TokenStream::empty())
                        .with_diagnostics(Diagnostic::error("Error from procedural macro").into())
                }

                #[derive_macro]
                pub fn simple_derive_macro(item: TokenStream) -> ProcMacroResult {
                    let result = indoc!(r#"
                        trait MyTrait<T> {
                            fn foo(t: T);
                        }

                        impl MyTraitImpl of MyTrait<felt252> {
                            fn foo(t: felt252) {}
                        }
                    "#);

                    ProcMacroResult::new(TokenStream::new(result.to_string()))
                }

                #[derive_macro]
                pub fn complex_derive_macro(item: TokenStream) -> ProcMacroResult {
                    let result = indoc!(r#"
                        #[simple_attribute_macro]
                        fn generated_function() {}

                        trait MyTrait<T> {
                            fn foo(t: T);
                        }

                        impl MyTraitImpl of MyTrait<felt252> {
                            fn foo(t: felt252) {}
                        }
                    "#);

                    ProcMacroResult::new(TokenStream::new(result.to_string()))
                }

                #[derive_macro]
                pub fn improper_derive_macro(item: TokenStream) -> ProcMacroResult {
                    let result = indoc!(r#"
                        fn generated_function() {
                            some <*> haskell <$> syntax
                        }
                    "#);

                    ProcMacroResult::new(TokenStream::new(result.to_string()))
                }

                #[derive_macro]
                pub fn error_derive_macro(item: TokenStream) -> ProcMacroResult {
                    ProcMacroResult::new(TokenStream::empty())
                        .with_diagnostics(Diagnostic::error("Error from procedural macro").into())
                }
            "##)
        }
    }
}
