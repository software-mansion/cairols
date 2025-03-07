use crate::rename::rename;
use crate::support::insta::test_transform;

#[test]
fn enum_name() {
    test_transform!(rename, r#"
    enum Fo<caret>o {
        Bar,
        Baz,
    }

    fn main() {
        let foo = Foo::Bar;
        let foobar: Foo = foo;
    }

    fn calc(foo: Foo) {}

    mod rectangle {
        use super::Foo;
    }

    mod trick {
        struct Foo {}
    }
    "#, @r"
    enum RENAMED {
        Bar,
        Baz,
    }

    fn main() {
        let foo = RENAMED::Bar;
        let foobar: RENAMED = foo;
    }

    fn calc(foo: RENAMED) {}

    mod rectangle {
        use super::RENAMED;
    }

    mod trick {
        struct Foo {}
    }
    ")
}

#[test]
fn enum_variants_via_declaration() {
    test_transform!(rename, r#"
    enum Foo { Ba<caret>r, Baz }
    fn main() {
        let foo = Foo::Bar;
        match foo {
            Foo::Bar => {},
            _ => {}
        }
    }
    "#, @r"
    enum Foo { RENAMED, Baz }
    fn main() {
        let foo = Foo::RENAMED;
        match foo {
            Foo::RENAMED => {},
            _ => {}
        }
    }
    ")
}

#[test]
fn enum_variants_via_expr() {
    test_transform!(rename, r#"
    enum Foo { Bar, Baz }
    fn main() {
        let foo = Foo::Ba<caret>r;
        match foo {
            Foo::Bar => {},
            _ => {}
        }
    }
    "#, @r"
    enum Foo { RENAMED, Baz }
    fn main() {
        let foo = Foo::RENAMED;
        match foo {
            Foo::RENAMED => {},
            _ => {}
        }
    }
    ")
}

#[test]
fn enum_variants_via_pattern() {
    test_transform!(rename, r#"
    enum Foo { Bar, Baz }
    fn main() {
        let foo = Foo::Bar;
        match foo {
            Foo::B<caret>ar => {},
            _ => {}
        }
    }
    "#, @r"
    enum Foo { RENAMED, Baz }
    fn main() {
        let foo = Foo::RENAMED;
        match foo {
            Foo::RENAMED => {},
            _ => {}
        }
    }
    ")
}
