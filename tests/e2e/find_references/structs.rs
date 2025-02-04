use crate::find_references::find_references;
use crate::support::insta::test_transform;

#[test]
fn struct_by_name() {
    test_transform!(find_references, r#"
    #[derive(Drop)]
    struct Fo<caret>o { field: felt252 }
    fn main() {
        let foo: Foo = Foo { field: 0 };
    }
    fn calc(foo: Foo) {}
    mod rectangle {
        use super::Foo;
    }
    "#, @r"
    #[derive(Drop)]
    struct <sel=declaration>Foo</sel> { field: felt252 }
    fn main() {
        let foo: <sel>Foo</sel> = <sel>Foo</sel> { field: 0 };
    }
    fn calc(foo: <sel>Foo</sel>) {}
    mod rectangle {
        use super::<sel>Foo</sel>;
    }
    ")
}

// FIXME(#129): The results for this are very off.
#[test]
fn struct_member_via_definition() {
    test_transform!(find_references, r#"
    #[derive(Drop)]
    struct Foo { wi<caret>dth: u64 }
    fn main() {
        let foo = Foo { width: 0 };
        let x = foo.width * 2;
    }
    "#, @r"
    #[derive(Drop)]
    struct <sel=declaration>Foo</sel> { width: u64 }
    fn main() {
        let foo = <sel>Foo</sel> { width: 0 };
        let x = foo.width * 2;
    }
    ")
}

#[test]
fn struct_member_via_constructor() {
    test_transform!(find_references, r#"
    #[derive(Drop)]
    struct Foo { width: u64 }
    fn main() {
        let foo = Foo { wid<caret>th: 0 };
        let x = foo.width * 2;
    }
    "#, @r"
    #[derive(Drop)]
    struct Foo { <sel=declaration>width: u64</sel> }
    fn main() {
        let foo = Foo { <sel>width</sel>: 0 };
        let x = foo.<sel>width</sel> * 2;
    }
    ")
}

#[test]
fn struct_member_via_field_access() {
    test_transform!(find_references, r#"
    #[derive(Drop)]
    struct Foo { width: u64 }
    fn main() {
        let foo = Foo { wid<caret>th: 0 };
        let x = foo.width * 2;
    }
    "#, @r"
    #[derive(Drop)]
    struct Foo { <sel=declaration>width: u64</sel> }
    fn main() {
        let foo = Foo { <sel>width</sel>: 0 };
        let x = foo.<sel>width</sel> * 2;
    }
    ")
}
