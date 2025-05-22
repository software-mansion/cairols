use crate::rename::rename;

#[test]
fn inline_macro() {
    rename(
        r#"
    fn main() {
        print!("Hello world!");
        let arr = arr<caret>ay![1, 2, 3, 4, 5];
    }
    fn bar() {
        let forty_two = array![42];
    }
    "#,
    );
}
