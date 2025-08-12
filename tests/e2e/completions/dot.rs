use lsp_types::request::Completion;

use crate::{completions::completion_fixture, support::insta::test_transform_plain};

#[test]
fn simple_struct() {
    test_transform_plain!(Completion, completion_fixture(),
    "
    struct Foo {
        bar: felt252
    }

    fn test() {
        let foo = Foo {
            bar: 123
        };

        foo.<caret>
    }
    ",
    @r#"
    caret = """
        foo.<caret>
    """

    [[completions]]
    completion_label = "bar"
    detail = "felt252"

    [[completions]]
    completion_label = "get_descriptor()"
    detail = "core::circuit::GetCircuitDescriptor"
    insert_text = "get_descriptor($1)"

    [[completions]]
    completion_label = "into()"
    detail = "core::traits::Into"
    insert_text = "into($1)"

    [[completions]]
    completion_label = "new_inputs()"
    detail = "core::circuit::CircuitInputs"
    insert_text = "new_inputs($1)"
    text_edits = ["""
    use core::circuit::CircuitInputs;

    """]

    [[completions]]
    completion_label = "try_into()"
    detail = "core::traits::TryInto"
    insert_text = "try_into($1)"
    "#);
}

#[test]
fn simple_struct_semicolon() {
    test_transform_plain!(Completion, completion_fixture(),
    "
    struct Foo {
        bar: felt252
    }

    fn test() {
        let foo = Foo {
            bar: 123
        };

        foo.<caret>;
    }
    ",
    @r#"
    caret = """
        foo.<caret>;
    """

    [[completions]]
    completion_label = "bar"
    detail = "felt252"

    [[completions]]
    completion_label = "get_descriptor()"
    detail = "core::circuit::GetCircuitDescriptor"
    insert_text = "get_descriptor($1)"

    [[completions]]
    completion_label = "into()"
    detail = "core::traits::Into"
    insert_text = "into($1)"

    [[completions]]
    completion_label = "new_inputs()"
    detail = "core::circuit::CircuitInputs"
    insert_text = "new_inputs($1)"
    text_edits = ["""
    use core::circuit::CircuitInputs;

    """]

    [[completions]]
    completion_label = "try_into()"
    detail = "core::traits::TryInto"
    insert_text = "try_into($1)"
    "#);
}
