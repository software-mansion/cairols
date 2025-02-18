use cairo_language_server::lsp;
use indoc::indoc;

use crate::support::normalize::normalize;
use crate::support::sandbox;

#[test]
fn test_unmanaged_core_on_invalid_scarb_toml() {
    let mut ls = sandbox! {
        files {
            "Scarb.toml" => indoc! { r#"
                [package]
                version = "0.1.0"
            "#},
            "src/lib.cairo" => "",
        }
    };

    ls.open_and_wait_for_project_update("src/lib.cairo");

    let analyzed_crates = ls.send_request::<lsp::ext::ViewAnalyzedCrates>(());

    insta::assert_snapshot!(normalize(&ls, analyzed_crates), @r##"
    # Analyzed Crates

    - `core`: `["[SCARB_REGISTRY_STD]/core/src/lib.cairo"]`
        ```rust
        CrateSettings {
            name: None,
            edition: V2024_07,
            version: Some(
                Version {
                    major: 2,
                    minor: 10,
                    patch: 0,
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
    "##);
}
