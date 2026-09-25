use std::path::PathBuf;

use scarb_proc_macro_host::ExpansionQuery;
use scarb_proc_macro_server_types::methods::defined_macros::DebugInfo;
use scarb_proc_macro_server_types::scope::{CompilationUnitComponent, Workspace};

use super::*;

fn macro_with_hash(name: &str, cairo_name: &str, hash: u64) -> MacroWithHash {
    MacroWithHash { name: name.to_string(), cairo_name: cairo_name.to_string(), hash }
}

fn backend() -> PmsBackend {
    let scope = ProcMacroScope {
        workspace: Workspace { manifest_path: PathBuf::from("/tmp/Scarb.toml") },
        component: CompilationUnitComponent {
            name: "test_package".to_string(),
            discriminator: None,
        },
    };
    PmsBackend::new(
        scope,
        CompilationUnitComponentMacros {
            component: CompilationUnitComponent {
                name: "test_package".to_string(),
                discriminator: None,
            },
            attributes: vec![macro_with_hash("some_attr", "some_attr", 1)],
            inline_macros: vec![macro_with_hash("some_inline", "some_inline", 2)],
            derives: vec![macro_with_hash("some_derive", "SomeDerive", 3)],
            executables: vec!["some_executable".to_string()],
            debug_info: DebugInfo { source_packages: vec!["some_package".to_string()] },
        },
    )
}

#[test]
fn derives_are_declared_under_the_cairo_name_reported_by_the_server() {
    // Cairo code writes derives as `#[derive(SomeDerive)]`. The server decides that name, so
    // the language server never has to repeat Scarb's casing rules.
    assert_eq!(backend().declared_derives(), vec!["SomeDerive".to_string()]);
}

#[test]
fn attributes_and_executables_are_both_declared_as_attributes() {
    // The compiler has to know about executable attributes and the full path marker so it
    // does not report them as unknown, even though they are never expanded.
    let backend = backend();
    assert_eq!(
        backend.declared_attributes(),
        vec![
            "some_attr".to_string(),
            "some_executable".to_string(),
            scarb_proc_macro_host::FULL_PATH_MARKER_KEY.to_string(),
        ]
    );
    assert_eq!(backend.executable_attributes(), vec!["some_executable".to_string()]);
}

#[test]
fn inline_macros_are_listed_for_plugin_registration() {
    let backend = backend();
    let inline = backend.inline_macros();
    assert_eq!(inline.len(), 1);
    assert_eq!(inline[0].expansion.cairo_name.as_str(), "some_inline");
    assert_eq!(inline[0].fingerprint, 2);
}

#[test]
fn expansions_are_found_by_the_name_used_in_cairo_code() {
    let backend = backend();

    let found = backend
        .find_expansion(&ExpansionQuery::with_cairo_name("SomeDerive", ExpansionKind::Derive))
        .expect("derive should be found by its Cairo name");
    // The request sent to the server uses the expansion name, not the Cairo one.
    assert_eq!(found.expansion.expansion_name.as_str(), "some_derive");
    assert_eq!(found.fingerprint, 3);

    assert!(
        backend
            .find_expansion(&ExpansionQuery::with_cairo_name("some_derive", ExpansionKind::Derive))
            .is_none(),
        "the snake case name is not what Cairo code writes"
    );
}

#[test]
fn expansions_of_a_different_kind_do_not_match() {
    let backend = backend();
    assert!(
        backend
            .find_expansion(&ExpansionQuery::with_cairo_name("some_attr", ExpansionKind::Inline))
            .is_none()
    );
}

#[test]
fn source_packages_are_reported_for_introspection() {
    assert_eq!(backend().source_packages(), ["some_package".to_string()]);
}
