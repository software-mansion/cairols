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
    completion_label = "Debug"

    [[completions]]
    completion_label = "Default"

    [[completions]]
    completion_label = "Destruct"

    [[completions]]
    completion_label = "Drop"
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
    completion_label = "Clone"

    [[completions]]
    completion_label = "Copy"

    [[completions]]
    completion_label = "Debug"

    [[completions]]
    completion_label = "Default"

    [[completions]]
    completion_label = "Destruct"

    [[completions]]
    completion_label = "Drop"

    [[completions]]
    completion_label = "Hash"

    [[completions]]
    completion_label = "PanicDestruct"

    [[completions]]
    completion_label = "PartialEq"

    [[completions]]
    completion_label = "Serde"

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
    completion_label = "default"

    [[completions]]
    completion_label = "derive"

    [[completions]]
    completion_label = "doc"
    "#);
}
