use indoc::{formatdoc, indoc};

use crate::support::fixture::{Fixture, fixture};

use super::{MacroTest, SCARB_TEST_MACROS_PACKAGE};

pub struct ProjectWithCustomMacros;
pub struct ProjectWithMultipleCrates;

impl MacroTest for ProjectWithCustomMacros {
    fn fixture() -> Fixture {
        fixture! {
            "test_package/Scarb.toml" => formatdoc!(
                r#"
                [package]
                name = "test_package"
                version = "0.1.0"
                edition = "2024_07"

                [dependencies]
                cairols_test_macros = {{ path = "{}" }}
                "#,
                SCARB_TEST_MACROS_PACKAGE.display()
            )
        }
    }
}

impl MacroTest for ProjectWithMultipleCrates {
    fn fixture() -> Fixture {
        fixture! {
            "workspace/Scarb.toml" => formatdoc!(r#"
                [workspace]
                members = ["package_a", "package_b"]

                [workspace.dependencies]
                package_a = {{ path = "./package_a" }}
                package_b = {{ path = "./package_b" }}
                another_test_macros = {{ path = "./another_test_macros" }}
                cairols_test_macros = {{ path = "{}" }}
            "#, SCARB_TEST_MACROS_PACKAGE.display()),

            "workspace/package_a/Scarb.toml" => indoc!(r#"
                [package]
                name = "package_a"
                version = "0.1.0"
                edition = "2024_07"

                [dependencies]
                package_b.workspace = true
                cairols_test_macros.workspace = true
            "#),

            "workspace/package_b/Scarb.toml" => indoc!(r#"
                [package]
                name = "package_b"
                version = "0.1.0"
                edition = "2024_07"

                [dependencies]
                another_test_macros.workspace = true
            "#),

            "workspace/another_test_macros/Cargo.toml" => indoc!(r#"
                [package]
                name = "another_test_macros"
                version = "1.0.0"
                edition = "2024"

                [lib]
                crate-type = ["rlib", "cdylib"]

                [dependencies]
                cairo-lang-macro = "0.1.1"
            "#),

            "workspace/another_test_macros/Scarb.toml" => indoc!(r#"
                [package]
                name = "another_test_macros"
                version = "1.0.0"
                edition = "2024_07"

                [cairo-plugin]
            "#),

            "workspace/another_test_macros/src/lib.rs" => indoc!(r##"
                use cairo_lang_macro::{ProcMacroResult, TokenStream, inline_macro};

                #[inline_macro]
                pub fn which_macro_package(item: TokenStream) -> ProcMacroResult {
                    let result = String::from("'another_test_macros'");
                    ProcMacroResult::new(TokenStream::new(result))
                }
            "##),
        }
    }
}
