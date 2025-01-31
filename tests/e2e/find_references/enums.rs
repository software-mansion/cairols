use crate::find_references::find_references;
use crate::support::insta::test_transform;

#[test]
fn enum_name() {
    test_transform!(find_references, r#"
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
    <sel=declaration>enum <sel>Foo</sel> {
        Bar,
        Baz,
    }</sel>

    fn main() {
        let foo = <sel>Foo</sel>::Bar;
        let foobar: <sel>Foo</sel> = foo;
    }

    fn calc(foo: <sel>Foo</sel>) {}

    mod rectangle {
        use super::<sel>Foo</sel>;
    }

    mod trick {
        struct Foo {}
    }
    ")
}

// FIXME(#129): Pattern should also be selected.
#[test]
fn enum_variants() {
    test_transform!(find_references, r#"
    enum Foo { Bar, Baz }
    fn main() {
        let foo = Foo::Ba<caret>r;
        match foo {
            Foo::Bar => {}
            _ => {}
        }
    }
    "#, @r"
    enum Foo { <sel=declaration>Bar</sel>, Baz }
    fn main() {
        let foo = Foo::Bar;
        match foo {
            Foo::Bar => {}
            _ => {}
        }
    }
    ")
}
