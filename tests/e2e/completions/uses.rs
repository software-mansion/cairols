use lsp_types::request::Completion;

use crate::{completions::completion_fixture, support::insta::test_transform_plain};

#[test]
fn helper_module() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod helper_module {
        pub trait Trait1<T> {
            fn some_method(self: @T);
        }

        pub const CONST: felt252 = 0x0;

        fn foo() {}
        pub fn bar() {}
    }

    mod not_exporting_module {
        const CONST: u32 = 0;
        fn foo() {}
        fn bar() {}
    }

    mod nested_module {
        pub mod inner {}
    }

    use helper_module::<caret>;
    ",@r#"
    caret = """
    use helper_module::<caret>;
    """

    [[completions]]
    completion_label = "CONST"
    detail = "felt252"

    [[completions]]
    completion_label = "Trait1"

    [[completions]]
    completion_label = "bar"
    detail = "fn() -> ()"
    "#);
}

#[test]
fn non_exporting_module() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod helper_module {
        pub trait Trait1<T> {
            fn some_method(self: @T);
        }

        pub const CONST: felt252 = 0x0;

        fn foo() {}
        pub fn bar() {}
    }

    mod not_exporting_module {
        const CONST: u32 = 0;
        fn foo() {}
        fn bar() {}
    }

    mod nested_module {
        pub mod inner {}
    }

    use non_exporting_module::<caret>;
    ",@r#"
    caret = """
    use non_exporting_module::<caret>;
    """
    completions = []
    "#);
}

#[test]
fn nested_module() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod helper_module {
        pub trait Trait1<T> {
            fn some_method(self: @T);
        }

        pub const CONST: felt252 = 0x0;

        fn foo() {}
        pub fn bar() {}
    }

    mod not_exporting_module {
        const CONST: u32 = 0;
        fn foo() {}
        fn bar() {}
    }

    mod nested_module {
        pub mod inner {}
    }

    use nested_module::<caret>;
    ",@r#"
    caret = """
    use nested_module::<caret>;
    """

    [[completions]]
    completion_label = "inner"
    "#);
}

#[test]
fn non_existent_module() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod helper_module {
        pub trait Trait1<T> {
            fn some_method(self: @T);
        }

        pub const CONST: felt252 = 0x0;

        fn foo() {}
        pub fn bar() {}
    }

    mod not_exporting_module {
        const CONST: u32 = 0;
        fn foo() {}
        fn bar() {}
    }

    mod nested_module {
        pub mod inner {}
    }

    use non_existent_module::<caret>;
    ",@r#"
    caret = """
    use non_existent_module::<caret>;
    """
    completions = []
    "#);
}

#[test]
fn in_use_path_multi() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod module {
        pub fn x() {}
        pub fn y() {}
    }

    use module::{<caret>
    ",@r#"
    caret = """
    use module::{<caret>
    """

    [[completions]]
    completion_label = "x"
    detail = "fn() -> ()"

    [[completions]]
    completion_label = "y"
    detail = "fn() -> ()"
    "#);
}

#[test]
fn in_use_path_multi_macro() {
    test_transform_plain!(Completion, completion_fixture(), "
    #[complex_attribute_macro_v2]
    mod module {
        pub fn x() {}
        pub fn y() {}
    }

    #[complex_attribute_macro_v2]
    use module::{<caret>
    ",@r#"
    caret = """
    use module::{<caret>
    """

    [[completions]]
    completion_label = "x"
    detail = "fn() -> ()"

    [[completions]]
    completion_label = "y"
    detail = "fn() -> ()"
    "#);
}

// FIXME(#673)
#[test]
fn in_use_path_multi_with_one_in_scope() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod module {
        pub fn x() {}
        pub fn y() {}
    }

    use module::{x, <caret>
    ",@r#"
    caret = """
    use module::{x,<caret>
    """

    [[completions]]
    completion_label = "x"
    detail = "fn() -> ()"

    [[completions]]
    completion_label = "y"
    detail = "fn() -> ()"
    "#);
}

#[test]
fn first_segment() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod my_module {
        pub trait Trait1<T> {
            fn some_method(self: @T);
        }

        pub const CONST: felt252 = 0x0;

        fn foo() {}
        pub fn bar() {}
    }

    use my_mod<caret>;
    ",@r#"
    caret = """
    use my_mod<caret>;
    """

    [[completions]]
    completion_label = "my_module"
    "#);
}

#[test]
fn first_segment_core() {
    test_transform_plain!(Completion, completion_fixture(), "
    use co<caret>;
    ",@r#"
    caret = """
    use co<caret>;
    """

    [[completions]]
    completion_label = "core"
    "#);
}

#[test]
fn first_segment_enum() {
    test_transform_plain!(Completion, completion_fixture(), "
    use My<caret>

    enum MyEnum {}
    ",@r#"
    caret = """
    use My<caret>
    """

    [[completions]]
    completion_label = "MyEnum"
    "#);
}

#[test]
fn nested_enum() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod modzik {
        pub enum MyAnotherEnum {
            A,
            B,
        }
    }

    use modzik::<caret>
    ",@r#"
    caret = """
    use modzik::<caret>
    """

    [[completions]]
    completion_label = "MyAnotherEnum"
    detail = "hello::modzik::MyAnotherEnum"
    "#);
}

#[test]
fn enum_variant() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod struct_module {
        struct MyStruct {
            field: felt252,
        }
    }

    mod modzik {
        use super::struct_module::MyStruct;
        pub enum MyAnotherEnum {
            A,
            B: MyStruct,
        }
    }

    use modzik::MyAnotherEnum::<caret>
    ",@r#"
    caret = """
    use modzik::MyAnotherEnum::<caret>
    """

    [[completions]]
    completion_label = "A"
    detail = "MyAnotherEnum::A"

    [[completions]]
    completion_label = "B"
    detail = "MyAnotherEnum::B: MyStruct"
    "#);
}

#[test]
fn no_text_in_use_statement() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod my_mod {
       pub const MY_CONST: u8 = 5;
       pub fn my_func() {}
    }

    use <caret>

    fn a() {}
    ",@r#"
    caret = """
    use <caret>
    """

    [[completions]]
    completion_label = "Option"

    [[completions]]
    completion_label = "PanicResult"

    [[completions]]
    completion_label = "Result"

    [[completions]]
    completion_label = "bool"

    [[completions]]
    completion_label = "core"

    [[completions]]
    completion_label = "dep"

    [[completions]]
    completion_label = "hello"

    [[completions]]
    completion_label = "my_mod"

    [[completions]]
    completion_label = "starknet"
    "#);
}

#[test]
fn no_text_last_segment_in_use_statement() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod my_mod {
       pub const MY_CONST: u8 = 5;
       pub fn my_func() {}
    }

    use my_mod::<caret>

    fn a() {}
    ",@r#"
    caret = """
    use my_mod::<caret>
    """

    [[completions]]
    completion_label = "MY_CONST"
    detail = "u8"

    [[completions]]
    completion_label = "my_func"
    detail = "fn() -> ()"
    "#);
}

#[test]
fn only_proposing_currently_visible_items() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod foorator_mod {
        pub fn foorator_func() {}
    }

    mod wrong_module {
        // Shouldn't be proposed in use statement since it is not visible yet.
        pub fn foorator_wrong() {}
    }

    use foorator_<caret>;
     ",@r#"
    caret = """
    use foorator_<caret>;
    """

    [[completions]]
    completion_label = "foorator_mod"
    "#);
}

#[test]
fn declarative_macro() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod my_mod {
        pub macro my_own_macro {
            ($x:ident) => {
                1
            };
        }
    }

    use my_mod::<caret>
    ",@r#"
    caret = """
    use my_mod::<caret>
    """

    [[completions]]
    completion_label = "my_own_macro"
    "#);
}
