use lsp_types::request::Completion;

use crate::support::insta::{test_transform_plain, test_transform_with_macros};

#[test]
fn exp_inline_macro() {
    test_transform_plain!(Completion,"
    fn a() {
        let x = 5;
        prin<caret>
        let z = 6;
    }
    ",@r#"
    caret = """
        prin<caret>
    """

    [[completions]]
    completion_label = "Option"
    completion_label_path = "Option"

    [[completions]]
    completion_label = "WrappingAdd"
    completion_label_path = "core::num::traits::WrappingAdd"
    text_edits = ["""
    use core::num::traits::WrappingAdd;

    """]

    [[completions]]
    completion_label = "WrappingMul"
    completion_label_path = "core::num::traits::WrappingMul"
    text_edits = ["""
    use core::num::traits::WrappingMul;

    """]

    [[completions]]
    completion_label = "WrappingSub"
    completion_label_path = "core::num::traits::WrappingSub"
    text_edits = ["""
    use core::num::traits::WrappingSub;

    """]

    [[completions]]
    completion_label = "min"
    completion_label_path = "core::cmp::min"
    text_edits = ["""
    use core::cmp::min;

    """]

    [[completions]]
    completion_label = "option"
    completion_label_path = "core::option"
    text_edits = ["""
    use core::option;

    """]

    [[completions]]
    completion_label = "panic"
    completion_label_path = "panic"

    [[completions]]
    completion_label = "panic!"
    insert_text = 'panic!("$1")'

    [[completions]]
    completion_label = "print!"
    insert_text = 'print!("$1")'

    [[completions]]
    completion_label = "print_byte_array_as_string"
    completion_label_path = "core::debug::print_byte_array_as_string"
    text_edits = ["""
    use core::debug::print_byte_array_as_string;

    """]

    [[completions]]
    completion_label = "println!"
    insert_text = 'println!("$1")'

    [[completions]]
    completion_label = "string"
    completion_label_path = "core::string"
    text_edits = ["""
    use core::string;

    """]

    [[completions]]
    completion_label = "wrapping"
    completion_label_path = "core::num::traits::ops::wrapping"
    text_edits = ["""
    use core::num::traits::ops::wrapping;

    """]
    "#);
}

#[test]
fn exp_inline_macro_in_let_statement() {
    test_transform_plain!(Completion,"
    fn a() {
        let x = 5;
        let y = arra<caret>
        let z = 6;
    }
    ",@r#"
    caret = """
        let y = arra<caret>
    """

    [[completions]]
    completion_label = "Array"
    completion_label_path = "Array"

    [[completions]]
    completion_label = "ArrayTrait"
    completion_label_path = "ArrayTrait"

    [[completions]]
    completion_label = "Err"
    completion_label_path = "Err"

    [[completions]]
    completion_label = "Err"
    completion_label_path = "PanicResult::Err"
    text_edits = ["""
    use PanicResult::Err;

    """]

    [[completions]]
    completion_label = "a"
    completion_label_path = "a"

    [[completions]]
    completion_label = "array"
    completion_label_path = "core::array"
    text_edits = ["""
    use core::array;

    """]

    [[completions]]
    completion_label = "array!"
    insert_text = "array![$1]"

    [[completions]]
    completion_label = "metaprogramming"
    completion_label_path = "core::metaprogramming"
    text_edits = ["""
    use core::metaprogramming;

    """]
    "#);
}

#[test]
fn top_level_macro_before_items() {
    test_transform_plain!(Completion,"
    compile_er<caret>
    pub struct Struct {}
    fn a() {}
    ",@r#"
    caret = """
    compile_er<caret>
    """

    [[completions]]
    completion_label = "compile_error!"
    insert_text = 'compile_error!("$1");'
    "#);
}

#[test]
fn top_level_macro_between_items() {
    test_transform_plain!(Completion,"
    pub struct Struct {}
    compile_er<caret>
    fn a() {}
    ",@r#"
    caret = """
    compile_er<caret>
    """

    [[completions]]
    completion_label = "compile_error!"
    insert_text = 'compile_error!("$1");'
    "#);
}

#[test]
fn top_level_macro_after_items() {
    test_transform_plain!(Completion,"
    pub struct Struct {}
    fn a() {}
    compile_er<caret>
    ",@r#"
    caret = """
    compile_er<caret>
    """

    [[completions]]
    completion_label = "compile_error!"
    insert_text = 'compile_error!("$1");'
    "#);
}

#[test]
fn top_level_macro_before_items_in_module() {
    test_transform_plain!(Completion,"
    mod my_mod {
        compile_er<caret>
        pub struct Struct {}
        fn a() {}
    }
    ",@r#"
    caret = """
        compile_er<caret>
    """

    [[completions]]
    completion_label = "compile_error!"
    insert_text = 'compile_error!("$1");'
    "#);
}

#[test]
fn top_level_macro_between_items_in_module() {
    test_transform_plain!(Completion,"
    mod my_mod {
        pub struct Struct {}
        compile_er<caret>
        fn a() {}
    }
    ",@r#"
    caret = """
        compile_er<caret>
    """

    [[completions]]
    completion_label = "compile_error!"
    insert_text = 'compile_error!("$1");'
    "#);
}

#[test]
fn top_level_macro_after_items_in_module() {
    test_transform_plain!(Completion,"
    mod my_mod {
        pub struct Struct {}
        fn a() {}
        compile_er<caret>
    }
    ",@r#"
    caret = """
        compile_er<caret>
    """

    [[completions]]
    completion_label = "compile_error!"
    insert_text = 'compile_error!("$1");'
    "#);
}

#[test]
fn top_level_macro_between_items_in_module_with_macros() {
    test_transform_with_macros!(Completion,"
    #[complex_attribute_macro_v2]
    mod my_mod {
        #[complex_attribute_macro_v2]
        pub struct Struct {}
        compile_er<caret>
        #[complex_attribute_macro_v2]
        fn a() {}
    }
    ",@r#"
    caret = """
        compile_er<caret>
    """

    [[completions]]
    completion_label = "compile_error!"
    insert_text = 'compile_error!("$1");'
    "#);
}

#[test]
fn component_top_level_macro() {
    test_transform_plain!(Completion,"
    #[starknet::contract]
    mod Contract {
        #[storage]
        struct Storage {}

        fn a() {}

        compo<caret>
    }
    ",@r#"
    caret = """
        compo<caret>
    """

    [[completions]]
    completion_label = "compile_error!"
    insert_text = 'compile_error!("$1");'

    [[completions]]
    completion_label = "component!"
    insert_text = "component!(path: $1, storage: $2, event: $3);"
    "#);
}

#[test]
fn partially_typed_top_level_macro_after_items() {
    test_transform_plain!(Completion,"
    pub struct Struct {}
    fn a() {}
    compile_er<caret> ()
    ",@r#"
    caret = """
    compile_er<caret> ()
    """

    [[completions]]
    completion_label = "cmp"
    completion_label_path = "core::cmp"
    text_edits = ["""
    use core::cmp;

    """]

    [[completions]]
    completion_label = "compile_error!"
    insert_text = 'compile_error!("$1");'

    [[completions]]
    completion_label = "compute_keccak_byte_array"
    completion_label_path = "core::keccak::compute_keccak_byte_array"
    text_edits = ["""
    use core::keccak::compute_keccak_byte_array;

    """]

    [[completions]]
    completion_label = "compute_sha256_byte_array"
    completion_label_path = "core::sha256::compute_sha256_byte_array"
    text_edits = ["""
    use core::sha256::compute_sha256_byte_array;

    """]

    [[completions]]
    completion_label = "compute_sha256_u32_array"
    completion_label_path = "core::sha256::compute_sha256_u32_array"
    text_edits = ["""
    use core::sha256::compute_sha256_u32_array;

    """]

    [[completions]]
    completion_label = "core"
    completion_label_path = "core"
    "#);
}
