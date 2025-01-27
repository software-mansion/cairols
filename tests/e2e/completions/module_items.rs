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
    completion_label = "Trait1"

    [[completions]]
    completion_label = "CONST"

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
