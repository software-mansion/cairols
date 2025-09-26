use lsp_types::request::Completion;

use crate::{
    completions::completion_fixture,
    support::insta::{test_transform_plain, test_transform_with_macros},
};

#[test]
fn single_element_path() {
    test_transform_plain!(Completion, completion_fixture(), "
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
    insert_text = "ByteA_ActuallyNotByteArray {}"

    [[completions]]
    completion_label = "ByteArray"
    insert_text = "ByteArray { data: $1, pending_word: $2, pending_word_len: $3 }"

    [[completions]]
    completion_label = "ByteArrayImpl"
    text_edits = ["""
    use core::byte_array::ByteArrayImpl;

    """]

    [[completions]]
    completion_label = "ByteArrayIter"
    insert_text = "ByteArrayIter { ba: $1, current_index: $2 }"
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
    test_transform_plain!(Completion, completion_fixture(), "
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
    insert_text = "Baz {}"
    "#);
}

#[test]
fn multi_segment_path_partial() {
    test_transform_plain!(Completion, completion_fixture(), "
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
    insert_text = "Baz {}"
    text_edits = ["""
    use foo::bar;

    """]
    "#);
}

#[test]
fn multi_segment_path_partial_macro() {
    test_transform_with_macros!(Completion, completion_fixture(), "
    mod foo {
        pub mod bar {
            pub struct Baz {}
        }
        pub struct Boo {}
    }

    #[complex_attribute_macro_v2]
    fn a() {
        bar::B<caret>
    }
    ",@r#"
    caret = """
        bar::B<caret>
    """

    [[completions]]
    completion_label = "Baz"
    insert_text = "Baz {}"
    text_edits = ["""
    use foo::bar;

    """]
    "#);
}

#[test]
fn enum_variant() {
    test_transform_plain!(Completion, completion_fixture(), "
    enum Enumik {
        A,
        B,
    }

    fn func() {
        let x = Enumik::<caret>
    }
    ",@r#"
    caret = """
        let x = Enumik::<caret>
    """

    [[completions]]
    completion_label = "A"

    [[completions]]
    completion_label = "B"
    "#);
}

#[test]
fn type_annotation() {
    test_transform_plain!(Completion, completion_fixture(), "
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
    test_transform_plain!(Completion, completion_fixture(), "
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
    test_transform_plain!(Completion, completion_fixture(), "
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
    test_transform_plain!(Completion, completion_fixture(), "
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
    test_transform_plain!(Completion, completion_fixture(), "
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
fn function_implicit_parameter() {
    test_transform_plain!(Completion, completion_fixture(), "
    fn foo() implicits(core::Range<caret>) {}
    ",@r#"
    caret = """
    fn foo() implicits(core::Range<caret>) {}
    """

    [[completions]]
    completion_label = "RangeCheck"
    "#);
}

#[test]
fn simple_completion_without_explicit_path() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod a {
        pub fn xyz() {}
    }

    fn foo() {
        xy<caret>
    }
    ",@r#"
    caret = """
        xy<caret>
    """

    [[completions]]
    completion_label = "xyz"
    text_edits = ["""
    use a::xyz;

    """]
    "#);
}

#[test]
fn duplicated_completion_without_explicit_path() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod a {
        pub fn xyz() {}
    }

    mod b {
        pub fn xyz() {}
    }

    fn foo() {
        xy<caret>
    }
    ",@r#"
    caret = """
        xy<caret>
    """

    [[completions]]
    completion_label = "xyz"
    completion_label_path = "a::xyz"
    text_edits = ["""
    use a::xyz;

    """]

    [[completions]]
    completion_label = "xyz"
    completion_label_path = "b::xyz"
    text_edits = ["""
    use b::xyz;

    """]
    "#);
}
