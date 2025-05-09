use crate::rename::rename;
use crate::support::insta::test_transform;

#[test]
fn test_type_alias_in_use() {
    test_transform!(rename, r#"
    mod mod1 {
        struct Struct {};
        type TypeAlias = Struct;
    }
    use mod1::TypeAlias<caret>;
    "#, @r"
    mod mod1 {
        struct Struct {};
        type RENAMED = Struct;
    }
    use mod1::RENAMED;
    ")
}

#[test]
fn test_type_alias_in_alias_as_type() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    type AnotherTypeAlias = TypeAlias<caret>;
    "#, @r"
    struct Struct {};
    type RENAMED = Struct;
    type AnotherTypeAlias = RENAMED;
    ")
}

#[test]
fn test_type_alias_in_alias_as_type_parameter() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    type AnotherTypeAlias = Array<TypeAlias<caret>>;
    "#, @r"
    struct Struct {};
    type RENAMED = Struct;
    type AnotherTypeAlias = Array<RENAMED>;
    ")
}

#[test]
fn test_type_alias_in_variable_as_type() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    fn foo() {
        let x: TypeAlias<caret> = 0x0;
    }
    "#, @r"
    struct Struct {};
    type RENAMED = Struct;
    fn foo() {
        let x: RENAMED = 0x0;
    }
    ")
}

#[test]
fn test_type_alias_in_variable_as_type_parameter() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    fn foo() {
        let x: Array<TypeAlias<caret>> = 0x0;
    }
    "#, @r"
    struct Struct {};
    type RENAMED = Struct;
    fn foo() {
        let x: Array<RENAMED> = 0x0;
    }
    ")
}

#[test]
fn test_type_alias_in_function_argument_as_type() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    fn foo(x: TypeAlias<caret>) {}
    "#, @r"
    struct Struct {};
    type RENAMED = Struct;
    fn foo(x: RENAMED) {}
    ")
}

#[test]
fn test_type_alias_in_function_argument_as_type_parameter() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    fn foo(x: Array<TypeAlias<caret>>) {}
    "#, @r"
    struct Struct {};
    type RENAMED = Struct;
    fn foo(x: Array<RENAMED>) {}
    ")
}

#[test]
fn test_type_alias_in_return_type_as_type() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    fn foo() -> TypeAlias<caret> { 0x0 }
    "#, @r"
    struct Struct {};
    type RENAMED = Struct;
    fn foo() -> RENAMED { 0x0 }
    ")
}

#[test]
fn test_type_alias_in_return_type_as_type_parameter() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    fn foo() -> Array<TypeAlias<caret>> { 0x0 }
    "#, @r"
    struct Struct {};
    type RENAMED = Struct;
    fn foo() -> Array<RENAMED> { 0x0 }
    ")
}

#[test]
fn test_type_alias_in_struct_field_as_type() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    struct Struct {
        x: TypeAlias<caret>
    }
    "#, @r"
    struct Struct {};
    type RENAMED = Struct;
    struct Struct {
        x: RENAMED
    }
    ")
}

#[test]
fn test_type_alias_in_struct_field_as_type_parameter() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    struct Struct {
        x: Array<TypeAlias<caret>>
    }
    "#, @r"
    struct Struct {};
    type RENAMED = Struct;
    struct Struct {
        x: Array<RENAMED>
    }
    ")
}

#[test]
fn test_type_alias_in_turbofish_enum_as_type() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    fn foo() {
        let x = Result::<TypeAlias<caret>>::Err(());
    }
    "#, @r"
    struct Struct {};
    type RENAMED = Struct;
    fn foo() {
        let x = Result::<RENAMED>::Err(());
    }
    ")
}

#[test]
fn test_type_alias_in_turbofish_enum_as_type_parameter() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    fn foo() {
        let x = Result::<Array<TypeAlias<caret>>>::Err(());
    }
    "#, @r"
    struct Struct {};
    type RENAMED = Struct;
    fn foo() {
        let x = Result::<Array<RENAMED>>::Err(());
    }
    ")
}

// FIXME(#404)
#[test]
fn test_type_alias_in_trait_associated_type_as_type() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    trait Trait {
        type Type = TypeAlias<caret>
    }
    "#, @"none response")
}

// FIXME(#404)
#[test]
fn test_type_alias_in_trait_associated_type_as_type_parameter() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    trait Trait {
        type Type = Array<TypeAlias<caret>>
    }
    "#, @"none response")
}

#[test]
fn test_type_alias_in_trait_associated_const_as_type() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    trait Trait {
        const Const: TypeAlias<caret>;
    }
    "#, @r"
    struct Struct {};
    type RENAMED = Struct;
    trait Trait {
        const Const: RENAMED;
    }
    ")
}

#[test]
fn test_type_alias_in_trait_associated_const_as_type_parameter() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    trait Trait {
        const Const: Array<TypeAlias<caret>>;
    }
    "#, @r"
    struct Struct {};
    type RENAMED = Struct;
    trait Trait {
        const Const: Array<RENAMED>;
    }
    ")
}

#[test]
fn test_type_alias_in_trait_generic_bound_as_type() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    trait Trait<T, +Into<TypeAlias<caret>, T>> {}
    "#, @r"
    struct Struct {};
    type RENAMED = Struct;
    trait Trait<T, +Into<RENAMED, T>> {}
    ")
}

#[test]
fn test_type_alias_in_trait_generic_bound_as_type_parameter() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    trait Trait<T, +Into<Array<TypeAlias<caret>>, T>> {}
    "#, @r"
    struct Struct {};
    type RENAMED = Struct;
    trait Trait<T, +Into<Array<RENAMED>, T>> {}
    ")
}

#[test]
fn test_type_alias_in_impl_associated_type_as_type() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    trait Trait {
        type Type;
    }
    impl Impl of Trait {
        type Type = TypeAlias<caret>;
    }
    "#, @r"
    struct Struct {};
    type RENAMED = Struct;
    trait Trait {
        type Type;
    }
    impl Impl of Trait {
        type Type = RENAMED;
    }
    ")
}

#[test]
fn test_type_alias_in_impl_associated_type_as_type_parameter() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    trait Trait {
        type Type;
    }
    impl Impl of Trait {
        type Type = Array<TypeAlias<caret>>;
    }
    "#, @r"
    struct Struct {};
    type RENAMED = Struct;
    trait Trait {
        type Type;
    }
    impl Impl of Trait {
        type Type = Array<RENAMED>;
    }
    ")
}

#[test]
fn test_type_alias_in_impl_associated_const_as_type() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    trait Trait {
        const Const: TypeAlias;
    }
    impl Impl of Trait {
        const Const: TypeAlias<caret> = 0x0;
    }
    "#, @r"
    struct Struct {};
    type RENAMED = Struct;
    trait Trait {
        const Const: RENAMED;
    }
    impl Impl of Trait {
        const Const: RENAMED = 0x0;
    }
    ")
}

#[test]
fn test_type_alias_in_impl_associated_const_as_type_parameter() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    trait Trait {
        const Const: TypeAlias;
    }
    impl Impl of Trait {
        const Const: Array<TypeAlias<caret>> = 0x0;
    }
    "#, @r"
    struct Struct {};
    type RENAMED = Struct;
    trait Trait {
        const Const: RENAMED;
    }
    impl Impl of Trait {
        const Const: Array<RENAMED> = 0x0;
    }
    ")
}

#[test]
fn test_type_alias_in_impl_generic_bound_as_type() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    trait Trait<T, +Into<TypeAlias, T>> {}
    impl<T, +Into<TypeAlias<caret>, T>> Impl of Trait<T> {}
    "#, @r"
    struct Struct {};
    type RENAMED = Struct;
    trait Trait<T, +Into<RENAMED, T>> {}
    impl<T, +Into<RENAMED, T>> Impl of Trait<T> {}
    ")
}

#[test]
fn test_type_alias_in_impl_generic_bound_as_type_parameter() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    trait Trait<T, +Into<TypeAlias, T>> {}
    impl<T, +Into<Array<TypeAlias<caret>>, T>> Impl of Trait<T> {}
    "#, @r"
    struct Struct {};
    type RENAMED = Struct;
    trait Trait<T, +Into<RENAMED, T>> {}
    impl<T, +Into<Array<RENAMED>, T>> Impl of Trait<T> {}
    ")
}
