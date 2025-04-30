use crate::rename::rename;
use crate::support::insta::test_transform;

#[test]
fn felt_in_struct() {
    test_transform!(rename, r#"
    #[derive(Drop)]
    struct Foo { field: felt2<caret>52 }
    "#, @r"
    // found renames in the core crate
    #[derive(Drop)]
    struct Foo { field: RENAMED }
    ")
}

#[test]
fn usize_in_struct() {
    test_transform!(rename, r#"
    #[derive(Drop)]
    struct Foo { field: usi<caret>ze }
    "#, @r"
    // found renames in the core crate
    #[derive(Drop)]
    struct Foo { field: RENAMED }
    ")
}

#[test]
fn struct_by_name() {
    test_transform!(rename, r#"
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
    struct RENAMED { field: felt252 }
    fn main() {
        let foo: RENAMED = RENAMED { field: 0 };
    }
    fn calc(foo: RENAMED) {}
    mod rectangle {
        use super::RENAMED;
    }
    ")
}

#[test]
fn struct_member_via_definition() {
    test_transform!(rename, r#"
    #[derive(Drop)]
    struct Foo { wi<caret>dth: u64 }
    fn main() {
        let foo = Foo { width: 0 };
        let x = foo.width * 2;
    }
    "#, @r"
    #[derive(Drop)]
    struct Foo { RENAMED: u64 }
    fn main() {
        let foo = Foo { RENAMED: 0 };
        let x = foo.RENAMED * 2;
    }
    ")
}

#[test]
fn struct_member_via_constructor() {
    test_transform!(rename, r#"
    #[derive(Drop)]
    struct Foo { width: u64 }
    fn main() {
        let foo = Foo { wid<caret>th: 0 };
        let x = foo.width * 2;
    }
    "#, @r"
    #[derive(Drop)]
    struct Foo { RENAMED: u64 }
    fn main() {
        let foo = Foo { RENAMED: 0 };
        let x = foo.RENAMED * 2;
    }
    ")
}

#[test]
fn struct_member_via_field_access() {
    test_transform!(rename, r#"
    #[derive(Drop)]
    struct Foo { width: u64 }
    fn main() {
        let foo = Foo { wid<caret>th: 0 };
        let x = foo.width * 2;
    }
    "#, @r"
    #[derive(Drop)]
    struct Foo { RENAMED: u64 }
    fn main() {
        let foo = Foo { RENAMED: 0 };
        let x = foo.RENAMED * 2;
    }
    ")
}
