use crate::rename::{rename, rename_with_additional_files};
use crate::support::insta::test_transform;
use indoc::indoc;
use std::collections::HashMap;

#[test]
fn non_inline_module_on_definition() {
    let result = rename_with_additional_files(HashMap::from([
        (
            "src/lib.cairo",
            indoc! {r#"
                use crate::modzik::stuff;

                mod mod<caret>zik;

                fn main() {
                    modzik::more_stuff();
                }
            "#},
        ),
        (
            "src/modzik.cairo",
            indoc! {r#"
                use crate::modzik::stuff;

                fn stuff() {}
            "#},
        ),
    ]));

    insta::assert_snapshot!(result, @r"
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
    let result = rename_with_additional_files(HashMap::from([
        (
            "src/lib.cairo",
            indoc! {r#"
                use crate::mo<caret>dzik::stuff;

                mod modzik;

                fn main() {
                    modzik::more_stuff();
                }
            "#},
        ),
        (
            "src/modzik.cairo",
            indoc! {r#"
                use crate::modzik::stuff;

                fn stuff() {}
            "#},
        ),
    ]));

    insta::assert_snapshot!(result, @r"
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
    test_transform!(rename ,r#"
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
    test_transform!(rename ,r#"
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
