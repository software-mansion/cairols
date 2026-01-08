use lsp_types::request::Completion;

use crate::{
    completions::completion_fixture,
    support::insta::{test_transform_plain, test_transform_with_macros},
};

#[test]
fn partially_typed() {
    test_transform_plain!(Completion, completion_fixture(), "
    struct Foo {
        abc: felt252,
        qwerty: felt252,
    }

    fn a() {
        let Foo {
            ab<caret>
        } = Foo { }
    }
    ",@r#"
    caret = """
            ab<caret>
    """

    [[completions]]
    completion_label = "abc"
    completion_label_type_info = "felt252"
    "#);
}

#[test]
fn partially_typed_macro() {
    test_transform_with_macros!(Completion, completion_fixture(), "
    #[complex_attribute_macro_v2]
    struct Foo {
        abc: felt252,
        qwerty: felt252,
    }

    #[complex_attribute_macro_v2]
    fn a() {
        let Foo {
            ab<caret>
        } = Foo { }
    }
    ",@r#"
    caret = """
            ab<caret>
    """

    [[completions]]
    completion_label = "abc"
    completion_label_type_info = "felt252"
    "#);
}

#[test]
fn partially_typed_wrong() {
    test_transform_plain!(Completion, completion_fixture(), "
    struct Foo {
        abc: felt252,
        qwerty: felt252,
    }

    fn a() {
        let Foo {
            this_literally_can_not_exists_<caret>
        } = Foo { }
    }
    ",@r#"
    caret = """
            this_literally_can_not_exists_<caret>
    """
    completions = []
    "#);
}

#[test]
fn partially_typed_some_filled() {
    test_transform_plain!(Completion, completion_fixture(), "
    struct Foo {
        abc: felt252,
        qwerty: felt252,
    }

    fn a() {
        let Foo {
            qwerty,
            ab<caret>
        } = Foo { }
    }
    ",@r#"
    caret = """
            ab<caret>
    """

    [[completions]]
    completion_label = "abc"
    completion_label_type_info = "felt252"
    "#);
}

#[test]
fn untyped() {
    test_transform_plain!(Completion, completion_fixture(), "
    struct Foo {
        abc: felt252,
        qwerty: felt252,
    }

    fn a() {
        let Foo {
            <caret>
        } = Foo { }
    }
    ",@r#"
    caret = """
            <caret>
    """

    [[completions]]
    completion_label = "abc"
    completion_label_type_info = "felt252"

    [[completions]]
    completion_label = "qwerty"
    completion_label_type_info = "felt252"
    "#);
}

#[test]
fn untyped_some_filled() {
    test_transform_plain!(Completion, completion_fixture(), "
    struct Foo {
        abc: felt252,
        qwerty: felt252,
    }

    fn a() {
        let Foo {
            qwerty,
            <caret>
        } = Foo { }
    }
    ",@r#"
    caret = """
            <caret>
    """

    [[completions]]
    completion_label = "abc"
    completion_label_type_info = "felt252"
    "#);
}

#[test]
fn struct_unavailable() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod foo {
        struct Foo {
            abc: felt252,
            qwerty: felt252,
        }
    }

    fn a() {
        let foo:Foo {
            qwerty,
            <caret>
        } = foo:Foo { }
    }
    ",@r#"
    caret = """
            <caret>
    """
    completions = []
    "#);
}

#[test]
fn wrong_value() {
    test_transform_plain!(Completion, completion_fixture(), "
    struct Foo {
        abc: felt252,
        qwerty: felt252,
    }

    fn a() {
        let Foo {
            qwerty,
            <caret>
        } = 1234567;
    }
    ",@r#"
    caret = """
            <caret>
    """

    [[completions]]
    completion_label = "abc"
    completion_label_type_info = "felt252"
    "#);
}

#[test]
fn partially_typed_nested() {
    test_transform_plain!(Completion, completion_fixture(), "
    struct Foo {
        abc: felt252,
        qwerty: felt252,
    }

    struct Boo {
        foo: Foo,
    }

    fn a() {
        let Boo {
            foo: Foo {
                ab<caret>
            }
        } = Foo { }
    }
    ",@r#"
    caret = """
                ab<caret>
    """

    [[completions]]
    completion_label = "abc"
    completion_label_type_info = "felt252"
    "#);
}

#[test]
fn enum_pattern() {
    test_transform_plain!(Completion, completion_fixture(), "
    enum Foo {
        Abc,
        Qwerty,
    }

    fn a() {
        let Foo::<caret> = 1234567;
    }
    ",@r#"
    caret = """
        let Foo::<caret> = 1234567;
    """

    [[completions]]
    completion_label = "Abc"
    completion_label_type_info = "Foo::Abc"

    [[completions]]
    completion_label = "Qwerty"
    completion_label_type_info = "Foo::Qwerty"
    "#);
}
