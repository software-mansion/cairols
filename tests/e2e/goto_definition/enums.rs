use lsp_types::request::GotoDefinition;

use crate::support::insta::{test_transform_plain, test_transform_with_macros};

#[test]
fn enum_item_in_type() {
    test_transform_plain!(GotoDefinition, r#"
    enum Foo { Bar }
    fn calc(foo: Fo<caret>o) {}
    "#, @r"
    enum <sel>Foo</sel> { Bar }
    fn calc(foo: Foo) {}
    ");
}

#[test]
fn enum_item_in_expr() {
    test_transform_plain!(GotoDefinition, r#"
    enum Foo { Bar }
    fn main() {
        let foo = Fo<caret>o::Bar;
    }
    "#, @r"
    enum <sel>Foo</sel> { Bar }
    fn main() {
        let foo = Foo::Bar;
    }
    ");
}

#[test]
fn enum_item_in_declaration() {
    test_transform_plain!(GotoDefinition, r#"
    enum Fo<caret>o { Bar }
    fn main() { let _foo = Foo::Bar; }
    "#, @r"
    enum <sel>Foo</sel> { Bar }
    fn main() { let _foo = Foo::Bar; }
    ");
}

#[test]
fn enum_variant_in_declaration() {
    test_transform_plain!(GotoDefinition, r#"
    enum Foo { Ba<caret>r: felt252 }
    fn main() { let _foo = Foo::Bar(0); }
    "#, @r"
    enum Foo { <sel>Bar</sel>: felt252 }
    fn main() { let _foo = Foo::Bar(0); }
    ");
}

#[test]
fn enum_variant_in_expr() {
    test_transform_plain!(GotoDefinition, r#"
    enum Foo { Bar: felt252 }
    fn main() { let foo = Foo::Ba<caret>r(0); }
    "#, @r"
    enum Foo { <sel>Bar</sel>: felt252 }
    fn main() { let foo = Foo::Bar(0); }
    ");
}

#[test]
fn enum_variant_in_pattern() {
    test_transform_plain!(GotoDefinition, r#"
    enum Foo { Bar }
    fn main() {
        let foo = Foo::Bar;
        match foo {
            Foo::Ba<caret>r => {}
        }
    }
    "#, @r"
    enum Foo { <sel>Bar</sel> }
    fn main() {
        let foo = Foo::Bar;
        match foo {
            Foo::Bar => {}
        }
    }
    ");
}

#[test]
fn enum_item_in_expr_with_macros() {
    test_transform_with_macros!(GotoDefinition, r#"
    #[complex_attribute_macro_v2]
    enum Foo { Bar }
    #[complex_attribute_macro_v2]
    fn main() {
        let foo = Fo<caret>o::Bar;
    }
    "#, @r"
    #[complex_attribute_macro_v2]
    enum <sel>Foo</sel> { Bar }
    #[complex_attribute_macro_v2]
    fn main() {
        let foo = Foo::Bar;
    }
    ");
}
