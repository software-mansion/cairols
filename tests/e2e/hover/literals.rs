use lsp_types::Hover;

use crate::support::insta::{test_transform_plain, test_transform_with_macros};

#[test]
fn const_initializer_value() {
    test_transform_plain!(Hover,r#"
    const SOME_CONST: felt252 = 0x<caret>123;
    "#,@r#"
    source_context = """
    const SOME_CONST: felt252 = 0x<caret>123;
    """
    highlight = """
    const SOME_CONST: felt252 = <sel>0x123</sel>;
    """
    popover = """
    ```cairo
    felt252
    ```
    ---
    value of literal: `291 (0x123 | 0b100100011)`
    """
    "#)
}

#[test]
fn const_initializer_value_macro() {
    test_transform_with_macros!(Hover,r#"
    #[complex_attribute_macro_v2]
    const SOME_CONST: felt252 = 0x<caret>123;
    "#,@r#"
    source_context = """
    const SOME_CONST: felt252 = 0x<caret>123;
    """
    highlight = """
    const SOME_CONST: felt252 = <sel>0x123</sel>;
    """
    popover = """
    ```cairo
    felt252
    ```
    ---
    value of literal: `291 (0x123 | 0b100100011)`
    """
    "#)
}

#[test]
fn const_initializer_value_wrong_type() {
    test_transform_plain!(Hover,r#"
    const WRONG_TYPE_CONST: u8 = 123_<caret>felt252;
    "#,@r#"
    source_context = """
    const WRONG_TYPE_CONST: u8 = 123_<caret>felt252;
    """
    highlight = """
    const WRONG_TYPE_CONST: u8 = <sel>123_felt252</sel>;
    """
    popover = """
    ```cairo
    felt252
    ```
    ---
    value of literal: `123 (0x7b | 0b1111011)`
    """
    "#)
}

#[test]
fn const_initializer_value_wrong_type_shortstring() {
    test_transform_plain!(Hover,r#"
    const WRONG_TYPE_CONST: u8 = 'ab<caret>cd';
    "#,@r#"
    source_context = """
    const WRONG_TYPE_CONST: u8 = 'ab<caret>cd';
    """
    highlight = """
    const WRONG_TYPE_CONST: u8 = <sel>'abcd'</sel>;
    """
    popover = """
    ```cairo
    u8
    ```
    ---
    value of literal: `'abcd' (0x61626364)`
    """
    "#)
}

#[test]
fn const_initializer_value_wrong_type_bytearray() {
    test_transform_plain!(Hover,r#"
    const WRONG_TYPE_CONST: u8 = "ab<caret>cd";
    "#,@r#"
    source_context = """
    const WRONG_TYPE_CONST: u8 = "ab<caret>cd";
    """
    highlight = """
    const WRONG_TYPE_CONST: u8 = <sel>"abcd"</sel>;
    """
    popover = """
    ```cairo
    u8
    ```
    """
    "#)
}

#[test]
fn var_short_string() {
    test_transform_plain!(Hover,r#"
    /// Declares many variables and does nothing.
    fn many_variables() {
        let a = 'short<caret>_string';
    }
    "#,@r#"
    source_context = """
        let a = 'short<caret>_string';
    """
    highlight = """
        let a = <sel>'short_string'</sel>;
    """
    popover = """
    ```cairo
    felt252
    ```
    ---
    value of literal: `'short_string' (0x73686f72745f737472696e67)`
    """
    "#)
}

#[test]
fn var_byte_array() {
    test_transform_plain!(Hover,r#"
    /// Declares many variables and does nothing.
    fn many_variables() {
        let b: ByteArray = "normal_string<caret>";
    }
    "#,@r#"
    source_context = """
        let b: ByteArray = "normal_string<caret>";
    """
    highlight = """
        let b: ByteArray = <sel>"normal_string"</sel>;
    """
    popover = """
    ```cairo
    ByteArray
    ```
    """
    "#)
}

#[test]
fn var_explicit_felt() {
    test_transform_plain!(Hover,r#"
    /// Declares many variables and does nothing.
    fn many_variables() {
        let c = <caret>0x21_felt252;
    }
    "#,@r#"
    source_context = """
        let c = <caret>0x21_felt252;
    """
    highlight = """
        let c = <sel>0x21_felt252</sel>;
    """
    popover = """
    ```cairo
    felt252
    ```
    ---
    value of literal: `33 (0x21 | 0b100001)`
    """
    "#)
}

#[test]
fn var_explicit_u32() {
    test_transform_plain!(Hover,r#"
    /// Declares many variables and does nothing.
    fn many_variables() {
        let d = 0x3<caret>3_u32;
    }
    "#,@r#"
    source_context = """
        let d = 0x3<caret>3_u32;
    """
    highlight = """
        let d = <sel>0x33_u32</sel>;
    """
    popover = """
    ```cairo
    u32
    ```
    ---
    value of literal: `51 (0x33 | 0b110011)`
    """
    "#)
}

#[test]
fn var_dec_felt() {
    test_transform_plain!(Hover,r#"
    /// Declares many variables and does nothing.
    fn many_variables() {
        let e = 5<caret>0;
    }
    "#,@r#"
    source_context = """
        let e = 5<caret>0;
    """
    highlight = """
        let e = <sel>50</sel>;
    """
    popover = """
    ```cairo
    felt252
    ```
    ---
    value of literal: `50 (0x32 | 0b110010)`
    """
    "#)
}

#[test]
fn var_bin_felt() {
    test_transform_plain!(Hover,r#"
    /// Declares many variables and does nothing.
    fn many_variables() {
        let f = 0b<caret>1111;
    }
    "#,@r#"
    source_context = """
        let f = 0b<caret>1111;
    """
    highlight = """
        let f = <sel>0b1111</sel>;
    """
    popover = """
    ```cairo
    felt252
    ```
    ---
    value of literal: `15 (0xf | 0b1111)`
    """
    "#)
}

#[test]
fn var_oct_felt() {
    test_transform_plain!(Hover,r#"
    /// Declares many variables and does nothing.
    fn many_variables() {
        let g = 0o<caret>77;
    }
    "#,@r#"
    source_context = """
        let g = 0o<caret>77;
    """
    highlight = """
        let g = <sel>0o77</sel>;
    """
    popover = """
    ```cairo
    felt252
    ```
    ---
    value of literal: `63 (0x3f | 0b111111)`
    """
    "#)
}

#[test]
fn var_explicit_hex_felt_to_u32() {
    test_transform_plain!(Hover,r#"
    /// Declares many variables generating errors.
    fn many_wrong_variables() {
        let a: u32 = 0x<caret>170_felt252;
    }
    "#,@r#"
    source_context = """
        let a: u32 = 0x<caret>170_felt252;
    """
    highlight = """
        let a: u32 = <sel>0x170_felt252</sel>;
    """
    popover = """
    ```cairo
    felt252
    ```
    ---
    value of literal: `368 (0x170 | 0b101110000)`
    """
    "#)
}

#[test]
fn var_string_to_u8() {
    test_transform_plain!(Hover,r#"
    /// Declares many variables generating errors.
    fn many_wrong_variables() {
        let b: u8 = "some_st<caret>ring";
    }
    "#,@r#"
    source_context = """
        let b: u8 = "some_st<caret>ring";
    """
    highlight = """
        let b: u8 = <sel>"some_string"</sel>;
    """
    popover = """
    ```cairo
    u8
    ```
    """
    "#)
}

#[test]
fn var_hex_felt_to_u8() {
    test_transform_plain!(Hover,r#"
    /// Declares many variables generating errors.
    fn many_wrong_variables() {
        let c: u8 = 0x<caret>fff;
    }
    "#,@r#"
    source_context = """
        let c: u8 = 0x<caret>fff;
    """
    highlight = """
        let c: u8 = <sel>0xfff</sel>;
    """
    popover = """
    ```cairo
    u8
    ```
    ---
    value of literal: `4095 (0xfff | 0b111111111111)`
    """
    "#)
}
