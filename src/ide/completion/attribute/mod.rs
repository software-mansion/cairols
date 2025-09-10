use cairo_lang_defs::db::DefsGroup;
use cairo_lang_defs::plugin::MacroPlugin;
use cairo_lang_filesystem::ids::CrateId;
use cairo_lang_syntax::node::ast::Attribute;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{SyntaxNode, TypedSyntaxNode};
use lsp_types::{CompletionItem, CompletionItemKind};

use crate::ide::completion::{CompletionItemOrderable, CompletionRelevance};
use crate::lang::{db::AnalysisDatabase, text_matching::text_matches};

pub mod derive;

pub fn attribute_completions<'db>(
    db: &'db AnalysisDatabase,
    node: SyntaxNode<'db>,
    crate_id: CrateId<'db>,
) -> Vec<CompletionItemOrderable> {
    // Check if cursor is on attribute name. `#[my_a<cursor>ttr(arg1, args2: 1234)]`
    if let Some(node) = node.ancestor_of_kind(db, SyntaxKind::ExprPath)
        && let Some(attr) = node.parent_of_type::<Attribute>(db)
        && let Some(attr_completions) = attribute_completions_ex(db, attr, crate_id)
    {
        return attr_completions;
    }

    vec![]
}

fn attribute_completions_ex<'db>(
    db: &'db AnalysisDatabase,
    attribute: Attribute<'db>,
    crate_id: CrateId<'db>,
) -> Option<Vec<CompletionItemOrderable>> {
    let plugins = db.crate_macro_plugins(crate_id);

    let attr_name = attribute.attr(db).as_syntax_node().get_text(db);

    Some(
        plugins
            .iter()
            .flat_map(|plugin_id| db.lookup_intern_macro_plugin(*plugin_id).declared_attributes())
            .filter(|name| text_matches(name, attr_name))
            .map(|name| CompletionItemOrderable {
                item: CompletionItem {
                    label: name,
                    kind: Some(CompletionItemKind::FUNCTION),
                    ..Default::default()
                },
                relevance: Some(CompletionRelevance::High),
            })
            .collect(),
    )
}
