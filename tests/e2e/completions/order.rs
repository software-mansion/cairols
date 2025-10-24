use lsp_types::request::Completion;

use crate::{
    completions::completion_fixture_with_pub_dep_items, support::insta::test_transform_plain,
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
    completion_label_path = "AddAssign"

    [[completions]]
    completion_label = "AddAssignImpl"

    [[completions]]
    completion_label = "Add"

    [[completions]]
    completion_label = "AddAssign"
    completion_label_path = "sub_module::AddAssign"
    text_edits = ["""
    use sub_module::AddAssign;

    """]

    [[completions]]
    completion_label = "AddAssign"
    completion_label_path = "dep::AddAssign"
    text_edits = ["""
    use dep::AddAssign;

    """]

    [[completions]]
    completion_label = "AddAssign"
    completion_label_path = "core::ops::AddAssign"
    text_edits = ["""
    use core::ops::AddAssign;

    """]

    [[completions]]
    completion_label = "AddEq"
    text_edits = ["""
    use core::traits::AddEq;

    """]

    [[completions]]
    completion_label = "AddInputResult"
    text_edits = ["""
    use core::circuit::AddInputResult;

    """]

    [[completions]]
    completion_label = "AddInputResultImpl"
    text_edits = ["""
    use core::circuit::AddInputResultImpl;

    """]

    [[completions]]
    completion_label = "AddMod"
    text_edits = ["""
    use core::circuit::AddMod;

    """]

    [[completions]]
    completion_label = "DivAssign"
    text_edits = ["""
    use core::ops::DivAssign;

    """]

    [[completions]]
    completion_label = "EthAddress"
    text_edits = ["""
    use starknet::EthAddress;

    """]

    [[completions]]
    completion_label = "MulAssign"
    text_edits = ["""
    use core::ops::MulAssign;

    """]

    [[completions]]
    completion_label = "RemAssign"
    text_edits = ["""
    use core::ops::RemAssign;

    """]

    [[completions]]
    completion_label = "SubAssign"
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
    completion_label_path = "AddAssign"

    [[completions]]
    completion_label = "AddAssignImpl"

    [[completions]]
    completion_label = "Add"

    [[completions]]
    completion_label = "AddAssign"
    completion_label_path = "sub_module::AddAssign"
    text_edits = ["""
    use sub_module::AddAssign;

    """]

    [[completions]]
    completion_label = "AddAssign"
    completion_label_path = "dep::AddAssign"
    text_edits = ["""
    use dep::AddAssign;

    """]

    [[completions]]
    completion_label = "AddAssign"
    completion_label_path = "core::ops::AddAssign"
    text_edits = ["""
    use core::ops::AddAssign;

    """]

    [[completions]]
    completion_label = "AddEq"
    text_edits = ["""
    use core::traits::AddEq;

    """]

    [[completions]]
    completion_label = "AddInputResult"
    text_edits = ["""
    use core::circuit::AddInputResult;

    """]

    [[completions]]
    completion_label = "AddInputResultImpl"
    text_edits = ["""
    use core::circuit::AddInputResultImpl;

    """]

    [[completions]]
    completion_label = "AddMod"
    text_edits = ["""
    use core::circuit::AddMod;

    """]

    [[completions]]
    completion_label = "DivAssign"
    text_edits = ["""
    use core::ops::DivAssign;

    """]

    [[completions]]
    completion_label = "EthAddress"
    text_edits = ["""
    use starknet::EthAddress;

    """]

    [[completions]]
    completion_label = "MulAssign"
    text_edits = ["""
    use core::ops::MulAssign;

    """]

    [[completions]]
    completion_label = "RemAssign"
    text_edits = ["""
    use core::ops::RemAssign;

    """]

    [[completions]]
    completion_label = "SubAssign"
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
    completion_label_path = "ResultTraitImpl"

    [[completions]]
    completion_label = "Result"

    [[completions]]
    completion_label = "ResultTrait"

    [[completions]]
    completion_label = "ResultTraitCustom"
    text_edits = ["""
    use dep::ResultTraitCustom;

    """]

    [[completions]]
    completion_label = "AddInputResultTrait"
    text_edits = ["""
    use core::circuit::AddInputResultTrait;

    """]

    [[completions]]
    completion_label = "LoopResult"
    text_edits = ["""
    use core::internal::LoopResult;

    """]

    [[completions]]
    completion_label = "RangeTrait"
    text_edits = ["""
    use core::ops::RangeTrait;

    """]

    [[completions]]
    completion_label = "ResourceBounds"
    text_edits = ["""
    use starknet::ResourceBounds;

    """]

    [[completions]]
    completion_label = "ResultTraitImpl"
    completion_label_path = "core::result::ResultTraitImpl"
    text_edits = ["""
    use core::result::ResultTraitImpl;

    """]

    [[completions]]
    completion_label = "result"
    text_edits = ["""
    use core::result;

    """]
    "#);
}
