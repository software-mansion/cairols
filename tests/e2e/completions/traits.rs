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
