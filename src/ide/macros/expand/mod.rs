use std::cmp::Reverse;

use cairo_lang_filesystem::db::FilesGroup;
use cairo_lang_filesystem::ids::FileId;
use cairo_lang_filesystem::span::{TextOffset, TextSpan, TextWidth};
use cairo_lang_parser::db::ParserGroup;
use cairo_lang_semantic::lsp_helpers::LspHelpers;
use cairo_lang_syntax::node::TypedSyntaxNode;
use cairo_lang_syntax::node::ast::{ExprInlineMacro, ModuleItem};
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_language_common::{CommonGroup, FileIdExt};
use format::format_output;
use lsp_types::TextDocumentPositionParams;
use salsa::Database;

use crate::ide::macros::expand::recovery::expand_inline_macro_no_context;
use crate::lang::db::{AnalysisDatabase, LsSyntaxGroup};
use crate::lang::lsp::{LsProtoGroup, ToCairo};

mod format;
mod recovery;

/// Tries to expand macro, returns it as string.
pub fn expand_macro(db: &AnalysisDatabase, params: &TextDocumentPositionParams) -> Option<String> {
    let file_id = db.file_for_url(&params.text_document.uri)?;
    let node = db.find_syntax_node_at_position(file_id, params.position.to_cairo())?;

    let item_node = node.ancestors_with_self(db).find(|node| {
        let kind = node.kind(db);
        ModuleItem::is_variant(kind) || kind == SyntaxKind::ExprInlineMacro
    })?;
    let item_node_span = item_node.span(db);

    let (files, _) = db.file_and_subfiles_with_corresponding_modules_without_inline(file_id)?;
    let (files_with_inlines, _) = db.file_and_subfiles_with_corresponding_modules(file_id)?;

    let mut inline_files: Vec<_> = (files_with_inlines - files).into_iter().collect();

    // Filter out files pointing to main file or outside of interesting span.
    // This way macros from other ModuleItem/Expr will not be applied keeping prefix and suffix code constant.
    let filter = |file: &FileId<'_>| {
        file.maybe_as_virtual(db)
            .and_then(|vfs| vfs.parent)
            .is_some_and(|parent| parent.file_id != file_id || item_node_span.contains(parent.span))
    };

    if let Some(syntax) = ExprInlineMacro::cast(db, item_node) {
        inline_files.retain(filter);

        // If there is no inline macro file pointing to origin file, then we can only expand with no context.
        if inline_files.iter().all(|file| file.as_virtual(db).parent.unwrap().file_id != file_id) {
            let module_id = db.find_module_containing_node(item_node)?;

            return expand_inline_macro_no_context(db, syntax, module_id);
        }
    }
    let files: Vec<_> = files.into_iter().copied().filter(filter).collect();

    let file_end = db.file_syntax(file_id).unwrap().span(db).end;
    let suffix = file_end - item_node_span.end;
    let expansion = expand(db, file_id, Some(suffix), &files, &inline_files)?;

    let expansion_end = TextOffset::from_str(&expansion);

    // Code could only change in item_node_span, so code in 0..item_node_span.start and item_node_span.end..file_end - item_node_span.end
    // is guaranteed to stay untouched, use this property to strip code from user defined file that should *not* be a part of expansion.
    let expansion = TextSpan { start: item_node_span.start, end: expansion_end.sub_width(suffix) }
        .take(&expansion);

    Some(format_output(expansion, item_node.kind(db)))
}

fn expand<'db>(
    db: &'db dyn Database,
    start_file: FileId<'db>,
    suffix: Option<TextWidth>,
    og_files: &[FileId<'db>],
    inline_files: &[FileId<'db>],
) -> Option<String> {
    let mut files = direct_child_files(db, og_files, start_file);

    files.sort_by_key(|file| file.as_virtual(db).original_item_removed);

    let mut files = files.into_iter().peekable();

    let mut content = expand_inline(db, start_file, inline_files)?;

    let Some(first_file) = files.peek() else {
        return Some(content);
    };

    let first_file_virtual = first_file.as_virtual(db);
    if first_file_virtual.original_item_removed {
        let first_file_content = expand(db, *first_file, None, og_files, inline_files)?;
        files.next(); // Consume it
        content.replace_range(
            first_file_virtual.parent.unwrap().span.to_str_range(),
            &first_file_content,
        );
    }

    // There should be at most 1 file that replaces it.
    // This is true because compiler will stop processing module item when any plugin will return `remove_original_item == true`.
    // See: https://github.com/starkware-libs/cairo/blob/482afce5f4cd2c1c2e0c6b2357b5021695904877/crates/cairo-lang-defs/src/db.rs#L1220
    if let Some(f) = files.peek() {
        assert!(!f.as_virtual(db).original_item_removed);
    }
    let file_end = TextOffset::from_str(&content);
    // We want to insert files that does not remove original item right after original code/file that removed original code.
    // Easiest way is to determine position is using prefix which is constant.
    let insert_extra_at = TextSpan::new_with_width(
        suffix.map(|suffix| file_end.sub_width(suffix)).unwrap_or(file_end),
        TextWidth::ZERO,
    )
    .to_str_range();
    let extra_content: String =
        files.filter_map(|file| expand(db, file, None, og_files, inline_files)).collect();

    content.replace_range(insert_extra_at, &extra_content);

    Some(content)
}

fn expand_inline<'db>(
    db: &'db dyn Database,
    start_file: FileId<'db>,
    og_files: &[FileId<'db>],
) -> Option<String> {
    let mut files = direct_child_files(db, og_files, start_file);

    files.sort_by_key(|file| Reverse(file.as_virtual(db).parent.unwrap().span));

    let mut content = db.file_content(start_file)?.to_string();

    for file in files {
        let range = file.as_virtual(db).parent.unwrap().span.to_str_range();

        content.replace_range(range, &expand_inline(db, file, og_files)?);
    }

    Some(content)
}

fn direct_child_files<'db>(
    db: &'db dyn Database,
    files: &[FileId<'db>],
    start_file: FileId<'db>,
) -> Vec<FileId<'db>> {
    files
        .iter()
        .filter(|file| {
            file.maybe_as_virtual(db)
                .and_then(|vfs| vfs.parent)
                .is_some_and(|parent| parent.file_id == start_file)
        })
        .copied()
        .collect()
}
