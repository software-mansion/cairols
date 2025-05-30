use lsp_types::request::GotoDefinition;

use crate::support::insta::{test_transform_plain, test_transform_with_macros};

#[test]
fn module_in_path1() {
    test_transform_plain!(GotoDefinition, r"
    fn main() {
        modu<caret>le::bar::foo();
    }
    mod module { // good
        mod module {} // bad
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
    ")
}

#[test]
fn module_in_path2() {
    test_transform_plain!(GotoDefinition, r"
    fn main() {
        module::ba<caret>r::foo();
    }
    mod module {
        mod bar { // good
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
    ")
}

#[test]
fn fn_in_submodule() {
    test_transform_plain!(GotoDefinition, r"
    fn main() {
        module::fo<caret>o();
    }
    fn foo() {} // bad
    mod module {
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
    ")
}

#[test]
fn crate_in_use() {
    test_transform_plain!(GotoDefinition, r"
    use cra<caret>te::foo::func;
    mod foo {
        pub fn func() {}
    }
    ", @r"
    <sel>use crate::foo::func;
    mod foo {
        pub fn func() {}
    }</sel>
    ")
}

#[test]
fn crate_in_use_in_submodule() {
    test_transform_plain!(GotoDefinition, r"
    mod bar {
        use cra<caret>te::foo::func;
    }
    mod foo {
        pub fn func() {}
    }
    ", @r"
    <sel>mod bar {
        use crate::foo::func;
    }
    mod foo {
        pub fn func() {}
    }</sel>
    ")
}

#[test]
fn crate_in_path_in_expr() {
    test_transform_plain!(GotoDefinition, r"
    fn main() {
        let _ = cr<caret>ate::foo::func();
    }
    mod foo {
        pub fn func() {}
    }
    ", @r"
    <sel>fn main() {
        let _ = crate::foo::func();
    }
    mod foo {
        pub fn func() {}
    }</sel>
    ")
}

#[test]
fn use_item_via_crate() {
    test_transform_plain!(GotoDefinition, r"
    pub trait Foo<T> {
        fn foo(self: T);
    }
    mod module {
        use crate::Fo<caret>o;
    }
    ", @r"
    pub trait <sel>Foo</sel><T> {
        fn foo(self: T);
    }
    mod module {
        use crate::Foo;
    }
    ")
}

#[test]
fn use_item_via_super() {
    test_transform_plain!(GotoDefinition, r"
    pub trait Foo<T> {
        fn foo(self: T);
    }
    mod module {
        use super::Fo<caret>o;
    }
    ", @r"
    pub trait <sel>Foo</sel><T> {
        fn foo(self: T);
    }
    mod module {
        use super::Foo;
    }
    ")
}

#[test]
fn module_in_path1_with_macros() {
    test_transform_with_macros!(GotoDefinition, r"
    #[complex_attribute_macro_v2]
    fn main() {
        modu<caret>le::bar::foo();
    }
    #[complex_attribute_macro_v2]
    mod module { // good
        mod module {} // bad
        fn foo() {}
    }
    ", @r"
    #[complex_attribute_macro_v2]
    fn main() {
        module::bar::foo();
    }
    #[complex_attribute_macro_v2]
    mod <sel>module</sel> { // good
        mod module {} // bad
        fn foo() {}
    }
    ")
}

// FIXME(#721)
#[test]
fn crate_in_use_in_submodule_with_macros() {
    test_transform_with_macros!(GotoDefinition, r"
    #[complex_attribute_macro_v2]
    mod bar {
        #[complex_attribute_macro_v2]
        use cra<caret>te::foo::func;
    }

    #[complex_attribute_macro_v2]
    mod foo {
        #[complex_attribute_macro_v2]
        pub fn func() {}
    }
    ", @"none response")
}

// FIXME(#721)
#[test]
fn use_item_via_crate_with_macros() {
    test_transform_with_macros!(GotoDefinition, r"
    #[complex_attribute_macro_v2]
    pub trait Foo<T> {
        fn foo(self: T);
    }

    #[complex_attribute_macro_v2]
    mod module {
        #[complex_attribute_macro_v2]
        use crate::Fo<caret>o;
    }
    ", @"none response")
}

// FIXME(#721)
#[test]
fn use_item_via_super_with_macros() {
    test_transform_with_macros!(GotoDefinition, r"
    #[complex_attribute_macro_v2]
    pub trait Foo<T> {
        fn foo(self: T);
    }

    #[complex_attribute_macro_v2]
    mod module {
        #[complex_attribute_macro_v2]
        use super::Fo<caret>o;
    }
    ", @"none response")
}
