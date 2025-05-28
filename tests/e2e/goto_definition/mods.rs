use indoc::indoc;

use crate::goto_definition::{GotoDefinitionTest, goto_definition};
use crate::support::cairo_project_toml::CAIRO_PROJECT_TOML_2024_07;
use crate::support::insta::test_transform;
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
        "src/lib.cairo" => lib_cairo,
        "src/something.cairo" => "pub fn hello() {}",
    });

    let result = test.request_snapshot("src/lib.cairo", cursors.assert_single_caret());

    insta::assert_snapshot!(result, @r"
    // → src/something.cairo
    pub fn <sel>hello</sel>() {}
    ")
}

#[test]
fn non_inline_module_on_definition() {
    let (lib_cairo, cursors) = cursors(indoc! {r#"
        use crate::something::hello;

        mod some<caret>thing;

        fn main() {
            hello();
        }
    "#});

    let mut test = GotoDefinitionTest::begin(fixture! {
        "cairo_project.toml" => CAIRO_PROJECT_TOML_2024_07,
        "src/lib.cairo" => lib_cairo,
        "src/something.cairo" => "pub fn hello() {\n}",
    });

    let result = test.request_snapshot("src/lib.cairo", cursors.assert_single_caret());

    insta::assert_snapshot!(result, @r"
    // → src/something.cairo
    <sel>pub fn hello() {
    }</sel>
    ")
}

#[test]
fn non_inline_module_on_usage() {
    let (lib_cairo, cursors) = cursors(indoc! {r#"
        use crate::som<caret>ething::hello;

        mod something;

        fn main() {
            hello();
        }
    "#});

    let mut test = GotoDefinitionTest::begin(fixture! {
        "cairo_project.toml" => CAIRO_PROJECT_TOML_2024_07,
        "src/lib.cairo" => lib_cairo,
        "src/something.cairo" => "pub fn hello() {\n}",
    });

    let result = test.request_snapshot("src/lib.cairo", cursors.assert_single_caret());

    insta::assert_snapshot!(result, @r"
    // → src/something.cairo
    <sel>pub fn hello() {
    }</sel>
    ")
}

#[test]
fn inline_module_on_usage() {
    test_transform!(goto_definition, r#"
    use crate::some<caret>thing::hello;

    mod something {
      fn hello() {}
    }

    fn main() {
        hello();
    }
    "#, @r"
    use crate::something::hello;

    mod <sel>something</sel> {
      fn hello() {}
    }

    fn main() {
        hello();
    }
    ")
}

#[test]
fn inline_module_on_definition() {
    test_transform!(goto_definition, r#"
    use crate::something::hello;

    mod some<caret>thing {
      fn hello() {}
    }

    fn main() {
        hello();
    }
    "#, @r"
    use crate::something::hello;

    mod <sel>something</sel> {
      fn hello() {}
    }

    fn main() {
        hello();
    }
    ")
}

#[test]
fn crate_module() {
    test_transform!(goto_definition, r#"
    use cra<caret>te::main;

    fn main() {
        hello();
    }
    "#, @r"
    <sel>use crate::main;

    fn main() {
        hello();
    }</sel>
    ")
}
