use cairo_lang_defs::db::DefsGroup;
use cairo_lang_defs::plugin::MacroPlugin;
use cairo_lang_filesystem::ids::CrateId;
use cairo_lang_syntax::node::TypedSyntaxNode;
use cairo_lang_syntax::node::ast::Attribute;
use lsp_types::{CompletionItem, CompletionItemKind};

use crate::lang::db::AnalysisDatabase;

pub fn derive_completions(
    db: &AnalysisDatabase,
    derive_name: &str,
    attribute: Attribute,
    crate_id: CrateId,
) -> Option<Vec<CompletionItem>> {
    let plugins = db.crate_macro_plugins(crate_id);

    let attr_name = attribute.attr(db).as_syntax_node().get_text(db);
    let is_derive = attr_name == "derive";

    is_derive.then(|| {
        plugins
            .iter()
            .flat_map(|id| db.lookup_intern_macro_plugin(*id).declared_derives())
            .filter(|name| name.starts_with(derive_name) && name != derive_name)
            .map(|name| CompletionItem {
                label: name,
                kind: Some(CompletionItemKind::FUNCTION),
                ..Default::default()
            })
            .collect()
    })
}
