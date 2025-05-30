use crate::goto_definition::goto_definition;
use crate::support::insta::test_transform_and_macros;

#[test]
fn enum_item_in_type() {
    test_transform_and_macros!(goto_definition, r#"
    <macro>#[simple_attribute_macro_v2]</macro>
    enum Foo { Bar }

    <macro>#[simple_attribute_macro_v2]</macro>
    fn calc(foo: Fo<caret>o) {}
    "#, @r"
    enum <sel>Foo</sel> { Bar }


    fn calc(foo: Foo) {}

    ===== WITH MACROS =====

    #[simple_attribute_macro_v2]
    enum <sel>Foo</sel> { Bar }

    #[simple_attribute_macro_v2]
    fn calc(foo: Foo) {}
    ");
}

#[test]
fn enum_item_in_expr() {
    test_transform_and_macros!(goto_definition, r#"
    <macro>#[simple_attribute_macro_v2]</macro>
    enum Foo { Bar }

    <macro>#[simple_attribute_macro_v2]</macro>
    fn main() {
        let foo = Fo<caret>o::Bar;
    }
    "#, @r"
    enum <sel>Foo</sel> { Bar }


    fn main() {
        let foo = Foo::Bar;
    }

    ===== WITH MACROS =====

    #[simple_attribute_macro_v2]
    enum <sel>Foo</sel> { Bar }

    #[simple_attribute_macro_v2]
    fn main() {
        let foo = Foo::Bar;
    }
    ");
}

#[test]
fn enum_item_in_declaration() {
    test_transform_and_macros!(goto_definition, r#"
    <macro>#[simple_attribute_macro_v2]</macro>
    enum Fo<caret>o { Bar }

    <macro>#[simple_attribute_macro_v2]</macro>
    fn main() { let _foo = Foo::Bar; }
    "#, @r"
    enum <sel>Foo</sel> { Bar }


    fn main() { let _foo = Foo::Bar; }

    ===== WITH MACROS =====

    #[simple_attribute_macro_v2]
    enum <sel>Foo</sel> { Bar }

    #[simple_attribute_macro_v2]
    fn main() { let _foo = Foo::Bar; }
    ");
}

#[test]
fn enum_variant_in_declaration() {
    test_transform_and_macros!(goto_definition, r#"
    <macro>#[simple_attribute_macro_v2]</macro>
    enum Foo { Ba<caret>r: felt252 }

    <macro>#[simple_attribute_macro_v2]</macro>
    fn main() { let _foo = Foo::Bar(0); }
    "#, @r"
    enum Foo { <sel>Bar</sel>: felt252 }


    fn main() { let _foo = Foo::Bar(0); }

    ===== WITH MACROS =====

    #[simple_attribute_macro_v2]
    enum Foo { <sel>Bar</sel>: felt252 }

    #[simple_attribute_macro_v2]
    fn main() { let _foo = Foo::Bar(0); }
    ");
}

#[test]
fn enum_variant_in_expr() {
    test_transform_and_macros!(goto_definition, r#"
    <macro>#[simple_attribute_macro_v2]</macro>
    enum Foo { Bar: felt252 }

    <macro>#[simple_attribute_macro_v2]</macro>
    fn main() { let foo = Foo::Ba<caret>r(0); }
    "#, @r"
    enum Foo { <sel>Bar</sel>: felt252 }


    fn main() { let foo = Foo::Bar(0); }

    ===== WITH MACROS =====

    #[simple_attribute_macro_v2]
    enum Foo { <sel>Bar</sel>: felt252 }

    #[simple_attribute_macro_v2]
    fn main() { let foo = Foo::Bar(0); }
    ");
}

#[test]
fn enum_variant_in_pattern() {
    test_transform_and_macros!(goto_definition, r#"
    <macro>#[simple_attribute_macro_v2]</macro>
    enum Foo { Bar }

    <macro>#[simple_attribute_macro_v2]</macro>
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

    ===== WITH MACROS =====

    #[simple_attribute_macro_v2]
    enum Foo { <sel>Bar</sel> }

    #[simple_attribute_macro_v2]
    fn main() {
        let foo = Foo::Bar;
        match foo {
            Foo::Bar => {}
        }
    }
    ");
}
