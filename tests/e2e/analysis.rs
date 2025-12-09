use cairo_language_server::lsp;
use indoc::indoc;
use lsp_types::{ExecuteCommandParams, lsp_request};

use crate::support::cairo_project_toml::CAIRO_PROJECT_TOML;
use crate::support::normalize::normalize;
use crate::support::sandbox;

#[test]
fn cairo_projects() {
    let mut ls = sandbox! {
        files {
            "project1/cairo_project.toml" => indoc! {r#"
                [crate_roots]
                project1 = "src"
            "#},
            "project1/src/lib.cairo" => "fn main() {}",

            "project2/cairo_project.toml" => indoc! {r#"
                [crate_roots]
                project2 = "src"
            "#},
            "project2/src/lib.cairo" => "fn main() {}",

            "project2/subproject/cairo_project.toml" => indoc! {r#"
                [crate_roots]
                subproject = "src"
            "#},
            "project2/subproject/src/lib.cairo" => "fn main() {}"
        }
    };

    ls.open_all_cairo_files_and_wait_for_project_update();

    let output = ls.send_request::<lsp::ext::ViewAnalyzedCrates>(());

    insta::assert_snapshot!("view_analyzed_crates", normalize(&ls, output));
}

#[test]
fn test_reload() {
    let mut ls = sandbox! {
        files {
            "cairo_project.toml" => CAIRO_PROJECT_TOML,
            "src/lib.cairo" => "fn main() {}",
        }
    };

    ls.open_all_cairo_files_and_wait_for_project_update();

    let expected = ls.send_request::<lsp::ext::ViewAnalyzedCrates>(());

    ls.send_request::<lsp_request!("workspace/executeCommand")>(ExecuteCommandParams {
        command: "cairo.reload".into(),
        ..Default::default()
    });
    let actual = ls.wait_for_project_update();

    assert_eq!(expected, actual);
}

#[test]
fn assert_macros_with_no_cairo_test() {
    let mut ls = sandbox! {
        files {
            "Scarb.toml" => indoc! (r#"
                [package]
                name = "a"
                version = "0.1.0"
                edition = "2025_12"

                [dependencies]
                assert_macros = "2"
            "#),
            "src/lib.cairo" => "fn main() {}",
        }
    };

    ls.open_all_cairo_files_and_wait_for_project_update();

    let output = ls.send_request::<lsp::ext::ViewAnalyzedCrates>(());

    insta::assert_snapshot!("view_analyzed_crates_assert_macros", normalize(&ls, output));
}
