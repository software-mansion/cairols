use indoc::indoc;
use lsp_types::Diagnostic;
use serde::Serialize;
use serde_json::json;

use crate::support::diagnostics::get_related_diagnostic_code;
use crate::support::normalize::normalize_diagnostics;
use crate::support::sandbox;

#[derive(Serialize)]
pub struct DiagnosticAndRelatedInfo {
    related_code: String,
    diagnostic: Diagnostic,
}

#[derive(Serialize)]
pub struct DiagnosticsWithUrl {
    pub url: String,
    pub diagnostics: Vec<DiagnosticAndRelatedInfo>,
}

#[derive(Serialize)]
struct DiagnosticsReport {
    diagnostics: Vec<DiagnosticsWithUrl>,
}

#[test]
fn test_custom_macro() {
    let mut ls = sandbox! {
        files {
            "a/Scarb.toml" => indoc! {r#"
                [package]
                name = "a"
                version = "0.1.0"
                edition = "2024_07"
                [dependencies]
                macros = { path = "../macros" }
            "#},
            "a/src/lib.cairo" => indoc! {r#"
                #[decorate]
                fn decorated() {}
            "#},
            "macros/Cargo.toml" => indoc! {r#"
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

            "#},
            "macros/Scarb.toml" => indoc! {r#"
                [package]
                name = "macros"
                version = "0.1.0"

                [cairo-plugin]
            "#},
            "macros/src/lib.rs" => indoc! {r#"
                use cairo_lang_macro::{ProcMacroResult, TokenStream, attribute_macro};

                #[attribute_macro]
                pub fn decorate(_args: TokenStream, item: TokenStream) -> ProcMacroResult {
                    let new_res = format!("{} fn added_fun() {{ a = b; }}", item); // Syntax error
                    ProcMacroResult::new(TokenStream::new(new_res))
                }
            "#},
        }
        cwd = "a";
        workspace_configuration = json!({
            "cairo1": {
                "enableProcMacros": true,
                "traceMacroDiagnostics": true,
            }
        });
    };

    let newest_diagnostics = ls.open_and_wait_for_diagnostics_generation("a/src/lib.cairo");
    let diagnostics_with_url = normalize_diagnostics(&ls, newest_diagnostics.clone())
        .into_iter()
        .map(|(original_url, normalized_url, diagnostics)| DiagnosticsWithUrl {
            url: normalized_url,
            diagnostics: diagnostics
                .into_iter()
                .map(|diag| DiagnosticAndRelatedInfo {
                    related_code: get_related_diagnostic_code(&ls, &diag, &original_url),
                    diagnostic: diag,
                })
                .collect(),
        })
        .collect();

    insta::assert_json_snapshot!(DiagnosticsReport { diagnostics: diagnostics_with_url })
}
