use crate::document_highlight::document_highlight;
use crate::support::insta::test_transform;

#[test]
fn highlight() {
    test_transform!(document_highlight, r#"
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
