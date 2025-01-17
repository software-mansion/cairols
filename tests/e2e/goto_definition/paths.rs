use crate::goto_definition::goto_definition;
use crate::support::insta::test_transform;

#[test]
fn module_in_path1() {
    test_transform!(goto_definition, r"
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
    test_transform!(goto_definition, r"
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
    test_transform!(goto_definition, r"
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
    test_transform!(goto_definition, r"
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
    test_transform!(goto_definition, r"
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
    test_transform!(goto_definition, r"
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
    test_transform!(goto_definition, r"
    pub trait Foo<T> {
        fn foo(self: T);
    }
    mod module {
        use crate::Fo<caret>o;
    }
    ", @r"
    <sel>pub trait Foo<T> {
        fn foo(self: T);
    }</sel>
    mod module {
        use crate::Foo;
    }
    ")
}

#[test]
fn use_item_via_super() {
    test_transform!(goto_definition, r"
    pub trait Foo<T> {
        fn foo(self: T);
    }
    mod module {
        use super::Fo<caret>o;
    }
    ", @r"
    <sel>pub trait Foo<T> {
        fn foo(self: T);
    }</sel>
    mod module {
        use super::Foo;
    }
    ")
}
