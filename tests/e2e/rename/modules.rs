use crate::rename::rename_with_additional_modules;
use indoc::indoc;
use std::collections::HashMap;

#[test]
fn non_inline_module() {
    let result = rename_with_additional_modules(HashMap::from([
        (
            "src/lib.cairo",
            indoc! {r#"
                use crate::mod<caret>zik::stuff;

                mod mod<caret>zik;

                fn main() {
                    mod<caret>zik::more_stuff();
                }
            "#},
        ),
        (
            "src/modzik.cairo",
            indoc! {r#"
                use crate::mod<caret>zik::stuff;

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
