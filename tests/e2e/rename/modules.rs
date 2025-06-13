use indoc::indoc;
use lsp_types::request::Rename;

use crate::support::fixture;
use crate::support::insta::{test_transform_plain, test_transform_with_macros};

#[test]
fn non_inline_module_on_definition() {
    test_transform_plain!(Rename,
    fixture! {
        "src/modzik.cairo" => indoc!(r#"
            use crate::modzik::stuff;
            fn stuff() {}
        "#),
    }, r#"
    use crate::modzik::stuff;

    mod mod<caret>zik;

    fn main() {
        modzik::more_stuff();
    }
    "#, @r"
    // → src/lib.cairo
    use crate::RENAMED::stuff;

    mod RENAMED;

    fn main() {
        RENAMED::more_stuff();
    }
    // → src/modzik.cairo → src/RENAMED.cairo
    use crate::RENAMED::stuff;
    fn stuff() {}
    ");
}

#[test]
fn non_inline_module_on_usage() {
    test_transform_plain!(Rename,
    fixture! {
        "src/modzik.cairo" => indoc!(r#"
            use crate::modzik::stuff;
            fn stuff() {}
        "#),
    }, r#"
    use crate::mo<caret>dzik::stuff;

    mod modzik;

    fn main() {
        modzik::more_stuff();
    }
    "#, @r"
    // → src/lib.cairo
    use crate::RENAMED::stuff;

    mod RENAMED;

    fn main() {
        RENAMED::more_stuff();
    }
    // → src/modzik.cairo → src/RENAMED.cairo
    use crate::RENAMED::stuff;
    fn stuff() {}
    ");
}

#[test]
fn inline_module_on_definition() {
    test_transform_plain!(Rename ,r#"
    use crate::modzik::stuff;

    mod mod<caret>zik {
        use crate::modzik::stuff;

        fn stuff() {}
    }

    fn main() {
        modzik::more_stuff();
    }
    "#,@r"
    use crate::RENAMED::stuff;

    mod RENAMED {
        use crate::RENAMED::stuff;

        fn stuff() {}
    }

    fn main() {
        RENAMED::more_stuff();
    }
    ")
}

#[test]
fn inline_module_on_usage() {
    test_transform_plain!(Rename ,r#"
    use crate::modzik::stuff;

    mod modzik {
        use crate::mod<caret>zik::stuff;

        fn stuff() {}
    }

    fn main() {
        modzik::more_stuff();
    }
    "#,@r"
    use crate::RENAMED::stuff;

    mod RENAMED {
        use crate::RENAMED::stuff;

        fn stuff() {}
    }

    fn main() {
        RENAMED::more_stuff();
    }
    ")
}

#[test]
fn non_inline_module_on_usage_with_macros() {
    test_transform_with_macros!(Rename,
    fixture! {
        "src/modzik.cairo" => indoc!(r#"
            use crate::modzik::stuff;

            #[complex_attribute_macro_v2]
            fn stuff() {}
        "#),
    }, r#"
    #[complex_attribute_macro_v2]
    use crate::mo<caret>dzik::stuff;

    #[complex_attribute_macro_v2]
    mod modzik;

    #[complex_attribute_macro_v2]
    fn main() {
        modzik::more_stuff();
    }
    "#, @r"
    // → src/lib.cairo
    #[complex_attribute_macro_v2]
    use crate::RENAMED::stuff;

    #[complex_attribute_macro_v2]
    mod RENAMED;

    #[complex_attribute_macro_v2]
    fn main() {
        RENAMED::more_stuff();
    }
    // → src/modzik.cairo → src/RENAMED.cairo
    use crate::RENAMED::stuff;

    #[complex_attribute_macro_v2]
    fn stuff() {}
    ");
}

#[test]
fn non_inline_module_on_definition_with_macros() {
    test_transform_with_macros!(Rename,
    fixture! {
        "src/modzik.cairo" => indoc!(r#"
            use crate::modzik::stuff;

            #[complex_attribute_macro_v2]
            fn stuff() {}
        "#),
    }, r#"
    #[complex_attribute_macro_v2]
    use crate::modzik::stuff;

    #[complex_attribute_macro_v2]
    mod mod<caret>zik;

    #[complex_attribute_macro_v2]
    fn main() {
        modzik::more_stuff();
    }
    "#, @r"
    // → src/lib.cairo
    #[complex_attribute_macro_v2]
    use crate::RENAMED::stuff;

    #[complex_attribute_macro_v2]
    mod RENAMED;

    #[complex_attribute_macro_v2]
    fn main() {
        RENAMED::more_stuff();
    }
    // → src/modzik.cairo → src/RENAMED.cairo
    use crate::RENAMED::stuff;

    #[complex_attribute_macro_v2]
    fn stuff() {}
    ");
}
