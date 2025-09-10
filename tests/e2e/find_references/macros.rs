use lsp_types::request::References;

use crate::support::insta::{test_transform_plain, test_transform_with_macros};

#[test]
fn plugin_inline_macro() {
    test_transform_plain!(References, r#"
    fn main() {
        print!("Hello world!");
        let arr = arr<caret>ay![1, 2, 3, 4, 5];
    }
    fn bar() {
        let forty_two = array![42];
    }
    "#, @r#"
    // found several references in the core crate
    fn main() {
        print!("Hello world!");
        let arr = <sel>array</sel>![1, 2, 3, 4, 5];
    }
    fn bar() {
        let forty_two = <sel>array</sel>![42];
    }
    "#)
}

#[test]
fn plugin_inline_macro_with_macros() {
    test_transform_with_macros!(References, r#"
    #[complex_attribute_macro_v2]
    fn main() {
        print!("Hello world!");
        let arr = arr<caret>ay![1, 2, 3, 4, 5];
    }

    fn bar() {
        let forty_two = array![42];
    }
    "#, @r#"
    // found several references in the core crate
    #[complex_attribute_macro_v2]
    fn main() {
        print!("Hello world!");
        let arr = <sel>array</sel>![1, 2, 3, 4, 5];
    }

    fn bar() {
        let forty_two = <sel>array</sel>![42];
    }
    "#)
}

#[test]
fn declarative_inline_macro_on_usage() {
    test_transform_plain!(References, r#"
    fn main() {
        print!("Hello world!");
        let x = inc!(0);
        let y = other_inc!(0);
    }
    fn bar() {
        let forty_two = in<caret>c!(5);
    }

    pub macro inc {
        ($x:expr) => { $x + 1 };
    }

    pub macro other_inc {
        ($x:expr) => { $x + 1 };
    }
    "#, @r#"
    fn main() {
        print!("Hello world!");
        let x = <sel>inc</sel>!(0);
        let y = other_inc!(0);
    }
    fn bar() {
        let forty_two = <sel>inc</sel>!(5);
    }

    pub macro <sel=declaration>inc</sel> {
        ($x:expr) => { $x + 1 };
    }

    pub macro other_inc {
        ($x:expr) => { $x + 1 };
    }
    "#)
}

#[test]
fn declarative_inline_macro_on_definition() {
    test_transform_plain!(References, r#"
    fn main() {
        print!("Hello world!");
        let x = inc!(0);
        let y = other_inc!(0);
    }
    fn bar() {
        let forty_two = inc!(5);
    }

    pub macro i<caret>nc {
        ($x:expr) => { $x + 1 };
    }

    pub macro other_inc {
        ($x:expr) => { $x + 1 };
    }
    "#, @r#"
    fn main() {
        print!("Hello world!");
        let x = <sel>inc</sel>!(0);
        let y = other_inc!(0);
    }
    fn bar() {
        let forty_two = <sel>inc</sel>!(5);
    }

    pub macro <sel=declaration>inc</sel> {
        ($x:expr) => { $x + 1 };
    }

    pub macro other_inc {
        ($x:expr) => { $x + 1 };
    }
    "#)
}

#[test]
fn declarative_inline_macro_on_usage_with_macros() {
    test_transform_with_macros!(References, r#"
    fn main() {
        print!("Hello world!");
        let x = inc!(0);
        let y = other_inc!(0);
    }

    #[complex_attribute_macro_v2]
    fn bar() {
        let forty_two = in<caret>c!(5);
    }

    #[complex_attribute_macro_v2]
    pub macro inc {
        ($x:expr) => { $x + 1 };
    }

    pub macro other_inc {
        ($x:expr) => { $x + 1 };
    }
    "#, @r#"
    fn main() {
        print!("Hello world!");
        let x = <sel>inc</sel>!(0);
        let y = other_inc!(0);
    }

    #[complex_attribute_macro_v2]
    fn bar() {
        let forty_two = <sel>inc</sel>!(5);
    }

    #[complex_attribute_macro_v2]
    pub macro <sel=declaration>inc</sel> {
        ($x:expr) => { $x + 1 };
    }

    pub macro other_inc {
        ($x:expr) => { $x + 1 };
    }
    "#)
}
