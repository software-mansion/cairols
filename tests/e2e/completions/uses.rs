use super::test_completions_text_edits;
use crate::support::insta::test_transform;

#[test]
fn helper_module() {
    test_transform!(test_completions_text_edits,"
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

    [[completions]]
    completion_label = "Trait1"

    [[completions]]
    completion_label = "bar"
    "#);
}

#[test]
fn non_exporting_module() {
    test_transform!(test_completions_text_edits,"
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
    test_transform!(test_completions_text_edits,"
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
    test_transform!(test_completions_text_edits,"
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
    test_transform!(test_completions_text_edits,"
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

    [[completions]]
    completion_label = "y"
    "#);
}

// FIXME(#673)
#[test]
fn in_use_path_multi_with_one_in_scope() {
    test_transform!(test_completions_text_edits,"
    mod module {
        pub fn x() {}
        pub fn y() {}
    }

    use module::{x, <caret>
    ",@r#"
    caret = """
    use module::{x, <caret>
    """

    [[completions]]
    completion_label = "x"

    [[completions]]
    completion_label = "y"
    "#);
}

#[test]
fn first_segment() {
    test_transform!(test_completions_text_edits,"
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
    test_transform!(test_completions_text_edits,"
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
    test_transform!(test_completions_text_edits,"
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
    test_transform!(test_completions_text_edits,"
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
    "#);
}

#[test]
fn enum_variant() {
    test_transform!(test_completions_text_edits,"
    mod modzik {
        pub enum MyAnotherEnum {
            A,
            B,
        }
    }

    use modzik::MyAnotherEnum::<caret>
    ",@r#"
    caret = """
    use modzik::MyAnotherEnum::<caret>
    """

    [[completions]]
    completion_label = "A"

    [[completions]]
    completion_label = "B"
    "#);
}
