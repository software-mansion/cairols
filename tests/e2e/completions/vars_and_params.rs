use lsp_types::request::Completion;

use crate::{
    completions::completion_fixture,
    support::insta::{test_transform_plain, test_transform_with_macros},
};

#[test]
fn all_prefixed() {
    test_transform_plain!(Completion, completion_fixture(), "
    fn a(param: felt252, param2: felt252, param3: felt252){
        let foo = 1;
        let bar = 1;
        let baz = 1;
        let foo2 = 1;

        ba<caret>
    }
    ",@r#"
    caret = """
        ba<caret>
    """

    [[completions]]
    completion_label = "bar"
    detail = "felt252"

    [[completions]]
    completion_label = "baz"
    detail = "felt252"

    [[completions]]
    completion_label = "blake"
    text_edits = ["""
    use core::blake;

    """]

    [[completions]]
    completion_label = "blake2s_compress"
    insert_text = "blake2s_compress(${1:state}, ${2:byte_count}, ${3:msg})"
    text_edits = ["""
    use core::blake::blake2s_compress;

    """]

    [[completions]]
    completion_label = "blake2s_finalize"
    insert_text = "blake2s_finalize(${1:state}, ${2:byte_count}, ${3:msg})"
    text_edits = ["""
    use core::blake::blake2s_finalize;

    """]

    [[completions]]
    completion_label = "byte_array"
    text_edits = ["""
    use core::byte_array;

    """]

    [[completions]]
    completion_label = "library_call_syscall"
    insert_text = "library_call_syscall(${1:class_hash}, ${2:function_selector}, ${3:calldata})"
    text_edits = ["""
    use starknet::syscalls::library_call_syscall;

    """]
    "#);
}

#[test]
fn all_prefixed_macro() {
    test_transform_with_macros!(Completion, completion_fixture(), "
    #[complex_attribute_macro_v2]
    fn a(param: felt252, param2: felt252, param3: felt252){
        let foo = 1;
        let bar = 1;
        let baz = 1;
        let foo2 = 1;

        ba<caret>
    }
    ",@r#"
    caret = """
        ba<caret>
    """

    [[completions]]
    completion_label = "bar"
    detail = "felt252"

    [[completions]]
    completion_label = "baz"
    detail = "felt252"

    [[completions]]
    completion_label = "blake"
    text_edits = ["""
    use core::blake;

    """]

    [[completions]]
    completion_label = "blake2s_compress"
    insert_text = "blake2s_compress(${1:state}, ${2:byte_count}, ${3:msg})"
    text_edits = ["""
    use core::blake::blake2s_compress;

    """]

    [[completions]]
    completion_label = "blake2s_finalize"
    insert_text = "blake2s_finalize(${1:state}, ${2:byte_count}, ${3:msg})"
    text_edits = ["""
    use core::blake::blake2s_finalize;

    """]

    [[completions]]
    completion_label = "byte_array"
    text_edits = ["""
    use core::byte_array;

    """]

    [[completions]]
    completion_label = "library_call_syscall"
    insert_text = "library_call_syscall(${1:class_hash}, ${2:function_selector}, ${3:calldata})"
    text_edits = ["""
    use starknet::syscalls::library_call_syscall;

    """]
    "#);
}

#[test]
fn only_before_cursor() {
    test_transform_plain!(Completion, completion_fixture(), "
    fn a(param: felt252, param2: felt252, param3: felt252){
        let foo = 1;
        let bar = 1;
        ba<caret>
        let baz = 1;
        let foo2 = 1;

    }
    ",@r#"
    caret = """
        ba<caret>
    """

    [[completions]]
    completion_label = "bar"
    detail = "felt252"

    [[completions]]
    completion_label = "blake"
    text_edits = ["""
    use core::blake;

    """]

    [[completions]]
    completion_label = "blake2s_compress"
    insert_text = "blake2s_compress(${1:state}, ${2:byte_count}, ${3:msg})"
    text_edits = ["""
    use core::blake::blake2s_compress;

    """]

    [[completions]]
    completion_label = "blake2s_finalize"
    insert_text = "blake2s_finalize(${1:state}, ${2:byte_count}, ${3:msg})"
    text_edits = ["""
    use core::blake::blake2s_finalize;

    """]

    [[completions]]
    completion_label = "byte_array"
    text_edits = ["""
    use core::byte_array;

    """]

    [[completions]]
    completion_label = "library_call_syscall"
    insert_text = "library_call_syscall(${1:class_hash}, ${2:function_selector}, ${3:calldata})"
    text_edits = ["""
    use starknet::syscalls::library_call_syscall;

    """]
    "#);
}

#[test]
fn disallow_recursive_definition() {
    test_transform_plain!(Completion, completion_fixture(), "
    fn a(param: felt252, param2: felt252, param3: felt252){
        let foo = fo<caret>;
    }
    ",@r#"
    caret = """
        let foo = fo<caret>;
    """

    [[completions]]
    completion_label = "OverflowingAdd"
    text_edits = ["""
    use core::num::traits::OverflowingAdd;

    """]

    [[completions]]
    completion_label = "OverflowingMul"
    text_edits = ["""
    use core::num::traits::OverflowingMul;

    """]

    [[completions]]
    completion_label = "OverflowingSub"
    text_edits = ["""
    use core::num::traits::OverflowingSub;

    """]

    [[completions]]
    completion_label = "format!"
    insert_text = 'format!("$1")'
    "#);
}

#[test]
fn disallow_nested_recursive_definition() {
    test_transform_plain!(Completion, completion_fixture(), "
    fn a(param: felt252, param2: felt252, param3: felt252){
        let foo_bar_baz = {
            let b = foo_bar_b<caret>;
        };
    }
    ",@r#"
    caret = """
            let b = foo_bar_b<caret>;
    """
    completions = []
    "#);
}

#[test]
fn work_with_params() {
    test_transform_plain!(Completion, completion_fixture(), "
    // funny names so there is no corelib completion in test
    fn a(paxram: felt252, paxram2: felt252, paxram3: felt252){
        paxr<caret>
    }
    ",@r#"
    caret = """
        paxr<caret>
    """

    [[completions]]
    completion_label = "a"
    insert_text = "a(${1:paxram}, ${2:paxram2}, ${3:paxram3})"

    [[completions]]
    completion_label = "max"
    insert_text = "max(${1:a}, ${2:b})"
    text_edits = ["""
    use core::cmp::max;

    """]

    [[completions]]
    completion_label = "panic"
    insert_text = "panic(${1:data})"

    [[completions]]
    completion_label = "panic!"
    insert_text = 'panic!("$1")'

    [[completions]]
    completion_label = "paxram"
    detail = "felt252"

    [[completions]]
    completion_label = "paxram2"
    detail = "felt252"

    [[completions]]
    completion_label = "paxram3"
    detail = "felt252"
    "#);
}

#[test]
fn mixed_params_vars() {
    test_transform_plain!(Completion, completion_fixture(), "
    fn a(bar: felt252, param2: felt252, param3: felt252){
        let baz = 1;
        ba<caret>
    }
    ",@r#"
    caret = """
        ba<caret>
    """

    [[completions]]
    completion_label = "bar"
    detail = "felt252"

    [[completions]]
    completion_label = "baz"
    detail = "felt252"

    [[completions]]
    completion_label = "blake"
    text_edits = ["""
    use core::blake;

    """]

    [[completions]]
    completion_label = "blake2s_compress"
    insert_text = "blake2s_compress(${1:state}, ${2:byte_count}, ${3:msg})"
    text_edits = ["""
    use core::blake::blake2s_compress;

    """]

    [[completions]]
    completion_label = "blake2s_finalize"
    insert_text = "blake2s_finalize(${1:state}, ${2:byte_count}, ${3:msg})"
    text_edits = ["""
    use core::blake::blake2s_finalize;

    """]

    [[completions]]
    completion_label = "byte_array"
    text_edits = ["""
    use core::byte_array;

    """]

    [[completions]]
    completion_label = "library_call_syscall"
    insert_text = "library_call_syscall(${1:class_hash}, ${2:function_selector}, ${3:calldata})"
    text_edits = ["""
    use starknet::syscalls::library_call_syscall;

    """]
    "#);
}

#[test]
fn ignores_from_macros() {
    test_transform_plain!(Completion, completion_fixture(), "
    fn a(param: felt252, param2: felt252, param3: felt252){
        // this generates variable __array_builder_macro_result__ in nested block
        array![1_felt252];
        let foo2 = 1;

        _<caret>
    }
    ",@r#"
    caret = """
        _<caret>
    """
    completions = []
    "#);
}

#[test]
fn ignores_from_blocks() {
    test_transform_plain!(Completion, completion_fixture(), "
    fn a(param: felt252, param2: felt252, param3: felt252){
        {
            let bbb = 1234;
        }
        let foo2 = 1;

        bb<caret>
    }
    ",@r#"
    caret = """
        bb<caret>
    """
    completions = []
    "#);
}

#[test]
fn works_in_same_block() {
    test_transform_plain!(Completion, completion_fixture(), "
    fn a(param: felt252, param2: felt252, param3: felt252){
        {

            let bbb = 1234;

            let foo2 = 1;

            bb<caret>
        }
    }
    ",@r#"
    caret = """
            bb<caret>
    """

    [[completions]]
    completion_label = "bbb"
    detail = "felt252"
    "#);
}

#[test]
fn works_usage_in_block() {
    test_transform_plain!(Completion, completion_fixture(), "
    fn a(param: felt252, param2: felt252, param3: felt252){
        {

            let bbb = 1234;

            let foo2 = 1;

            {
                bb<caret>
            }
        }
    }
    ",@r#"
    caret = """
                bb<caret>
    """

    [[completions]]
    completion_label = "bbb"
    detail = "felt252"
    "#);
}
