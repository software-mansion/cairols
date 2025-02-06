use indoc::indoc;
use itertools::Itertools;
use lsp_types::Diagnostic;
use serde::Serialize;
use serde_json::json;

use crate::support::normalize::normalize;
use crate::support::sandbox;

#[derive(Serialize)]
struct DiagnosticsWithUrl {
    url: String,
    diagnostics: Vec<Diagnostic>,
}

#[derive(Serialize)]
struct DiagnosticsReport {
    diagnostics: String,
}

#[test]
fn test_custom_macro() {
    let mut ls = sandbox! {
        files {
            "a/Scarb.toml" => indoc! (r#"
                [package]
                name = "a"
                version = "0.1.0"
                edition = "2024_07"
                [dependencies]
                macros = { path = "../macros" }
            "#),
            "a/src/lib.cairo" => indoc!(r#"
                #[decorate]
                fn decorated() {}
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

            "#),
            "macros/Scarb.toml" => indoc!(r#"
                [package]
                name = "macros"
                version = "0.1.0"

                [cairo-plugin]
            "#),
            "macros/src/lib.rs" => indoc!(r#"
                use cairo_lang_macro::{ProcMacroResult, TokenStream, attribute_macro};

                #[attribute_macro]
                pub fn decorate(_args: TokenStream, item: TokenStream) -> ProcMacroResult {
                    let new_res = format!("{} fn added_fun() {{ a = b; }}", item); // Syntax error
                    ProcMacroResult::new(TokenStream::new(new_res))
                }
            "#),
        }
        cwd = "/a";
        workspace_configuration = json!({
            "cairo1": {
                "enableProcMacros": true,
            }
        });
    };

    let newest_diagnostics = ls.open_and_wait_for_diagnostics_generation("a/src/lib.cairo");
    let sorted_diagnostics: Vec<DiagnosticsWithUrl> = newest_diagnostics
        .into_iter()
        .filter(|(url, _)| !url.path().contains("core/src"))
        .sorted_by(|(url_a, _), (url_b, _)| url_a.path().cmp(url_b.path()))
        .map(|(url, diagnostics)| DiagnosticsWithUrl { url: url.to_string(), diagnostics })
        .collect();

    let serialized_diags = serde_json::to_string_pretty(&sorted_diagnostics).unwrap();
    let normalized_diags = normalize(&ls, serialized_diags);

    insta::assert_toml_snapshot!(DiagnosticsReport { diagnostics: normalized_diags })
}
