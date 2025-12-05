use indoc::indoc;
use lsp_types::{ClientCapabilities, GotoCapability, TextDocumentClientCapabilities};
use serde_json::json;

use crate::support::sandbox;

fn caps(base: ClientCapabilities) -> ClientCapabilities {
    ClientCapabilities {
        text_document: base.text_document.or_else(Default::default).map(|it| {
            TextDocumentClientCapabilities {
                definition: Some(GotoCapability {
                    dynamic_registration: Some(false),
                    link_support: None,
                }),
                ..it
            }
        }),
        ..base
    }
}

#[test]
fn test_ignore_warnings_from_non_path_deps() {
    let cairo = indoc! {r#"
        use snforge_std::byte_array::byte_array_as_felt_array;

        fn func() {
            byte_array_as_felt_array(@"abc");
        }
    "#};

    let mut ls = sandbox! {
        files {
            "Scarb.toml" => indoc! (r#"
                [package]
                name = "a"
                version = "0.1.0"
                edition = "2025_12"

                [dev-dependencies]
                snforge_std = "0.37.0"  # This version contains lint errors.
            "#),
            "src/lib.cairo" => cairo,
        }
        client_capabilities = caps;
        workspace_configuration = json!({
            "cairo1": {
                "enableLinter": true,
            }
        });
    };

    // Test for lack of diagnostics in the entire project even though the dep contains lint errors.
    ls.open_and_wait_for_diagnostics_generation("src/lib.cairo")
        .into_iter()
        .for_each(|(url, diags)| assert!(diags.is_empty(), "{url} → {diags:#?}"));
}
