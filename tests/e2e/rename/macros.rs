use lsp_types::request::Rename;

use crate::support::insta::{test_transform_plain, test_transform_with_macros};

#[test]
#[should_panic(expected = "not supported")]
fn inline_macro() {
    test_transform_plain!(Rename, r#"
    fn main() {
        print!("Hello world!");
        let arr = arr<caret>ay![1, 2, 3, 4, 5];
    }
    fn bar() {
        let forty_two = array![42];
    }
    "#, @"");
}

#[test]
#[should_panic(expected = "not supported")]
fn inline_macro_with_macros() {
    test_transform_with_macros!(Rename, r#"
    #[complex_attribute_macro_v2]
    fn main() {
        print!("Hello world!");
        let arr = arr<caret>ay![1, 2, 3, 4, 5];
    }

    #[complex_attribute_macro_v2]
    fn bar() {
        let forty_two = array![42];
    }
    "#, @"");
}
