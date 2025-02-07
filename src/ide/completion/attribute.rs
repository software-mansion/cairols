use cairo_lang_defs::db::DefsGroup;
use cairo_lang_syntax::node::TypedSyntaxNode;
use cairo_lang_syntax::node::ast::Attribute;
use lsp_types::{CompletionItem, CompletionItemKind};

use crate::lang::db::AnalysisDatabase;

pub fn attribute_completions(
    db: &AnalysisDatabase,
    attribute: Attribute,
) -> Option<Vec<CompletionItem>> {
    let plugins = db.macro_plugins();

    let attr_name = attribute.attr(db).as_syntax_node().get_text(db);

    Some(
        plugins
            .iter()
            .flat_map(|plugin| plugin.declared_attributes())
            .filter(|name| {
                // Don't suggest already typed one.
                name.starts_with(&attr_name) && name != &attr_name
            })
            .map(macro_completion)
            .collect(),
    )
}

pub fn derive_completions(
    db: &AnalysisDatabase,
    derive_name: &str,
    attribute: Attribute,
) -> Option<Vec<CompletionItem>> {
    let plugins = db.macro_plugins();

    let attr_name = attribute.attr(db).as_syntax_node().get_text(db);
    let is_derive = attr_name == "derive";

    is_derive.then(|| {
        plugins
            .iter()
            .flat_map(|plugin| plugin.declared_derives())
            .filter(|name| name.starts_with(derive_name) && name != derive_name)
            .map(macro_completion)
            .collect()
    })
}

fn macro_completion(name: String) -> CompletionItem {
    CompletionItem { label: name, kind: Some(CompletionItemKind::FUNCTION), ..Default::default() }
}
