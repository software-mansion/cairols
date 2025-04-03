use crate::hover::test_hover;
use crate::support::insta::test_transform;

#[test]
fn test_extern_type_in_alias_as_type() {
    test_transform!(test_hover, r#"
    type TypeAlias = u32<caret>;
    "#, @r#"
    source_context = """
    type TypeAlias = u32<caret>;
    """
    highlight = """
    type TypeAlias = <sel>u32</sel>;
    """
    popover = """
    ```cairo
    core::integer
    ```
    ```cairo
    pub extern type u32;
    ```
    ---
    The 32-bit unsigned integer type."""
    "#)
}

#[test]
fn test_extern_type_in_alias_as_type_parameter() {
    test_transform!(test_hover, r#"
    type TypeAlias = Array<u32<caret>>;
    "#, @r#"
    source_context = """
    type TypeAlias = Array<u32<caret>>;
    """
    highlight = """
    type TypeAlias = Array<<sel>u32</sel>>;
    """
    popover = """
    ```cairo
    core::integer
    ```
    ```cairo
    pub extern type u32;
    ```
    ---
    The 32-bit unsigned integer type."""
    "#)
}

#[test]
fn test_extern_type_in_variable_as_type() {
    test_transform!(test_hover, r#"
    fn foo() {
        let x: u32<caret> = 0x0;
    }
    "#, @r#"
    source_context = """
        let x: u32<caret> = 0x0;
    """
    highlight = """
        let x: <sel>u32</sel> = 0x0;
    """
    popover = """
    ```cairo
    core::integer
    ```
    ```cairo
    pub extern type u32;
    ```
    ---
    The 32-bit unsigned integer type."""
    "#)
}

#[test]
fn test_extern_type_in_variable_as_type_parameter() {
    test_transform!(test_hover, r#"
    fn foo() {
        let x: Array<u32<caret>> = 0x0;
    }
    "#, @r#"
    source_context = """
        let x: Array<u32<caret>> = 0x0;
    """
    highlight = """
        let x: Array<<sel>u32</sel>> = 0x0;
    """
    popover = """
    ```cairo
    core::integer
    ```
    ```cairo
    pub extern type u32;
    ```
    ---
    The 32-bit unsigned integer type."""
    "#)
}

#[test]
fn test_extern_type_in_function_argument_as_type() {
    test_transform!(test_hover, r#"
    fn foo(x: u32<caret>) {}
    "#, @r#"
    source_context = """
    fn foo(x: u32<caret>) {}
    """
    highlight = """
    fn foo(x: <sel>u32</sel>) {}
    """
    popover = """
    ```cairo
    core::integer
    ```
    ```cairo
    pub extern type u32;
    ```
    ---
    The 32-bit unsigned integer type."""
    "#)
}

#[test]
fn test_extern_type_in_function_argument_as_type_parameter() {
    test_transform!(test_hover, r#"
    fn foo(x: Array<u32<caret>>) {}
    "#, @r#"
    source_context = """
    fn foo(x: Array<u32<caret>>) {}
    """
    highlight = """
    fn foo(x: Array<<sel>u32</sel>>) {}
    """
    popover = """
    ```cairo
    core::integer
    ```
    ```cairo
    pub extern type u32;
    ```
    ---
    The 32-bit unsigned integer type."""
    "#)
}

#[test]
fn test_extern_type_in_return_type_as_type() {
    test_transform!(test_hover, r#"
    fn foo() -> u32<caret> { 0x0 }
    "#, @r#"
    source_context = """
    fn foo() -> u32<caret> { 0x0 }
    """
    highlight = """
    fn foo() -> <sel>u32</sel> { 0x0 }
    """
    popover = """
    ```cairo
    core::integer
    ```
    ```cairo
    pub extern type u32;
    ```
    ---
    The 32-bit unsigned integer type."""
    "#)
}

#[test]
fn test_extern_type_in_return_type_as_type_parameter() {
    test_transform!(test_hover, r#"
    fn foo() -> Array<u32<caret>> { 0x0 }
    "#, @r#"
    source_context = """
    fn foo() -> Array<u32<caret>> { 0x0 }
    """
    highlight = """
    fn foo() -> Array<<sel>u32</sel>> { 0x0 }
    """
    popover = """
    ```cairo
    core::integer
    ```
    ```cairo
    pub extern type u32;
    ```
    ---
    The 32-bit unsigned integer type."""
    "#)
}

#[test]
fn test_extern_type_in_struct_field_as_type() {
    test_transform!(test_hover, r#"
    struct Struct {
        x: u32<caret>
    }
    "#, @r#"
    source_context = """
        x: u32<caret>
    """
    highlight = """
        x: <sel>u32</sel>
    """
    popover = """
    ```cairo
    core::integer
    ```
    ```cairo
    pub extern type u32;
    ```
    ---
    The 32-bit unsigned integer type."""
    "#)
}

#[test]
fn test_extern_type_in_struct_field_as_type_parameter() {
    test_transform!(test_hover, r#"
    struct Struct {
        x: Array<u32<caret>>
    }
    "#, @r#"
    source_context = """
        x: Array<u32<caret>>
    """
    highlight = """
        x: Array<<sel>u32</sel>>
    """
    popover = """
    ```cairo
    core::integer
    ```
    ```cairo
    pub extern type u32;
    ```
    ---
    The 32-bit unsigned integer type."""
    "#)
}

#[test]
fn test_extern_type_in_turbofish_enum_as_type() {
    test_transform!(test_hover, r#"
    fn foo() {
        let x = Result::<u32<caret>>::Err(());
    }
    "#, @r#"
    source_context = """
        let x = Result::<u32<caret>>::Err(());
    """
    highlight = """
        let x = Result::<<sel>u32</sel>>::Err(());
    """
    popover = """
    ```cairo
    core::integer
    ```
    ```cairo
    pub extern type u32;
    ```
    ---
    The 32-bit unsigned integer type."""
    "#)
}

#[test]
fn test_extern_type_in_turbofish_enum_as_type_parameter() {
    test_transform!(test_hover, r#"
    fn foo() {
        let x = Result::<Array<u32<caret>>>::Err(());
    }
    "#, @r#"
    source_context = """
        let x = Result::<Array<u32<caret>>>::Err(());
    """
    highlight = """
        let x = Result::<Array<<sel>u32</sel>>>::Err(());
    """
    popover = """
    ```cairo
    core::integer
    ```
    ```cairo
    pub extern type u32;
    ```
    ---
    The 32-bit unsigned integer type."""
    "#)
}

// FIXME(#563) - hovers for trait associated items have not yet been implemented.
#[test]
fn test_extern_type_in_trait_associated_type_as_type() {
    test_transform!(test_hover, r#"
    trait Trait {
        type Type = u32<caret>
    }
    "#, @r#"
    source_context = """
        type Type = u32<caret>
    """
    "#)
}

// FIXME(#563)
#[test]
fn test_extern_type_in_trait_associated_type_as_type_parameter() {
    test_transform!(test_hover, r#"
    trait Trait {
        type Type = Array<u32<caret>>
    }
    "#, @r#"
    source_context = """
        type Type = Array<u32<caret>>
    """
    "#)
}

#[test]
fn test_extern_type_in_trait_associated_const_as_type() {
    test_transform!(test_hover, r#"
    trait Trait {
        const Const: u32<caret>;
    }
    "#, @r#"
    source_context = """
        const Const: u32<caret>;
    """
    highlight = """
        const Const: <sel>u32</sel>;
    """
    popover = """
    ```cairo
    core::integer
    ```
    ```cairo
    pub extern type u32;
    ```
    ---
    The 32-bit unsigned integer type."""
    "#)
}

#[test]
fn test_extern_type_in_trait_associated_const_as_type_parameter() {
    test_transform!(test_hover, r#"
    trait Trait {
        const Const: Array<u32<caret>>;
    }
    "#, @r#"
    source_context = """
        const Const: Array<u32<caret>>;
    """
    highlight = """
        const Const: Array<<sel>u32</sel>>;
    """
    popover = """
    ```cairo
    core::integer
    ```
    ```cairo
    pub extern type u32;
    ```
    ---
    The 32-bit unsigned integer type."""
    "#)
}

// FIXME(#565) - hovers for generic bounds have not yet been implemented.
#[test]
fn test_extern_type_in_trait_generic_bound_as_type() {
    test_transform!(test_hover, r#"
    trait Trait<T, +Into<u32<caret>, T>> {}
    "#, @r#"
    source_context = """
    trait Trait<T, +Into<u32<caret>, T>> {}
    """
    "#)
}

// FIXME(#565)
#[test]
fn test_extern_type_in_trait_generic_bound_as_type_parameter() {
    test_transform!(test_hover, r#"
    trait Trait<T, +Into<Array<u32<caret>>, T>> {}
    "#, @r#"
    source_context = """
    trait Trait<T, +Into<Array<u32<caret>>, T>> {}
    """
    "#)
}

// FIXME(#563) - hovers for impl associated items have not yet been implemented.
#[test]
fn test_extern_type_in_impl_associated_type_as_type() {
    test_transform!(test_hover, r#"
    trait Trait {
        type Type;
    }
    impl Impl of Trait {
        type Type = u32<caret>;
    }
    "#, @r#"
    source_context = """
        type Type = u32<caret>;
    """
    highlight = """
        type Type = <sel>u32</sel>;
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    trait Trait
    ```
    """
    "#)
}

// FIXME(#563)
#[test]
fn test_extern_type_in_impl_associated_type_as_type_parameter() {
    test_transform!(test_hover, r#"
    trait Trait {
        type Type;
    }
    impl Impl of Trait {
        type Type = Array<u32<caret>>;
    }
    "#, @r#"
    source_context = """
        type Type = Array<u32<caret>>;
    """
    highlight = """
        type Type = Array<<sel>u32</sel>>;
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    trait Trait
    ```
    """
    "#)
}

// FIXME(#563)
#[test]
fn test_extern_type_in_impl_associated_const_as_type() {
    test_transform!(test_hover, r#"
    trait Trait {
        const Const: u32;
    }
    impl Impl of Trait {
        const Const: u32<caret> = 0x0;
    }
    "#, @r#"
    source_context = """
        const Const: u32<caret> = 0x0;
    """
    highlight = """
        const Const: <sel>u32</sel> = 0x0;
    """
    popover = """
    ```cairo
    hello::Trait
    ```
    ```cairo
    trait Trait
    const Const: u32;
    ```
    """
    "#)
}

// FIXME(#563)
#[test]
fn test_extern_type_in_impl_associated_const_as_type_parameter() {
    test_transform!(test_hover, r#"
    trait Trait {
        const Const: u32;
    }
    impl Impl of Trait {
        const Const: Array<u32<caret>> = 0x0;
    }
    "#, @r#"
    source_context = """
        const Const: Array<u32<caret>> = 0x0;
    """
    highlight = """
        const Const: Array<<sel>u32</sel>> = 0x0;
    """
    popover = """
    ```cairo
    hello::Trait
    ```
    ```cairo
    trait Trait
    const Const: u32;
    ```
    """
    "#)
}

// FIXME(#565)
#[test]
fn test_extern_type_in_impl_generic_bound_as_type() {
    test_transform!(test_hover, r#"
    trait Trait<T, +Into<u32, T>> {}
    impl<T, +Into<u32<caret>, T>> Impl of Trait<T> {}
    "#, @r#"
    source_context = """
    impl<T, +Into<u32<caret>, T>> Impl of Trait<T> {}
    """
    "#)
}

// FIXME(#565)
#[test]
fn test_extern_type_in_impl_generic_bound_as_type_parameter() {
    test_transform!(test_hover, r#"
    trait Trait<T, +Into<u32, T>> {}
    impl<T, +Into<Array<u32<caret>>, T>> Impl of Trait<T> {}
    "#, @r#"
    source_context = """
    impl<T, +Into<Array<u32<caret>>, T>> Impl of Trait<T> {}
    """
    "#)
}

#[test]
fn test_builtin_alias_in_use() {
    test_transform!(test_hover, r#"
    use core::circuit::u96<caret>;
    "#, @r#"
    source_context = """
    use core::circuit::u96<caret>;
    """
    highlight = """
    use core::circuit::<sel>u96</sel>;
    """
    popover = """
    ```cairo
    core::circuit
    ```
    ```cairo
    pub type u96 =     crate::internal::bounded_int::BoundedInt<0, 79228162514264337593543950335>;
    ```
    ---
    A 96-bit unsigned integer type used as the basic building block for multi-limb arithmetic."""
    "#)
}

// FIXME(#466) - type aliases are not handled properly.
// Hover refers to the original type being a right-hand-side of the alias, not to the alias itself.
// Hover on type expression is different than on the import (see the test above).
#[test]
fn test_builtin_alias_in_alias_as_type() {
    test_transform!(test_hover, r#"
    use core::circuit::u96;
    type TypeAlias = u96<caret>
    "#, @r#"
    source_context = """
    type TypeAlias = u96<caret>
    """
    highlight = """
    type TypeAlias = <sel>u96</sel>
    """
    popover = """
    ```cairo
    core::internal::bounded_int
    ```
    ```cairo
    pub(crate) extern type BoundedInt<MINMAX>;
    ```
    """
    "#)
}

// FIXME(#466)
#[test]
fn test_builtin_alias_in_alias_as_type_parameter() {
    test_transform!(test_hover, r#"
    use core::circuit::u96;
    type TypeAlias = Array<u96<caret>>
    "#, @r#"
    source_context = """
    type TypeAlias = Array<u96<caret>>
    """
    highlight = """
    type TypeAlias = Array<<sel>u96</sel>>
    """
    popover = """
    ```cairo
    core::internal::bounded_int
    ```
    ```cairo
    pub(crate) extern type BoundedInt<MINMAX>;
    ```
    """
    "#)
}

// FIXME(#466)
#[test]
fn test_builtin_alias_in_variable_as_type() {
    test_transform!(test_hover, r#"
    use core::circuit::u96;
    fn foo() {
        let x: u96<caret> = 0x0;
    }
    "#, @r#"
    source_context = """
        let x: u96<caret> = 0x0;
    """
    highlight = """
        let x: <sel>u96</sel> = 0x0;
    """
    popover = """
    ```cairo
    core::internal::bounded_int
    ```
    ```cairo
    pub(crate) extern type BoundedInt<MINMAX>;
    ```
    """
    "#)
}

// FIXME(#466)
#[test]
fn test_builtin_alias_in_variable_as_type_parameter() {
    test_transform!(test_hover, r#"
    use core::circuit::u96;
    fn foo() {
        let x: Array<u96<caret>> = 0x0;
    }
    "#, @r#"
    source_context = """
        let x: Array<u96<caret>> = 0x0;
    """
    highlight = """
        let x: Array<<sel>u96</sel>> = 0x0;
    """
    popover = """
    ```cairo
    core::internal::bounded_int
    ```
    ```cairo
    pub(crate) extern type BoundedInt<MINMAX>;
    ```
    """
    "#)
}

// FIXME(#466)
#[test]
fn test_builtin_alias_in_function_argument_as_type() {
    test_transform!(test_hover, r#"
    use core::circuit::u96;
    fn foo(x: u96<caret>) {}
    "#, @r#"
    source_context = """
    fn foo(x: u96<caret>) {}
    """
    highlight = """
    fn foo(x: <sel>u96</sel>) {}
    """
    popover = """
    ```cairo
    core::internal::bounded_int
    ```
    ```cairo
    pub(crate) extern type BoundedInt<MINMAX>;
    ```
    """
    "#)
}

// FIXME(#466)
#[test]
fn test_builtin_alias_in_function_argument_as_type_parameter() {
    test_transform!(test_hover, r#"
    use core::circuit::u96;
    fn foo(x: Array<u96<caret>>) {}
    "#, @r#"
    source_context = """
    fn foo(x: Array<u96<caret>>) {}
    """
    highlight = """
    fn foo(x: Array<<sel>u96</sel>>) {}
    """
    popover = """
    ```cairo
    core::internal::bounded_int
    ```
    ```cairo
    pub(crate) extern type BoundedInt<MINMAX>;
    ```
    """
    "#)
}

// FIXME(#466)
#[test]
fn test_builtin_alias_in_return_type_as_type() {
    test_transform!(test_hover, r#"
    use core::circuit::u96;
    fn foo() -> u96<caret> { 0x0 }
    "#, @r#"
    source_context = """
    fn foo() -> u96<caret> { 0x0 }
    """
    highlight = """
    fn foo() -> <sel>u96</sel> { 0x0 }
    """
    popover = """
    ```cairo
    core::internal::bounded_int
    ```
    ```cairo
    pub(crate) extern type BoundedInt<MINMAX>;
    ```
    """
    "#)
}

// FIXME(#466)
#[test]
fn test_builtin_alias_in_return_type_as_type_parameter() {
    test_transform!(test_hover, r#"
    use core::circuit::u96;
    fn foo() -> Array<u96<caret>> { 0x0 }
    "#, @r#"
    source_context = """
    fn foo() -> Array<u96<caret>> { 0x0 }
    """
    highlight = """
    fn foo() -> Array<<sel>u96</sel>> { 0x0 }
    """
    popover = """
    ```cairo
    core::internal::bounded_int
    ```
    ```cairo
    pub(crate) extern type BoundedInt<MINMAX>;
    ```
    """
    "#)
}

// FIXME(#466)
#[test]
fn test_builtin_alias_in_struct_field_as_type() {
    test_transform!(test_hover, r#"
    use core::circuit::u96;
    struct Struct {
        x: u96<caret>
    }
    "#, @r#"
    source_context = """
        x: u96<caret>
    """
    highlight = """
        x: <sel>u96</sel>
    """
    popover = """
    ```cairo
    core::internal::bounded_int
    ```
    ```cairo
    pub(crate) extern type BoundedInt<MINMAX>;
    ```
    """
    "#)
}

// FIXME(#466)
#[test]
fn test_builtin_alias_in_struct_field_as_type_parameter() {
    test_transform!(test_hover, r#"
    use core::circuit::u96;
    struct Struct {
        x: Array<u96<caret>>
    }
    "#, @r#"
    source_context = """
        x: Array<u96<caret>>
    """
    highlight = """
        x: Array<<sel>u96</sel>>
    """
    popover = """
    ```cairo
    core::internal::bounded_int
    ```
    ```cairo
    pub(crate) extern type BoundedInt<MINMAX>;
    ```
    """
    "#)
}

// FIXME(#466)
#[test]
fn test_builtin_alias_in_turbofish_enum_as_type() {
    test_transform!(test_hover, r#"
    use core::circuit::u96;
    fn foo() {
        let x = Result::<u96<caret>>::Err(());
    }
    "#, @r#"
    source_context = """
        let x = Result::<u96<caret>>::Err(());
    """
    highlight = """
        let x = Result::<<sel>u96</sel>>::Err(());
    """
    popover = """
    ```cairo
    core::internal::bounded_int
    ```
    ```cairo
    pub(crate) extern type BoundedInt<MINMAX>;
    ```
    """
    "#)
}

// FIXME(#466)
#[test]
fn test_builtin_alias_in_turbofish_enum_as_type_parameter() {
    test_transform!(test_hover, r#"
    use core::circuit::u96;
    fn foo() {
        let x = Result::<Array<u96<caret>>>::Err(());
    }
    "#, @r#"
    source_context = """
        let x = Result::<Array<u96<caret>>>::Err(());
    """
    highlight = """
        let x = Result::<Array<<sel>u96</sel>>>::Err(());
    """
    popover = """
    ```cairo
    core::internal::bounded_int
    ```
    ```cairo
    pub(crate) extern type BoundedInt<MINMAX>;
    ```
    """
    "#)
}

// FIXME(#563)
#[test]
fn test_builtin_alias_in_trait_associated_type_as_type() {
    test_transform!(test_hover, r#"
    use core::circuit::u96;
    trait Trait {
        type Type = u96<caret>
    }
    "#, @r#"
    source_context = """
        type Type = u96<caret>
    """
    "#)
}

// FIXME(#563)
#[test]
fn test_builtin_alias_in_trait_associated_type_as_type_parameter() {
    test_transform!(test_hover, r#"
    use core::circuit::u96;
    trait Trait {
        type Type = Array<u96<caret>>
    }
    "#, @r#"
    source_context = """
        type Type = Array<u96<caret>>
    """
    "#)
}

// FIXME(#466)
#[test]
fn test_builtin_alias_in_trait_associated_const_as_type() {
    test_transform!(test_hover, r#"
    use core::circuit::u96;
    trait Trait {
        const Const: u96<caret>;
    }
    "#, @r#"
    source_context = """
        const Const: u96<caret>;
    """
    highlight = """
        const Const: <sel>u96</sel>;
    """
    popover = """
    ```cairo
    core::internal::bounded_int
    ```
    ```cairo
    pub(crate) extern type BoundedInt<MINMAX>;
    ```
    """
    "#)
}

// FIXME(#466)
#[test]
fn test_builtin_alias_in_trait_associated_const_as_type_parameter() {
    test_transform!(test_hover, r#"
    use core::circuit::u96;
    trait Trait {
        const Const: Array<u96<caret>>;
    }
    "#, @r#"
    source_context = """
        const Const: Array<u96<caret>>;
    """
    highlight = """
        const Const: Array<<sel>u96</sel>>;
    """
    popover = """
    ```cairo
    core::internal::bounded_int
    ```
    ```cairo
    pub(crate) extern type BoundedInt<MINMAX>;
    ```
    """
    "#)
}

// FIXME(#565)
#[test]
fn test_builtin_alias_in_trait_generic_bound_as_type() {
    test_transform!(test_hover, r#"
    use core::circuit::u96;

    trait Trait<T, +Into<u96<caret>, T>> {}
    "#, @r#"
    source_context = """
    trait Trait<T, +Into<u96<caret>, T>> {}
    """
    "#)
}

// FIXME(#565)
#[test]
fn test_builtin_alias_in_trait_generic_bound_as_type_parameter() {
    test_transform!(test_hover, r#"
    use core::circuit::u96;
    trait Trait<T, +Into<Array<u96<caret>>, T>> {}
    "#, @r#"
    source_context = """
    trait Trait<T, +Into<Array<u96<caret>>, T>> {}
    """
    "#)
}

// FIXME(#563)
#[test]
fn test_builtin_alias_in_impl_associated_type_as_type() {
    test_transform!(test_hover, r#"
    use core::circuit::u96;
    trait Trait {
        type Type;
    }
    impl Impl of Trait {
        type Type = u96<caret>;
    }
    "#, @r#"
    source_context = """
        type Type = u96<caret>;
    """
    highlight = """
        type Type = <sel>u96</sel>;
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    trait Trait
    ```
    """
    "#)
}

// FIXME(#563)
#[test]
fn test_builtin_alias_in_impl_associated_type_as_type_parameter() {
    test_transform!(test_hover, r#"
    use core::circuit::u96;
    trait Trait {
        type Type;
    }
    impl Impl of Trait {
        type Type = Array<u96<caret>>;
    }
    "#, @r#"
    source_context = """
        type Type = Array<u96<caret>>;
    """
    highlight = """
        type Type = Array<<sel>u96</sel>>;
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    trait Trait
    ```
    """
    "#)
}

// FIXME(#466)
#[test]
fn test_builtin_alias_in_impl_associated_const_as_type() {
    test_transform!(test_hover, r#"
    use core::circuit::u96;
    trait Trait {
        const Const: u96;
    }
    impl Impl of Trait {
        const Const: u96<caret> = 0x0;
    }
    "#, @r#"
    source_context = """
        const Const: u96<caret> = 0x0;
    """
    highlight = """
        const Const: <sel>u96</sel> = 0x0;
    """
    popover = """
    ```cairo
    hello::Trait
    ```
    ```cairo
    trait Trait
    const Const: BoundedInt<0, 79228162514264337593543950335>;
    ```
    """
    "#)
}

// FIXME(#466)
#[test]
fn test_builtin_alias_in_impl_associated_const_as_type_parameter() {
    test_transform!(test_hover, r#"
    use core::circuit::u96;
    trait Trait {
        const Const: u96;
    }
    impl Impl of Trait {
        const Const: Array<u96<caret>> = 0x0;
    }
    "#, @r#"
    source_context = """
        const Const: Array<u96<caret>> = 0x0;
    """
    highlight = """
        const Const: Array<<sel>u96</sel>> = 0x0;
    """
    popover = """
    ```cairo
    hello::Trait
    ```
    ```cairo
    trait Trait
    const Const: BoundedInt<0, 79228162514264337593543950335>;
    ```
    """
    "#)
}

// FIXME(#565)
#[test]
fn test_builtin_alias_in_impl_generic_bound_as_type() {
    test_transform!(test_hover, r#"
    use core::circuit::u96;
    trait Trait<T, +Into<u96, T>> {}
    impl<T, +Into<u96<caret>, T>> Impl of Trait<T> {}
    "#, @r#"
    source_context = """
    impl<T, +Into<u96<caret>, T>> Impl of Trait<T> {}
    """
    "#)
}

// FIXME(#565)
#[test]
fn test_builtin_alias_in_impl_generic_bound_as_type_parameter() {
    test_transform!(test_hover, r#"
    use core::circuit::u96;
    trait Trait<T, +Into<u96, T>> {}
    impl<T, +Into<Array<u96<caret>>, T>> Impl of Trait<T> {}
    "#, @r#"
    source_context = """
    impl<T, +Into<Array<u96<caret>>, T>> Impl of Trait<T> {}
    """
    "#)
}
