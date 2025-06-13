use lsp_types::request::Rename;

use crate::support::insta::{test_transform_plain, test_transform_with_macros};

#[test]
fn enum_name() {
    test_transform_plain!(Rename, r#"
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
    test_transform_plain!(Rename, r#"
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
    test_transform_plain!(Rename, r#"
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
    test_transform_plain!(Rename, r#"
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

#[test]
fn enum_variants_via_pattern_with_macros() {
    test_transform_with_macros!(Rename, r#"
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
    enum Foo { RENAMED, Baz }

    #[complex_attribute_macro_v2]
    fn main() {
        let foo = Foo::RENAMED;
        match foo {
            Foo::RENAMED => {},
            _ => {}
        }
    }
    ")
}
