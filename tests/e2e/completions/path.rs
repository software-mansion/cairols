use super::test_completions_text_edits;
use crate::support::insta::test_transform;

#[test]
fn single_element_path() {
    test_transform!(test_completions_text_edits,"
    struct ByteA_ActuallyNotByteArray {}

    fn a() {
        ByteA<caret>
    }
    ",@r#"
    caret = """
        ByteA<caret>
    """

    [[completions]]
    completion_label = "BitAnd"
    text_edits = ["""
    use core::traits::BitAnd;

    """]

    [[completions]]
    completion_label = "ByteA_ActuallyNotByteArray"

    [[completions]]
    completion_label = "ByteArray"

    [[completions]]
    completion_label = "ByteArrayImpl"
    text_edits = ["""
    use core::byte_array::ByteArrayImpl;

    """]

    [[completions]]
    completion_label = "ByteArrayIter"
    text_edits = ["""
    use core::byte_array::ByteArrayIter;

    """]

    [[completions]]
    completion_label = "ByteArrayTrait"

    [[completions]]
    completion_label = "Bytes31Impl"
    text_edits = ["""
    use core::bytes_31::Bytes31Impl;

    """]

    [[completions]]
    completion_label = "Bytes31Trait"

    [[completions]]
    completion_label = "System"
    "#);
}

#[test]
fn multi_segment_path() {
    test_transform!(test_completions_text_edits,"
    mod foo {
        struct Bar {}
        pub struct Baz {}
    }

    fn a() {
        foo::B<caret>
    }
    ",@r#"
    caret = """
        foo::B<caret>
    """

    [[completions]]
    completion_label = "Baz"
    "#);
}

#[test]
fn enum_pattern() {
    test_transform!(test_completions_text_edits,"
    enum Foo {
        Abc,
        Qwerty,
    }

    fn a() {
        let Foo::<caret> = 1234567;
    }
    ",@r#"
    caret = """
        let Foo::<caret> = 1234567;
    """

    [[completions]]
    completion_label = "Abc"

    [[completions]]
    completion_label = "Qwerty"
    "#);
}

#[test]
fn multi_segment_path_partial() {
    test_transform!(test_completions_text_edits,"
    mod foo {
        pub mod bar {
            pub struct Baz {}
        }
        pub struct Boo {}
    }

    fn a() {
        bar::B<caret>
    }
    ",@r#"
    caret = """
        bar::B<caret>
    """

    [[completions]]
    completion_label = "Baz"
    text_edits = ["""
    use foo::bar;

    """]
    "#);
}

#[test]
fn type_annotation() {
    test_transform!(test_completions_text_edits,"
    mod module {
        pub type felt = felt252;
        pub type int = i32;
        type priv_int = i32;
    }
    fn foo() {
        let x: module::<caret> = 0x0;
    }
    ",@r#"
    caret = """
        let x: module::<caret> = 0x0;
    """

    [[completions]]
    completion_label = "felt"

    [[completions]]
    completion_label = "int"
    "#);
}

#[test]
fn type_annotation_with_dangling_path() {
    test_transform!(test_completions_text_edits,"
    mod module {
        pub type felt = felt252;
        pub type int = i32;
        type priv_int = i32;

        pub const CONST: u32 = 0;

        pub mod nested_module {
            pub type T = u32;
        }
    }
    fn foo() -> u32 {
        let x: module::<caret>
            nested_module::T = 0x0;
    }
    ",@r#"
    caret = """
        let x: module::<caret>
    """

    [[completions]]
    completion_label = "CONST"

    [[completions]]
    completion_label = "felt"

    [[completions]]
    completion_label = "int"

    [[completions]]
    completion_label = "nested_module"
    "#);
}

#[test]
fn type_annotation_with_trivia() {
    test_transform!(test_completions_text_edits,"
    mod module {
        pub type felt = felt252;
        pub type int = i32;
        type priv_int = i32;
    }
    fn foo() {
        let x: module::<caret> // comment
            = 0x0;
    }
    ",@r#"
    caret = """
        let x: module::<caret> // comment
    """

    [[completions]]
    completion_label = "felt"

    [[completions]]
    completion_label = "int"
    "#);
}

#[test]
fn generic_parameter() {
    test_transform!(test_completions_text_edits,"
    mod module {
        pub type felt = felt252;
        pub type int = i32;
        type priv_int = i32;
    }
    fn foo() {
        let x = Into::<module::<caret>, u32>(0);
    }
    ",@r#"
    caret = """
        let x = Into::<module::<caret>, u32>(0);
    """

    [[completions]]
    completion_label = "felt"

    [[completions]]
    completion_label = "int"
    "#);
}

#[test]
fn generic_parameter_with_trivia() {
    test_transform!(test_completions_text_edits,"
    mod module {
        pub type felt = felt252;
        pub type int = i32;
        type priv_int = i32;
    }
    fn foo() {
        let x = Into::<module::<caret>//comment
        , u32>(0);
    }
    ",@r#"
    caret = """
        let x = Into::<module::<caret>//comment
    """

    [[completions]]
    completion_label = "felt"

    [[completions]]
    completion_label = "int"
    "#);
}

#[test]
fn function_generic_parameter() {
    test_transform!(test_completions_text_edits,"
    fn foo<T: core::num::traits::<caret>>(x: T) {}
    ",@r#"
    caret = """
    fn foo<T: core::num::traits::<caret>>(x: T) {}
    """
    completions = []
    "#);
}

#[test]
fn function_implicit_parameter() {
    test_transform!(test_completions_text_edits,"
    fn foo() implicits(core::Range<caret>) {}
    ",@r#"
    caret = """
    fn foo() implicits(core::Range<caret>) {}
    """

    [[completions]]
    completion_label = "RangeCheck"
    "#);
}
