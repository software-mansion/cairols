use cairo_language_server::lsp;
use indoc::indoc;
use similar::TextDiff;

use crate::scarb::AnalyzedCratesResult;
use crate::support::normalize::normalize;
use crate::support::sandbox;

#[test]
fn opening_dependency_first() {
    let mut ls = sandbox! {
        files {
            "pkg/Scarb.toml" => indoc! {r#"
                [package]
                name = "pkg"
                version = "0.1.0"
                edition = "2024_07"

                [[target.starknet-contract]]

                [dependencies]
                dep = { path = "../dep" }
            "#},
            "pkg/src/lib.cairo" => "",
            "dep/Scarb.toml" => indoc!(r#"
                [package]
                name = "dep"
                version = "0.1.0"
                edition = "2024_07"

                [dev-dependencies]
                cairo_test = "2"
                dev_dep = { path = "../dev_dep" }
            "#),
            "dep/src/lib.cairo" => "",
            "dev_dep/Scarb.toml" => indoc!(r#"
                [package]
                name = "dev_dep"
                version = "0.1.0"
                edition = "2024_07"
            "#),
            "dev_dep/src/lib.cairo" => "",
        }
    };

    // 1. Open `dep`.
    ls.open_and_wait_for_project_update("dep/src/lib.cairo");
    let analyzed_crates = ls.send_request::<lsp::ext::ViewAnalyzedCrates>(());
    let analyzed_crates = normalize(&ls, analyzed_crates);

    // 2. Open a package that has `dep` in dependencies.
    ls.open_and_wait_for_project_update("pkg/src/lib.cairo");
    let analyzed_crates_after = ls.send_request::<lsp::ext::ViewAnalyzedCrates>(());
    let analyzed_crates_after = normalize(&ls, analyzed_crates_after);

    // 3. Check if `dep` properties were not overwritten, but merged.
    let analyzed_crates_diff = TextDiff::from_lines(&analyzed_crates, &analyzed_crates_after)
        .unified_diff()
        .context_radius(15)
        .to_string();

    insta::assert_toml_snapshot!(AnalyzedCratesResult { analyzed_crates, analyzed_crates_diff })
}
