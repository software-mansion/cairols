use lsp_types::request::GotoDefinition;

use crate::support::fixture;
use crate::support::insta::test_transform_and_macros;

#[test]
fn item_defined_in_another_file() {
    test_transform_and_macros!(GotoDefinition,
    fixture! {
        "src/something.cairo" => "pub fn hello() {}",
    }, r#"
    use crate::something::hello;

    <macro>#[complex_attribute_macro_v2]</macro>
    mod something;

    <macro>#[complex_attribute_macro_v2]</macro>
    fn main() {
        hel<caret>lo();
    }
    "#, @r"
    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;

    ==============================

    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;
    ")
}

#[test]
fn non_inline_module_on_definition() {
    test_transform_and_macros!(GotoDefinition,
    fixture! {
        "src/something.cairo" => "pub fn hello() {}",
    }, r#"
    use crate::something::hello;

    <macro>#[complex_attribute_macro_v2]</macro>
    mod some<caret>thing;

    fn main() {
        hello();
    }
    "#, @r"
    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;

    ==============================

    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;
    ")
}

#[test]
fn non_inline_module_on_usage() {
    test_transform_and_macros!(GotoDefinition,
    fixture! {
        "src/something.cairo" => "pub fn hello() {}",
    }, r#"
    <macro>#[complex_attribute_macro_v2]</macro>
   use crate::som<caret>ething::hello;

    <macro>#[complex_attribute_macro_v2]</macro>
    mod something;

    fn main() {
        hello();
    }
    "#, @r"
    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;

    ==============================

    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;
    ")
}

#[test]
fn inline_module_on_usage() {
    test_transform_and_macros!(GotoDefinition, r#"
    <macro>#[complex_attribute_macro_v2]</macro>
    use crate::some<caret>thing::hello;

    <macro>#[complex_attribute_macro_v2]</macro>
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

    ==============================

    #[complex_attribute_macro_v2]
    use crate::something::hello;

    #[complex_attribute_macro_v2]
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
    test_transform_and_macros!(GotoDefinition, r#"
    use crate::something::hello;

    <macro>#[complex_attribute_macro_v2]</macro>
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

    ==============================

    use crate::something::hello;

    #[complex_attribute_macro_v2]
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
    test_transform_and_macros!(GotoDefinition, r#"
    <macro>#[complex_attribute_macro_v2]</macro>
    use cra<caret>te::main;

    fn main() {
        hello();
    }
    "#, @r"
    <sel>use crate::main;

    fn main() {
        hello();
    }</sel>

    ==============================

    <sel>#[complex_attribute_macro_v2]
    use crate::main;

    fn main() {
        hello();
    }</sel>
    ")
}
