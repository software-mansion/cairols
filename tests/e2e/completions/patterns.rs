use super::test_completions_text_edits;
use crate::support::insta::test_transform;

#[test]
fn partially_typed() {
    test_transform!(test_completions_text_edits,"
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
    test_transform!(test_completions_text_edits,"
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
    test_transform!(test_completions_text_edits,"
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
    test_transform!(test_completions_text_edits,"
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
    test_transform!(test_completions_text_edits,"
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
    test_transform!(test_completions_text_edits,"
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
    test_transform!(test_completions_text_edits,"
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
    test_transform!(test_completions_text_edits,"
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
    test_transform!(test_completions_text_edits,"
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
