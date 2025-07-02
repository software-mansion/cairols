use std::sync::Arc;

use cairo_lang_defs::db::DefsGroup;
use cairo_lang_defs::plugin::MacroPluginMetadata;
use cairo_lang_filesystem::db::FilesGroup;
use cairo_lang_filesystem::ids::{CrateId, FileId, FileKind, FileLongId, VirtualFile};
use cairo_lang_parser::db::ParserGroup;
use cairo_lang_syntax::node::ast::ModuleItem;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{SyntaxNode, TypedSyntaxNode};
use cairo_lang_utils::Intern;
use lsp_types::TextDocumentPositionParams;

use crate::lang::db::{AnalysisDatabase, LsSemanticGroup, LsSyntaxGroup};
use crate::lang::lsp::{LsProtoGroup, ToCairo};
use expr::expand_inline_macros_to_file;
use format::format_output;
use inlining::{inline_files, span_after_inlining};
use itertools::Itertools;
use module_item::expand_module_item_macros;

mod expr;
mod format;
mod inlining;
mod module_item;

/// Tries to expand macro, returns it as string.
pub fn expand_macro(db: &AnalysisDatabase, params: &TextDocumentPositionParams) -> Option<String> {
    let file_id = db.file_for_url(&params.text_document.uri)?;
    let node = db.find_syntax_node_at_position(file_id, params.position.to_cairo())?;

    let crate_id = db.find_module_file_containing_node(node)?.0.owning_crate(db);
    let cfg_set = db
        .crate_config(crate_id)
        .and_then(|cfg| cfg.settings.cfg_set.map(Arc::new))
        .unwrap_or(db.cfg_set());
    let edition = db.crate_config(crate_id).map(|cfg| cfg.settings.edition).unwrap_or_default();

    let metadata = MacroPluginMetadata {
        cfg_set: &cfg_set,
        declared_derives: &db.declared_derives(crate_id),
        allowed_features: &Default::default(),
        edition,
    };

    let item_node = node.ancestors_with_self(db).find(|node| {
        let kind = node.kind(db);
        ModuleItem::is_variant(kind) || kind == SyntaxKind::ExprInlineMacro
    })?;

    expand_macro_ex(db, file_id, crate_id, &metadata, item_node)
}

// Recursively expand macros.
fn expand_macro_ex(
    db: &AnalysisDatabase,
    file_to_process: FileId,
    crate_id: CrateId,
    metadata: &MacroPluginMetadata<'_>,
    item_node: SyntaxNode,
) -> Option<String> {
    let mut extra_files = vec![];

    let file_with_inlined_module_items = if item_node.kind(db) == SyntaxKind::ExprInlineMacro {
        file_to_process
    } else {
        // If it is not inline-expr macro, start with expanding regular plugins as these have to be done *before* inline-expr.
        let module_item = ModuleItem::from_syntax_node(db, item_node);

        let mut files = vec![];
        expand_module_item_macros(
            db,
            module_item,
            crate_id,
            metadata,
            &mut files,
            &mut extra_files,
        )?;

        // Inline all files that should be inlined, keep others in `extra_files` for further processing.
        let replaced_content = inline_files(db, file_to_process, &files)?;

        FileLongId::Virtual(VirtualFile {
            parent: None,
            name: "macro_expand".into(),
            content: replaced_content.into(),
            code_mappings: Default::default(),
            kind: FileKind::Module,
            original_item_removed: false,
        })
        .intern(db)
    };

    // Process `extra_files` by collecting all module items in these files and expanding same way as user file.
    let extra_files = extra_files
        .into_iter()
        .filter_map(|content| {
            let expanded_items = db
                .file_module_syntax(content.file)
                .ok()?
                .items(db)
                .elements(db)
                .map(|item| {
                    expand_macro_ex(db, content.file, crate_id, metadata, item.as_syntax_node())
                        .unwrap()
                })
                .join("\n\n");

            Some(expanded_items)
        })
        .collect::<Vec<_>>();

    // Expand inline macros only in this span, this prevents expansion of unwanted items
    let span = span_after_inlining(
        db,
        file_to_process,
        file_with_inlined_module_items,
        item_node.span(db),
    )?;

    let replaced_content_file =
        expand_inline_macros_to_file(db, crate_id, file_with_inlined_module_items, span, metadata)?;

    // Span of replaced content in new generated file.
    let new_span =
        span_after_inlining(db, file_to_process, replaced_content_file, item_node.span(db))?;

    let new_file_content = db.file_content(replaced_content_file)?;
    let replaced_content = new_span.take(&new_file_content);

    let replaced_content = if extra_files.is_empty() {
        replaced_content.to_string()
    } else {
        format!("{replaced_content}\n//-----\n{}", extra_files.join("\n//-----\n"))
    };

    Some(format_output(&replaced_content, item_node.kind(db)))
}
