use indoc::indoc;

use crate::goto_definition::GotoDefinitionTest;
use crate::support::cairo_project_toml::CAIRO_PROJECT_TOML_2024_07;
use crate::support::{cursors, fixture};

#[test]
fn item_defined_in_another_file() {
    let (lib_cairo, cursors) = cursors(indoc! {r#"
        use crate::something::hello;
        mod something;
        fn main() {
            hel<caret>lo();
        }
    "#});

    let mut test = GotoDefinitionTest::begin(fixture! {
        "cairo_project.toml" => CAIRO_PROJECT_TOML_2024_07,
        "src/lib.cairo" => lib_cairo.clone(),
        "src/something.cairo" => "pub fn hello() {}",
    });

    let result = test.request_snapshot("src/lib.cairo", cursors.caret(0));

    insta::with_settings!({ description => lib_cairo }, {
        insta::assert_snapshot!(result, @r"
        // → src/something.cairo
        pub fn <sel>hello</sel>() {}
        ")
    });
}
