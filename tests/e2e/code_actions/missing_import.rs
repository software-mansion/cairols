use crate::code_actions::{quick_fix, quick_fix_without_visibility_constraints};
use crate::support::insta::test_transform;

#[test]
fn single() {
    test_transform!(quick_fix, "
    mod aa {
        pub mod bb {
            fn cc() {}
        }

        pub fn cc() {}
    }

    fn test() {
        c<caret>c()
    }
    ", @r#"
    Title: Import `aa::cc`
    Add new text: "use aa::cc;
    "
    At: Range { start: Position { line: 0, character: 0 }, end: Position { line: 0, character: 0 } }
    Title: Fix All
    Add new text: "use aa::cc;
    "
    At: Range { start: Position { line: 0, character: 0 }, end: Position { line: 0, character: 0 } }
    "#);
}

#[test]
fn multiple() {
    test_transform!(quick_fix, "
    mod aa {
        pub mod bb {
            pub fn cc() {}
        }

        pub fn cc() {}
    }

    fn test() {
        c<caret>c()
    }
    ", @r#"
    Title: Import `aa::cc`
    Add new text: "use aa::cc;
    "
    At: Range { start: Position { line: 0, character: 0 }, end: Position { line: 0, character: 0 } }
    Title: Import `aa::bb::cc`
    Add new text: "use aa::bb::cc;
    "
    At: Range { start: Position { line: 0, character: 0 }, end: Position { line: 0, character: 0 } }
    "#);
}

#[test]
fn ignoring_visibility() {
    test_transform!(quick_fix_without_visibility_constraints, "
    mod aa {
        pub mod bb {
            fn cc() {}
        }

        pub fn cc() {}
    }

    fn test() {
        c<caret>c()
    }
    ", @r#"
    Title: Import `aa::cc`
    Add new text: "use aa::cc;
    "
    At: Range { start: Position { line: 0, character: 0 }, end: Position { line: 0, character: 0 } }
    Title: Import `aa::bb::cc`
    Add new text: "use aa::bb::cc;
    "
    At: Range { start: Position { line: 0, character: 0 }, end: Position { line: 0, character: 0 } }
    "#);
}

#[test]
fn non_existent() {
    test_transform!(quick_fix, "
    mod aa {
        pub mod bb {
            fn cc() {}
        }

        pub fn cc() {}
    }

    fn test() {
        now_way<caret>::this_exists
    }
    ", @"No code actions.");
}

#[test]
fn generic_path() {
    test_transform!(quick_fix, "
    mod aa {
        pub mod bb {
            pub fn cc<T>(a: T) {}
        }

        pub fn cc() {}
    }

    fn test() {
        bb<caret>::cc::<felt252>()
    }
    ", @r#"
    Title: Import `aa::bb`
    Add new text: "use aa::bb;
    "
    At: Range { start: Position { line: 0, character: 0 }, end: Position { line: 0, character: 0 } }
    Title: Fix All
    Add new text: "use aa::bb;
    "
    At: Range { start: Position { line: 0, character: 0 }, end: Position { line: 0, character: 0 } }
    "#);
}

#[test]
fn generic_path_on_generic() {
    test_transform!(quick_fix, "
    mod aa {
        pub mod bb {
            pub fn cc<T>(a: T) {}
        }

        pub fn cc() {}
    }

    fn test() {
        bb::cc::<fel<caret>t252>()
    }
    ", @"No code actions.");
}

#[test]
fn fix_all() {
    test_transform!(quick_fix, "
    mod aa {
        pub mod bb {
            pub fn cc<T>(a: @T) {}
        }

        pub fn dd() {}
    }

    fn test() {
        <sel>
        bb::cc();
        dd();
        </sel>
    }
    ", @r#"
    Title: Import `aa::bb`
    Add new text: "use aa::bb;
    "
    At: Range { start: Position { line: 0, character: 0 }, end: Position { line: 0, character: 0 } }
    Title: Import `aa::dd`
    Add new text: "use aa::dd;
    "
    At: Range { start: Position { line: 0, character: 0 }, end: Position { line: 0, character: 0 } }
    Title: Fix All
    Add new text: "use aa::bb;
    "
    At: Range { start: Position { line: 0, character: 0 }, end: Position { line: 0, character: 0 } }
    Add new text: "use aa::dd;
    "
    At: Range { start: Position { line: 0, character: 0 }, end: Position { line: 0, character: 0 } }
    "#);
}
