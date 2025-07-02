use lsp_types::DocumentHighlight;

use crate::support::insta::test_transform_plain;

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
