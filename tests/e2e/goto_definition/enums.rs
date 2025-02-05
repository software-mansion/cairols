use crate::goto_definition::goto_definition;
use crate::support::insta::test_transform;

#[test]
fn enum_item_in_type() {
    test_transform!(goto_definition, r#"
    enum Foo { Bar }
    fn calc(foo: Fo<caret>o) {}
    "#, @r"
    enum <sel>Foo</sel> { Bar }
    fn calc(foo: Foo) {}
    ");
}

#[test]
fn enum_item_in_expr() {
    test_transform!(goto_definition, r#"
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
    test_transform!(goto_definition, r#"
    enum Fo<caret>o { Bar }
    fn main() { let _foo = Foo::Bar; }
    "#, @r"
    enum <sel>Foo</sel> { Bar }
    fn main() { let _foo = Foo::Bar; }
    ");
}

#[test]
fn enum_variant_in_declaration() {
    test_transform!(goto_definition, r#"
    enum Foo { Ba<caret>r: felt252 }
    fn main() { let _foo = Foo::Bar(0); }
    "#, @r"
    enum Foo { <sel>Bar</sel>: felt252 }
    fn main() { let _foo = Foo::Bar(0); }
    ");
}

#[test]
fn enum_variant_in_expr() {
    test_transform!(goto_definition, r#"
    enum Foo { Bar: felt252 }
    fn main() { let foo = Foo::Ba<caret>r(0); }
    "#, @r"
    enum Foo { <sel>Bar</sel>: felt252 }
    fn main() { let foo = Foo::Bar(0); }
    ");
}

#[test]
fn enum_variant_in_pattern() {
    test_transform!(goto_definition, r#"
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
