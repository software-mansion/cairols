use indoc::{formatdoc, indoc};

use crate::support::fixture::{Fixture, fixture};

use super::{MacroTest, SCARB_TEST_MACROS_PACKAGE};

pub struct ProjectWithCustomMacros;
pub struct ProjectWithSnforgeUnitTest;
pub struct ProjectWithSnforgeIntegrationTest;
pub struct ProjectWithCairoProjectToml;

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

impl MacroTest for ProjectWithSnforgeUnitTest {
    const CWD: &str = "test_package";
    const SNIPPET_LOCATION: &str = "test_package/src/lib.cairo";

    fn fixture() -> Fixture {
        fixture! {
            "test_package/Scarb.toml" => indoc!(r#"
                [package]
                name = "test_package"
                version = "0.1.0"
                edition = "2024_07"

                [tool.scarb]
                allow-prebuilt-plugins = ["snforge_scarb_plugin"]

                [dev-dependencies]
                assert_macros = "2.10.0"
                snforge_std = "0.37.0"
                snforge_scarb_plugin = "0.37.0"
            "#),
        }
    }
}

impl MacroTest for ProjectWithSnforgeIntegrationTest {
    const CWD: &str = "test_package";
    const SNIPPET_LOCATION: &str = "test_package/tests/test.cairo";

    fn fixture() -> Fixture {
        fixture! {
            "test_package/lib.cairo" => "",

            "test_package/Scarb.toml" => indoc!(r#"
                [package]
                name = "test_package"
                version = "0.1.0"
                edition = "2024_07"

                [dev-dependencies]
                snforge_std = "0.37.0"
                snforge_scarb_plugin = "0.37.0"

                [tool.scarb]
                allow-prebuilt-plugins = ["snforge_scarb_plugin"]

                [[tool.snforge.fork]]
                name = "SEPOLIA_LATEST"
                url = "https://starknet-sepolia.public.blastapi.io/rpc/v0_7"
                block_id.tag = "latest"
            "#),
        }
    }
}

impl MacroTest for ProjectWithCairoProjectToml {
    const CWD: &str = "test_package";
    const SNIPPET_LOCATION: &str = "test_package/src/lib.cairo";

    fn fixture() -> Fixture {
        fixture! {
            "test_package/cairo_project.toml" => indoc!(r#"
                [crate-roots]
                test_package = "src"
            "#)
        }
    }
}
