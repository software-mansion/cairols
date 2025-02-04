use crate::goto_definition::goto_definition;
use crate::support::insta::test_transform;

#[test]
fn struct_item_in_constructor() {
    test_transform!(goto_definition, r"
    struct Foo { }
    fn main() {
        let foo = Fo<caret>o {};
    }
    ", @r"
    struct <sel>Foo</sel> { }
    fn main() {
        let foo = Foo {};
    }
    ")
}

#[test]
fn struct_item_in_type() {
    test_transform!(goto_definition, r"
    struct Foo { }
    fn calc(foo: Fo<caret>o) {}
    ", @r"
    struct <sel>Foo</sel> { }
    fn calc(foo: Foo) {}
    ")
}

#[test]
fn struct_member_via_field_access() {
    test_transform!(goto_definition, r"
    #[derive(Drop)]
    struct Circle { radius: u64 }
    fn foo(circle: Circle) -> u64 {
        circle.rad<caret>ius
    }
    ", @r"
    #[derive(Drop)]
    struct Circle { <sel>radius</sel>: u64 }
    fn foo(circle: Circle) -> u64 {
        circle.radius
    }
    ")
}

#[test]
fn struct_member_in_constructor() {
    test_transform!(goto_definition, r"
    #[derive(Drop)]
    struct Circle { radius: u64 }
    fn main() {
        let circle = Circle { rad<caret>ius: 42 };
    }
    ", @r"
    #[derive(Drop)]
    struct Circle { <sel>radius</sel>: u64 }
    fn main() {
        let circle = Circle { radius: 42 };
    }
    ")
}
