// Added because of the need to ignore the tests of proc macros.
// TODO(#453): Delete
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_macros)]

mod analysis;
mod code_actions;
mod completions;
mod document_highlight;
mod find_references;
mod goto_definition;
mod hover;
mod linter;
mod macros;
mod scarb;
mod semantic_tokens;
mod support;
mod workspace_configuration;
