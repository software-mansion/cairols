use crate::semantic_tokens::semantic_tokens;
use crate::support::insta::test_transform;

#[test]
fn complex() {
    test_transform!(semantic_tokens, r#"
    enum Foo {
        Bar,
        Baz,
    }

    fn main() {
        let foo = Foo::Bar;
        let foobar: Foo = foo;
    }

    fn calc(foo: Foo) {}

    #[cfg(test)]
    mod rectangle {
        use super::Foo;
    }

    mod b {
        mod a {
            mod trick {
                #[test]
                struct Foo {}
            }
        }
    }
    "#, @r"
    <token=keyword>enum</token> <token=enum>Foo</token> {
        <token=enumMember>Bar</token>,
        <token=enumMember>Baz</token>,
    }

    <token=keyword>fn</token> <token=function>main</token>() {
        <token=keyword>let</token> <token=variable>foo</token> = <token=type>Foo</token>::<token=enumMember>Bar</token>;
        <token=keyword>let</token> <token=variable>foobar</token>: <token=type>Foo</token> = <token=variable>foo</token>;
    }

    <token=keyword>fn</token> <token=function>calc</token>(<token=parameter>foo</token>: <token=type>Foo</token>) {}

    #[<token=decorator>cfg</token>(<token=decorator>test</token>)]
    <token=keyword>mod</token> <token=class>rectangle</token> {
        <token=keyword>use</token> <token=keyword>super</token>::<token=type>Foo</token>;
    }

    <token=keyword>mod</token> <token=class>b</token> {
        <token=keyword>mod</token> <token=class>a</token> {
            <token=keyword>mod</token> <token=class>trick</token> {
                #[<token=decorator>test</token>]
                <token=keyword>struct</token> <token=struct>Foo</token> {}
            }
        }
    }
    ")
}

#[test]
fn multiline() {
    test_transform!(semantic_tokens, r#"
    fn main() {
        let _ = "
        ";
    }
    "#, @r#"
    <token=keyword>fn</token> <token=function>main</token>() {
        <token=keyword>let</token> _ = <token=string>"</token>
    <token=string>    "</token>;
    }
    "#)
}

#[test]
fn on_mod() {
    test_transform!(semantic_tokens, r#"
    #[cfg(test, 1234)]
    mod rectangle { }
    "#, @r"
    #[<token=decorator>cfg</token>(<token=decorator>test</token>, <token=number>1234</token>)]
    <token=keyword>mod</token> <token=class>rectangle</token> { }
    ")
}

#[test]
fn on_fn() {
    test_transform!(semantic_tokens, r#"
    #[cfg(test, 1234)]
    fn rectangle() { }
    "#, @r"
    #[<token=decorator>cfg</token>(<token=decorator>test</token>, <token=number>1234</token>)]
    <token=keyword>fn</token> <token=function>rectangle</token>() { }
    ")
}

#[test]
fn consts() {
    test_transform!(semantic_tokens, r#"
    const STANDALONE: u32 = 3;

    trait Shape<T> {
        const SIDES: u8;
    }

    impl UnitShape of Shape<()> {
        const SIDES: u8 = 0;
    }

    fn func() {
        let x = STANDALONE;
        UnitShape::SIDES;
    }
    "#, @r"
    <token=keyword>const</token> <token=enumMember>STANDALONE</token>: <token=type>u32</token> = <token=number>3</token>;

    <token=keyword>trait</token> <token=class>Shape</token><token=operator><</token><token=typeParameter>T</token><token=operator>></token> {
        <token=keyword>const</token> <token=enumMember>SIDES</token>: <token=type>u8</token>;
    }

    <token=keyword>impl</token> <token=class>UnitShape</token> <token=keyword>of</token> <token=interface>Shape</token><token=operator><</token>()<token=operator>></token> {
        <token=keyword>const</token> <token=enumMember>SIDES</token>: <token=type>u8</token> = <token=number>0</token>;
    }

    <token=keyword>fn</token> <token=function>func</token>() {
        <token=keyword>let</token> <token=variable>x</token> = <token=enumMember>STANDALONE</token>;
        <token=class>UnitShape</token>::<token=enumMember>SIDES</token>;
    }
    ")
}

#[test]
fn inline_macro_with_same_name_as_module() {
    test_transform!(semantic_tokens, r#"
    use core::array;

    fn main() {
        array![5];
    }
    "#, @r"
    <token=keyword>use</token> <token=namespace>core</token>::<token=namespace>array</token>;

    <token=keyword>fn</token> <token=function>main</token>() {
        <token=macro>array</token><token=macro>!</token>[<token=number>5</token>];
    }
    ")
}

#[test]
fn inline_macro_with_same_name_as_enum() {
    test_transform!(semantic_tokens, r#"
    enum array {
        Abc
    }

    fn main() {
        array![5];
    }
    "#, @r"
    <token=keyword>enum</token> <token=enum>array</token> {
        <token=enumMember>Abc</token>
    }

    <token=keyword>fn</token> <token=function>main</token>() {
        <token=macro>array</token><token=macro>!</token>[<token=number>5</token>];
    }
    ")
}

#[test]
fn inline_macro_with_same_name_as_trait() {
    test_transform!(semantic_tokens, r#"
    trait array { }

    fn main() {
        array![5];
    }
    "#, @r"
    <token=keyword>trait</token> <token=class>array</token> { }

    <token=keyword>fn</token> <token=function>main</token>() {
        <token=macro>array</token><token=macro>!</token>[<token=number>5</token>];
    }
    ")
}

#[test]
fn doc_comment_with_link() {
    test_transform!(semantic_tokens, r#"
    //! Doc header comment
    //! And a one with a [`Link`]
    //! Plz refer to this external [source](https://zal.pl)

    /// A doc commment over an enum referring to [`Bar`]
    enum Foo {
        Bar,
        Baz,
    }

    /// A Function which plays with [`Foo`]
    /// I can also use relative [links](../Cargo.toml) and they will highlight correctly
    fn main() {
        let foo = Foo::Bar;
        let foobar: Foo = foo; /// Inline usage of [`Foo`]
    }


    fn calc(foo: Foo) {}

    /// Something over an attribute referring to [`Foo`]
    #[cfg(test)]
    mod rectangle {
        use super::Foo;
    }

    mod b {
        mod a {
            /// A module inside a module with a [`Foo`] inside
            mod trick {
                #[test]
                struct Foo {}
            }
        }
    }
    "#, @r"
    //! Doc header comment
    //! And a one with a <token=class>[`Link`]</token>
    //! Plz refer to this external <token=class>[source](https://zal.pl)</token>

    /// A doc commment over an enum referring to <token=class>[`Bar`]</token>
    <token=keyword>enum</token> <token=enum>Foo</token> {
        <token=enumMember>Bar</token>,
        <token=enumMember>Baz</token>,
    }

    /// A Function which plays with <token=class>[`Foo`]</token>
    /// I can also use relative <token=class>[links](../Cargo.toml)</token> and they will highlight correctly
    <token=keyword>fn</token> <token=function>main</token>() {
        <token=keyword>let</token> <token=variable>foo</token> = <token=type>Foo</token>::<token=enumMember>Bar</token>;
        <token=keyword>let</token> <token=variable>foobar</token>: <token=type>Foo</token> = <token=variable>foo</token>; /// Inline usage of <token=class>[`Foo`]</token>
    }


    <token=keyword>fn</token> <token=function>calc</token>(<token=parameter>foo</token>: <token=type>Foo</token>) {}

    /// Something over an attribute referring to <token=class>[`Foo`]</token>
    #[<token=decorator>cfg</token>(<token=decorator>test</token>)]
    <token=keyword>mod</token> <token=class>rectangle</token> {
        <token=keyword>use</token> <token=keyword>super</token>::<token=type>Foo</token>;
    }

    <token=keyword>mod</token> <token=class>b</token> {
        <token=keyword>mod</token> <token=class>a</token> {
            /// A module inside a module with a <token=class>[`Foo`]</token> inside
            <token=keyword>mod</token> <token=class>trick</token> {
                #[<token=decorator>test</token>]
                <token=keyword>struct</token> <token=struct>Foo</token> {}
            }
        }
    }
    ")
}
