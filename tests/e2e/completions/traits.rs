use super::test_completions_text_edits;
use crate::support::insta::test_transform;

#[test]
fn self_completions() {
    test_transform!(test_completions_text_edits,"
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
    test_transform!(test_completions_text_edits, r#"
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
    test_transform!(test_completions_text_edits, r#"
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
    test_transform!(test_completions_text_edits, r#"
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
    test_transform!(test_completions_text_edits, r#"
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
    text_edits = ["""
    use core::traits;

    """]

    [[completions]]
    completion_label = "traits"
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
