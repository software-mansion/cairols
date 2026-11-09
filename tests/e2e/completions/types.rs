use lsp_types::request::Completion;

use crate::{completions::completion_fixture, support::insta::test_transform_plain};

#[test]
fn let_type_annotation_empty() {
    test_transform_plain!(Completion, completion_fixture(), "
    fn a() {
        let a: <caret> = 123;
    }
    ",@r#"
    caret = """
        let a: <caret> = 123;
    """
    completions = []
    "#);
}

#[test]
fn let_type_annotation_partial() {
    test_transform_plain!(Completion, completion_fixture(), "
    struct FooStruct {}
    const FooConst: u32 = 0;
    fn FooFn() {}
    trait FooTrait {}

    fn a() {
        let a: Foo<caret> = 123;
    }
    ",@r#"
    caret = """
        let a: Foo<caret> = 123;
    """

    [[completions]]
    completion_label = "FooConst"

    [[completions]]
    completion_label = "FooFn(...)"
    completion_label_path = "(use FooFn)"
    completion_label_type_info = "fn() -> ()"
    insert_text = "FooFn()"

    [[completions]]
    completion_label = "FooStruct"

    [[completions]]
    completion_label = "FooTrait"

    [[completions]]
    completion_label = "bool"

    [[completions]]
    completion_label = "Foo"
    completion_label_path = "(use dep::Foo)"
    text_edits = ["""
    use dep::Foo;

    """]

    [[completions]]
    completion_label = "Felt252TryIntoContractAddress"
    completion_label_path = "(use starknet::contract_address::Felt252TryIntoContractAddress)"
    text_edits = ["""
    use starknet::contract_address::Felt252TryIntoContractAddress;

    """]

    [[completions]]
    completion_label = "Formatter"
    completion_label_path = "(use core::fmt::Formatter)"
    text_edits = ["""
    use core::fmt::Formatter;

    """]
    "#);
}

#[test]
fn let_type_annotation_module_path() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod module {
        pub struct FooStruct {}
        pub const FooConst: u32 = 0;
        pub fn FooFn() {}
        pub trait FooTrait {}
        pub mod nested {}
    }

    fn a() {
        let a: module::<caret> = 123;
    }
    ",@r#"
    caret = """
        let a: module::<caret> = 123;
    """

    [[completions]]
    completion_label = "FooConst"
    completion_label_path = "(use module::FooConst)"

    [[completions]]
    completion_label = "FooFn(...)"
    completion_label_path = "(use module::FooFn)"
    completion_label_type_info = "fn() -> ()"
    insert_text = "FooFn()"

    [[completions]]
    completion_label = "FooStruct"
    completion_label_path = "(use module::FooStruct)"

    [[completions]]
    completion_label = "FooTrait"
    completion_label_path = "(use module::FooTrait)"

    [[completions]]
    completion_label = "nested"
    completion_label_path = "(use module::nested)"
    "#);
}

#[test]
fn return_type_partial() {
    test_transform_plain!(Completion, completion_fixture(), "
    struct FooStruct {}
    const FooConst: u32 = 0;
    fn FooFn() {}
    trait FooTrait {}

    fn a() -> Foo<caret> {}
    ",@r#"
    caret = """
    fn a() -> Foo<caret> {}
    """

    [[completions]]
    completion_label = "FooConst"

    [[completions]]
    completion_label = "FooFn(...)"
    completion_label_path = "(use FooFn)"
    completion_label_type_info = "fn() -> ()"
    insert_text = "FooFn()"

    [[completions]]
    completion_label = "FooStruct"

    [[completions]]
    completion_label = "FooTrait"

    [[completions]]
    completion_label = "bool"

    [[completions]]
    completion_label = "Foo"
    completion_label_path = "(use dep::Foo)"
    text_edits = ["""
    use dep::Foo;

    """]

    [[completions]]
    completion_label = "Felt252TryIntoContractAddress"
    completion_label_path = "(use starknet::contract_address::Felt252TryIntoContractAddress)"
    text_edits = ["""
    use starknet::contract_address::Felt252TryIntoContractAddress;

    """]

    [[completions]]
    completion_label = "Formatter"
    completion_label_path = "(use core::fmt::Formatter)"
    text_edits = ["""
    use core::fmt::Formatter;

    """]
    "#);
}
