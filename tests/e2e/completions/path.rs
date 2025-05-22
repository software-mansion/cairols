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
fn enum_variant() {
    test_transform!(test_completions_text_edits,"
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
