use crate::support::insta::test_transform_plain;
use lsp_types::request::Completion;

#[test]
fn partially_typed() {
    test_transform_plain!(Completion,"
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
    "#);
}

#[test]
fn partially_typed_wrong() {
    test_transform_plain!(Completion,"
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
    test_transform_plain!(Completion,"
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
    "#);
}

#[test]
fn untyped() {
    test_transform_plain!(Completion,"
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

    [[completions]]
    completion_label = "qwerty"
    "#);
}

#[test]
fn untyped_some_filled() {
    test_transform_plain!(Completion,"
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
    "#);
}

#[test]
fn struct_unavailable() {
    test_transform_plain!(Completion,"
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
    test_transform_plain!(Completion,"
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
    "#);
}

#[test]
fn partially_typed_nested() {
    test_transform_plain!(Completion,"
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
    "#);
}

#[test]
fn enum_pattern() {
    test_transform_plain!(Completion,"
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

    [[completions]]
    completion_label = "Qwerty"
    "#);
}
