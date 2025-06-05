use lsp_types::request::GotoDefinition;

use crate::support::insta::{test_transform_plain, test_transform_with_macros};

#[test]
fn struct_item_in_constructor() {
    test_transform_plain!(GotoDefinition, r"
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
    test_transform_plain!(GotoDefinition, r"
    struct Foo { }
    fn calc(foo: Fo<caret>o) {}
    ", @r"
    struct <sel>Foo</sel> { }
    fn calc(foo: Foo) {}
    ")
}

#[test]
fn struct_member_via_field_access() {
    test_transform_plain!(GotoDefinition, r"
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
    test_transform_plain!(GotoDefinition, r"
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

#[test]
fn struct_member_right_side() {
    test_transform_plain!(GotoDefinition, r"
    struct ExecutionInfoMock {
        caller_address: Operation,
    }
    enum Operation {
        Retain
    }
    fn func() {
        ExecutionInfoMock {
            caller_address: Oper<caret>ation::Retain,
        };
    }
    ", @r"
    struct ExecutionInfoMock {
        caller_address: Operation,
    }
    enum <sel>Operation</sel> {
        Retain
    }
    fn func() {
        ExecutionInfoMock {
            caller_address: Operation::Retain,
        };
    }
    ")
}

#[test]
fn struct_item_in_constructor_with_macros() {
    test_transform_with_macros!(GotoDefinition, r"
    #[complex_attribute_macro_v2]
    struct Foo { }

    #[complex_attribute_macro_v2]
    fn main() {
        let foo = Fo<caret>o {};
    }
    ", @r"
    #[complex_attribute_macro_v2]
    struct <sel>Foo</sel> { }

    #[complex_attribute_macro_v2]
    fn main() {
        let foo = Foo {};
    }
    ")
}
