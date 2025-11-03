use lsp_types::request::Completion;

use crate::{
    completions::completion_fixture,
    support::insta::{test_transform_plain, test_transform_with_macros},
};

#[test]
fn self_completions_trait() {
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
    detail = "fn() -> ()"
    "#);
}

#[test]
fn self_completions_impl() {
    test_transform_plain!(Completion, completion_fixture(), "
    trait AbMaker<T> {
        fn a() -> T;
        fn b(c: T) -> T;
    }

    impl AbMakerFelt252 of AbMaker<felt252> {
        fn a() -> felt252 {
            Self::<caret>
        }
    }
    ",@r#"
    caret = """
            Self::<caret>
    """

    [[completions]]
    completion_label = "a"
    detail = "fn() -> T"

    [[completions]]
    completion_label = "b"
    detail = "fn(c: T) -> T"
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
    detail = "fn() -> ()"
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
    completion_label = "Default"
    completion_label_path = "Default"

    [[completions]]
    completion_label = "Deref"
    completion_label_path = "Deref"

    [[completions]]
    completion_label = "Destruct"
    completion_label_path = "Destruct"

    [[completions]]
    completion_label = "Div"
    completion_label_path = "Div"

    [[completions]]
    completion_label = "DivRem"
    completion_label_path = "DivRem"

    [[completions]]
    completion_label = "Drop"
    completion_label_path = "Drop"

    [[completions]]
    completion_label = "Debug"
    completion_label_path = "core::fmt::Debug"
    text_edits = ["""
    use core::fmt::Debug;

    """]

    [[completions]]
    completion_label = "DebugImpl"
    completion_label_path = "core::fmt::into_felt252_based::DebugImpl"
    text_edits = ["""
    use core::fmt::into_felt252_based::DebugImpl;

    """]

    [[completions]]
    completion_label = "DeploymentParams"
    completion_label_path = "starknet::deployment::DeploymentParams"
    text_edits = ["""
    use starknet::deployment::DeploymentParams;

    """]

    [[completions]]
    completion_label = "DerefMut"
    completion_label_path = "core::ops::DerefMut"
    text_edits = ["""
    use core::ops::DerefMut;

    """]

    [[completions]]
    completion_label = "DestructFailureGuarantee"
    completion_label_path = "core::circuit::DestructFailureGuarantee"
    text_edits = ["""
    use core::circuit::DestructFailureGuarantee;

    """]

    [[completions]]
    completion_label = "DestructOption"
    completion_label_path = "core::option::DestructOption"
    text_edits = ["""
    use core::option::DestructOption;

    """]

    [[completions]]
    completion_label = "DestructWith"
    completion_label_path = "core::internal::DestructWith"
    text_edits = ["""
    use core::internal::DestructWith;

    """]

    [[completions]]
    completion_label = "Display"
    completion_label_path = "core::fmt::Display"
    text_edits = ["""
    use core::fmt::Display;

    """]

    [[completions]]
    completion_label = "DivAssign"
    completion_label_path = "core::ops::DivAssign"
    text_edits = ["""
    use core::ops::DivAssign;

    """]

    [[completions]]
    completion_label = "DivEq"
    completion_label_path = "core::traits::DivEq"
    text_edits = ["""
    use core::traits::DivEq;

    """]

    [[completions]]
    completion_label = "DivRem"
    completion_label_path = "core::num::traits::DivRem"
    text_edits = ["""
    use core::num::traits::DivRem;

    """]

    [[completions]]
    completion_label = "DivRemHelper"
    completion_label_path = "core::internal::bounded_int::DivRemHelper"
    text_edits = ["""
    use core::internal::bounded_int::DivRemHelper;

    """]

    [[completions]]
    completion_label = "Done"
    completion_label_path = "core::circuit::AddInputResult::Done"
    text_edits = ["""
    use core::circuit::AddInputResult::Done;

    """]

    [[completions]]
    completion_label = "DropWith"
    completion_label_path = "core::internal::DropWith"
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
    completion_label = "Default"
    completion_label_path = "Default"

    [[completions]]
    completion_label = "Deref"
    completion_label_path = "Deref"

    [[completions]]
    completion_label = "Destruct"
    completion_label_path = "Destruct"

    [[completions]]
    completion_label = "Div"
    completion_label_path = "Div"

    [[completions]]
    completion_label = "DivRem"
    completion_label_path = "DivRem"

    [[completions]]
    completion_label = "Drop"
    completion_label_path = "Drop"

    [[completions]]
    completion_label = "Debug"
    completion_label_path = "core::fmt::Debug"
    text_edits = ["""
    use core::fmt::Debug;

    """]

    [[completions]]
    completion_label = "DebugImpl"
    completion_label_path = "core::fmt::into_felt252_based::DebugImpl"
    text_edits = ["""
    use core::fmt::into_felt252_based::DebugImpl;

    """]

    [[completions]]
    completion_label = "DeploymentParams"
    completion_label_path = "starknet::deployment::DeploymentParams"
    text_edits = ["""
    use starknet::deployment::DeploymentParams;

    """]

    [[completions]]
    completion_label = "DerefMut"
    completion_label_path = "core::ops::DerefMut"
    text_edits = ["""
    use core::ops::DerefMut;

    """]

    [[completions]]
    completion_label = "DestructFailureGuarantee"
    completion_label_path = "core::circuit::DestructFailureGuarantee"
    text_edits = ["""
    use core::circuit::DestructFailureGuarantee;

    """]

    [[completions]]
    completion_label = "DestructOption"
    completion_label_path = "core::option::DestructOption"
    text_edits = ["""
    use core::option::DestructOption;

    """]

    [[completions]]
    completion_label = "DestructWith"
    completion_label_path = "core::internal::DestructWith"
    text_edits = ["""
    use core::internal::DestructWith;

    """]

    [[completions]]
    completion_label = "Display"
    completion_label_path = "core::fmt::Display"
    text_edits = ["""
    use core::fmt::Display;

    """]

    [[completions]]
    completion_label = "DivAssign"
    completion_label_path = "core::ops::DivAssign"
    text_edits = ["""
    use core::ops::DivAssign;

    """]

    [[completions]]
    completion_label = "DivEq"
    completion_label_path = "core::traits::DivEq"
    text_edits = ["""
    use core::traits::DivEq;

    """]

    [[completions]]
    completion_label = "DivRem"
    completion_label_path = "core::num::traits::DivRem"
    text_edits = ["""
    use core::num::traits::DivRem;

    """]

    [[completions]]
    completion_label = "DivRemHelper"
    completion_label_path = "core::internal::bounded_int::DivRemHelper"
    text_edits = ["""
    use core::internal::bounded_int::DivRemHelper;

    """]

    [[completions]]
    completion_label = "Done"
    completion_label_path = "core::circuit::AddInputResult::Done"
    text_edits = ["""
    use core::circuit::AddInputResult::Done;

    """]

    [[completions]]
    completion_label = "DropWith"
    completion_label_path = "core::internal::DropWith"
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
    completion_label = "Default"
    completion_label_path = "Default"

    [[completions]]
    completion_label = "Deref"
    completion_label_path = "Deref"

    [[completions]]
    completion_label = "Destruct"
    completion_label_path = "Destruct"

    [[completions]]
    completion_label = "Div"
    completion_label_path = "Div"

    [[completions]]
    completion_label = "DivRem"
    completion_label_path = "DivRem"

    [[completions]]
    completion_label = "Drop"
    completion_label_path = "Drop"

    [[completions]]
    completion_label = "Debug"
    completion_label_path = "core::fmt::Debug"
    text_edits = ["""
    use core::fmt::Debug;

    """]

    [[completions]]
    completion_label = "DebugImpl"
    completion_label_path = "core::fmt::into_felt252_based::DebugImpl"
    text_edits = ["""
    use core::fmt::into_felt252_based::DebugImpl;

    """]

    [[completions]]
    completion_label = "DeploymentParams"
    completion_label_path = "starknet::deployment::DeploymentParams"
    text_edits = ["""
    use starknet::deployment::DeploymentParams;

    """]

    [[completions]]
    completion_label = "DerefMut"
    completion_label_path = "core::ops::DerefMut"
    text_edits = ["""
    use core::ops::DerefMut;

    """]

    [[completions]]
    completion_label = "DestructFailureGuarantee"
    completion_label_path = "core::circuit::DestructFailureGuarantee"
    text_edits = ["""
    use core::circuit::DestructFailureGuarantee;

    """]

    [[completions]]
    completion_label = "DestructOption"
    completion_label_path = "core::option::DestructOption"
    text_edits = ["""
    use core::option::DestructOption;

    """]

    [[completions]]
    completion_label = "DestructWith"
    completion_label_path = "core::internal::DestructWith"
    text_edits = ["""
    use core::internal::DestructWith;

    """]

    [[completions]]
    completion_label = "Display"
    completion_label_path = "core::fmt::Display"
    text_edits = ["""
    use core::fmt::Display;

    """]

    [[completions]]
    completion_label = "DivAssign"
    completion_label_path = "core::ops::DivAssign"
    text_edits = ["""
    use core::ops::DivAssign;

    """]

    [[completions]]
    completion_label = "DivEq"
    completion_label_path = "core::traits::DivEq"
    text_edits = ["""
    use core::traits::DivEq;

    """]

    [[completions]]
    completion_label = "DivRem"
    completion_label_path = "core::num::traits::DivRem"
    text_edits = ["""
    use core::num::traits::DivRem;

    """]

    [[completions]]
    completion_label = "DivRemHelper"
    completion_label_path = "core::internal::bounded_int::DivRemHelper"
    text_edits = ["""
    use core::internal::bounded_int::DivRemHelper;

    """]

    [[completions]]
    completion_label = "Done"
    completion_label_path = "core::circuit::AddInputResult::Done"
    text_edits = ["""
    use core::circuit::AddInputResult::Done;

    """]

    [[completions]]
    completion_label = "DropWith"
    completion_label_path = "core::internal::DropWith"
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
    completion_label = "Traicik"
    completion_label_path = "Traicik"

    [[completions]]
    completion_label = "BoxTrait"
    completion_label_path = "BoxTrait"

    [[completions]]
    completion_label = "ResultTraitImpl"
    completion_label_path = "core::result::ResultTraitImpl"
    text_edits = ["""
    use core::result::ResultTraitImpl;

    """]

    [[completions]]
    completion_label = "TrimMaxHelper"
    completion_label_path = "core::internal::bounded_int::TrimMaxHelper"
    text_edits = ["""
    use core::internal::bounded_int::TrimMaxHelper;

    """]

    [[completions]]
    completion_label = "TrimMinHelper"
    completion_label_path = "core::internal::bounded_int::TrimMinHelper"
    text_edits = ["""
    use core::internal::bounded_int::TrimMinHelper;

    """]

    [[completions]]
    completion_label = "True"
    completion_label_path = "bool::True"
    text_edits = ["""
    use bool::True;

    """]

    [[completions]]
    completion_label = "VecTrait"
    completion_label_path = "starknet::storage::VecTrait"
    text_edits = ["""
    use starknet::storage::VecTrait;

    """]

    [[completions]]
    completion_label = "traits"
    completion_label_path = "core::num::traits"
    text_edits = ["""
    use core::num::traits;

    """]

    [[completions]]
    completion_label = "traits"
    completion_label_path = "core::traits"
    text_edits = ["""
    use core::traits;

    """]

    [[completions]]
    completion_label = "wrapping"
    completion_label_path = "core::num::traits::ops::wrapping"
    text_edits = ["""
    use core::num::traits::ops::wrapping;

    """]
    "#);
}
