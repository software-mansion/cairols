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

    insta::assert_snapshot!(normalize(&ls, analyzed_crates), @r#"
    # Analyzed Crates
    ---
    ```json
    {
      "name": "core",
      "source_paths": [
        "[SCARB_REGISTRY_STD]/core/src/lib.cairo"
      ],
      "settings": {
        "name": null,
        "edition": "2024_07",
        "version": "2.12.0",
        "cfg_set": null,
        "dependencies": {},
        "experimental_features": {
          "negative_impls": true,
          "associated_item_constraints": true,
          "coupons": true,
          "user_defined_inline_macros": true
        }
      },
      "linter_configuration": "Off",
      "plugins": {
        "builtin_plugins": [
          "AssertMacros",
          "Executable",
          "CairoTest"
        ]
      }
    }
    ```
    "#);
}
