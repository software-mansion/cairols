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
    completion_label_type_info = "felt252"

    [[completions]]
    completion_label = "baz"
    completion_label_type_info = "felt252"

    [[completions]]
    completion_label = "blake"
    completion_label_path = "(use core::blake)"
    text_edits = ["""
    use core::blake;

    """]

    [[completions]]
    completion_label = "blake2s_compress(...)"
    completion_label_path = "(use core::blake::blake2s_compress)"
    completion_label_type_info = "fn(state: Box<[u32; 8]>, byte_count: u32, msg: Box<[u32; 16]>) -> Box<[u32; 8]> nopanic"
    insert_text = "blake2s_compress(${1:state}, ${2:byte_count}, ${3:msg})"
    text_edits = ["""
    use core::blake::blake2s_compress;

    """]

    [[completions]]
    completion_label = "blake2s_finalize(...)"
    completion_label_path = "(use core::blake::blake2s_finalize)"
    completion_label_type_info = "fn(state: Box<[u32; 8]>, byte_count: u32, msg: Box<[u32; 16]>) -> Box<[u32; 8]> nopanic"
    insert_text = "blake2s_finalize(${1:state}, ${2:byte_count}, ${3:msg})"
    text_edits = ["""
    use core::blake::blake2s_finalize;

    """]

    [[completions]]
    completion_label = "byte_array"
    completion_label_path = "(use core::byte_array)"
    text_edits = ["""
    use core::byte_array;

    """]

    [[completions]]
    completion_label = "library_call_syscall(...)"
    completion_label_path = "(use starknet::syscalls::library_call_syscall)"
    completion_label_type_info = "fn(class_hash: ClassHash, function_selector: felt252, calldata: Span<felt252>) -> Result<Span<felt252>, Array<felt252>> nopanic"
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
    completion_label_type_info = "felt252"

    [[completions]]
    completion_label = "baz"
    completion_label_type_info = "felt252"

    [[completions]]
    completion_label = "blake"
    completion_label_path = "(use core::blake)"
    text_edits = ["""
    use core::blake;

    """]

    [[completions]]
    completion_label = "blake2s_compress(...)"
    completion_label_path = "(use core::blake::blake2s_compress)"
    completion_label_type_info = "fn(state: Box<[u32; 8]>, byte_count: u32, msg: Box<[u32; 16]>) -> Box<[u32; 8]> nopanic"
    insert_text = "blake2s_compress(${1:state}, ${2:byte_count}, ${3:msg})"
    text_edits = ["""
    use core::blake::blake2s_compress;

    """]

    [[completions]]
    completion_label = "blake2s_finalize(...)"
    completion_label_path = "(use core::blake::blake2s_finalize)"
    completion_label_type_info = "fn(state: Box<[u32; 8]>, byte_count: u32, msg: Box<[u32; 16]>) -> Box<[u32; 8]> nopanic"
    insert_text = "blake2s_finalize(${1:state}, ${2:byte_count}, ${3:msg})"
    text_edits = ["""
    use core::blake::blake2s_finalize;

    """]

    [[completions]]
    completion_label = "byte_array"
    completion_label_path = "(use core::byte_array)"
    text_edits = ["""
    use core::byte_array;

    """]

    [[completions]]
    completion_label = "library_call_syscall(...)"
    completion_label_path = "(use starknet::syscalls::library_call_syscall)"
    completion_label_type_info = "fn(class_hash: ClassHash, function_selector: felt252, calldata: Span<felt252>) -> Result<Span<felt252>, Array<felt252>> nopanic"
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
    completion_label_type_info = "felt252"

    [[completions]]
    completion_label = "blake"
    completion_label_path = "(use core::blake)"
    text_edits = ["""
    use core::blake;

    """]

    [[completions]]
    completion_label = "blake2s_compress(...)"
    completion_label_path = "(use core::blake::blake2s_compress)"
    completion_label_type_info = "fn(state: Box<[u32; 8]>, byte_count: u32, msg: Box<[u32; 16]>) -> Box<[u32; 8]> nopanic"
    insert_text = "blake2s_compress(${1:state}, ${2:byte_count}, ${3:msg})"
    text_edits = ["""
    use core::blake::blake2s_compress;

    """]

    [[completions]]
    completion_label = "blake2s_finalize(...)"
    completion_label_path = "(use core::blake::blake2s_finalize)"
    completion_label_type_info = "fn(state: Box<[u32; 8]>, byte_count: u32, msg: Box<[u32; 16]>) -> Box<[u32; 8]> nopanic"
    insert_text = "blake2s_finalize(${1:state}, ${2:byte_count}, ${3:msg})"
    text_edits = ["""
    use core::blake::blake2s_finalize;

    """]

    [[completions]]
    completion_label = "byte_array"
    completion_label_path = "(use core::byte_array)"
    text_edits = ["""
    use core::byte_array;

    """]

    [[completions]]
    completion_label = "library_call_syscall(...)"
    completion_label_path = "(use starknet::syscalls::library_call_syscall)"
    completion_label_type_info = "fn(class_hash: ClassHash, function_selector: felt252, calldata: Span<felt252>) -> Result<Span<felt252>, Array<felt252>> nopanic"
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
    completion_label = "format!"
    insert_text = 'format!("$1")'

    [[completions]]
    completion_label = "OverflowingAdd"
    completion_label_path = "(use core::num::traits::OverflowingAdd)"
    text_edits = ["""
    use core::num::traits::OverflowingAdd;

    """]

    [[completions]]
    completion_label = "OverflowingMul"
    completion_label_path = "(use core::num::traits::OverflowingMul)"
    text_edits = ["""
    use core::num::traits::OverflowingMul;

    """]

    [[completions]]
    completion_label = "OverflowingSub"
    completion_label_path = "(use core::num::traits::OverflowingSub)"
    text_edits = ["""
    use core::num::traits::OverflowingSub;

    """]
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
    completion_label = "paxram"
    completion_label_type_info = "felt252"

    [[completions]]
    completion_label = "paxram2"
    completion_label_type_info = "felt252"

    [[completions]]
    completion_label = "paxram3"
    completion_label_type_info = "felt252"

    [[completions]]
    completion_label = "a(...)"
    completion_label_path = "(use a)"
    completion_label_type_info = "fn(paxram: felt252, paxram2: felt252, paxram3: felt252) -> ()"
    insert_text = "a(${1:paxram}, ${2:paxram2}, ${3:paxram3})"

    [[completions]]
    completion_label = "panic!"
    insert_text = 'panic!("$1")'

    [[completions]]
    completion_label = "panic(...)"
    completion_label_path = "(use panic)"
    completion_label_type_info = "fn(data: Array<felt252>) -> crate::never"
    insert_text = "panic(${1:data})"

    [[completions]]
    completion_label = "max(...)"
    completion_label_path = "(use core::cmp::max)"
    completion_label_type_info = "fn(a: T, b: T) -> T"
    insert_text = "max(${1:a}, ${2:b})"
    text_edits = ["""
    use core::cmp::max;

    """]
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
    completion_label_type_info = "felt252"

    [[completions]]
    completion_label = "baz"
    completion_label_type_info = "felt252"

    [[completions]]
    completion_label = "blake"
    completion_label_path = "(use core::blake)"
    text_edits = ["""
    use core::blake;

    """]

    [[completions]]
    completion_label = "blake2s_compress(...)"
    completion_label_path = "(use core::blake::blake2s_compress)"
    completion_label_type_info = "fn(state: Box<[u32; 8]>, byte_count: u32, msg: Box<[u32; 16]>) -> Box<[u32; 8]> nopanic"
    insert_text = "blake2s_compress(${1:state}, ${2:byte_count}, ${3:msg})"
    text_edits = ["""
    use core::blake::blake2s_compress;

    """]

    [[completions]]
    completion_label = "blake2s_finalize(...)"
    completion_label_path = "(use core::blake::blake2s_finalize)"
    completion_label_type_info = "fn(state: Box<[u32; 8]>, byte_count: u32, msg: Box<[u32; 16]>) -> Box<[u32; 8]> nopanic"
    insert_text = "blake2s_finalize(${1:state}, ${2:byte_count}, ${3:msg})"
    text_edits = ["""
    use core::blake::blake2s_finalize;

    """]

    [[completions]]
    completion_label = "byte_array"
    completion_label_path = "(use core::byte_array)"
    text_edits = ["""
    use core::byte_array;

    """]

    [[completions]]
    completion_label = "library_call_syscall(...)"
    completion_label_path = "(use starknet::syscalls::library_call_syscall)"
    completion_label_type_info = "fn(class_hash: ClassHash, function_selector: felt252, calldata: Span<felt252>) -> Result<Span<felt252>, Array<felt252>> nopanic"
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
    completion_label_type_info = "felt252"
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
    completion_label_type_info = "felt252"
    "#);
}
