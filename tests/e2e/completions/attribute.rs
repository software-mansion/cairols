use super::test_completions_text_edits;
use crate::support::insta::test_transform;

#[test]
fn derive() {
    test_transform!(test_completions_text_edits,"
    #[derive(D<caret>)]
    pub struct Struct {
        x: u32,
        pub y: felt252,
        pub z: i16
    }
    ",@r#"
    caret = """
    #[derive(D<caret>)]
    """

    [[completions]]
    completion_label = "core"

    [[completions]]
    completion_label = "hello"

    [[completions]]
    completion_label = "Struct"
    "#);
}

#[test]
fn derive_after_comma() {
    test_transform!(test_completions_text_edits,"
    #[derive(D,<caret>)]
    pub struct Struct {
        x: u32,
        pub y: felt252,
        pub z: i16
    }
    ",@r#"
    caret = """
    #[derive(D,<caret>)]
    """

    [[completions]]
    completion_label = "starknet::Event"

    [[completions]]
    completion_label = "starknet::Store"
    "#);
}

#[test]
fn attribute() {
    test_transform!(test_completions_text_edits,"
    #[d<caret>]
    pub struct Struct {
        x: u32,
        pub y: felt252,
        pub z: i16
    }
    ",@r#"
    caret = """
    #[d<caret>]
    """

    [[completions]]
    completion_label = "derive"

    [[completions]]
    completion_label = "default"

    [[completions]]
    completion_label = "doc"
    "#);
}
