use lsp_types::request::Rename;

use crate::support::insta::test_transform_plain;

#[test]
fn alias_via_usage() {
    test_transform_plain!(Rename, r#"
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

    use xyz::A as RENAMED;

    fn f() {
        let _x: RENAMED = 5;
        let _y: RENAMED = 6;
    }
    ")
}

#[test]
fn alias_via_definition() {
    test_transform_plain!(Rename, r#"
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

    use xyz::A as RENAMED;

    fn f() {
        let _x: RENAMED = 5;
        let _y: RENAMED = 6;
    }
    ")
}

#[test]
fn fn_alias_via_call_site() {
    test_transform_plain!(Rename, r#"
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

    use xyz::foo as RENAMED;

    fn main() {
        RENAMED();
        RENAMED();
    }
    ")
}

#[test]
fn fn_alias_via_alias_definition() {
    test_transform_plain!(Rename, r#"
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

    use xyz::foo as RENAMED;

    fn main() {
        RENAMED();
        RENAMED();
    }
    ")
}

#[test]
fn struct_alias_via_usage() {
    test_transform_plain!(Rename, r#"
    mod xyz {
        pub struct Foo {}
    }

    use xyz::Foo as Bar;

    fn f(_a: <caret>Bar, _b: Bar) {}
    "#, @r"
    mod xyz {
        pub struct Foo {}
    }

    use xyz::Foo as RENAMED;

    fn f(_a: RENAMED, _b: RENAMED) {}
    ")
}

#[test]
fn struct_alias_via_alias_definition() {
    test_transform_plain!(Rename, r#"
    mod xyz {
        pub struct Foo {}
    }

    use xyz::Foo as <caret>Bar;

    fn f(_a: Bar, _b: Bar) {}
    "#, @r"
    mod xyz {
        pub struct Foo {}
    }

    use xyz::Foo as RENAMED;

    fn f(_a: RENAMED, _b: RENAMED) {}
    ")
}

#[test]
fn trait_alias_via_generic_constraint() {
    test_transform_plain!(Rename, r#"
    mod xyz {
        pub trait MyTrait<T> {}
    }

    use xyz::MyTrait as AliasedTrait;

    fn foo<T, +<caret>AliasedTrait<T>>() {}
    fn bar<T, +AliasedTrait<T>>() {}
    "#, @r"
    mod xyz {
        pub trait MyTrait<T> {}
    }

    use xyz::MyTrait as RENAMED;

    fn foo<T, +RENAMED<T>>() {}
    fn bar<T, +RENAMED<T>>() {}
    ")
}

#[test]
fn trait_alias_via_alias_definition() {
    test_transform_plain!(Rename, r#"
    mod xyz {
        pub trait MyTrait<T> {}
    }

    use xyz::MyTrait as <caret>AliasedTrait;

    fn foo<T, +AliasedTrait<T>>() {}
    fn bar<T, +AliasedTrait<T>>() {}
    "#, @r"
    mod xyz {
        pub trait MyTrait<T> {}
    }

    use xyz::MyTrait as RENAMED;

    fn foo<T, +RENAMED<T>>() {}
    fn bar<T, +RENAMED<T>>() {}
    ")
}

#[test]
fn original_rename_does_not_rename_alias_name() {
    test_transform_plain!(Rename, r#"
    mod xyz {
        pub struct Fo<caret>o {}
    }

    use xyz::Foo as Bar;

    fn f(_a: Bar) {}
    "#, @r"
    mod xyz {
        pub struct RENAMED {}
    }

    use xyz::RENAMED as Bar;

    fn f(_a: Bar) {}
    ")
}
