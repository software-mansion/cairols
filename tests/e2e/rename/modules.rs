use crate::rename::rename_with_additional_modules;
use indoc::indoc;

#[test]
fn non_inline_module() {
    let result = rename_with_additional_modules(
        indoc! {r#"
            use crate::mod<caret>zik::stuff;

            mod mod<caret>zik;

            fn main() {
                mod<caret>zik::more_stuff();
            }
        "#},
        [("src/modzik.cairo", "fn stuff () {}")].into_iter().collect(),
    );

    insta::assert_snapshot!(result, @r"
    File renames:
    - src/modzik.cairo -> src/RENAMED.cairo
    ================================

    use crate::RENAMED::stuff;

    mod RENAMED;

    fn main() {
        RENAMED::more_stuff();
    }
    ");
}
