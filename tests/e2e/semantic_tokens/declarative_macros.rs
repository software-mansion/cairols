use lsp_types::SemanticTokens;

use crate::support::insta::test_transform_plain;

// Bug: `x` inside the invocation is colored as the macro instead of a variable.
#[test]
fn inline_macro_content_variable() {
    test_transform_plain!(SemanticTokens, r#"
    fn main() {
        let x = 5;
        array![x];
    }
    "#, @r"
    <token=keyword>fn</token> <token=function>main</token>() {
        <token=keyword>let</token> <token=variable>x</token> = <token=number>5</token>;
        <token=macro>array</token><token=macro>!</token>[<token=macro>x</token>];
    }
    ")
}

// Bug: the variable, enum, variant and function inside the invocation are all
// colored as the macro instead of their real kinds.
#[test]
fn inline_macro_content_expressions() {
    test_transform_plain!(SemanticTokens, r#"
    enum Foo {
        Bar,
    }

    fn make() -> felt252 {
        0
    }

    fn main() {
        let x = 5;
        array![x, Foo::Bar, make()];
    }
    "#, @r"
    <token=keyword>enum</token> <token=enum>Foo</token> {
        <token=enumMember>Bar</token>,
    }

    <token=keyword>fn</token> <token=function>make</token>() -> <token=type>felt252</token> {
        <token=number>0</token>
    }

    <token=keyword>fn</token> <token=function>main</token>() {
        <token=keyword>let</token> <token=variable>x</token> = <token=number>5</token>;
        <token=macro>array</token><token=macro>!</token>[<token=macro>x</token>, <token=macro>Foo</token>::<token=macro>Bar</token>, <token=macro>make</token>()];
    }
    ")
}

// Bug: `x` inside the invocation is colored as the macro instead of a variable.
#[test]
fn inline_macro_content_operators_and_literals() {
    test_transform_plain!(SemanticTokens, r#"
    fn main() {
        let x = 5;
        array![x + 1, "abc"];
    }
    "#, @r#"
    <token=keyword>fn</token> <token=function>main</token>() {
        <token=keyword>let</token> <token=variable>x</token> = <token=number>5</token>;
        <token=macro>array</token><token=macro>!</token>[<token=macro>x</token> <token=operator>+</token> <token=number>1</token>, <token=string>"abc"</token>];
    }
    "#)
}

// Bug: the field access and method call inside the invocation are colored as the
// macro instead of their real kinds.
#[test]
fn inline_macro_content_field_and_method() {
    test_transform_plain!(SemanticTokens, r#"
    struct Point {
        x: felt252,
    }

    #[generate_trait]
    impl PointImpl of PointTrait {
        fn get(self: @Point) -> felt252 {
            *self.x
        }
    }

    fn main() {
        let p = Point { x: 5 };
        array![p.x, p.get()];
    }
    "#, @r"
    <token=keyword>struct</token> <token=struct>Point</token> {
        <token=variable>x</token>: <token=type>felt252</token>,
    }

    #[<token=decorator>generate_trait</token>]
    <token=keyword>impl</token> <token=class>PointImpl</token> <token=keyword>of</token> <token=interface>PointTrait</token> {
        <token=keyword>fn</token> <token=function>get</token>(<token=parameter>self</token>: @<token=struct>Point</token>) -> <token=type>felt252</token> {
            <token=operator>*</token><token=variable>self</token>.x
        }
    }

    <token=keyword>fn</token> <token=function>main</token>() {
        <token=keyword>let</token> <token=variable>p</token> = <token=struct>Point</token> { <token=property>x</token>: <token=number>5</token> };
        <token=macro>array</token><token=macro>!</token>[<token=macro>p</token>.<token=macro>x</token>, <token=macro>p</token>.<token=macro>get</token>()];
    }
    ")
}

// Bug: the module path and function call inside the invocation are colored as the
// macro instead of a namespace and a function.
#[test]
fn inline_macro_content_path_and_call() {
    test_transform_plain!(SemanticTokens, r#"
    mod utils {
        pub fn helper() -> felt252 {
            0
        }
    }

    fn main() {
        array![utils::helper()];
    }
    "#, @r"
    <token=keyword>mod</token> <token=namespace>utils</token> {
        <token=keyword>pub</token> <token=keyword>fn</token> <token=function>helper</token>() -> <token=type>felt252</token> {
            <token=number>0</token>
        }
    }

    <token=keyword>fn</token> <token=function>main</token>() {
        <token=macro>array</token><token=macro>!</token>[<token=macro>utils</token>::<token=macro>helper</token>()];
    }
    ")
}

// Bug: the struct name and field inside the invocation are colored as the macro
// instead of a struct and a field.
#[test]
fn inline_macro_content_struct_literal() {
    test_transform_plain!(SemanticTokens, r#"
    struct Point {
        x: felt252,
    }

    fn main() {
        let x = 5;
        array![Point { x }];
    }
    "#, @r"
    <token=keyword>struct</token> <token=struct>Point</token> {
        <token=variable>x</token>: <token=type>felt252</token>,
    }

    <token=keyword>fn</token> <token=function>main</token>() {
        <token=keyword>let</token> <token=variable>x</token> = <token=number>5</token>;
        <token=macro>array</token><token=macro>!</token>[<token=macro>Point</token> { <token=macro>x</token> }];
    }
    ")
}

// Bug: the inner macro's `!` is colored as an operator, and its content is colored
// as the macro.
#[test]
fn nested_inline_macros() {
    test_transform_plain!(SemanticTokens, r#"
    fn main() {
        let x = 5;
        array![array![x]];
    }
    "#, @r"
    <token=keyword>fn</token> <token=function>main</token>() {
        <token=keyword>let</token> <token=variable>x</token> = <token=number>5</token>;
        <token=macro>array</token><token=macro>!</token>[<token=macro>array</token><token=operator>!</token>[<token=macro>x</token>]];
    }
    ")
}

// Bug: the interpolated variable is colored as the macro instead of a variable.
#[test]
fn format_macro_content() {
    test_transform_plain!(SemanticTokens, r#"
    fn main() {
        let x = 5;
        println!("{}", x);
    }
    "#, @r#"
    <token=keyword>fn</token> <token=function>main</token>() {
        <token=keyword>let</token> <token=variable>x</token> = <token=number>5</token>;
        <token=macro>println</token><token=macro>!</token>(<token=string>"{}"</token>, <token=macro>x</token>);
    }
    "#)
}

// Bug: the macro name in its definition is colored as a class, and the variable in
// the invocation is colored as the macro.
#[test]
fn user_defined_macro() {
    test_transform_plain!(SemanticTokens, r#"
    pub macro add_one {
        ($x: expr) => { $x + 1 };
    }

    fn main() {
        let x = 5;
        add_one!(x);
    }
    "#, @r"
    <token=keyword>pub</token> <token=keyword>macro</token> <token=class>add_one</token> {
        ($x: expr) => { $x <token=operator>+</token> <token=number>1</token> };
    }

    <token=keyword>fn</token> <token=function>main</token>() {
        <token=keyword>let</token> <token=variable>x</token> = <token=number>5</token>;
        <token=macro>add_one</token><token=macro>!</token>(<token=macro>x</token>);
    }
    ")
}

// Bug: the function call inside the item-level macro is not highlighted at all.
#[test]
fn item_level_macro_content() {
    test_transform_plain!(SemanticTokens, r#"
    fn make() -> felt252 {
        0
    }

    consteval_int!(make());
    "#, @r"
    <token=keyword>fn</token> <token=function>make</token>() -> <token=type>felt252</token> {
        <token=number>0</token>
    }

    <token=macro>consteval_int</token><token=macro>!</token>(make());
    ")
}

// Bug: the macro name in its definition is colored as a class.
#[test]
fn module_level_user_macro() {
    test_transform_plain!(SemanticTokens, r#"
    macro define_fn {
        ($name:ident) => {
            expose! {
                fn $name() -> felt252 {
                    42
                }
            }
        };
    }

    define_fn!(the_answer);

    fn main() -> felt252 {
        the_answer()
    }
    "#, @r"
    <token=keyword>macro</token> <token=class>define_fn</token> {
        ($name:ident) => {
            expose<token=operator>!</token> {
                <token=keyword>fn</token> $name() -> felt252 {
                    <token=number>42</token>
                }
            }
        };
    }

    <token=macro>define_fn</token><token=macro>!</token>(the_answer);

    <token=keyword>fn</token> <token=function>main</token>() -> <token=type>felt252</token> {
        <token=function>the_answer</token>()
    }
    ")
}
