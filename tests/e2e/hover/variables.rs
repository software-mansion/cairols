use lsp_types::Hover;

use crate::support::insta::{test_transform_plain, test_transform_with_macros};

#[test]
fn ident_typed() {
    test_transform_plain!(Hover,r#"
    fn main() {
        let ab<caret>c: felt252 = 0;
    }
    "#,@r#"
    source_context = """
        let ab<caret>c: felt252 = 0;
    """
    highlight = """
        let <sel>abc</sel>: felt252 = 0;
    """
    popover = """
    ```cairo
    let abc: felt252
    ```
    """
    "#)
}

#[test]
fn ident_macro() {
    test_transform_with_macros!(Hover,r#"
    #[complex_attribute_macro_v2]
    fn main() {
        let xy<caret>z = 3;
    }
    "#,@r#"
    source_context = """
        let xy<caret>z = 3;
    """
    highlight = """
        let <sel>xyz</sel> = 3;
    """
    popover = """
    ```cairo
    let xyz: felt252
    ```
    """
    "#)
}

#[test]
fn ident() {
    test_transform_plain!(Hover,r#"
    fn main() {
        let xy<caret>z = 3;
    }
    "#,@r#"
    source_context = """
        let xy<caret>z = 3;
    """
    highlight = """
        let <sel>xyz</sel> = 3;
    """
    popover = """
    ```cairo
    let xyz: felt252
    ```
    """
    "#)
}

#[test]
fn ident_mut() {
    test_transform_plain!(Hover,r#"
    fn main() {
        let abc: felt252 = 0;
        let mut de<caret>f = abc * 2;
    }
    "#,@r#"
    source_context = """
        let mut de<caret>f = abc * 2;
    """
    highlight = """
        let mut <sel>def</sel> = abc * 2;
    """
    popover = """
    ```cairo
    let mut def: felt252
    ```
    """
    "#)
}

#[test]
fn value_mut() {
    test_transform_plain!(Hover,r#"
    fn main() {
        let abc: felt252 = 0;
        let mut def = ab<caret>c * 2;
    }
    "#,@r#"
    source_context = """
        let mut def = ab<caret>c * 2;
    """
    highlight = """
        let mut def = <sel>abc</sel> * 2;
    """
    popover = """
    ```cairo
    let abc: felt252
    ```
    """
    "#)
}

#[test]
fn star_lhs() {
    test_transform_plain!(Hover,r#"
    fn main() {
        let mut def: felt252 = 0;
        let xyz = 0;
        let _ = de<caret>f * xyz;
    }
    "#,@r#"
    source_context = """
        let _ = de<caret>f * xyz;
    """
    highlight = """
        let _ = <sel>def</sel> * xyz;
    """
    popover = """
    ```cairo
    let mut def: felt252
    ```
    """
    "#)
}

#[test]
fn star_rhs() {
    test_transform_plain!(Hover,r#"
    fn main() {
        let mut def: felt252 = 0;
        let xyz = 0;
        let _ = def * xy<caret>z;
    }
    "#,@r#"
    source_context = """
        let _ = def * xy<caret>z;
    """
    highlight = """
        let _ = def * <sel>xyz</sel>;
    """
    popover = """
    ```cairo
    let xyz: felt252
    ```
    """
    "#)
}

/// https://github.com/software-mansion/cairols/issues/80
#[test]
fn issue_80() {
    test_transform_plain!(Hover,r#"
    use core::to_byte_array::FormatAsByteArray;
    pub fn to_base_16_string_no_padding(value: felt252) -> ByteArray {
        let string = value.format_as_byte_array(16);
        format!("0x{}", str<caret>ing)
    }
    "#,@r#"
    source_context = """
        format!("0x{}", str<caret>ing)
    """
    "#)
}
