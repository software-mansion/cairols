use lsp_types::DocumentHighlight;

use crate::support::insta::test_transform_plain;

#[test]
fn declarative_macros() {
    test_transform_plain!(DocumentHighlight, r#"
    fn main() {
        print!("Hello world!");
        let x = inc!(0);
        let y = other_inc!(0);
    }
    fn bar() {
        let forty_two = in<caret>c!(5);
    }

    mod not_in_scope {
        macro inc {
            ($x:expr) => { $x + 1 };
        }
    }

    mod other_mod {
        use super::inc;

        hello::inc!(2);
        super::other_inc!(10);
    }

    pub macro inc {
        ($x:expr) => { $x + 1 };
    }

    pub macro other_inc {
        ($x:expr) => { $x + 1 };
    }
    "#, @r#"
    "#
    );
}
