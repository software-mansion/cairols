use lsp_types::request::GotoDefinition;

use crate::support::insta::test_transform_plain;

#[test]
fn crate_path_in_doc_link() {
    test_transform_plain!(GotoDefinition, r"
    mod inner {
        /// See [`crate::Structu<caret>re`].
        fn foo() {}
    }
    struct Structure {}
    ", @r"
    mod inner {
        /// See [`crate::Structure`].
        fn foo() {}
    }
    struct <sel>Structure</sel> {}
    ")
}

#[test]
fn super_path_in_doc_link() {
    test_transform_plain!(GotoDefinition, r"
    mod parent {
        pub struct Thing {}
        mod child {
            /// See [`super::Thi<caret>ng`].
            fn foo() {}
        }
    }
    ", @r"
    mod parent {
        pub struct <sel>Thing</sel> {}
        mod child {
            /// See [`super::Thing`].
            fn foo() {}
        }
    }
    ")
}

#[test]
fn nested_path_in_doc_link() {
    test_transform_plain!(GotoDefinition, r"
    /// See [`my_module::something::Stru<caret>ct`].
    fn foo() {}
    mod my_module {
        pub mod something {
            pub struct Struct {}
        }
    }
    ", @r"
    /// See [`my_module::something::Struct`].
    fn foo() {}
    mod my_module {
        pub mod something {
            pub struct <sel>Struct</sel> {}
        }
    }
    ")
}

#[test]
fn doc_link_resolves_to_path_prefix_segment() {
    test_transform_plain!(GotoDefinition, r"
    /// See [`my_module::some<caret>thing::Struct`].
    fn foo() {}
    mod my_module {
        pub mod something {
            pub struct Struct {}
        }
    }
    ", @r"
    /// See [`my_module::something::Struct`].
    fn foo() {}
    mod my_module {
        pub mod <sel>something</sel> {
            pub struct Struct {}
        }
    }
    ")
}

#[test]
fn doc_link_unresolved_path() {
    test_transform_plain!(GotoDefinition, r"
    /// See [`crate::Missin<caret>g`].
    fn foo() {}
    ", @r"
    none response
    ")
}

#[test]
fn doc_link_cursor_outside_label() {
    test_transform_plain!(GotoDefinition, r"
    /// See [<caret>`crate::Struct`].
    struct Struct {}
    ", @r"
    none response
    ")
}

#[test]
fn doc_link_to_corelib() {
    test_transform_plain!(GotoDefinition, r"
    /// See [`core::option::Opti<caret>on`].
    fn foo() {}
    ", @r"
    // → core/src/option.cairo
    pub enum <sel>Option</sel><T> {
    ")
}
