use crate::code_actions::quick_fix;
use crate::support::insta::test_transform;

#[test]
fn on_mod_keyword() {
    test_transform!(quick_fix, "mo<caret>d some_module;", @r#"
    Title: Create module file `some_module`
    Document changes json: [
      {
        "kind": "create",
        "uri": "file:///src/some_module.cairo"
      }
    ]
    "#);
}

#[test]
fn on_module_name() {
    test_transform!(quick_fix, "mod some_mo<caret>dule;", @r#"
    Title: Create module file `some_module`
    Document changes json: [
      {
        "kind": "create",
        "uri": "file:///src/some_module.cairo"
      }
    ]
    "#);
}

#[test]
fn after_semicolon() {
    test_transform!(quick_fix, "mod some_module;<caret>", @r#"
    Title: Create module file `some_module`
    Document changes json: [
      {
        "kind": "create",
        "uri": "file:///src/some_module.cairo"
      }
    ]
    "#);
}

#[test]
fn on_inline_module_mod_keyword() {
    test_transform!(quick_fix, "m<caret>od abc {}", @"No code actions.");
}

#[test]
fn on_inline_module_name() {
    test_transform!(quick_fix, "mod a<caret>bc {}", @"No code actions.");
}

#[test]
fn on_inline_module_body() {
    test_transform!(quick_fix, "mod abc {<caret>}", @"No code actions.");
}

#[test]
fn without_module_name() {
    test_transform!(quick_fix, "mod<caret>;", @"No code actions.");
}

#[test]
fn without_module_name_and_semicolon() {
    test_transform!(quick_fix, "mod <caret>", @"No code actions.");
}
