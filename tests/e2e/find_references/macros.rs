use crate::find_references::find_references;
use crate::support::insta::test_transform;

#[test]
fn inline_macro() {
    test_transform!(find_references, r#"
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
