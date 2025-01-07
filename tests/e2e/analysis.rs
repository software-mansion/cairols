use cairo_language_server::lsp;
use indoc::indoc;
use lsp_types::{ExecuteCommandParams, lsp_request};
use pretty_assertions::assert_eq;

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

    assert_eq!(normalize(&ls, output), indoc! {r#"
            # Analyzed Crates

            - `core`: `["[SCARB_REGISTRY_STD]/core/src/lib.cairo"]`
                ```rust
                CrateSettings {
                    name: None,
                    edition: V2024_07,
                    version: Some(
                        Version {
                            major: 2,
                            minor: 9,
                            patch: 2,
                        },
                    ),
                    cfg_set: None,
                    dependencies: {},
                    experimental_features: ExperimentalFeaturesConfig {
                        negative_impls: true,
                        associated_item_constraints: true,
                        coupons: true,
                    },
                }
                ```
            - `project1`: `["[ROOT]/project1/src/lib.cairo"]`
                ```rust
                CrateSettings {
                    name: None,
                    edition: V2023_01,
                    version: None,
                    cfg_set: None,
                    dependencies: {},
                    experimental_features: ExperimentalFeaturesConfig {
                        negative_impls: false,
                        associated_item_constraints: false,
                        coupons: false,
                    },
                }
                ```
            - `project2`: `["[ROOT]/project2/src/lib.cairo"]`
                ```rust
                CrateSettings {
                    name: None,
                    edition: V2023_01,
                    version: None,
                    cfg_set: None,
                    dependencies: {},
                    experimental_features: ExperimentalFeaturesConfig {
                        negative_impls: false,
                        associated_item_constraints: false,
                        coupons: false,
                    },
                }
                ```
            - `subproject`: `["[ROOT]/project2/subproject/src/lib.cairo"]`
                ```rust
                CrateSettings {
                    name: None,
                    edition: V2023_01,
                    version: None,
                    cfg_set: None,
                    dependencies: {},
                    experimental_features: ExperimentalFeaturesConfig {
                        negative_impls: false,
                        associated_item_constraints: false,
                        coupons: false,
                    },
                }
                ```
        "#});
}

#[test]
fn test_reload() {
    let mut ls = sandbox! {
        files {
            "cairo_project.toml" => indoc! {r#"
                [crate_roots]
                hello = "src"
            "#},
            "src/lib.cairo" => "fn main() {}",
        }
    };

    ls.open_all_cairo_files_and_wait_for_project_update();

    let expected = ls.send_request::<lsp::ext::ViewAnalyzedCrates>(());

    ls.send_request::<lsp_request!("workspace/executeCommand")>(ExecuteCommandParams {
        command: "cairo.reload".into(),
        ..Default::default()
    });
    ls.wait_for_project_update();

    let actual = ls.send_request::<lsp::ext::ViewAnalyzedCrates>(());

    assert_eq!(expected, actual);
}
