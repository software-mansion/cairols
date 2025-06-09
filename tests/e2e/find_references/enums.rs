use lsp_types::request::References;

use crate::support::insta::{test_transform_plain, test_transform_with_macros};

#[test]
fn enum_name() {
    test_transform_plain!(References, r#"
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
    enum <sel=declaration>Foo</sel> {
        Bar,
        Baz,
    }

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

#[test]
fn enum_variants_via_declaration() {
    test_transform_plain!(References, r#"
    enum Foo { Ba<caret>r, Baz }
    fn main() {
        let foo = Foo::Bar;
        match foo {
            Foo::Bar => {},
            _ => {}
        }
    }
    "#, @r"
    enum Foo { <sel=declaration>Bar</sel>, Baz }
    fn main() {
        let foo = Foo::<sel>Bar</sel>;
        match foo {
            Foo::<sel>Bar</sel> => {},
            _ => {}
        }
    }
    ")
}

#[test]
fn enum_variants_via_expr() {
    test_transform_plain!(References, r#"
    enum Foo { Bar, Baz }
    fn main() {
        let foo = Foo::Ba<caret>r;
        match foo {
            Foo::Bar => {},
            _ => {}
        }
    }
    "#, @r"
    enum Foo { <sel=declaration>Bar</sel>, Baz }
    fn main() {
        let foo = Foo::<sel>Bar</sel>;
        match foo {
            Foo::<sel>Bar</sel> => {},
            _ => {}
        }
    }
    ")
}

#[test]
fn enum_variants_via_pattern() {
    test_transform_plain!(References, r#"
    enum Foo { Bar, Baz }
    fn main() {
        let foo = Foo::Bar;
        match foo {
            Foo::B<caret>ar => {},
            _ => {}
        }
    }
    "#, @r"
    enum Foo { <sel=declaration>Bar</sel>, Baz }
    fn main() {
        let foo = Foo::<sel>Bar</sel>;
        match foo {
            Foo::<sel>Bar</sel> => {},
            _ => {}
        }
    }
    ")
}

#[test]
fn enum_variants_via_pattern_with_macros() {
    test_transform_with_macros!(References, r#"
    #[complex_attribute_macro_v2]
    enum Foo { Bar, Baz }

    #[complex_attribute_macro_v2]
    fn main() {
        let foo = Foo::Bar;
        match foo {
            Foo::B<caret>ar => {},
            _ => {}
        }
    }
    "#, @r"
    #[complex_attribute_macro_v2]
    enum Foo { <sel=declaration>Bar</sel>, Baz }

    #[complex_attribute_macro_v2]
    fn main() {
        let foo = Foo::<sel>Bar</sel>;
        match foo {
            Foo::<sel>Bar</sel> => {},
            _ => {}
        }
    }
    ")
}
