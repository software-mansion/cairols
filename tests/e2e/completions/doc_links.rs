use lsp_types::request::Completion;

use crate::{completions::completion_fixture, support::insta::test_transform_plain};

#[test]
fn crate_module_path_in_doc_link() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod docs {
        pub mod nested {
            pub struct Structure {}
        }
    }

    /// See [`docs::nested::Stru<caret>`].
    fn foo() {}
    ", @r#"
    caret = """
    /// See [`docs::nested::Stru<caret>`].
    """

    [[completions]]
    completion_label = "Structure"
    completion_label_type_info = "hello::docs::nested::Structure"
    "#);
}

#[test]
fn dependency_path_in_doc_link() {
    test_transform_plain!(Completion, completion_fixture(), "
    /// See [`dep::Fo<caret>`].
    fn foo() {}
    ", @r#"
    caret = """
    /// See [`dep::Fo<caret>`].
    """

    [[completions]]
    completion_label = "Foo"
    completion_label_type_info = "dep::Foo"
    "#);
}

#[test]
fn first_segment_doc_link_includes_label_details() {
    test_transform_plain!(Completion, completion_fixture(), "
    fn doc_link_unique_function(a: felt252) -> felt252 { a }

    /// See [`doc_link_uni<caret>`].
    fn foo() {}
    ", @r#"
    caret = """
    /// See [`doc_link_uni<caret>`].
    """

    [[completions]]
    completion_label = "doc_link_unique_function"
    completion_label_type_info = "fn(a: felt252) -> felt252"
    "#);
}

#[test]
fn one_segment_code_fragment() {
    test_transform_plain!(Completion, completion_fixture(), "
    /// See [`co<caret>`].
    fn foo() {}
    ", @r#"
    caret = """
    /// See [`co<caret>`].
    """

    [[completions]]
    completion_label = "core"
    "#);
}

#[test]
fn cursor_on_separator_in_doc_link_destination() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod docs {
        pub mod nested {
            pub struct Structure {}
        }
    }

    /// See [`docs:<caret>:nested::Structure`].
    fn foo() {}
    ", @r#"
    caret = """
    /// See [`docs:<caret>:nested::Structure`].
    """
    completions = []
    "#);
}

#[test]
fn cursor_after_separator_without_typed_text_in_doc_link_destination() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod docs {
        pub mod nested {
            pub struct Structure {}
        }
    }

    /// See [`docs::nested::<caret>`].
    fn foo() {}
    ", @r#"
    caret = """
    /// See [`docs::nested::<caret>`].
    """

    [[completions]]
    completion_label = "Structure"
    completion_label_type_info = "hello::docs::nested::Structure"
    "#);
}

#[test]
fn cursor_in_doc_link_label_has_no_completions() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod docs {
        pub mod nested {
            pub struct Structure {}
        }
    }

    /// See [Stru<caret>](docs::nested::Structure).
    fn foo() {}
    ", @r#"
    caret = """
    /// See [Stru<caret>](docs::nested::Structure).
    """
    completions = []
    "#);
}

#[test]
fn cursor_outside_markdown_link_has_no_completions() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod docs {
        pub mod nested {
            pub struct Structure {}
        }
    }

    /// See docs::nested::Stru<caret>
    fn foo() {}
    ", @r#"
    caret = """
    /// See docs::nested::Stru<caret>
    """
    completions = []
    "#);
}
