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
    completion_label = "ByteA_ActuallyNotByteArray"

    [[completions]]
    completion_label = "ByteArray"

    [[completions]]
    completion_label = "ByteArrayImpl"

    [[completions]]
    completion_label = "ByteArrayIter"

    [[completions]]
    completion_label = "ByteArrayTrait"
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
