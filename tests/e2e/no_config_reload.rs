use indoc::indoc;
use lsp_types::{ClientCapabilities, WorkspaceClientCapabilities};

use crate::support::sandbox;

fn caps(base: ClientCapabilities) -> ClientCapabilities {
    ClientCapabilities {
        workspace: base
            .workspace
            .or_else(Default::default)
            .map(|it| WorkspaceClientCapabilities { configuration: Some(false), ..it }),
        ..base
    }
}

#[test]
// Proc macros are on by default on the server side.
fn default_config_is_applied() {
    let mut ls = sandbox! {
        files {
            "Scarb.toml" => indoc! (r#"
                [package]
                name = "a"
                version = "0.1.0"
                edition = "2024_07"

                [dependencies]
                snforge_std = "0.38.0"
            "#),
            "src/lib.cairo" => indoc! {r#"
                #[test]
                fn test() {
                    assert(1 == 1, '');
                }
            "#
            },
        }
        cwd = "./";
        client_capabilities = caps;
    };

    ls.open_and_wait_for_diagnostics_generation("src/lib.cairo")
        .into_iter()
        .for_each(|(url, diags)| assert!(diags.is_empty(), "{url} → {diags:#?}"));
}
