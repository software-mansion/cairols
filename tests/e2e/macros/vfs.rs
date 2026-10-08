use cairo_language_server::lsp::ext::{ProvideVirtualFile, ProvideVirtualFileRequest};
use indoc::indoc;

use crate::macros::MacroTest;
use crate::macros::fixtures::ProjectWithCustomMacrosV2;
use crate::support::sandbox;

/// Diagnostics are computed on a disposable database, while `vfs/provide` is answered from the
/// main one. A `vfs://` URL taken from a diagnostic must therefore identify the generated file.
#[test]
fn generated_file_url_from_diagnostics_opens_in_main_database() {
    // `ImproperDeriveMacroV2` emits `fn generated_function_v2` with a syntax error in its body.
    // With `traceMacroDiagnostics` enabled (see `MacroTest::workspace_configuration`), the error
    // reported on the derive carries a "Diagnostic mapped from here" link into the generated file.
    let mut ls = sandbox! {
        fixture = ProjectWithCustomMacrosV2::fixture();
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                #[derive(ImproperDeriveMacroV2)]
                struct Test { a: felt252 }
            "#)
        }
        cwd = "test_package";
        workspace_configuration = ProjectWithCustomMacrosV2::workspace_configuration();
    };
    let diagnostics = ls.open_and_wait_for_diagnostics_generation("test_package/src/lib.cairo");

    // Follow the link into the generated file the way the editor does when the user clicks it.
    let vfs_url = diagnostics
        .values()
        .flatten()
        .flat_map(|diagnostic| diagnostic.related_information.iter().flatten())
        .map(|related| related.location.uri.clone())
        .find(|uri| uri.scheme() == "vfs")
        .expect("diagnostic should link into the generated file");

    let content = ls
        .send_request::<ProvideVirtualFile>(ProvideVirtualFileRequest { uri: vfs_url.clone() })
        .content;

    assert!(
        content.as_deref().is_some_and(|content| content.contains("fn generated_function_v2")),
        "{vfs_url} should open the generated file, got {content:?}"
    );
}
