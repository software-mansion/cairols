use lsp_types::request::Completion;

use crate::{completions::completion_fixture, support::insta::test_transform_plain};

#[test]
fn empty() {
    test_transform_plain!(Completion, completion_fixture(), "
    pub struct Struct {
        x: u32,
        pub y: felt252,
        pub z: i16
    }

    fn build_struct() {
        let s = Struct {
            x: 0x0,
            y: 0x0,
            z: 0x0
        };

        let a = Struct { <caret> };
    }
    ",@r#"
    caret = """
        let a = Struct { <caret> };
    """

    [[completions]]
    completion_label = "x"
    detail = "u32"

    [[completions]]
    completion_label = "y"
    detail = "felt252"

    [[completions]]
    completion_label = "z"
    detail = "i16"
    "#);
}

#[test]
fn after_prop() {
    test_transform_plain!(Completion, completion_fixture(), "
    pub struct Struct {
        x: u32,
        pub y: felt252,
        pub z: i16
    }

    fn build_struct() {
        let s = Struct {
            x: 0x0,
            y: 0x0,
            z: 0x0
        };

        let b = Struct { x: 0x0, <caret> };
    }
    ",@r#"
    caret = """
        let b = Struct { x: 0x0, <caret> };
    """

    [[completions]]
    completion_label = "y"
    detail = "felt252"

    [[completions]]
    completion_label = "z"
    detail = "i16"
    "#);
}

#[test]
fn after_prop_macro() {
    test_transform_plain!(Completion, completion_fixture(), "
    #[complex_attribute_macro_v2]
    pub struct Struct {
        x: u32,
        pub y: felt252,
        pub z: i16
    }

    fn build_struct() {
        let s = Struct {
            x: 0x0,
            y: 0x0,
            z: 0x0
        };

        let b = Struct { x: 0x0, <caret> };
    }
    ",@r#"
    caret = """
        let b = Struct { x: 0x0, <caret> };
    """

    [[completions]]
    completion_label = "y"
    detail = "felt252"

    [[completions]]
    completion_label = "z"
    detail = "i16"
    "#);
}

#[test]
fn after_prop_before_spread() {
    test_transform_plain!(Completion, completion_fixture(), "
    pub struct Struct {
        x: u32,
        pub y: felt252,
        pub z: i16
    }

    fn build_struct() {
        let s = Struct {
            x: 0x0,
            y: 0x0,
            z: 0x0
        };

        let c = Struct {
            x: 0x0,
            <caret>
            ..s
        };
    }
    ",@r#"
    caret = """
            <caret>
    """

    [[completions]]
    completion_label = "y"
    detail = "felt252"

    [[completions]]
    completion_label = "z"
    detail = "i16"
    "#);
}

#[test]
fn after_prop_before_spread_same_line() {
    test_transform_plain!(Completion, completion_fixture(), "
    pub struct Struct {
        x: u32,
        pub y: felt252,
        pub z: i16
    }

    fn build_struct() {
        let s = Struct {
            x: 0x0,
            y: 0x0,
            z: 0x0
        };

        let d = Struct {
            x: 0x0,
            <caret>..s
        };
    }
    ",@r#"
    caret = """
            <caret>..s
    """

    [[completions]]
    completion_label = "y"
    detail = "felt252"

    [[completions]]
    completion_label = "z"
    detail = "i16"
    "#);
}

#[test]
fn before_spread_same_line() {
    test_transform_plain!(Completion, completion_fixture(), "
    pub struct Struct {
        x: u32,
        pub y: felt252,
        pub z: i16
    }

    fn build_struct() {
        let s = Struct {
            x: 0x0,
            y: 0x0,
            z: 0x0
        };

        let e = Struct { <caret>..s };
    }
    ",@r#"
    caret = """
        let e = Struct { <caret>..s };
    """

    [[completions]]
    completion_label = "x"
    detail = "u32"

    [[completions]]
    completion_label = "y"
    detail = "felt252"

    [[completions]]
    completion_label = "z"
    detail = "i16"
    "#);
}

#[test]
fn imported_empty() {
    test_transform_plain!(Completion, completion_fixture(), "
    pub struct Struct {
        x: u32,
        pub y: felt252,
        pub z: i16
    }

    mod happy_cases {
        use super::Struct;

        fn foo() {
            let a = Struct { <caret> };
        }
    }
    ",@r#"
    caret = """
            let a = Struct { <caret> };
    """

    [[completions]]
    completion_label = "x"
    detail = "u32"

    [[completions]]
    completion_label = "y"
    detail = "felt252"

    [[completions]]
    completion_label = "z"
    detail = "i16"
    "#);
}

#[test]
fn some_field_private() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod wrapper {
        pub struct Struct {
            x: u32,
            pub y: felt252,
            pub z: i16
        }
    }

    mod struct_is_not_in_ancestor_module {
        use super::wrapper::Struct;

        fn foo() {
            let a = Struct { <caret> };
        }
    }
    ",@r#"
    caret = """
            let a = Struct { <caret> };
    """
    completions = []
    "#);
}

#[test]
fn imported_after_prop() {
    test_transform_plain!(Completion, completion_fixture(), "
    pub struct Struct {
        x: u32,
        pub y: felt252,
        pub z: i16
    }

    mod happy_cases {
        use super::Struct;

        fn foo() {
            let b = Struct { y: 0x0, <caret> };
        }
    }
    ",@r#"
    caret = """
            let b = Struct { y: 0x0, <caret> };
    """

    [[completions]]
    completion_label = "x"
    detail = "u32"

    [[completions]]
    completion_label = "z"
    detail = "i16"
    "#);
}

#[test]
fn imported_after_two_prop() {
    test_transform_plain!(Completion, completion_fixture(), "
    pub struct Struct {
        x: u32,
        pub y: felt252,
        pub z: i16
    }

    mod happy_cases {
        use super::Struct;

        fn foo() {
            let c = Struct { y: 0x0, x: 0x0, <caret> }
        }
    }
    ",@r#"
    caret = """
            let c = Struct { y: 0x0, x: 0x0, <caret> }
    """

    [[completions]]
    completion_label = "z"
    detail = "i16"
    "#);
}

#[test]
fn not_imported() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod unhappy_cases {
        fn foo() {
            let a = NonexsitentStruct { <caret> };
        }
    }
    ",@r#"
    caret = """
            let a = NonexsitentStruct { <caret> };
    """
    completions = []
    "#);
}

#[test]
fn dep_without_visibility_support() {
    test_transform_plain!(Completion, completion_fixture(), "
    fn a() {
        dep::Foo { // This struct is partially private
            <caret>
        };
    }
    ",@r#"
    caret = """
            <caret>
    """

    [[completions]]
    completion_label = "a"
    detail = "felt252"

    [[completions]]
    completion_label = "b"
    detail = "felt252"
    "#);
}

#[test]
fn basic_initialization() {
    test_transform_plain!(Completion, completion_fixture(), "
    #[derive(Drop)]
    struct Abc {
        pub a: u128,
        pub b: u128,
        pub c: u128,
    }

    fn func() {
        let a = Ab<caret>
    }
    ",@r#"
    caret = """
        let a = Ab<caret>
    """

    [[completions]]
    completion_label = "Abc"
    insert_text = """
    Abc {
    \ta: $1,
    \tb: $2,
    \tc: $3,
    }"""

    [[completions]]
    completion_label = "AbcDrop"

    [[completions]]
    completion_label = "AccountContractLibraryDispatcherSubPointers"
    insert_text = """
    AccountContractLibraryDispatcherSubPointers {
    \tclass_hash: $1,
    }"""
    text_edits = ["""
    use starknet::account::AccountContractLibraryDispatcherSubPointers;

    """]

    [[completions]]
    completion_label = "AccountContractLibraryDispatcherSubPointersMut"
    insert_text = """
    AccountContractLibraryDispatcherSubPointersMut {
    \tclass_hash: $1,
    }"""
    text_edits = ["""
    use starknet::account::AccountContractLibraryDispatcherSubPointersMut;

    """]

    [[completions]]
    completion_label = "AccountContractSafeLibraryDispatcherSubPointers"
    insert_text = """
    AccountContractSafeLibraryDispatcherSubPointers {
    \tclass_hash: $1,
    }"""
    text_edits = ["""
    use starknet::account::AccountContractSafeLibraryDispatcherSubPointers;

    """]

    [[completions]]
    completion_label = "AccountContractSafeLibraryDispatcherSubPointersMut"
    insert_text = """
    AccountContractSafeLibraryDispatcherSubPointersMut {
    \tclass_hash: $1,
    }"""
    text_edits = ["""
    use starknet::account::AccountContractSafeLibraryDispatcherSubPointersMut;

    """]
    "#);
}

#[test]
fn initialization_non_imported_struct() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod m {
        #[derive(Drop)]
        pub struct Abc {
            pub a: u128,
            pub b: u128,
            pub c: u128,
        }
    }

    fn func() {
        let a = Ab<caret>
    }
    ",@r#"
    caret = """
        let a = Ab<caret>
    """

    [[completions]]
    completion_label = "Abc"
    insert_text = """
    Abc {
    \ta: $1,
    \tb: $2,
    \tc: $3,
    }"""
    text_edits = ["""
    use m::Abc;

    """]

    [[completions]]
    completion_label = "AccountContractLibraryDispatcherSubPointers"
    insert_text = """
    AccountContractLibraryDispatcherSubPointers {
    \tclass_hash: $1,
    }"""
    text_edits = ["""
    use starknet::account::AccountContractLibraryDispatcherSubPointers;

    """]

    [[completions]]
    completion_label = "AccountContractLibraryDispatcherSubPointersMut"
    insert_text = """
    AccountContractLibraryDispatcherSubPointersMut {
    \tclass_hash: $1,
    }"""
    text_edits = ["""
    use starknet::account::AccountContractLibraryDispatcherSubPointersMut;

    """]

    [[completions]]
    completion_label = "AccountContractSafeLibraryDispatcherSubPointers"
    insert_text = """
    AccountContractSafeLibraryDispatcherSubPointers {
    \tclass_hash: $1,
    }"""
    text_edits = ["""
    use starknet::account::AccountContractSafeLibraryDispatcherSubPointers;

    """]

    [[completions]]
    completion_label = "AccountContractSafeLibraryDispatcherSubPointersMut"
    insert_text = """
    AccountContractSafeLibraryDispatcherSubPointersMut {
    \tclass_hash: $1,
    }"""
    text_edits = ["""
    use starknet::account::AccountContractSafeLibraryDispatcherSubPointersMut;

    """]
    "#);
}

#[test]
fn initialization_with_macro() {
    test_transform_plain!(Completion, completion_fixture(), "
    #[derive(Drop)]
    struct Abc {
        pub a: u128,
        pub b: u128,
        pub c: u128,
    }

    #[complex_attribute_macro_v2]
    fn func() {
        let s = Ab<caret>
    }
    ",@r#"
    caret = """
        let s = Ab<caret>
    """

    [[completions]]
    completion_label = "Abc"
    insert_text = """
    Abc {
    \ta: $1,
    \tb: $2,
    \tc: $3,
    }"""

    [[completions]]
    completion_label = "AbcDrop"

    [[completions]]
    completion_label = "AccountContractLibraryDispatcherSubPointers"
    insert_text = """
    AccountContractLibraryDispatcherSubPointers {
    \tclass_hash: $1,
    }"""
    text_edits = ["""
    use starknet::account::AccountContractLibraryDispatcherSubPointers;

    """]

    [[completions]]
    completion_label = "AccountContractLibraryDispatcherSubPointersMut"
    insert_text = """
    AccountContractLibraryDispatcherSubPointersMut {
    \tclass_hash: $1,
    }"""
    text_edits = ["""
    use starknet::account::AccountContractLibraryDispatcherSubPointersMut;

    """]

    [[completions]]
    completion_label = "AccountContractSafeLibraryDispatcherSubPointers"
    insert_text = """
    AccountContractSafeLibraryDispatcherSubPointers {
    \tclass_hash: $1,
    }"""
    text_edits = ["""
    use starknet::account::AccountContractSafeLibraryDispatcherSubPointers;

    """]

    [[completions]]
    completion_label = "AccountContractSafeLibraryDispatcherSubPointersMut"
    insert_text = """
    AccountContractSafeLibraryDispatcherSubPointersMut {
    \tclass_hash: $1,
    }"""
    text_edits = ["""
    use starknet::account::AccountContractSafeLibraryDispatcherSubPointersMut;

    """]
    "#);
}

#[test]
fn initialization_non_imported_struct_with_macro() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod m {
        #[derive(Drop)]
        pub struct Abc {
            pub a: u128,
            pub b: u128,
            pub c: u128,
        }
    }

    #[complex_attribute_macro_v2]
    fn func() {
        let s = Ab<caret>
    }
    ",@r#"
    caret = """
        let s = Ab<caret>
    """

    [[completions]]
    completion_label = "Abc"
    insert_text = """
    Abc {
    \ta: $1,
    \tb: $2,
    \tc: $3,
    }"""
    text_edits = ["""
    use m::Abc;

    """]

    [[completions]]
    completion_label = "AccountContractLibraryDispatcherSubPointers"
    insert_text = """
    AccountContractLibraryDispatcherSubPointers {
    \tclass_hash: $1,
    }"""
    text_edits = ["""
    use starknet::account::AccountContractLibraryDispatcherSubPointers;

    """]

    [[completions]]
    completion_label = "AccountContractLibraryDispatcherSubPointersMut"
    insert_text = """
    AccountContractLibraryDispatcherSubPointersMut {
    \tclass_hash: $1,
    }"""
    text_edits = ["""
    use starknet::account::AccountContractLibraryDispatcherSubPointersMut;

    """]

    [[completions]]
    completion_label = "AccountContractSafeLibraryDispatcherSubPointers"
    insert_text = """
    AccountContractSafeLibraryDispatcherSubPointers {
    \tclass_hash: $1,
    }"""
    text_edits = ["""
    use starknet::account::AccountContractSafeLibraryDispatcherSubPointers;

    """]

    [[completions]]
    completion_label = "AccountContractSafeLibraryDispatcherSubPointersMut"
    insert_text = """
    AccountContractSafeLibraryDispatcherSubPointersMut {
    \tclass_hash: $1,
    }"""
    text_edits = ["""
    use starknet::account::AccountContractSafeLibraryDispatcherSubPointersMut;

    """]
    "#);
}
