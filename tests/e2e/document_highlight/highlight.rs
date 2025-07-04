use lsp_types::DocumentHighlight;

use crate::support::insta::{test_transform_plain, test_transform_with_macros};

#[test]
fn highlight() {
    test_transform_plain!(DocumentHighlight, r#"
    fn a(a: BBB, b: felt252) {
        core::array::ArrayTrait::<felt252>::ne<caret>w();

        ArrayTrait::<felt252>::new();

        new();
    }

    fn new() {}

    mod c {
        fn a() {
            core::array::ArrayTrait::<felt252>::new();
        }
    }
    "#, @r"
    fn a(a: BBB, b: felt252) {
        core::array::ArrayTrait::<felt252>::<sel>new</sel>();

        ArrayTrait::<felt252>::<sel>new</sel>();

        new();
    }

    fn new() {}

    mod c {
        fn a() {
            core::array::ArrayTrait::<felt252>::<sel>new</sel>();
        }
    }
    ")
}

#[test]
fn identical_structs_in_different_scopes_with_macros() {
    test_transform_with_macros!(DocumentHighlight, r#"
    mod module {
        #[derive(ComplexDeriveMacroV2)]
        struct S {
            x: felt252
        }
    }

    #[derive(ComplexDeriveMacroV2)]
    struct S {
        x: felt252
    }

    #[complex_attribute_macro_v2]
    fn foo() {
        let s = S<caret> { x: 0x0 };
    }
    "#, @r"
    mod module {
        #[derive(ComplexDeriveMacroV2)]
        struct S {
            x: felt252
        }
    }

    #[derive(ComplexDeriveMacroV2)]
    struct <sel>S</sel> {
        x: felt252
    }

    #[complex_attribute_macro_v2]
    fn foo() {
        let s = <sel>S</sel> { x: 0x0 };
    }
    ")
}

#[test]
fn multiple_resultants_with_macros() {
    test_transform_with_macros!(DocumentHighlight, r#"
    #[generate_trait]
    impl Impl1 of MyTrait {
        fn some_co<caret>nst() -> u8 {
            1
        }
    }

    #[complex_attribute_macro_v2]
    impl Impl2 of MyTrait {
        fn some_const() -> u8 {
            2
        }
    }

    #[complex_attribute_macro_v2]
    fn main() {
        Impl2::some_const();
    }

    fn some_const() {}
    "#, @r"
    #[generate_trait]
    impl Impl1 of MyTrait {
        fn <sel>some_const</sel>() -> u8 {
            1
        }
    }

    #[complex_attribute_macro_v2]
    impl Impl2 of MyTrait {
        fn <sel>some_const</sel>() -> u8 {
            2
        }
    }

    #[complex_attribute_macro_v2]
    fn main() {
        Impl2::<sel>some_const</sel>();
    }

    fn some_const() {}
    "
    );
}
