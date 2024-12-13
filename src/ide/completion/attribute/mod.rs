use cairo_lang_defs::db::DefsGroup;
use cairo_lang_defs::plugin::MacroPlugin;
use cairo_lang_filesystem::ids::CrateId;
use cairo_lang_syntax::node::TypedSyntaxNode;
use cairo_lang_syntax::node::ast::Attribute;
use lsp_types::{CompletionItem, CompletionItemKind};

use crate::lang::db::AnalysisDatabase;

pub mod derive;

pub fn attribute_completions(
    db: &AnalysisDatabase,
    attribute: Attribute,
    crate_id: CrateId,
) -> Option<Vec<CompletionItem>> {
    let plugins = db.crate_macro_plugins(crate_id);

    let attr_name = attribute.attr(db).as_syntax_node().get_text(db);

    Some(
        plugins
            .iter()
            .flat_map(|plugin_id| db.lookup_intern_macro_plugin(*plugin_id).declared_attributes())
            .filter(|name| {
                // Don't suggest already typed one.
                name.starts_with(&attr_name) && name != &attr_name
            })
            .map(|name| CompletionItem {
                label: name,
                kind: Some(CompletionItemKind::FUNCTION),
                ..Default::default()
            })
            .collect(),
    )
}
