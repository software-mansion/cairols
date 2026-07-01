use indoc::{formatdoc, indoc};
use serde_json::json;

use super::{MacroTest, SCARB_TEST_MACROS_PACKAGE, SCARB_TEST_MACROS_V2_PACKAGE};
use crate::support::fixture::{Fixture, fixture};

pub struct ProjectWithCustomMacros;
pub struct ProjectWithMultipleCrates;
pub struct ProjectWithSnforgeUnitTest;
pub struct ProjectWithSnforgeIntegrationTest;
pub struct ProjectWithCairoProjectToml;
pub struct ProjectWithCustomMacrosV2;
pub struct ProjectWithCustomMacrosV1AndV2;
pub struct ProjectWithUserDefinedInlineMacros;

impl MacroTest for ProjectWithCustomMacros {
    fn fixture() -> Fixture {
        fixture! {
            "test_package/Scarb.toml" => formatdoc!(
                r#"
                [package]
                name = "test_package"
                version = "0.1.0"
                edition = "2025_12"

                [dependencies]
                cairols_test_macros = {{ path = "{}" }}
                "#,
                SCARB_TEST_MACROS_PACKAGE.display()
            )
        }
    }
}

impl MacroTest for ProjectWithCustomMacrosV2 {
    fn fixture() -> Fixture {
        fixture! {
            "test_package/Scarb.toml" => formatdoc!(
                r#"
                [package]
                name = "test_package"
                version = "0.1.0"
                edition = "2025_12"

                [dependencies]
                cairols_test_macros_v2 = {{ path = "{}" }}
                "#,
                SCARB_TEST_MACROS_V2_PACKAGE.display()
            )
        }
    }
}

impl MacroTest for ProjectWithCustomMacrosV1AndV2 {
    fn fixture() -> Fixture {
        fixture! {
            "test_package/Scarb.toml" => formatdoc!(
                r#"
                [package]
                name = "test_package"
                version = "0.1.0"
                edition = "2025_12"

                [dependencies]
                cairols_test_macros = {{ path = "{}" }}
                cairols_test_macros_v2 = {{ path = "{}" }}
                "#,
                SCARB_TEST_MACROS_PACKAGE.display(),
                SCARB_TEST_MACROS_V2_PACKAGE.display()
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
                edition = "2025_12"

                [dependencies]
                package_b.workspace = true
                cairols_test_macros.workspace = true
            "#),

            "workspace/package_b/Scarb.toml" => indoc!(r#"
                [package]
                name = "package_b"
                version = "0.1.0"
                edition = "2025_12"

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
                edition = "2025_12"

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

impl MacroTest for ProjectWithSnforgeUnitTest {
    fn fixture() -> Fixture {
        fixture! {
            "test_package/Scarb.toml" => indoc!(r#"
                [package]
                name = "test_package"
                version = "0.1.0"
                edition = "2025_12"

                [tool.scarb]
                allow-prebuilt-plugins = ["snforge_scarb_plugin"]

                [dev-dependencies]
                assert_macros = "2.10.0"
                snforge_std = "0.60.0"
                snforge_scarb_plugin = "0.60.0"
            "#),
        }
    }
}

impl MacroTest for ProjectWithSnforgeIntegrationTest {
    fn fixture() -> Fixture {
        fixture! {
            "test_package/src/lib.cairo" => "",

            "test_package/Scarb.toml" => indoc!(r#"
                [package]
                name = "test_package"
                version = "0.1.0"
                edition = "2025_12"

                [dev-dependencies]
                snforge_std = "0.60.0"
                snforge_scarb_plugin = "0.60.0"

                [tool.scarb]
                allow-prebuilt-plugins = ["snforge_scarb_plugin"]

                [[tool.snforge.fork]]
                name = "SEPOLIA_LATEST"
                url = "https://starknet-sepolia.public.blastapi.io/rpc/v0_7"
                block_id.tag = "latest"
            "#),
        }
    }
}

impl MacroTest for ProjectWithCairoProjectToml {
    fn fixture() -> Fixture {
        fixture! {
            "test_package/cairo_project.toml" => indoc!(r#"
                [crate_roots]
                test_package = "src"

                [config.global]
                edition = "2025_12"
            "#)
        }
    }

    fn workspace_configuration() -> serde_json::Value {
        json!({
            // MockClient::open_and_wait_for_diagnostics_generation timeouts when used with
            // cairo_project.toml with proc macros enabled.
            "enableProcMacros": false,
            "traceMacroDiagnostics": false,
        })
    }
}

impl MacroTest for ProjectWithUserDefinedInlineMacros {
    fn fixture() -> Fixture {
        fixture! {
            "test_package/src/lib.cairo" => indoc!(r#"
                mod a;

                macro add_one {
                    ($x:expr) => {
                        $x + 1
                    };
                }

                macro add_many {
                    ($x:expr, $y:expr) => {
                        $x + $y
                    };
                    ($x:expr, $y:expr, $z:expr) => {
                        $x + $y + $z
                    };
                }

                macro build_array {
                    ($($x:expr), *) => {
                        {
                            let mut result = $defsite::ArrayTrait::new();
                            $(result.append($x);)*
                            result
                        }
                    };
                }

                macro declare_two {
                    ($x:expr) => {
                        let first = $x;
                        let _second = first + 1;
                    };
                }

                macro append_twice {
                    ($arr:ident, $value:expr) => {
                        $arr.append($value);
                        $arr.append($value + 1);
                    };
                }
            "#),

            "test_package/Scarb.toml" => indoc!(r#"
                [package]
                name = "test_package"
                version = "0.1.0"
                edition = "2025_12"
                experimental-features = ["user_defined_inline_macros"]
            "#),
        }
    }
}
