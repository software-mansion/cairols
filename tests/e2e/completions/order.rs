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
    completion_label = "Add"

    [[completions]]
    completion_label = "AddAssign"
    completion_label_path = "AddAssign"

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
    completion_label = "AddAssignImpl"

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
    completion_label = "Add"

    [[completions]]
    completion_label = "AddAssign"
    completion_label_path = "AddAssign"

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
    completion_label = "AddAssignImpl"

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
