use lsp_types::request::Rename;
use serde_json::json;

use crate::support::insta::{test_transform_plain, test_transform_with_macros};

#[test]
#[should_panic(expected = "`invalid^name` is not a valid identifier")]
fn invalid_new_name() {
    let additional_data = json!( {
        "new_name" : r"invalid^name"
    });

    test_transform_plain!(Rename, r"
    fn fu<caret>nc() {}
    ", @"",
    Some(additional_data)
    );
}

#[test]
#[should_panic(expected = "`invalid^name` is not a valid identifier")]
fn invalid_new_name_with_macros() {
    let additional_data = json!( {
        "new_name" : r"invalid^name"
    });

    test_transform_with_macros!(Rename, r"
    #[complex_attribute_macro_v2]
    fn fu<caret>nc() {}
    ", @"",
    Some(additional_data)
    );
}
