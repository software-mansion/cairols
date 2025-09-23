use cairo_lang_diagnostics::DiagnosticsBuilder;
use cairo_lang_filesystem::ids::{FileKind, FileLongId, SmolStrId, VirtualFile};
use cairo_lang_formatter::FormatterConfig;
use cairo_lang_parser::parser::Parser;
use cairo_lang_parser::utils::SimpleParserDatabase;
use cairo_lang_syntax::node::TypedSyntaxNode;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_utils::Intern;

/// Formats output string.
pub fn format_output(output: &str, kind: SyntaxKind) -> String {
    let db = &SimpleParserDatabase::default();
    let virtual_file = FileLongId::Virtual(VirtualFile {
        parent: Default::default(),
        name: SmolStrId::from(db, ""),
        content: SmolStrId::from(db, output),
        code_mappings: Default::default(),
        kind: FileKind::Module,
        original_item_removed: false,
    })
    .intern(db);

    let syntax_root = match kind {
        SyntaxKind::ExprInlineMacro => {
            Parser::parse_file_expr(db, &mut DiagnosticsBuilder::default(), virtual_file, output)
                .as_syntax_node()
        }
        _ => Parser::parse_file(db, &mut DiagnosticsBuilder::default(), virtual_file, output)
            .as_syntax_node(),
    };

    cairo_lang_formatter::get_formatted_file(db, &syntax_root, FormatterConfig::default())
        .trim_end_matches(";")
        .trim_end()
        .to_owned()
}
