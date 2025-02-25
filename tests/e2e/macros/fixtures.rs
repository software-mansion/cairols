use indoc::formatdoc;

use crate::support::fixture::{Fixture, fixture};

use super::{MacroTest, SCARB_TEST_MACROS_PACKAGE};

pub struct ProjectWithCustomMacros;

impl MacroTest for ProjectWithCustomMacros {
    const CWD: &str = "test_package";
    const SNIPPET_LOCATION: &str = "test_package/src/lib.cairo";

    fn fixture() -> Fixture {
        fixture! {
            "test_package/Scarb.toml" => formatdoc!(
                r#"
                [package]
                name = "test_package"
                version = "0.1.0"
                edition = "2024_07"

                [dependencies]
                cairols_test_macros = {{ path = "{}" }}
                "#,
                SCARB_TEST_MACROS_PACKAGE.display()
            )
        }
    }
}
