use indoc::formatdoc;
use lsp_types::ClientCapabilities;
use serde_json::{Value, json};

use crate::macros::SCARB_TEST_MACROS_V2_PACKAGE;
use crate::support::cairo_project_toml::CAIRO_PROJECT_TOML_2024_07;
use crate::support::cursor::Cursors;
use crate::support::fixture::Fixture;
use crate::support::{MockClient, cursors, sandbox};

pub fn conduct_transformation<T: Transformer>(
    cairo_code_with_cursors: &str,
    with_macros: bool,
    initial_fixture: Fixture,
) -> String {
    let (cairo, cursors) = cursors(cairo_code_with_cursors);

    let mut fixture = initial_fixture;
    fixture.add_file(T::main_file(), cairo.clone());

    if with_macros {
        fixture.add_file_if_not_exists(
            "Scarb.toml",
            formatdoc!(
                r#"
                [package]
                name = "hello"
                version = "0.1.0"
                edition = "2024_07"

                [dependencies]
                cairols_test_macros_v2 = {{ path = "{}" }}
                starknet = "*"
            "#,
                SCARB_TEST_MACROS_V2_PACKAGE.display()
            ),
        );
    } else {
        fixture.add_file_if_not_exists("cairo_project.toml", CAIRO_PROJECT_TOML_2024_07);
    };

    let workspace_config = if with_macros {
        json!({
            "cairo1": {
                "enableProcMacros": true,
            }
        })
    } else {
        Value::Object(Default::default())
    };

    let mut ls = sandbox! {
        fixture = fixture;
        cwd = "./";
        client_capabilities = T::capabilities;
        workspace_configuration = workspace_config;
    };

    if with_macros {
        ls.open_all_and_wait_for_diagnostics_generation();
    } else {
        ls.open_all_cairo_files_and_wait_for_project_update();
    }

    T::transform(ls, cursors)
}

pub trait Transformer {
    fn capabilities(base: ClientCapabilities) -> ClientCapabilities;

    fn transform(ls: MockClient, cursors: Cursors) -> String;

    fn main_file() -> &'static str {
        "src/lib.cairo"
    }
}
