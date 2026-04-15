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
    completion_label = "panic!"
    insert_text = 'panic!("$1")'

    [[completions]]
    completion_label = "print!"
    insert_text = 'print!("$1")'

    [[completions]]
    completion_label = "println!"
    insert_text = 'println!("$1")'

    [[completions]]
    completion_label = "Option"

    [[completions]]
    completion_label = "panic(...)"
    completion_label_path = "(use panic)"
    completion_label_type_info = "fn(data: Array<felt252>) -> crate::never"
    insert_text = "panic(${1:data})"

    [[completions]]
    completion_label = "WrappingAdd"
    completion_label_path = "(use core::num::traits::WrappingAdd)"
    text_edits = ["""
    use core::num::traits::WrappingAdd;

    """]

    [[completions]]
    completion_label = "WrappingMul"
    completion_label_path = "(use core::num::traits::WrappingMul)"
    text_edits = ["""
    use core::num::traits::WrappingMul;

    """]

    [[completions]]
    completion_label = "WrappingSub"
    completion_label_path = "(use core::num::traits::WrappingSub)"
    text_edits = ["""
    use core::num::traits::WrappingSub;

    """]

    [[completions]]
    completion_label = "min(...)"
    completion_label_path = "(use core::cmp::min)"
    completion_label_type_info = "fn(a: T, b: T) -> T"
    insert_text = "min(${1:a}, ${2:b})"
    text_edits = ["""
    use core::cmp::min;

    """]

    [[completions]]
    completion_label = "option"
    completion_label_path = "(use core::option)"
    text_edits = ["""
    use core::option;

    """]

    [[completions]]
    completion_label = "print_byte_array_as_string(...)"
    completion_label_path = "(use core::debug::print_byte_array_as_string)"
    completion_label_type_info = "fn(self: @ByteArray) -> ()"
    insert_text = "print_byte_array_as_string()"
    text_edits = ["""
    use core::debug::print_byte_array_as_string;

    """]

    [[completions]]
    completion_label = "string"
    completion_label_path = "(use core::string)"
    text_edits = ["""
    use core::string;

    """]

    [[completions]]
    completion_label = "wrapping"
    completion_label_path = "(use core::num::traits::ops::wrapping)"
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
    completion_label = "a(...)"
    completion_label_path = "(use a)"
    completion_label_type_info = "fn() -> ()"
    insert_text = "a()"

    [[completions]]
    completion_label = "array!"
    insert_text = "array![$1]"

    [[completions]]
    completion_label = "Array"

    [[completions]]
    completion_label = "ArrayTrait"

    [[completions]]
    completion_label = "Err"

    [[completions]]
    completion_label = "Err"
    completion_label_path = "(use PanicResult::Err)"
    text_edits = ["""
    use PanicResult::Err;

    """]

    [[completions]]
    completion_label = "array"
    completion_label_path = "(use core::array)"
    text_edits = ["""
    use core::array;

    """]

    [[completions]]
    completion_label = "metaprogramming"
    completion_label_path = "(use core::metaprogramming)"
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
    completion_label = "compile_error!"
    insert_text = 'compile_error!("$1");'

    [[completions]]
    completion_label = "core"

    [[completions]]
    completion_label = "cmp"
    completion_label_path = "(use core::cmp)"
    text_edits = ["""
    use core::cmp;

    """]

    [[completions]]
    completion_label = "compute_keccak_byte_array(...)"
    completion_label_path = "(use core::keccak::compute_keccak_byte_array)"
    completion_label_type_info = "fn(arr: @ByteArray) -> u256"
    insert_text = "compute_keccak_byte_array(${1:arr})"
    text_edits = ["""
    use core::keccak::compute_keccak_byte_array;

    """]

    [[completions]]
    completion_label = "compute_sha256_byte_array(...)"
    completion_label_path = "(use core::sha256::compute_sha256_byte_array)"
    completion_label_type_info = "fn(arr: @ByteArray) -> [u32; 8]"
    insert_text = "compute_sha256_byte_array(${1:arr})"
    text_edits = ["""
    use core::sha256::compute_sha256_byte_array;

    """]

    [[completions]]
    completion_label = "compute_sha256_u32_array(...)"
    completion_label_path = "(use core::sha256::compute_sha256_u32_array)"
    completion_label_type_info = "fn(input: Array<u32>, last_input_word: u32, last_input_num_bytes: u32) -> [u32; 8]"
    insert_text = "compute_sha256_u32_array(${1:input}, ${2:last_input_word}, ${3:last_input_num_bytes})"
    text_edits = ["""
    use core::sha256::compute_sha256_u32_array;

    """]

    [[completions]]
    completion_label = "compute_sha256_u32_array_safe(...)"
    completion_label_path = "(use core::sha256::compute_sha256_u32_array_safe)"
    completion_label_type_info = "fn(input: Array<u32>, last_input_word: u32, last_input_num_bytes: BoundedInt<0, 3>) -> [u32; 8]"
    insert_text = "compute_sha256_u32_array_safe(${1:input}, ${2:last_input_word}, ${3:last_input_num_bytes})"
    text_edits = ["""
    use core::sha256::compute_sha256_u32_array_safe;

    """]
    "#);
}

#[test]
fn top_level_inline_macro() {
    test_transform_plain!(Completion,"
    macro my_own_macro {
        ($x:ident) => {
            fn foo() {}
        };
    }

    my_own_m<caret>

    ",@r#"
    caret = """
    my_own_m<caret>
    """

    [[completions]]
    completion_label = "my_own_macro!"
    completion_label_path = "(use my_own_macro)"
    insert_text = "my_own_macro!($1)"
    "#);
}
