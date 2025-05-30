use crate::hover::test_hover;
use crate::support::insta::test_transform;

#[test]
fn generated_element_use() {
    test_transform!(test_hover,r#"
    #[generate_trait]
    impl MyTraitImpl<SelfType> of MyTrait<SelfType> {
        fn some_method(ref self: SelfType) { }
    }

    mod nested {
        use super::MyTr<caret>ait;
    }
    "#,@r#"
    source_context = """
        use super::MyTr<caret>ait;
    """
    highlight = """
        use super::<sel>MyTrait</sel>;
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    impl MyTraitImpl<SelfType> of MyTrait<SelfType>;
    trait MyTrait<SelfType>
    ```
    """
    "#)
}

#[test]
fn ref_self() {
    test_transform!(test_hover,r#"
    #[generate_trait]
    impl MyTraitImpl<SelfType> of MyTrait<SelfType> {
        fn some_method(ref se<caret>lf: SelfType) { }
    }
    "#,@r#"
    source_context = """
        fn some_method(ref se<caret>lf: SelfType) { }
    """
    highlight = """
        fn some_method(ref <sel>self</sel>: SelfType) { }
    """
    popover = """
    ```cairo
    ref self: SelfType
    ```
    """
    "#)
}

#[test]
fn self_type() {
    test_transform!(test_hover,r#"
    struct SelfType {
        a: felt252,
        b: felt252,
    }

    #[generate_trait]
    impl MyTraitImpl of MyTrait {
        fn some_method(ref self: SelfT<caret>ype) { }
    }
    "#,@r#"
    source_context = """
        fn some_method(ref self: SelfT<caret>ype) { }
    """
    highlight = """
        fn some_method(ref self: <sel>SelfType</sel>) { }
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    struct SelfType {
        a: felt252,
        b: felt252,
    }
    ```
    """
    "#)
}

#[test]
fn self_type_member() {
    test_transform!(test_hover,r#"
    struct SelfType {
        aaa: felt252,
        b: felt252,
    }

    #[generate_trait]
    impl MyTraitImpl of MyTrait {
        fn some_method(ref self: SelfType) {
            self.a<caret>aa
        }
    }
    "#,@r#"
    source_context = """
            self.a<caret>aa
    """
    highlight = """
            self.<sel>aaa</sel>
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    struct SelfType {
        aaa: felt252,
        b: felt252,
    }
    ```
    """
    "#)
}

#[test]
fn self_type_member_method_call() {
    test_transform!(test_hover,r#"
    use core::num::traits::One;

    struct SelfType {
        aaa: felt252,
        b: felt252,
    }

    #[generate_trait]
    impl MyTraitImpl of MyTrait {
        fn some_method(ref self: SelfType) {
            self.aaa.is_o<caret>ne();
        }
    }
    "#,@r#"
    source_context = """
            self.aaa.is_o<caret>ne();
    """
    highlight = """
            self.aaa.<sel>is_one</sel>();
    """
    popover = """
    ```cairo
    core::felt_252::Felt252One
    ```
    ```cairo
    pub(crate) impl Felt252One of One<felt252>;
    fn is_one(self: felt252) -> bool
    ```
    """
    "#)
}

#[test]
fn trait_name_generated() {
    test_transform!(test_hover,r#"
    #[generate_trait]
    impl MyTraitImpl of MyTra<caret>it {
        fn some_method() { }
    }
    "#,@r#"
    source_context = """
    impl MyTraitImpl of MyTra<caret>it {
    """
    highlight = """
    impl MyTraitImpl of <sel>MyTrait</sel> {
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    impl MyTraitImpl of MyTrait;
    trait MyTrait
    ```
    """
    "#)
}

#[test]
fn trait_name_generic_name_generated() {
    test_transform!(test_hover,r#"
    #[generate_trait]
    impl MyTraitImpl<SelfType> of MyTrait<Self<caret>Type> {
        fn some_method() { }
    }
    "#,@r#"
    source_context = """
    impl MyTraitImpl<SelfType> of MyTrait<Self<caret>Type> {
    """
    highlight = """
    impl MyTraitImpl<SelfType> of MyTrait<<sel>SelfType</sel>> {
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    impl MyTraitImpl<SelfType> of MyTrait<SelfType>;
    ```
    """
    "#)
}

#[test]
fn trait_name_generic_name() {
    test_transform!(test_hover,r#"
    struct Ab {}

    trait MyTrait<SelfType> {
        fn some_method(ref self: SelfType);
    }

    impl MyTraitImpl of MyTrait<A<caret>b> {
        fn some_method(ref self: Ab) { }
    }
    "#,@r#"
    source_context = """
    impl MyTraitImpl of MyTrait<A<caret>b> {
    """
    highlight = """
    impl MyTraitImpl of MyTrait<<sel>Ab</sel>> {
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    struct Ab {}
    ```
    """
    "#)
}
