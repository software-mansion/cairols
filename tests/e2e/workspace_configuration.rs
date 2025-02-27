use lsp_server::Message;
use lsp_types::lsp_request;
use lsp_types::request::Request as _;
use serde_json::json;

use crate::support::cairo_project_toml::CAIRO_PROJECT_TOML;
use crate::support::sandbox;
use crate::support::scarb::scarb_core_path;

/// The LS used to panic when some files in Salsa database were interned with a relative path.
/// The panic happened while trying to create a `file://` URL to affected file.
/// The easiest way to reproduce this was to pass a relative path as `cairo1.corelibPath` in
/// workspace configuration.
///
/// This test checks that:
/// 1. The workspace configuration flow between language server and language client is working as
///    expected.
/// 2. The LS does not panic when it receives a relative path to the core crate.
#[test]
fn relative_path_to_core() {
    let core_path = {
        let detected = scarb_core_path();
        let pwd = std::env::current_dir().unwrap();
        let path = pathdiff::diff_paths(detected, pwd).unwrap();
        assert!(path.is_relative());
        path.to_string_lossy().to_string()
    };

    let mut ls = sandbox! {
        files {
            "cairo_project.toml" => CAIRO_PROJECT_TOML,
            "src/lib.cairo" => r#"fn main() -> u8 { 42 }"#,
        }
        workspace_configuration = json!({
            "cairo1": {
                "corelibPath": core_path,
            }
        });
    };

    let diags = ls.open_and_wait_for_diagnostics("src/lib.cairo");
    assert!(diags.is_empty());

    assert_eq!(
        ls.trace()
            .iter()
            .filter(|msg| {
                let Message::Request(req) = msg else { return false };
                req.method == <lsp_request!("workspace/configuration")>::METHOD
            })
            .count(),
        1
    );
}
