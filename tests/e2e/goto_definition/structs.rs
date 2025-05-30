use lsp_types::request::GotoDefinition;

use crate::support::insta::test_transform_and_macros;

#[test]
fn struct_item_in_constructor() {
    test_transform_and_macros!(GotoDefinition, r"
    <macro>#[complex_attribute_macro_v2]</macro>
    struct Foo { }

    fn main() {
        <macro>#[complex_attribute_macro_v2]</macro>
        let foo = Fo<caret>o {};
    }
    ", @r"
    struct <sel>Foo</sel> { }

    fn main() {
            let foo = Foo {};
    }

    ==============================

    #[complex_attribute_macro_v2]
    struct <sel>Foo</sel> { }

    fn main() {
        #[complex_attribute_macro_v2]
        let foo = Foo {};
    }
    ")
}

#[test]
fn struct_item_in_type() {
    test_transform_and_macros!(GotoDefinition, r"
    <macro>#[complex_attribute_macro_v2]</macro>
    struct Foo { }

    <macro>#[complex_attribute_macro_v2]</macro>
    fn calc(foo: Fo<caret>o) {}
    ", @r"
    struct <sel>Foo</sel> { }

    fn calc(foo: Foo) {}

    ==============================

    #[complex_attribute_macro_v2]
    struct <sel>Foo</sel> { }

    #[complex_attribute_macro_v2]
    fn calc(foo: Foo) {}
    ")
}

#[test]
fn struct_member_via_field_access() {
    test_transform_and_macros!(GotoDefinition, r"
    <macro>#[complex_attribute_macro_v2]</macro>
    #[derive(Drop)]
    struct Circle { radius: u64 }

    <macro>#[complex_attribute_macro_v2]</macro>
    fn foo(circle: Circle) -> u64 {
        circle.rad<caret>ius
    }
    ", @r"
    #[derive(Drop)]
    struct Circle { <sel>radius</sel>: u64 }

    fn foo(circle: Circle) -> u64 {
        circle.radius
    }

    ==============================

    #[complex_attribute_macro_v2]
    #[derive(Drop)]
    struct Circle { <sel>radius</sel>: u64 }

    #[complex_attribute_macro_v2]
    fn foo(circle: Circle) -> u64 {
        circle.radius
    }
    ")
}

#[test]
fn struct_member_in_constructor() {
    test_transform_and_macros!(GotoDefinition, r"
    <macro>#[complex_attribute_macro_v2]</macro>
    #[derive(Drop)]
    struct Circle { radius: u64 }

    <macro>#[complex_attribute_macro_v2]</macro>
    fn main() {
        let circle = Circle { rad<caret>ius: 42 };
    }
    ", @r"
    #[derive(Drop)]
    struct Circle { <sel>radius</sel>: u64 }

    fn main() {
        let circle = Circle { radius: 42 };
    }

    ==============================

    #[complex_attribute_macro_v2]
    #[derive(Drop)]
    struct Circle { <sel>radius</sel>: u64 }

    #[complex_attribute_macro_v2]
    fn main() {
        let circle = Circle { radius: 42 };
    }
    ")
}

#[test]
fn struct_member_right_side() {
    test_transform_and_macros!(GotoDefinition, r"
    <macro>#[complex_attribute_macro_v2]</macro>
    struct ExecutionInfoMock {
        caller_address: Operation,
    }

    <macro>#[complex_attribute_macro_v2]</macro>
    enum Operation {
        Retain
    }

    <macro>#[complex_attribute_macro_v2]</macro>
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

    ==============================

    #[complex_attribute_macro_v2]
    struct ExecutionInfoMock {
        caller_address: Operation,
    }

    #[complex_attribute_macro_v2]
    enum <sel>Operation</sel> {
        Retain
    }

    #[complex_attribute_macro_v2]
    fn func() {
        ExecutionInfoMock {
            caller_address: Operation::Retain,
        };
    }
    ")
}
