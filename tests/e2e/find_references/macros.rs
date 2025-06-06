use lsp_types::request::References;

use crate::support::insta::{test_transform_plain, test_transform_with_macros};

#[test]
fn inline_macro() {
    test_transform_plain!(References, r#"
    fn main() {
        print!("Hello world!");
        let arr = arr<caret>ay![1, 2, 3, 4, 5];
    }
    fn bar() {
        let forty_two = array![42];
    }
    "#, @r#"
    // found several references in the core crate
    fn main() {
        print!("Hello world!");
        let arr = <sel>array</sel>![1, 2, 3, 4, 5];
    }
    fn bar() {
        let forty_two = <sel>array</sel>![42];
    }
    "#)
}

#[test]
fn inline_macro_with_macros() {
    test_transform_with_macros!(References, r#"
    #[complex_attribute_macro_v2]
    fn main() {
        print!("Hello world!");
        let arr = arr<caret>ay![1, 2, 3, 4, 5];
    }

    fn bar() {
        let forty_two = array![42];
    }
    "#, @r#"
    // found several references in the core crate
    #[complex_attribute_macro_v2]
    fn main() {
        print!("Hello world!");
        let arr = <sel>array</sel>![1, 2, 3, 4, 5];
    }

    fn bar() {
        let forty_two = <sel>array</sel>![42];
    }
    "#)
}
