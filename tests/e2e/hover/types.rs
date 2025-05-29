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
fn test_generic_type_alias() {
    test_transform!(test_hover, r#"
    type GenericTypeAlias<T> = Result<T, ()>;
    fn foo() -> Generic<caret>TypeAlias<()> {
        Result::Ok(())
    }
    "#, @r#"
    source_context = """
    fn foo() -> Generic<caret>TypeAlias<()> {
    """
    highlight = """
    fn foo() -> <sel>GenericTypeAlias</sel><()> {
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    type GenericTypeAlias = Result<T, ()>;
    ```
    """
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

#[test]
fn test_extern_type_in_trait_generic_bound_as_type() {
    test_transform!(test_hover, r#"
    trait Trait<T, +Into<u32<caret>, T>> {}
    "#, @r#"
    source_context = """
    trait Trait<T, +Into<u32<caret>, T>> {}
    """
    highlight = """
    trait Trait<T, +Into<<sel>u32</sel>, T>> {}
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
fn test_extern_type_in_trait_generic_bound_as_type_parameter() {
    test_transform!(test_hover, r#"
    trait Trait<T, +Into<Array<u32<caret>>, T>> {}
    "#, @r#"
    source_context = """
    trait Trait<T, +Into<Array<u32<caret>>, T>> {}
    """
    highlight = """
    trait Trait<T, +Into<Array<<sel>u32</sel>>, T>> {}
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
fn test_simple_impl_in_impl_associated_impl() {
    test_transform!(test_hover, r#"
    trait TraitA {}
    impl ImplA of TraitA {}

    trait TraitB {
        impl Impl: TraitA;
    }
    impl ImplB of TraitB {
        impl Impl = ImplA<caret>;
    }
    "#, @r#"
    source_context = """
        impl Impl = ImplA<caret>;
    """
    highlight = """
        impl Impl = <sel>ImplA</sel>;
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    impl ImplA of TraitA;
    ```
    """
    "#)
}

#[test]
fn test_impl_alias_in_impl_associated_impl() {
    test_transform!(test_hover, r#"
    trait TraitA {}
    impl ImplA of TraitA {}
    impl ImplAAlias = ImplA;

    trait TraitB {
        impl Impl: TraitA;
    }
    impl ImplB of TraitB {
        impl Impl = ImplAAlias<caret>;
    }
    "#, @r#"
    source_context = """
        impl Impl = ImplAAlias<caret>;
    """
    highlight = """
        impl Impl = <sel>ImplAAlias</sel>;
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    impl ImplAAlias = ImplA;
    ```
    """
    "#)
}

#[test]
fn test_builtin_generic_impl_in_impl_associated_impl() {
    test_transform!(test_hover, r#"
    use core::circuit::{AddInputResultImpl, AddInputResultTrait, u96};

    trait Trait {
        impl AssociatedImpl: AddInputResultTrait<u96>;
    }

    impl Impl of Trait {
        impl AssociatedImpl = AddInput<caret>ResultImpl<u96>;
    }
    "#, @r#"
    source_context = """
        impl AssociatedImpl = AddInput<caret>ResultImpl<u96>;
    """
    highlight = """
        impl AssociatedImpl = <sel>AddInputResultImpl</sel><u96>;
    """
    popover = """
    ```cairo
    core::circuit
    ```
    ```cairo
    pub impl AddInputResultImpl<C> of AddInputResultTrait<C>;
    ```
    ---
    A trait for filling inputs in a circuit instance's data.
    This trait provides methods to add input values to a circuit instance and
    finalize the input process.
    # Examples

    ```cairo
    let a = CircuitElement::<CircuitInput<0>> {};
    let b = CircuitElement::<CircuitInput<1>> {};
    let modulus = TryInto::<_, CircuitModulus>::try_into([2, 0, 0, 0]).unwrap();
    let circuit = (a,b).new_inputs()
        .next([10, 0, 0, 0]) // returns AddInputResult::More, inputs are not yet filled
        .next([11, 0, 0, 0]) // returns AddInputResult::Done, inputs are filled
        .done() // returns CircuitData<C>, inputs are filled
        .eval(modulus)
        .unwrap();
    assert!(circuit.get_output(a) == 0.into());
    assert!(circuit.get_output(b) == 1.into());
    ```"""
    "#)
}

#[test]
fn test_extern_type_in_impl_generic_bound_as_type() {
    test_transform!(test_hover, r#"
    trait Trait<T, +Into<u32, T>> {}
    impl<T, +Into<u32<caret>, T>> Impl of Trait<T> {}
    "#, @r#"
    source_context = """
    impl<T, +Into<u32<caret>, T>> Impl of Trait<T> {}
    """
    highlight = """
    impl<T, +Into<<sel>u32</sel>, T>> Impl of Trait<T> {}
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
fn test_extern_type_in_impl_generic_bound_as_type_parameter() {
    test_transform!(test_hover, r#"
    trait Trait<T, +Into<u32, T>> {}
    impl<T, +Into<Array<u32<caret>>, T>> Impl of Trait<T> {}
    "#, @r#"
    source_context = """
    impl<T, +Into<Array<u32<caret>>, T>> Impl of Trait<T> {}
    """
    highlight = """
    impl<T, +Into<Array<<sel>u32</sel>>, T>> Impl of Trait<T> {}
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
fn test_builtin_impl_in_impl_generic_bound() {
    test_transform!(test_hover, r#"
    use core::circuit::{AddInputResultImpl, AddInputResultTrait};
    trait Trait<T, impl I: AddInputResultTrait<T>> {}
    impl Impl of Trait<felt252, AddInputResult<caret>Impl<felt252>> {}
    "#, @r#"
    source_context = """
    impl Impl of Trait<felt252, AddInputResult<caret>Impl<felt252>> {}
    """
    highlight = """
    impl Impl of Trait<felt252, <sel>AddInputResultImpl</sel><felt252>> {}
    """
    popover = """
    ```cairo
    core::circuit
    ```
    ```cairo
    pub impl AddInputResultImpl<C> of AddInputResultTrait<C>;
    ```
    ---
    A trait for filling inputs in a circuit instance's data.
    This trait provides methods to add input values to a circuit instance and
    finalize the input process.
    # Examples

    ```cairo
    let a = CircuitElement::<CircuitInput<0>> {};
    let b = CircuitElement::<CircuitInput<1>> {};
    let modulus = TryInto::<_, CircuitModulus>::try_into([2, 0, 0, 0]).unwrap();
    let circuit = (a,b).new_inputs()
        .next([10, 0, 0, 0]) // returns AddInputResult::More, inputs are not yet filled
        .next([11, 0, 0, 0]) // returns AddInputResult::Done, inputs are filled
        .done() // returns CircuitData<C>, inputs are filled
        .eval(modulus)
        .unwrap();
    assert!(circuit.get_output(a) == 0.into());
    assert!(circuit.get_output(b) == 1.into());
    ```"""
    "#)
}

#[test]
fn test_impl_alias_in_impl_generic_bound() {
    test_transform!(test_hover, r#"
    use core::circuit::{AddInputResultImpl, AddInputResultTrait};
    impl ImplAlias<T> = AddInputResultImpl<T>;
    trait Trait<T, impl I: AddInputResultTrait<T>> {}
    impl Impl of Trait<felt252, ImplAlias<caret><felt252>> {}
    "#, @r#"
    source_context = """
    impl Impl of Trait<felt252, ImplAlias<caret><felt252>> {}
    """
    highlight = """
    impl Impl of Trait<felt252, <sel>ImplAlias</sel><felt252>> {}
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    impl ImplAlias = AddInputResultImpl<T>;
    ```
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
    core::circuit
    ```
    ```cairo
    pub type u96 =     crate::internal::bounded_int::BoundedInt<0, 79228162514264337593543950335>;
    ```
    ---
    A 96-bit unsigned integer type used as the basic building block for multi-limb arithmetic."""
    "#)
}

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
    core::circuit
    ```
    ```cairo
    pub type u96 =     crate::internal::bounded_int::BoundedInt<0, 79228162514264337593543950335>;
    ```
    ---
    A 96-bit unsigned integer type used as the basic building block for multi-limb arithmetic."""
    "#)
}

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
    core::circuit
    ```
    ```cairo
    pub type u96 =     crate::internal::bounded_int::BoundedInt<0, 79228162514264337593543950335>;
    ```
    ---
    A 96-bit unsigned integer type used as the basic building block for multi-limb arithmetic."""
    "#)
}

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
    core::circuit
    ```
    ```cairo
    pub type u96 =     crate::internal::bounded_int::BoundedInt<0, 79228162514264337593543950335>;
    ```
    ---
    A 96-bit unsigned integer type used as the basic building block for multi-limb arithmetic."""
    "#)
}

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
    core::circuit
    ```
    ```cairo
    pub type u96 =     crate::internal::bounded_int::BoundedInt<0, 79228162514264337593543950335>;
    ```
    ---
    A 96-bit unsigned integer type used as the basic building block for multi-limb arithmetic."""
    "#)
}

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
    core::circuit
    ```
    ```cairo
    pub type u96 =     crate::internal::bounded_int::BoundedInt<0, 79228162514264337593543950335>;
    ```
    ---
    A 96-bit unsigned integer type used as the basic building block for multi-limb arithmetic."""
    "#)
}

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
    core::circuit
    ```
    ```cairo
    pub type u96 =     crate::internal::bounded_int::BoundedInt<0, 79228162514264337593543950335>;
    ```
    ---
    A 96-bit unsigned integer type used as the basic building block for multi-limb arithmetic."""
    "#)
}

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
    core::circuit
    ```
    ```cairo
    pub type u96 =     crate::internal::bounded_int::BoundedInt<0, 79228162514264337593543950335>;
    ```
    ---
    A 96-bit unsigned integer type used as the basic building block for multi-limb arithmetic."""
    "#)
}

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
    core::circuit
    ```
    ```cairo
    pub type u96 =     crate::internal::bounded_int::BoundedInt<0, 79228162514264337593543950335>;
    ```
    ---
    A 96-bit unsigned integer type used as the basic building block for multi-limb arithmetic."""
    "#)
}

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
    core::circuit
    ```
    ```cairo
    pub type u96 =     crate::internal::bounded_int::BoundedInt<0, 79228162514264337593543950335>;
    ```
    ---
    A 96-bit unsigned integer type used as the basic building block for multi-limb arithmetic."""
    "#)
}

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
    core::circuit
    ```
    ```cairo
    pub type u96 =     crate::internal::bounded_int::BoundedInt<0, 79228162514264337593543950335>;
    ```
    ---
    A 96-bit unsigned integer type used as the basic building block for multi-limb arithmetic."""
    "#)
}

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
    core::circuit
    ```
    ```cairo
    pub type u96 =     crate::internal::bounded_int::BoundedInt<0, 79228162514264337593543950335>;
    ```
    ---
    A 96-bit unsigned integer type used as the basic building block for multi-limb arithmetic."""
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
    core::circuit
    ```
    ```cairo
    pub type u96 =     crate::internal::bounded_int::BoundedInt<0, 79228162514264337593543950335>;
    ```
    ---
    A 96-bit unsigned integer type used as the basic building block for multi-limb arithmetic."""
    "#)
}

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
    core::circuit
    ```
    ```cairo
    pub type u96 =     crate::internal::bounded_int::BoundedInt<0, 79228162514264337593543950335>;
    ```
    ---
    A 96-bit unsigned integer type used as the basic building block for multi-limb arithmetic."""
    "#)
}

#[test]
fn test_builtin_alias_in_trait_generic_bound_as_type() {
    test_transform!(test_hover, r#"
    use core::circuit::u96;

    trait Trait<T, +Into<u96<caret>, T>> {}
    "#, @r#"
    source_context = """
    trait Trait<T, +Into<u96<caret>, T>> {}
    """
    highlight = """
    trait Trait<T, +Into<<sel>u96</sel>, T>> {}
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

#[test]
fn test_builtin_alias_in_trait_generic_bound_as_type_parameter() {
    test_transform!(test_hover, r#"
    use core::circuit::u96;
    trait Trait<T, +Into<Array<u96<caret>>, T>> {}
    "#, @r#"
    source_context = """
    trait Trait<T, +Into<Array<u96<caret>>, T>> {}
    """
    highlight = """
    trait Trait<T, +Into<Array<<sel>u96</sel>>, T>> {}
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
    core::circuit
    ```
    ```cairo
    pub type u96 =     crate::internal::bounded_int::BoundedInt<0, 79228162514264337593543950335>;
    ```
    ---
    A 96-bit unsigned integer type used as the basic building block for multi-limb arithmetic."""
    "#)
}

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
    core::circuit
    ```
    ```cairo
    pub type u96 =     crate::internal::bounded_int::BoundedInt<0, 79228162514264337593543950335>;
    ```
    ---
    A 96-bit unsigned integer type used as the basic building block for multi-limb arithmetic."""
    "#)
}

// FIXME(#660): This should be inferred
#[test]
fn test_impl_associated_type_with_path_reference() {
    test_transform!(test_hover, r#"
    trait Trait {
        type Type;
    }

    impl Impl of Trait {
        type Type = felt252;
    }

    fn foo() {
         let num: Impl::Typ<caret>e = 123;
    }
    "#, @r#"
    source_context = """
         let num: Impl::Typ<caret>e = 123;
    """
    highlight = """
         let num: Impl::<sel>Type</sel> = 123;
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    impl Impl of Trait;
    ```
    """
    "#)
}

// FIXME(#660): This should be inferred
#[test]
fn test_impl_associated_impl_with_path_reference() {
    test_transform!(test_hover, r#"
    trait SubTraitor {
        const ABC: felt252;
    }

    trait Trait {
        impl SubTrait: SubTraitor;
    }

    impl SubTraitor911 of SubTraitor {
        const ABC: felt252 = 123;
    }

    impl Impl of Trait {
        impl SubTrait = SubTraitor911;
    }

    fn foo() {
         let num = Impl::SubTr<caret>ait::ABC;
    }
    "#, @r#"
    source_context = """
         let num = Impl::SubTr<caret>ait::ABC;
    """
    highlight = """
         let num = Impl::<sel>SubTrait</sel>::ABC;
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    impl Impl of Trait;
    ```
    """
    "#)
}

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
    core::circuit
    ```
    ```cairo
    pub type u96 =     crate::internal::bounded_int::BoundedInt<0, 79228162514264337593543950335>;
    ```
    ---
    A 96-bit unsigned integer type used as the basic building block for multi-limb arithmetic."""
    "#)
}

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
    core::circuit
    ```
    ```cairo
    pub type u96 =     crate::internal::bounded_int::BoundedInt<0, 79228162514264337593543950335>;
    ```
    ---
    A 96-bit unsigned integer type used as the basic building block for multi-limb arithmetic."""
    "#)
}

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
    highlight = """
    impl<T, +Into<<sel>u96</sel>, T>> Impl of Trait<T> {}
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
    highlight = """
    impl<T, +Into<Array<<sel>u96</sel>>, T>> Impl of Trait<T> {}
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
