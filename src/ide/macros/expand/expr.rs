use std::collections::HashSet;

use cairo_lang_defs::{
    db::DefsGroup,
    plugin::{InlineMacroExprPlugin, MacroPluginMetadata},
};
use cairo_lang_filesystem::{
    ids::{CrateId, FileId, FileKind, FileLongId, VirtualFile},
    span::TextSpan,
};
use cairo_lang_parser::db::ParserGroup;
use cairo_lang_syntax::node::{TypedSyntaxNode, ast::ExprInlineMacro};
use cairo_lang_utils::Intern;

use super::inlining::{FileWithOrigin, inline_files, span_after_inlining};
use crate::lang::db::AnalysisDatabase;

/// Expands inline macros for this file.
pub fn expand_inline_macros_to_file<'db>(
    db: &'db AnalysisDatabase,
    crate_id: CrateId<'db>,
    file_to_process: FileId<'db>,
    expand_in: TextSpan,
    metadata: &MacroPluginMetadata<'_>,
) -> Option<FileId<'db>> {
    let mut files = vec![];
    let plugins = db.crate_inline_macro_plugins(crate_id);

    let inline_macros_in_span: Vec<_> = db
        .file_syntax(file_to_process)
        .ok()?
        .descendants(db)
        .filter(|node| expand_in.contains(node.span(db)))
        .filter_map(|node| ExprInlineMacro::cast(db, node))
        .collect();

    let mut inline_macros: HashSet<_> = inline_macros_in_span.iter().cloned().collect();

    // In case of nested macro calls, resolve only these on bottom of ast, so we can pass resolved code to higher ones.
    for inline_macro in inline_macros_in_span {
        for unclear_macro_call in inline_macro
            .as_syntax_node()
            .ancestors(db)
            .filter_map(|node| ExprInlineMacro::cast(db, node))
        {
            inline_macros.remove(&unclear_macro_call);
        }
    }

    for inline_macro in inline_macros {
        let macro_name = inline_macro.path(db).as_syntax_node().get_text_without_trivia(db);
        let &plugin_id = plugins.get(&macro_name.to_string())?;

        let plugin = db.lookup_intern_inline_macro_plugin(plugin_id);
        let generated = plugin.generate_code(db, &inline_macro, metadata).code?; // None here means macro failed.

        let file = FileLongId::Virtual(VirtualFile {
            parent: Some(file_to_process),
            name: generated.name,
            content: generated.content.into(),
            code_mappings: generated.code_mappings.into(),
            kind: FileKind::Module,
            original_item_removed: false,
        })
        .intern(db);

        files.push(FileWithOrigin { file, generated_from: inline_macro.as_syntax_node() });
    }

    // Recursive break condition.
    if files.is_empty() {
        return Some(file_to_process);
    }

    // Inline resolved macro calls.
    let replaced_content = inline_files(db, file_to_process, &files)?;

    let new_file = FileLongId::Virtual(VirtualFile {
        parent: None,
        name: "macro_expand".into(),
        content: replaced_content.into(),
        code_mappings: Default::default(),
        kind: file_to_process.kind(db),
        original_item_removed: false,
    })
    .intern(db);
    let span = span_after_inlining(db, file_to_process, new_file, expand_in)?;

    // If we resolved anything, try recursive call to resolve any potential untouched macro call.
    expand_inline_macros_to_file(db, crate_id, new_file, span, metadata)
}
