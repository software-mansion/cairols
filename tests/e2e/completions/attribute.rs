use lsp_types::request::Completion;

use crate::{completions::completion_fixture, support::insta::test_transform_plain};

#[test]
fn derive() {
    test_transform_plain!(Completion, completion_fixture(),"
    #[derive(D<caret>)]
    pub struct Struct {
        x: u32,
        pub y: felt252,
        pub z: i16
    }
    ",@r##"
    caret = """
    #[derive(D<caret>)]
    """

    [[completions]]
    completion_label = "Debug"
    applied = "#[derive(Debug)]"

    [[completions]]
    completion_label = "Default"
    applied = "#[derive(Default)]"

    [[completions]]
    completion_label = "Destruct"
    applied = "#[derive(Destruct)]"

    [[completions]]
    completion_label = "Drop"
    applied = "#[derive(Drop)]"
    "##);
}

#[test]
fn derive_after_comma() {
    test_transform_plain!(Completion, completion_fixture(),"
    #[derive(D,<caret>)]
    pub struct Struct {
        x: u32,
        pub y: felt252,
        pub z: i16
    }
    ",@r##"
    caret = """
    #[derive(D,<caret>)]
    """

    [[completions]]
    completion_label = "Clone"
    applied = "#[derive(D,Clone)]"

    [[completions]]
    completion_label = "Copy"
    applied = "#[derive(D,Copy)]"

    [[completions]]
    completion_label = "Debug"
    applied = "#[derive(D,Debug)]"

    [[completions]]
    completion_label = "Default"
    applied = "#[derive(D,Default)]"

    [[completions]]
    completion_label = "Destruct"
    applied = "#[derive(D,Destruct)]"

    [[completions]]
    completion_label = "Drop"
    applied = "#[derive(D,Drop)]"

    [[completions]]
    completion_label = "Hash"
    applied = "#[derive(D,Hash)]"

    [[completions]]
    completion_label = "PanicDestruct"
    applied = "#[derive(D,PanicDestruct)]"

    [[completions]]
    completion_label = "PartialEq"
    applied = "#[derive(D,PartialEq)]"

    [[completions]]
    completion_label = "Serde"
    applied = "#[derive(D,Serde)]"

    [[completions]]
    completion_label = "starknet::Event"
    applied = "#[derive(D,starknet::Event)]"

    [[completions]]
    completion_label = "starknet::Store"
    applied = "#[derive(D,starknet::Store)]"
    "##);
}

#[test]
fn attribute() {
    test_transform_plain!(Completion, completion_fixture(),"
    #[d<caret>]
    pub struct Struct {
        x: u32,
        pub y: felt252,
        pub z: i16
    }
    ",@r##"
    caret = """
    #[d<caret>]
    """

    [[completions]]
    completion_label = "default"
    applied = "#[default]"

    [[completions]]
    completion_label = "derive"
    applied = "#[derive]"

    [[completions]]
    completion_label = "doc"
    applied = "#[doc]"
    "##);
}

#[test]
fn derive_caret_before_existing_text() {
    test_transform_plain!(Completion, completion_fixture(),"
    #[derive(Dr<caret>Copy)]
    pub struct Struct {}
    ",@r##"
    caret = """
    #[derive(Dr<caret>Copy)]
    """

    [[completions]]
    completion_label = "Destruct"
    applied = "#[derive(DestructCopy)]"

    [[completions]]
    completion_label = "Drop"
    applied = "#[derive(DropCopy)]"
    "##);
}

#[test]
fn derive_caret_before_next_arg_with_space() {
    test_transform_plain!(Completion, completion_fixture(),"
    #[derive(Dr<caret> Copy)]
    pub struct Struct {}
    ",@r##"
    caret = """
    #[derive(Dr<caret> Copy)]
    """

    [[completions]]
    completion_label = "Destruct"
    applied = "#[derive(Destruct Copy)]"

    [[completions]]
    completion_label = "Drop"
    applied = "#[derive(Drop Copy)]"
    "##);
}

#[test]
fn derive_trailing_whitespace() {
    test_transform_plain!(Completion, completion_fixture(),"
    #[derive(Copy, Dr<caret>   )]
    pub struct Struct {}
    ",@r##"
    caret = """
    #[derive(Copy, Dr<caret>   )]
    """

    [[completions]]
    completion_label = "Destruct"
    applied = "#[derive(Copy, Destruct   )]"

    [[completions]]
    completion_label = "Drop"
    applied = "#[derive(Copy, Drop   )]"
    "##);
}

#[test]
fn derive_unclosed_parenthesis() {
    test_transform_plain!(Completion, completion_fixture(),"
    #[derive(Copy, Dr<caret>
    pub struct Struct {}
    ",@r##"
    caret = """
    #[derive(Copy, Dr<caret>
    """

    [[completions]]
    completion_label = "Destruct"
    applied = "#[derive(Copy, Destruct"

    [[completions]]
    completion_label = "Drop"
    applied = "#[derive(Copy, Drop"
    "##);
}

#[test]
fn derive_followed_by_comma() {
    test_transform_plain!(Completion, completion_fixture(),"
    #[derive(Dr<caret>, Copy)]
    pub struct Struct {}
    ",@r##"
    caret = """
    #[derive(Dr<caret>, Copy)]
    """

    [[completions]]
    completion_label = "Destruct"
    applied = "#[derive(Destruct, Copy)]"

    [[completions]]
    completion_label = "Drop"
    applied = "#[derive(Drop, Copy)]"
    "##);
}

#[test]
fn derive_multiline_with_comment() {
    test_transform_plain!(Completion, completion_fixture(),"
    #[derive(
        Copy,
        // comment
        Dr<caret>
    )]
    pub struct Struct {}
    ",@r##"
    caret = """
        Dr<caret>
    """

    [[completions]]
    completion_label = "Destruct"
    applied = "    Destruct"

    [[completions]]
    completion_label = "Drop"
    applied = "    Drop"
    "##);
}

#[test]
fn attribute_caret_inside_name() {
    test_transform_plain!(Completion, completion_fixture(),"
    #[der<caret>ive]
    pub struct Struct {}
    ",@r##"
    caret = """
    #[der<caret>ive]
    """

    [[completions]]
    completion_label = "default"
    applied = "#[defaultive]"

    [[completions]]
    completion_label = "derive"
    applied = "#[deriveive]"
    "##);
}

#[test]
fn attribute_multiline() {
    test_transform_plain!(Completion, completion_fixture(),"
    #[
        d<caret>]
    pub struct Struct {}
    ",@r##"
    caret = """
        d<caret>]
    """

    [[completions]]
    completion_label = "default"
    applied = "    default]"

    [[completions]]
    completion_label = "derive"
    applied = "    derive]"

    [[completions]]
    completion_label = "doc"
    applied = "    doc]"
    "##);
}
