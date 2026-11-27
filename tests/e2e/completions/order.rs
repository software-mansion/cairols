use lsp_types::request::Completion;

use crate::{
    completions::{completion_fixture_with_dep_methods, completion_fixture_with_pub_dep_items},
    support::insta::test_transform_plain,
};

#[test]
fn order_with_current_crate_and_core_deps() {
    test_transform_plain!(Completion, completion_fixture_with_pub_dep_items(), "
    mod sub_module {
        pub trait AddAssign {
            fn add_assign() -> felt252;
        }
    }

    trait AddAssign {
        fn add_assign() -> felt252;
    }

    impl AddAssignImpl of AddAssig<caret>
    ",@r#"
    caret = """
    impl AddAssignImpl of AddAssig<caret>
    """

    [[completions]]
    completion_label = "AddAssign"

    [[completions]]
    completion_label = "AddAssignImpl"

    [[completions]]
    completion_label = "Add"

    [[completions]]
    completion_label = "AddAssign"
    completion_label_path = "(use sub_module::AddAssign)"
    text_edits = ["""
    use sub_module::AddAssign;

    """]

    [[completions]]
    completion_label = "AddAssign"
    completion_label_path = "(use dep::AddAssign)"
    text_edits = ["""
    use dep::AddAssign;

    """]

    [[completions]]
    completion_label = "AddAssign"
    completion_label_path = "(use core::ops::AddAssign)"
    text_edits = ["""
    use core::ops::AddAssign;

    """]

    [[completions]]
    completion_label = "AddEq"
    completion_label_path = "(use core::traits::AddEq)"
    text_edits = ["""
    use core::traits::AddEq;

    """]

    [[completions]]
    completion_label = "AddInputResult"
    completion_label_path = "(use core::circuit::AddInputResult)"
    text_edits = ["""
    use core::circuit::AddInputResult;

    """]

    [[completions]]
    completion_label = "AddInputResultImpl"
    completion_label_path = "(use core::circuit::AddInputResultImpl)"
    text_edits = ["""
    use core::circuit::AddInputResultImpl;

    """]

    [[completions]]
    completion_label = "AddMod"
    completion_label_path = "(use core::circuit::AddMod)"
    text_edits = ["""
    use core::circuit::AddMod;

    """]

    [[completions]]
    completion_label = "DivAssign"
    completion_label_path = "(use core::ops::DivAssign)"
    text_edits = ["""
    use core::ops::DivAssign;

    """]

    [[completions]]
    completion_label = "EthAddress"
    completion_label_path = "(use starknet::EthAddress)"
    text_edits = ["""
    use starknet::EthAddress;

    """]

    [[completions]]
    completion_label = "MulAssign"
    completion_label_path = "(use core::ops::MulAssign)"
    text_edits = ["""
    use core::ops::MulAssign;

    """]

    [[completions]]
    completion_label = "RemAssign"
    completion_label_path = "(use core::ops::RemAssign)"
    text_edits = ["""
    use core::ops::RemAssign;

    """]

    [[completions]]
    completion_label = "SubAssign"
    completion_label_path = "(use core::ops::SubAssign)"
    text_edits = ["""
    use core::ops::SubAssign;

    """]
    "#);
}

#[test]
fn order_with_current_crate_and_core_deps_macros() {
    test_transform_plain!(Completion, completion_fixture_with_pub_dep_items(), "
    #[complex_attribute_macro_v2]
    mod sub_module {
        pub trait AddAssign {
            fn add_assign() -> felt252;
        }
    }

    #[complex_attribute_macro_v2]
    trait AddAssign {
        fn add_assign() -> felt252;
    }

    impl AddAssignImpl of AddAssig<caret>
    ",@r#"
    caret = """
    impl AddAssignImpl of AddAssig<caret>
    """

    [[completions]]
    completion_label = "AddAssign"

    [[completions]]
    completion_label = "AddAssignImpl"

    [[completions]]
    completion_label = "Add"

    [[completions]]
    completion_label = "AddAssign"
    completion_label_path = "(use sub_module::AddAssign)"
    text_edits = ["""
    use sub_module::AddAssign;

    """]

    [[completions]]
    completion_label = "AddAssign"
    completion_label_path = "(use dep::AddAssign)"
    text_edits = ["""
    use dep::AddAssign;

    """]

    [[completions]]
    completion_label = "AddAssign"
    completion_label_path = "(use core::ops::AddAssign)"
    text_edits = ["""
    use core::ops::AddAssign;

    """]

    [[completions]]
    completion_label = "AddEq"
    completion_label_path = "(use core::traits::AddEq)"
    text_edits = ["""
    use core::traits::AddEq;

    """]

    [[completions]]
    completion_label = "AddInputResult"
    completion_label_path = "(use core::circuit::AddInputResult)"
    text_edits = ["""
    use core::circuit::AddInputResult;

    """]

    [[completions]]
    completion_label = "AddInputResultImpl"
    completion_label_path = "(use core::circuit::AddInputResultImpl)"
    text_edits = ["""
    use core::circuit::AddInputResultImpl;

    """]

    [[completions]]
    completion_label = "AddMod"
    completion_label_path = "(use core::circuit::AddMod)"
    text_edits = ["""
    use core::circuit::AddMod;

    """]

    [[completions]]
    completion_label = "DivAssign"
    completion_label_path = "(use core::ops::DivAssign)"
    text_edits = ["""
    use core::ops::DivAssign;

    """]

    [[completions]]
    completion_label = "EthAddress"
    completion_label_path = "(use starknet::EthAddress)"
    text_edits = ["""
    use starknet::EthAddress;

    """]

    [[completions]]
    completion_label = "MulAssign"
    completion_label_path = "(use core::ops::MulAssign)"
    text_edits = ["""
    use core::ops::MulAssign;

    """]

    [[completions]]
    completion_label = "RemAssign"
    completion_label_path = "(use core::ops::RemAssign)"
    text_edits = ["""
    use core::ops::RemAssign;

    """]

    [[completions]]
    completion_label = "SubAssign"
    completion_label_path = "(use core::ops::SubAssign)"
    text_edits = ["""
    use core::ops::SubAssign;

    """]
    "#);
}

#[test]
fn prelude_ordering() {
    test_transform_plain!(Completion, completion_fixture_with_pub_dep_items(), "
    #[derive(Copy, Drop, Debug, Serde, PartialEq)]
    trait ResultTraitCustom2 {}

    impl ResultTraitImpl of ResultTrai<caret>
    ",@r#"
    caret = """
    impl ResultTraitImpl of ResultTrai<caret>
    """

    [[completions]]
    completion_label = "ResultTraitCustom2"

    [[completions]]
    completion_label = "ResultTraitImpl"

    [[completions]]
    completion_label = "Result"

    [[completions]]
    completion_label = "ResultTrait"

    [[completions]]
    completion_label = "ResultTraitCustom"
    completion_label_path = "(use dep::ResultTraitCustom)"
    text_edits = ["""
    use dep::ResultTraitCustom;

    """]

    [[completions]]
    completion_label = "AddInputResultTrait"
    completion_label_path = "(use core::circuit::AddInputResultTrait)"
    text_edits = ["""
    use core::circuit::AddInputResultTrait;

    """]

    [[completions]]
    completion_label = "LoopResult"
    completion_label_path = "(use core::internal::LoopResult)"
    text_edits = ["""
    use core::internal::LoopResult;

    """]

    [[completions]]
    completion_label = "RangeTrait"
    completion_label_path = "(use core::ops::RangeTrait)"
    text_edits = ["""
    use core::ops::RangeTrait;

    """]

    [[completions]]
    completion_label = "ResourceBounds"
    completion_label_path = "(use starknet::ResourceBounds)"
    text_edits = ["""
    use starknet::ResourceBounds;

    """]

    [[completions]]
    completion_label = "ResultTraitImpl"
    completion_label_path = "(use core::result::ResultTraitImpl)"
    text_edits = ["""
    use core::result::ResultTraitImpl;

    """]

    [[completions]]
    completion_label = "result"
    completion_label_path = "(use core::result)"
    text_edits = ["""
    use core::result;

    """]
    "#);
}

#[test]
fn methods_order_with_scope_crate_deps_and_core() {
    test_transform_plain!(Completion, completion_fixture_with_dep_methods(), "
    mod scoped {
        pub trait ScopedTrait<T> {
            fn scoped_method(self: T) -> felt252;
        }

        impl ScopedImpl<T, +Drop<T>> of ScopedTrait<T> {
            fn scoped_method(self: T) -> felt252 {
                0
            }
        }
    }

    mod not_scoped {
        pub trait CrateTrait<T> {
            fn crate_method(self: T) -> felt252;
        }

        impl CrateImpl<T, +Drop<T>> of CrateTrait<T> {
            fn crate_method(self: T) -> felt252 {
                0
            }
        }
    }

    use scoped::ScopedTrait;

    #[derive(Drop)]
    struct Foo {
        member: felt252
    }

    fn test() {
        let foo = Foo { member: 0 };
        foo.<caret>
    }
    ",@r#"
    caret = """
        foo.<caret>
    """

    [[completions]]
    completion_label = "member"
    completion_label_type_info = "felt252"

    [[completions]]
    completion_label = "destruct()"
    completion_label_type_info = "fn(self: T) -> () nopanic"
    insert_text = "destruct()"

    [[completions]]
    completion_label = "into()"
    completion_label_type_info = "fn(self: T) -> S"
    insert_text = "into()"

    [[completions]]
    completion_label = "panic_destruct()"
    completion_label_type_info = "fn(self: T, ref panic: Panic) -> () nopanic"
    insert_text = "panic_destruct(${1:panic})"

    [[completions]]
    completion_label = "scoped_method()"
    completion_label_type_info = "fn(self: T) -> felt252"
    insert_text = "scoped_method()"

    [[completions]]
    completion_label = "try_into()"
    completion_label_type_info = "fn(self: T) -> Option<S>"
    insert_text = "try_into()"

    [[completions]]
    completion_label = "crate_method()"
    completion_label_type_info = "fn(self: T) -> felt252"
    insert_text = "crate_method()"
    text_edits = ["""
    use not_scoped::CrateTrait;
    """]

    [[completions]]
    completion_label = "dep_method()"
    completion_label_type_info = "fn(self: T) -> felt252"
    insert_text = "dep_method()"
    text_edits = ["""
    use dep::DepTrait;
    """]

    [[completions]]
    completion_label = "get_descriptor()"
    completion_label_type_info = "fn(self: CES) -> CircuitDescriptor<CD::CircuitType>"
    insert_text = "get_descriptor()"

    [[completions]]
    completion_label = "new_inputs()"
    completion_label_type_info = "fn(self: CES) -> AddInputResult<CD::CircuitType>"
    insert_text = "new_inputs()"
    text_edits = ["""
    use core::circuit::CircuitInputs;
    """]
    "#);
}
