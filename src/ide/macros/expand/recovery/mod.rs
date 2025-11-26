use cairo_lang_defs::ids::ModuleId;
use cairo_lang_filesystem::db::FilesGroup;
use cairo_lang_filesystem::ids::{FileId, FileKind, FileLongId, VirtualFile};
use cairo_lang_filesystem::ids::{SmolStrId, SpanInFile};
use cairo_lang_parser::db::ParserGroup;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{TypedSyntaxNode, ast::ExprInlineMacro};
use cairo_lang_utils::Intern;
use compiler::expand_single_inline_macro_no_context;
use indoc::indoc;

use crate::ide::macros::expand::format::format_output;
use crate::lang::db::AnalysisDatabase;

mod compiler;

const HEADER: &str = indoc!(
    "
        // ==================================================================
        // This expansion was done out of context.
        // It may be inaccurate — consider expanding the surrounding function
        // for more reliable results.
        // ==================================================================
    "
);

pub fn expand_inline_macro_no_context<'db>(
    db: &'db AnalysisDatabase,
    syntax: ExprInlineMacro<'db>,
    module_id: ModuleId<'db>,
) -> Option<String> {
    let content = expand_single_inline_macro_no_context(db, &syntax, module_id)?;

    let file = FileLongId::Virtual(VirtualFile {
        parent: None,
        name: SmolStrId::from(db, ""),
        content: SmolStrId::from(db, content.as_str()),
        code_mappings: Default::default(),
        kind: FileKind::Expr,
        original_item_removed: true,
    })
    .intern(db);

    let expansion = expand_inline_macro_recursively_no_context(db, file, module_id)?;

    Some(format!("{HEADER}\n\n{}", format_output(&expansion, SyntaxKind::ExprInlineMacro)))
}

pub fn expand_inline_macro_recursively_no_context<'db>(
    db: &'db AnalysisDatabase,
    file: FileId<'db>,
    module_id: ModuleId<'db>,
) -> Option<String> {
    let inline_macros = db
        .file_syntax(file)
        .ok()?
        .descendants(db)
        .filter_map(|node| ExprInlineMacro::cast(db, node))
        .collect::<Vec<_>>()
        .into_iter()
        .rev();

    let mut content = db.file_content(file)?.to_string();

    for inline_macro in inline_macros {
        let macro_content = expand_single_inline_macro_no_context(db, &inline_macro, module_id)?;
        let span = inline_macro.as_syntax_node().span_without_trivia(db);

        let file = FileLongId::Virtual(VirtualFile {
            parent: Some(SpanInFile { file_id: file, span }),
            name: SmolStrId::from(db, ""),
            content: SmolStrId::from(db, macro_content.as_str()),
            code_mappings: Default::default(),
            kind: FileKind::Expr,
            original_item_removed: true,
        })
        .intern(db);

        let expansion = expand_inline_macro_recursively_no_context(db, file, module_id)?;

        content.replace_range(span.to_str_range(), &expansion);
    }

    Some(content)
}
