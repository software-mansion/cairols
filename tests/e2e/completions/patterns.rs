use super::test_completions_text_edits;
use crate::support::insta::test_transform;

#[test]
fn partailly_typed() {
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

    [[completions]]
    completion_label = "qwerty"
    "#);
}

#[test]
fn partailly_typed_some_filled() {
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
fn partailly_typed_nested() {
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

    [[completions]]
    completion_label = "qwerty"
    "#);
}
