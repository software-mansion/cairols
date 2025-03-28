use cairo_lang_defs::db::DefsGroup;
use cairo_lang_defs::plugin::InlineMacroExprPlugin;
use cairo_lang_doc::db::DocGroup;
use cairo_lang_filesystem::ids::FileId;
use cairo_lang_syntax::node::TypedSyntaxNode;
use cairo_lang_syntax::node::ast::TerminalIdentifier;
use cairo_lang_utils::Upcast;
use lsp_types::Hover;

use crate::ide::hover::markdown_contents;
use crate::ide::hover::render::markdown::{RULE, fenced_code_block};
use crate::lang::db::AnalysisDatabase;
use crate::lang::defs::SymbolDef;
use crate::lang::lsp::ToLsp;

/// Get declaration and documentation "definition" of an item referred by the given identifier.
pub fn definition(
    db: &AnalysisDatabase,
    identifier: &TerminalIdentifier,
    file_id: FileId,
) -> Option<Hover> {
    let symbol = SymbolDef::find(db, identifier)?;

    let md = match &symbol {
        SymbolDef::Item(item) => {
            let mut md = String::new();
            md += &fenced_code_block(&item.definition_path(db));
            md += &fenced_code_block(&item.signature(db));
            if let Some(doc) = item.documentation(db) {
                md += RULE;
                md += &doc;
            }
            md
        }

        SymbolDef::Module(module) => {
            let mut md = String::new();
            md += &fenced_code_block(&module.definition_path());
            md += &fenced_code_block(&module.signature(db));
            if let Some(doc) = module.documentation(db) {
                md += RULE;
                md += &doc;
            }
            md
        }

        SymbolDef::Variable(var) => fenced_code_block(&var.signature(db)?),
        SymbolDef::ExprInlineMacro(macro_name) => {
            let crate_id = db.file_modules(file_id).ok()?.first()?.owning_crate(db);

            let mut md = fenced_code_block(macro_name);

            if let Some(doc) = db
                .crate_inline_macro_plugins(crate_id)
                .get(macro_name.as_str())
                .map(|&id| db.lookup_intern_inline_macro_plugin(id))?
                .documentation()
            {
                md += RULE;
                md += &doc;
            }
            md
        }
        SymbolDef::Member(member) => {
            let mut md = String::new();

            // Signature is the signature of the struct, so it makes sense that the definition
            // path is too.
            md += &fenced_code_block(&member.struct_item().definition_path(db));
            md += &fenced_code_block(&member.struct_item().signature(db));

            if let Some(doc) = db.get_item_documentation(member.member_id().into()) {
                md += RULE;
                md += &doc;
            }
            md
        }
        SymbolDef::Variant(variant) => {
            let mut md = String::new();

            // Signature is the signature of the enum, so it makes sense that the definition
            // path is too.
            md += &fenced_code_block(&variant.enum_item().definition_path(db));
            md += &fenced_code_block(&variant.enum_item().signature(db));

            if let Some(doc) = db.get_item_documentation(variant.variant_id().into()) {
                md += RULE;
                md += &doc;
            }
            md
        }
    };

    Some(Hover {
        contents: markdown_contents(md),
        range: identifier
            .as_syntax_node()
            .span_without_trivia(db.upcast())
            .position_in_file(db.upcast(), file_id)
            .map(|p| p.to_lsp()),
    })
}
