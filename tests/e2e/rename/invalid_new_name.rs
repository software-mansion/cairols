use crate::rename::rename_with_new_name;

#[test]
#[should_panic(expected = "`invalid^name` is not a valid identifier")]
fn invalid_new_name() {
    rename_with_new_name("fn fu<caret>nc() {}", "invalid^name");
}
