use lsp_types::request::GotoDefinition;

use crate::support::insta::test_transform_and_macros;

#[test]
fn module_in_path1() {
    test_transform_and_macros!(GotoDefinition, r"
    <macro>#[complex_attribute_macro_v2]</macro>
    fn main() {
        modu<caret>le::bar::foo();
    }

    <macro>#[complex_attribute_macro_v2]</macro>
    mod module { // good
        mod module {} // bad
        <macro>#[complex_attribute_macro_v2]</macro>
        fn foo() {}
    }
    ", @r"
    fn main() {
        module::bar::foo();
    }

    mod <sel>module</sel> { // good
        mod module {} // bad
            fn foo() {}
    }

    ==============================

    #[complex_attribute_macro_v2]
    fn main() {
        module::bar::foo();
    }

    #[complex_attribute_macro_v2]
    mod <sel>module</sel> { // good
        mod module {} // bad
        #[complex_attribute_macro_v2]
        fn foo() {}
    }
    ")
}

#[test]
fn module_in_path2() {
    test_transform_and_macros!(GotoDefinition, r"
    <macro>#[complex_attribute_macro_v2]</macro>
    fn main() {
        module::ba<caret>r::foo();
    }

    <macro>#[complex_attribute_macro_v2]</macro>
    mod module {
        <macro>#[complex_attribute_macro_v2]</macro>
        mod bar { // good
            <macro>#[complex_attribute_macro_v2]</macro>
            fn foo() {}
        }
    }
    ", @r"
    fn main() {
        module::bar::foo();
    }

    mod module {
            mod <sel>bar</sel> { // good
                    fn foo() {}
        }
    }

    ==============================

    #[complex_attribute_macro_v2]
    fn main() {
        module::bar::foo();
    }

    #[complex_attribute_macro_v2]
    mod module {
        #[complex_attribute_macro_v2]
        mod <sel>bar</sel> { // good
            #[complex_attribute_macro_v2]
            fn foo() {}
        }
    }
    ")
}

#[test]
fn fn_in_submodule() {
    test_transform_and_macros!(GotoDefinition, r"
    <macro>#[complex_attribute_macro_v2]</macro>
    fn main() {
        module::fo<caret>o();
    }
    fn foo() {} // bad
    <macro>#[complex_attribute_macro_v2]</macro>
    mod module {
        <macro>#[complex_attribute_macro_v2]</macro>
        fn foo() {} // good
    }
    ", @r"
    fn main() {
        module::foo();
    }
    fn foo() {} // bad
    mod module {
            fn <sel>foo</sel>() {} // good
    }

    ==============================

    #[complex_attribute_macro_v2]
    fn main() {
        module::foo();
    }
    fn foo() {} // bad
    #[complex_attribute_macro_v2]
    mod module {
        #[complex_attribute_macro_v2]
        fn <sel>foo</sel>() {} // good
    }
    ")
}

#[test]
fn crate_in_use() {
    test_transform_and_macros!(GotoDefinition, r"
    <macro>#[complex_attribute_macro_v2]</macro>
    use cra<caret>te::foo::func;
    <macro>#[complex_attribute_macro_v2]</macro>
    mod foo {
        <macro>#[complex_attribute_macro_v2]</macro>
        pub fn func() {}
    }
    ", @r"
    <sel>use crate::foo::func;
    mod foo {
            pub fn func() {}
    }</sel>

    ==============================

    <sel>#[complex_attribute_macro_v2]
    use crate::foo::func;
    #[complex_attribute_macro_v2]
    mod foo {
        #[complex_attribute_macro_v2]
        pub fn func() {}
    }</sel>
    ")
}

#[test]
fn crate_in_use_in_submodule() {
    test_transform_and_macros!(GotoDefinition, r"
    mod bar {
        <macro>#[complex_attribute_macro_v2]</macro>
        use cra<caret>te::foo::func;
    }
    <macro>#[complex_attribute_macro_v2]</macro>
    mod foo {
        <macro>#[complex_attribute_macro_v2]</macro>
        pub fn func() {}
    }
    ", @r"
    <sel>mod bar {
            use crate::foo::func;
    }
    mod foo {
            pub fn func() {}
    }</sel>

    ==============================

    <sel>mod bar {
        #[complex_attribute_macro_v2]
        use crate::foo::func;
    }
    #[complex_attribute_macro_v2]
    mod foo {
        #[complex_attribute_macro_v2]
        pub fn func() {}
    }</sel>
    ")
}

#[test]
fn crate_in_path_in_expr() {
    test_transform_and_macros!(GotoDefinition, r"
    <macro>#[complex_attribute_macro_v2]</macro>
    fn main() {
        let _ = cr<caret>ate::foo::func();
    }
    <macro>#[complex_attribute_macro_v2]</macro>
    mod foo {
        <macro>#[complex_attribute_macro_v2]</macro>
        pub fn func() {}
    }
    ", @r"
    <sel>fn main() {
        let _ = crate::foo::func();
    }
    mod foo {
            pub fn func() {}
    }</sel>

    ==============================

    <sel>#[complex_attribute_macro_v2]
    fn main() {
        let _ = crate::foo::func();
    }
    #[complex_attribute_macro_v2]
    mod foo {
        #[complex_attribute_macro_v2]
        pub fn func() {}
    }</sel>
    ")
}

#[test]
fn use_item_via_crate() {
    test_transform_and_macros!(GotoDefinition, r"
    <macro>#[complex_attribute_macro_v2]</macro>
    pub trait Foo<T> {
        fn foo(self: T);
    }
    <macro>#[complex_attribute_macro_v2]</macro>
    mod module {
        <macro>#[complex_attribute_macro_v2]</macro>
        use crate::Fo<caret>o;
    }
    ", @r"
    pub trait <sel>Foo</sel><T> {
        fn foo(self: T);
    }
    mod module {
            use crate::Foo;
    }

    ==============================

    #[complex_attribute_macro_v2]
    pub trait <sel>Foo</sel><T> {
        fn foo(self: T);
    }
    #[complex_attribute_macro_v2]
    mod module {
        #[complex_attribute_macro_v2]
        use crate::Foo;
    }
    ")
}

#[test]
fn use_item_via_super() {
    test_transform_and_macros!(GotoDefinition, r"
    <macro>#[complex_attribute_macro_v2]</macro>
    pub trait Foo<T> {
        fn foo(self: T);
    }

    <macro>#[complex_attribute_macro_v2]</macro>
    mod module {
        <macro>#[complex_attribute_macro_v2]</macro>
        use super::Fo<caret>o;
    }
    ", @r"
    pub trait <sel>Foo</sel><T> {
        fn foo(self: T);
    }

    mod module {
            use super::Foo;
    }

    ==============================

    #[complex_attribute_macro_v2]
    pub trait <sel>Foo</sel><T> {
        fn foo(self: T);
    }

    #[complex_attribute_macro_v2]
    mod module {
        #[complex_attribute_macro_v2]
        use super::Foo;
    }
    ")
}
