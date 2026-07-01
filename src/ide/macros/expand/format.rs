use cairo_lang_diagnostics::DiagnosticsBuilder;
use cairo_lang_filesystem::ids::{FileKind, FileLongId, SmolStrId, VirtualFile};
use cairo_lang_formatter::FormatterConfig;
use cairo_lang_parser::parser::Parser;
use cairo_lang_parser::utils::SimpleParserDatabase;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{SyntaxNode, TypedSyntaxNode};
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
            let mut diagnostics = DiagnosticsBuilder::default();

            let green = Parser::parse_file_expr_green(db, &mut diagnostics, virtual_file, output);

            if diagnostics.build().is_empty() {
                // The expansion is a single expression, so the expr parse is complete.
                SyntaxNode::new_detached_root(db, virtual_file, green.0)
            } else {
                // The expansion is not a single expression: an inline macro used in statement
                // position expands to a list of statements. `parse_file_expr` stops at the
                // first statement boundary (truncating the rest) and reports the leftover
                // tokens as diagnostics, so reparse it as a statement list instead.
                //
                // Example:
                // macro my_macro() {
                //     () => {
                //         print!("");
                //         print!("");
                //         print!("");
                //     };
                // }
                //
                // fn foo() {
                //     my_macro!();
                // }
                let green = Parser::parse_file_statement_list_green(
                    db,
                    &mut DiagnosticsBuilder::default(),
                    virtual_file,
                    output,
                );
                SyntaxNode::new_detached_root(db, virtual_file, green.0)
            }
        }
        _ => Parser::parse_file(db, &mut DiagnosticsBuilder::default(), virtual_file, output)
            .as_syntax_node(),
    };

    cairo_lang_formatter::get_formatted_file(db, &syntax_root, FormatterConfig::default())
        .trim_end_matches(";")
        .trim_end()
        .to_owned()
}
