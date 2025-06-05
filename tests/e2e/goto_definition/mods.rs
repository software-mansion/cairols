use lsp_types::request::GotoDefinition;

use crate::support::fixture;
use crate::support::insta::{test_transform_plain, test_transform_with_macros};

#[test]
fn item_defined_in_another_file() {
    test_transform_plain!(GotoDefinition,
    fixture! {
        "src/something.cairo" => "pub fn hello() {}",
    }, r#"
    use crate::something::hello;

    mod something;

    fn main() {
        hel<caret>lo();
    }
    "#, @r"
    // → src/something.cairo
    pub fn <sel>hello</sel>() {}
    ")
}

#[test]
fn non_inline_module_on_definition() {
    test_transform_plain!(GotoDefinition,
    fixture! {
        "src/something.cairo" => "pub fn hello() {}",
    }, r#"
    use crate::something::hello;

    mod some<caret>thing;

    fn main() {
        hello();
    }
    "#, @r"
    // → src/something.cairo
    <sel>pub fn hello() {}</sel>
    ")
}

#[test]
fn non_inline_module_on_usage() {
    test_transform_plain!(GotoDefinition,
    fixture! {
        "src/something.cairo" => "pub fn hello() {}",
    }, r#"
    use crate::som<caret>ething::hello;

    mod something;

    fn main() {
        hello();
    }
    "#, @r"
    // → src/something.cairo
    <sel>pub fn hello() {}</sel>
    ")
}

#[test]
fn inline_module_on_usage() {
    test_transform_plain!(GotoDefinition, r#"
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
    test_transform_plain!(GotoDefinition, r#"
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
    test_transform_plain!(GotoDefinition, r#"
    mod modzik {
        use cra<caret>te::main;
    }

    fn main() {
        hello();
    }
    "#, @r"
    <sel>mod modzik {
        use crate::main;
    }

    fn main() {
        hello();
    }</sel>
    ")
}

// FIXME(#721)
#[test]
fn crate_module_with_macros() {
    test_transform_with_macros!(GotoDefinition, r#"
    #[complex_attribute_macro_v2]
    mod modzik {
        #[complex_attribute_macro_v2]
        use cra<caret>te::main;
    }

    fn main() {
        hello();
    }
    "#, @"none response")
}

#[test]
fn item_defined_in_another_file_with_macros() {
    test_transform_with_macros!(GotoDefinition,
    fixture! {
        "src/something.cairo" => "pub fn hello() {}",
    }, r#"
    use crate::something::hello;

    #[complex_attribute_macro_v2]
    mod something;

    #[complex_attribute_macro_v2]
    fn main() {
        hel<caret>lo();
    }
    "#, @r"
    // → src/something.cairo
    pub fn <sel>hello</sel>() {}
    ")
}
