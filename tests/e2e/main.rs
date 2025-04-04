#![expect(dead_code)]

// TODO(#539) Unignore tests.
// They fail because PMS API is incompatible and requests get timeout.
// Unignoring requires new Scarb nightly.

mod analysis;
mod code_actions;
// mod code_lens;
mod completions;
mod document_highlight;
mod find_references;
mod goto_definition;
mod hover;
mod linter;
// mod macros;
mod no_config_reload;
mod rename;
mod scarb;
mod semantic_tokens;
mod support;
mod workspace_configuration;
