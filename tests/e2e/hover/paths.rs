use crate::hover::test_hover;
use crate::support::insta::test_transform;

#[test]
fn simple_mod() {
    test_transform!(test_hover,r#"
    /// some_module docstring.
    mod some_<caret>module { }
    "#,@r#"
    source_context = """
    mod some_<caret>module { }
    """
    highlight = """
    mod <sel>some_module</sel> { }
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    mod some_module
    ```
    ---
    some_module docstring."""
    "#)
}

#[test]
fn simple_nested_mod() {
    test_transform!(test_hover,r#"
    /// some_module docstring.
    mod some_module {
        /// internal_module docstring.
        pub mod internal_module<caret> { }
    }
    "#,@r#"
    source_context = """
        pub mod internal_module<caret> { }
    """
    highlight = """
        pub mod <sel>internal_module</sel> { }
    """
    popover = """
    ```cairo
    hello::some_module
    ```
    ```cairo
    mod internal_module
    ```
    ---
    internal_module docstring."""
    "#)
}

#[test]
fn use_in_nested() {
    test_transform!(test_hover,r#"
    /// some_module docstring.
    mod some_module {
        /// internal_module docstring.
        pub mod internal_module { }

        /// internal_module2 docstring.
        pub mod internal_module2 {
            use super::internal_module<caret>;
        }
    }
    "#,@r#"
    source_context = """
            use super::internal_module<caret>;
    """
    highlight = """
            use super::<sel>internal_module</sel>;
    """
    popover = """
    ```cairo
    hello::some_module
    ```
    ```cairo
    mod internal_module
    ```
    ---
    internal_module docstring."""
    "#)
}

#[test]
fn mod_in_nested() {
    test_transform!(test_hover,r#"
    /// some_module docstring.
    mod some_module {
        /// internal_module2 docstring.
        pub mod internal_module2 {
            /// private_submodule docstring.
            mod <caret>private_submodule {
                struct PrivateStruct2 {}
            }
        }
    }
    "#,@r#"
    source_context = """
            mod <caret>private_submodule {
    """
    highlight = """
            mod <sel>private_submodule</sel> {
    """
    popover = """
    ```cairo
    hello::some_module::internal_module2
    ```
    ```cairo
    mod private_submodule
    ```
    ---
    private_submodule docstring."""
    "#)
}

#[test]
fn super_mod() {
    test_transform!(test_hover,r#"
    /// some_module docstring.
    mod some_module {
        /// internal_module docstring.
        pub mod internal_module { }
    }

    mod happy_cases {
        use super::some_<caret>module;
    }
    "#,@r#"
    source_context = """
        use super::some_<caret>module;
    """
    highlight = """
        use super::<sel>some_module</sel>;
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    mod some_module
    ```
    ---
    some_module docstring."""
    "#)
}

#[test]
fn super_mod_nested_trait() {
    test_transform!(test_hover,r#"
    /// some_module docstring.
    mod some_module {
        /// internal_module docstring.
        pub mod internal_module {
            pub trait MyTrait<T> {
                fn foo(t: T);
            }
        }
    }

    mod happy_cases {
        use super::some_module::internal_<caret>module::MyTrait;
    }
    "#,@r#"
    source_context = """
        use super::some_module::internal_<caret>module::MyTrait;
    """
    highlight = """
        use super::some_module::<sel>internal_module</sel>::MyTrait;
    """
    popover = """
    ```cairo
    hello::some_module
    ```
    ```cairo
    mod internal_module
    ```
    ---
    internal_module docstring."""
    "#)
}

#[test]
fn super_mod_nested_nested() {
    test_transform!(test_hover,r#"
    /// some_module docstring.
    mod some_module {
        /// internal_module docstring.
        pub mod internal_module {
            pub mod nested_internal_module { }
        }
    }

    mod happy_cases {
        use super::some_module::internal_module::nested_inte<caret>rnal_module;
    }
    "#,@r#"
    source_context = """
        use super::some_module::internal_module::nested_inte<caret>rnal_module;
    """
    highlight = """
        use super::some_module::internal_module::<sel>nested_internal_module</sel>;
    """
    popover = """
    ```cairo
    hello::some_module::internal_module
    ```
    ```cairo
    mod nested_internal_module
    ```
    """
    "#)
}

#[test]
fn trait_impl() {
    test_transform!(test_hover,r#"
    /// some_module docstring.
    mod some_module {
        /// internal_module docstring.
        pub mod internal_module {
            pub trait MyTrait<T> {
                fn foo(t: T);
            }
        }
    }

    mod happy_cases {
        impl TraitImpl of super::some_module::internal<caret>_module::MyTrait<felt252> {
            fn foo(t: felt252) {}
        }
    }
    "#,@r#"
    source_context = """
        impl TraitImpl of super::some_module::internal<caret>_module::MyTrait<felt252> {
    """
    highlight = """
        impl TraitImpl of super::some_module::<sel>internal_module</sel>::MyTrait<felt252> {
    """
    popover = """
    ```cairo
    hello::some_module
    ```
    ```cairo
    mod internal_module
    ```
    ---
    internal_module docstring."""
    "#)
}

#[test]
fn trait_impl_partial() {
    test_transform!(test_hover,r#"
    /// some_module docstring.
    mod some_module {
        /// internal_module docstring.
        pub mod internal_module {
            pub trait MyTrait<T> {
                fn foo(t: T);
            }
        }
    }

    mod happy_cases {
        use super::some_module::internal_module;

        impl TraitImpl of internal<caret>_module::MyTrait<felt252> {
            fn foo(t: felt252) {}
        }
    }
    "#,@r#"
    source_context = """
        impl TraitImpl of internal<caret>_module::MyTrait<felt252> {
    """
    highlight = """
        impl TraitImpl of <sel>internal_module</sel>::MyTrait<felt252> {
    """
    popover = """
    ```cairo
    hello::some_module
    ```
    ```cairo
    mod internal_module
    ```
    ---
    internal_module docstring."""
    "#)
}

#[test]
fn function_with_path_super() {
    test_transform!(test_hover,r#"
    /// some_module docstring.
    mod some_module {
        /// internal_module docstring.
        pub mod internal_module {
            pub mod nested_internal_module {
                pub fn foo() {}
            }
        }
    }

    mod happy_cases {
        fn function_with_path() {
            <caret>super::some_module::internal_module::nested_internal_module::foo();
        }
    }
    "#,@r#"
    source_context = """
            <caret>super::some_module::internal_module::nested_internal_module::foo();
    """
    "#)
}

#[test]
fn function_with_path_middle() {
    test_transform!(test_hover,r#"
    /// some_module docstring.
    mod some_module {
        /// internal_module docstring.
        pub mod internal_module {
            pub mod nested_internal_module {
                pub fn foo() {}
            }
        }
    }

    mod happy_cases {
        fn function_with_path() {
            super::some_module::inte<caret>rnal_module::nested_internal_module::foo();
        }
    }
    "#,@r#"
    source_context = """
            super::some_module::inte<caret>rnal_module::nested_internal_module::foo();
    """
    highlight = """
            super::some_module::<sel>internal_module</sel>::nested_internal_module::foo();
    """
    popover = """
    ```cairo
    hello::some_module
    ```
    ```cairo
    mod internal_module
    ```
    ---
    internal_module docstring."""
    "#)
}

#[test]
fn function_with_path_last() {
    test_transform!(test_hover,r#"
    /// some_module docstring.
    mod some_module {
        /// internal_module docstring.
        pub mod internal_module {
            pub mod nested_internal_module {
                pub fn foo() {}
            }
        }
    }

    mod happy_cases {
        fn function_with_path() {
            super::some_module::internal_module::nested_internal<caret>_module::foo();
        }
    }
    "#,@r#"
    source_context = """
            super::some_module::internal_module::nested_internal<caret>_module::foo();
    """
    highlight = """
            super::some_module::internal_module::<sel>nested_internal_module</sel>::foo();
    """
    popover = """
    ```cairo
    hello::some_module::internal_module
    ```
    ```cairo
    mod nested_internal_module
    ```
    """
    "#)
}

#[test]
fn function_with_partial_path_nested() {
    test_transform!(test_hover,r#"
    /// some_module docstring.
    mod some_module {
        /// internal_module docstring.
        pub mod internal_module {
            pub mod nested_internal_module {
                pub fn foo() {}
            }
        }
    }

    mod happy_cases {
        use super::some_module::internal_module::nested_internal_module;

        fn function_with_partial_path() {
            nested_in<caret>ternal_module::foo();
        }
    }
    "#,@r#"
    source_context = """
            nested_in<caret>ternal_module::foo();
    """
    highlight = """
            <sel>nested_internal_module</sel>::foo();
    """
    popover = """
    ```cairo
    hello::some_module::internal_module
    ```
    ```cairo
    mod nested_internal_module
    ```
    """
    "#)
}

#[test]
fn function_with_partial_path() {
    test_transform!(test_hover,r#"
    /// some_module docstring.
    mod some_module {
        /// internal_module docstring.
        pub mod internal_module {
            pub trait MyTrait<T> {
                fn foo(t: T);
            }

            pub impl MyTraitImpl of MyTrait<u32> {
                fn foo(t: u32) {}
            }
        }
    }

    mod happy_cases {
        use super::some_module::internal_module;

        fn function_with_partial_path() {
            internal_<caret>module::MyTraitImpl::foo(0_u32);
        }
    }
    "#,@r#"
    source_context = """
            internal_<caret>module::MyTraitImpl::foo(0_u32);
    """
    highlight = """
            <sel>internal_module</sel>::MyTraitImpl::foo(0_u32);
    """
    popover = """
    ```cairo
    hello::some_module
    ```
    ```cairo
    mod internal_module
    ```
    ---
    internal_module docstring."""
    "#)
}

#[test]
fn struct_constructor_with_path_first() {
    test_transform!(test_hover,r#"
    /// some_module docstring.
    mod some_module {
        /// internal_module docstring.
        pub mod internal_module {
            pub mod nested_internal_module {
                pub struct PublicStruct {}
            }
        }
    }

    mod happy_cases {
        fn constructor_with_path() {
            let _ = <caret>super::some_module::internal_module::nested_internal_module::PublicStruct {};
        }
    }
    "#,@r#"
    source_context = """
            let _ = <caret>super::some_module::internal_module::nested_internal_module::PublicStruct {};
    """
    "#)
}

#[test]
fn struct_constructor_with_path_middle() {
    test_transform!(test_hover,r#"
    /// some_module docstring.
    mod some_module {
        /// internal_module docstring.
        pub mod internal_module {
            pub mod nested_internal_module {
                pub struct PublicStruct {}
            }
        }
    }

    mod happy_cases {
        fn constructor_with_path() {
            let _ = super::some_module::inte<caret>rnal_module::nested_internal_module::PublicStruct {};
        }
    }
    "#,@r#"
    source_context = """
            let _ = super::some_module::inte<caret>rnal_module::nested_internal_module::PublicStruct {};
    """
    highlight = """
            let _ = super::some_module::<sel>internal_module</sel>::nested_internal_module::PublicStruct {};
    """
    popover = """
    ```cairo
    hello::some_module
    ```
    ```cairo
    mod internal_module
    ```
    ---
    internal_module docstring."""
    "#)
}

#[test]
fn struct_constructor_with_path_last() {
    test_transform!(test_hover,r#"
    /// some_module docstring.
    mod some_module {
        /// internal_module docstring.
        pub mod internal_module {
            pub mod nested_internal_module {
                pub struct PublicStruct {}
            }
        }
    }

    mod happy_cases {
        fn constructor_with_path() {
            let _ = super::some_module::internal_module::nested_internal<caret>_module::PublicStruct {};
        }
    }
    "#,@r#"
    source_context = """
            let _ = super::some_module::internal_module::nested_internal<caret>_module::PublicStruct {};
    """
    highlight = """
            let _ = super::some_module::internal_module::<sel>nested_internal_module</sel>::PublicStruct {};
    """
    popover = """
    ```cairo
    hello::some_module::internal_module
    ```
    ```cairo
    mod nested_internal_module
    ```
    """
    "#)
}

#[test]
fn private() {
    test_transform!(test_hover,r#"
    /// some_module docstring.
    mod some_module {
        /// internal_module docstring.
        pub mod internal_module {
            /// private_submodule docstring.
            mod private_submodule { }
        }
    }

    // Although the code itself is semantically invalid because of items' visibility, paths should be shown correctly.
    mod unhappy_cases {
        use super::some_module::internal_module::private<caret>_submodule;
    }
    "#,@r#"
    source_context = """
        use super::some_module::internal_module::private<caret>_submodule;
    """
    highlight = """
        use super::some_module::internal_module::<sel>private_submodule</sel>;
    """
    popover = """
    ```cairo
    hello::some_module::internal_module
    ```
    ```cairo
    mod private_submodule
    ```
    ---
    private_submodule docstring."""
    "#)
}

#[test]
fn private_struct() {
    test_transform!(test_hover,r#"
    /// some_module docstring.
    mod some_module {
        /// internal_module docstring.
        pub mod internal_module {
            pub mod nested_internal_module {
                struct PrivateStruct {}
            }
        }
    }

    // Although the code itself is semantically invalid because of items' visibility, paths should be shown correctly.
    mod unhappy_cases {
        fn private_item {
            let _ = super::some_module::internal<caret>_module::PrivateStruct {};
        }
    }
    "#,@r#"
    source_context = """
            let _ = super::some_module::internal<caret>_module::PrivateStruct {};
    """
    highlight = """
            let _ = super::some_module::<sel>internal_module</sel>::PrivateStruct {};
    """
    popover = """
    ```cairo
    hello::some_module
    ```
    ```cairo
    mod internal_module
    ```
    ---
    internal_module docstring."""
    "#)
}

#[test]
fn private_struct_partial_path() {
    test_transform!(test_hover,r#"
    /// some_module docstring.
    mod some_module {
        /// internal_module2 docstring.
        pub mod internal_module2 {
            /// private_submodule docstring.
            mod private_submodule {
                struct PrivateStruct2 {}
            }
        }
    }

    // Although the code itself is semantically invalid because of items' visibility, paths should be shown correctly.
    mod unhappy_cases {
        use super::some_module::internal_module2::private_submodule;

        fn private_item {
            let _ = private_sub<caret>module::PrivateStruct2 {};
        }
    }
    "#,@r#"
    source_context = """
            let _ = private_sub<caret>module::PrivateStruct2 {};
    """
    highlight = """
            let _ = <sel>private_submodule</sel>::PrivateStruct2 {};
    """
    popover = """
    ```cairo
    hello::some_module::internal_module2
    ```
    ```cairo
    mod private_submodule
    ```
    ---
    private_submodule docstring."""
    "#)
}
