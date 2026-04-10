use lsp_types::request::References;

use crate::support::insta::test_transform_plain;

#[test]
fn alias_via_usage() {
    test_transform_plain!(References, r#"
    mod xyz {
        pub type A = u8;
    }

    use xyz::A as B;

    fn f() {
        let _x: <caret>B = 5;
        let _y: B = 6;
    }
    "#, @r"
    mod xyz {
        pub type A = u8;
    }

    use xyz::A as <sel=declaration>B</sel>;

    fn f() {
        let _x: <sel>B</sel> = 5;
        let _y: <sel>B</sel> = 6;
    }
    ")
}

#[test]
fn alias_via_definition() {
    test_transform_plain!(References, r#"
    mod xyz {
        pub type A = u8;
    }

    use xyz::A as <caret>B;

    fn f() {
        let _x: B = 5;
        let _y: B = 6;
    }
    "#, @r"
    mod xyz {
        pub type A = u8;
    }

    use xyz::A as <sel=declaration>B</sel>;

    fn f() {
        let _x: <sel>B</sel> = 5;
        let _y: <sel>B</sel> = 6;
    }
    ")
}

#[test]
fn fn_alias_via_call_site() {
    test_transform_plain!(References, r#"
    mod xyz {
        pub fn foo() {}
    }

    use xyz::foo as bar;

    fn main() {
        <caret>bar();
        bar();
    }
    "#, @r"
    mod xyz {
        pub fn foo() {}
    }

    use xyz::foo as <sel=declaration>bar</sel>;

    fn main() {
        <sel>bar</sel>();
        <sel>bar</sel>();
    }
    ")
}

#[test]
fn fn_alias_via_alias_definition() {
    test_transform_plain!(References, r#"
    mod xyz {
        pub fn foo() {}
    }

    use xyz::foo as <caret>bar;

    fn main() {
        bar();
        bar();
    }
    "#, @r"
    mod xyz {
        pub fn foo() {}
    }

    use xyz::foo as <sel=declaration>bar</sel>;

    fn main() {
        <sel>bar</sel>();
        <sel>bar</sel>();
    }
    ")
}

#[test]
fn struct_alias_via_usage() {
    test_transform_plain!(References, r#"
    mod xyz {
        pub struct Foo {}
    }

    use xyz::Foo as Bar;

    fn f(_a: <caret>Bar, _b: Bar) {}
    "#, @r"
    mod xyz {
        pub struct Foo {}
    }

    use xyz::Foo as <sel=declaration>Bar</sel>;

    fn f(_a: <sel>Bar</sel>, _b: <sel>Bar</sel>) {}
    ")
}

#[test]
fn struct_alias_via_alias_definition() {
    test_transform_plain!(References, r#"
    mod xyz {
        pub struct Foo {}
    }

    use xyz::Foo as <caret>Bar;

    fn f(_a: Bar, _b: Bar) {}
    "#, @r"
    mod xyz {
        pub struct Foo {}
    }

    use xyz::Foo as <sel=declaration>Bar</sel>;

    fn f(_a: <sel>Bar</sel>, _b: <sel>Bar</sel>) {}
    ")
}

#[test]
fn original_symbol_does_not_find_alias_usages() {
    test_transform_plain!(References, r#"
    mod xyz {
        pub struct Fo<caret>o {}
    }

    use xyz::Foo as Bar;

    fn f(_a: Bar) {}
    "#, @r"
    mod xyz {
        pub struct <sel=declaration>Foo</sel> {}
    }

    use xyz::<sel>Foo</sel> as Bar;

    fn f(_a: Bar) {}
    ")
}
