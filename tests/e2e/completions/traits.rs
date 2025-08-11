use crate::{
    completions::completion_fixture,
    support::insta::{test_transform_plain, test_transform_with_macros},
};
use lsp_types::request::Completion;

#[test]
fn self_completions() {
    test_transform_plain!(Completion, completion_fixture(), "
    trait Foo {
        fn bar() {
            Self::<caret>
        }
    }
    ",@r#"
    caret = """
            Self::<caret>
    """

    [[completions]]
    completion_label = "bar"
    "#);
}

#[test]
fn self_completions_macro() {
    test_transform_with_macros!(Completion, completion_fixture(), "
    #[complex_attribute_macro_v2]
    trait Foo {
        fn bar() {
            Self::<caret>
        }
    }
    ",@r#"
    caret = """
            Self::<caret>
    """

    [[completions]]
    completion_label = "bar"
    "#);
}

#[test]
fn type_bound() {
    test_transform_plain!(Completion, completion_fixture(), r#"
    fn foo<T, +D<caret>>() {}
    "#, @r#"
    caret = """
    fn foo<T, +D<caret>>() {}
    """

    [[completions]]
    completion_label = "Debug"
    text_edits = ["""
    use core::fmt::Debug;

    """]

    [[completions]]
    completion_label = "DebugImpl"
    text_edits = ["""
    use core::fmt::into_felt252_based::DebugImpl;

    """]

    [[completions]]
    completion_label = "Default"

    [[completions]]
    completion_label = "DeploymentParams"
    text_edits = ["""
    use starknet::deployment::DeploymentParams;

    """]

    [[completions]]
    completion_label = "Deref"

    [[completions]]
    completion_label = "DerefMut"
    text_edits = ["""
    use core::ops::DerefMut;

    """]

    [[completions]]
    completion_label = "Destruct"

    [[completions]]
    completion_label = "DestructFailureGuarantee"
    text_edits = ["""
    use core::circuit::DestructFailureGuarantee;

    """]

    [[completions]]
    completion_label = "DestructOption"
    text_edits = ["""
    use core::option::DestructOption;

    """]

    [[completions]]
    completion_label = "DestructWith"
    text_edits = ["""
    use core::internal::DestructWith;

    """]

    [[completions]]
    completion_label = "Display"
    text_edits = ["""
    use core::fmt::Display;

    """]

    [[completions]]
    completion_label = "Div"

    [[completions]]
    completion_label = "DivAssign"
    text_edits = ["""
    use core::ops::DivAssign;

    """]

    [[completions]]
    completion_label = "DivEq"
    text_edits = ["""
    use core::traits::DivEq;

    """]

    [[completions]]
    completion_label = "DivRem"
    completion_label_path = "DivRem"

    [[completions]]
    completion_label = "DivRem"
    completion_label_path = "core::num::traits::DivRem"
    text_edits = ["""
    use core::num::traits::DivRem;

    """]

    [[completions]]
    completion_label = "DivRemHelper"
    text_edits = ["""
    use core::internal::bounded_int::DivRemHelper;

    """]

    [[completions]]
    completion_label = "Done"
    text_edits = ["""
    use core::circuit::AddInputResult::Done;

    """]

    [[completions]]
    completion_label = "Drop"

    [[completions]]
    completion_label = "DropWith"
    text_edits = ["""
    use core::internal::DropWith;

    """]
    "#);
}

#[test]
fn negative_type_bound() {
    test_transform_plain!(Completion, completion_fixture(), r#"
    trait Trait<T> {}
    impl Impl<T, -D<caret>> of Trait<T> {}
    "#, @r#"
    caret = """
    impl Impl<T, -D<caret>> of Trait<T> {}
    """

    [[completions]]
    completion_label = "Debug"
    text_edits = ["""
    use core::fmt::Debug;

    """]

    [[completions]]
    completion_label = "DebugImpl"
    text_edits = ["""
    use core::fmt::into_felt252_based::DebugImpl;

    """]

    [[completions]]
    completion_label = "Default"

    [[completions]]
    completion_label = "DeploymentParams"
    text_edits = ["""
    use starknet::deployment::DeploymentParams;

    """]

    [[completions]]
    completion_label = "Deref"

    [[completions]]
    completion_label = "DerefMut"
    text_edits = ["""
    use core::ops::DerefMut;

    """]

    [[completions]]
    completion_label = "Destruct"

    [[completions]]
    completion_label = "DestructFailureGuarantee"
    text_edits = ["""
    use core::circuit::DestructFailureGuarantee;

    """]

    [[completions]]
    completion_label = "DestructOption"
    text_edits = ["""
    use core::option::DestructOption;

    """]

    [[completions]]
    completion_label = "DestructWith"
    text_edits = ["""
    use core::internal::DestructWith;

    """]

    [[completions]]
    completion_label = "Display"
    text_edits = ["""
    use core::fmt::Display;

    """]

    [[completions]]
    completion_label = "Div"

    [[completions]]
    completion_label = "DivAssign"
    text_edits = ["""
    use core::ops::DivAssign;

    """]

    [[completions]]
    completion_label = "DivEq"
    text_edits = ["""
    use core::traits::DivEq;

    """]

    [[completions]]
    completion_label = "DivRem"
    completion_label_path = "DivRem"

    [[completions]]
    completion_label = "DivRem"
    completion_label_path = "core::num::traits::DivRem"
    text_edits = ["""
    use core::num::traits::DivRem;

    """]

    [[completions]]
    completion_label = "DivRemHelper"
    text_edits = ["""
    use core::internal::bounded_int::DivRemHelper;

    """]

    [[completions]]
    completion_label = "Done"
    text_edits = ["""
    use core::circuit::AddInputResult::Done;

    """]

    [[completions]]
    completion_label = "Drop"

    [[completions]]
    completion_label = "DropWith"
    text_edits = ["""
    use core::internal::DropWith;

    """]
    "#);
}

#[test]
fn impl_bound() {
    test_transform_plain!(Completion, completion_fixture(), r#"
    fn foo<T, +D<caret>>() {}
    "#, @r#"
    caret = """
    fn foo<T, +D<caret>>() {}
    """

    [[completions]]
    completion_label = "Debug"
    text_edits = ["""
    use core::fmt::Debug;

    """]

    [[completions]]
    completion_label = "DebugImpl"
    text_edits = ["""
    use core::fmt::into_felt252_based::DebugImpl;

    """]

    [[completions]]
    completion_label = "Default"

    [[completions]]
    completion_label = "DeploymentParams"
    text_edits = ["""
    use starknet::deployment::DeploymentParams;

    """]

    [[completions]]
    completion_label = "Deref"

    [[completions]]
    completion_label = "DerefMut"
    text_edits = ["""
    use core::ops::DerefMut;

    """]

    [[completions]]
    completion_label = "Destruct"

    [[completions]]
    completion_label = "DestructFailureGuarantee"
    text_edits = ["""
    use core::circuit::DestructFailureGuarantee;

    """]

    [[completions]]
    completion_label = "DestructOption"
    text_edits = ["""
    use core::option::DestructOption;

    """]

    [[completions]]
    completion_label = "DestructWith"
    text_edits = ["""
    use core::internal::DestructWith;

    """]

    [[completions]]
    completion_label = "Display"
    text_edits = ["""
    use core::fmt::Display;

    """]

    [[completions]]
    completion_label = "Div"

    [[completions]]
    completion_label = "DivAssign"
    text_edits = ["""
    use core::ops::DivAssign;

    """]

    [[completions]]
    completion_label = "DivEq"
    text_edits = ["""
    use core::traits::DivEq;

    """]

    [[completions]]
    completion_label = "DivRem"
    completion_label_path = "DivRem"

    [[completions]]
    completion_label = "DivRem"
    completion_label_path = "core::num::traits::DivRem"
    text_edits = ["""
    use core::num::traits::DivRem;

    """]

    [[completions]]
    completion_label = "DivRemHelper"
    text_edits = ["""
    use core::internal::bounded_int::DivRemHelper;

    """]

    [[completions]]
    completion_label = "Done"
    text_edits = ["""
    use core::circuit::AddInputResult::Done;

    """]

    [[completions]]
    completion_label = "Drop"

    [[completions]]
    completion_label = "DropWith"
    text_edits = ["""
    use core::internal::DropWith;

    """]
    "#);
}

#[test]
fn type_bound_user_trait() {
    test_transform_plain!(Completion, completion_fixture(), r#"
    trait Traicik<T> {}
    fn foo<T, +Trai<caret>>() {}
    "#, @r#"
    caret = """
    fn foo<T, +Trai<caret>>() {}
    """

    [[completions]]
    completion_label = "BoxTrait"

    [[completions]]
    completion_label = "ResultTraitImpl"
    text_edits = ["""
    use core::result::ResultTraitImpl;

    """]

    [[completions]]
    completion_label = "Traicik"

    [[completions]]
    completion_label = "TrimMaxHelper"
    text_edits = ["""
    use core::internal::bounded_int::TrimMaxHelper;

    """]

    [[completions]]
    completion_label = "TrimMinHelper"
    text_edits = ["""
    use core::internal::bounded_int::TrimMinHelper;

    """]

    [[completions]]
    completion_label = "True"
    text_edits = ["""
    use bool::True;

    """]

    [[completions]]
    completion_label = "VecTrait"
    text_edits = ["""
    use starknet::storage::VecTrait;

    """]

    [[completions]]
    completion_label = "traits"
    completion_label_path = "core::traits"
    text_edits = ["""
    use core::traits;

    """]

    [[completions]]
    completion_label = "traits"
    completion_label_path = "core::num::traits"
    text_edits = ["""
    use core::num::traits;

    """]

    [[completions]]
    completion_label = "wrapping"
    text_edits = ["""
    use core::num::traits::ops::wrapping;

    """]
    "#);
}
