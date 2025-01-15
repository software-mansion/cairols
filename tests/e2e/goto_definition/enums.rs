use crate::goto_definition::goto_definition;
use crate::support::insta::test_transform;

#[test]
fn enum_item_in_type() {
    test_transform!(goto_definition, r#"
    enum Foo { Bar }
    fn calc(foo: Fo<caret>o) {}
    "#, @r"
    <sel>enum Foo { Bar }</sel>
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
    <sel>enum Foo { Bar }</sel>
    fn main() {
        let foo = Foo::Bar;
    }
    ");
}

#[test]
fn enum_variant_in_expr() {
    test_transform!(goto_definition, r#"
    enum Foo { Bar }
    fn main() { let foo = Foo::Ba<caret>r; }
    "#, @r"
    enum Foo { <sel>Bar</sel> }
    fn main() { let foo = Foo::Bar; }
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
