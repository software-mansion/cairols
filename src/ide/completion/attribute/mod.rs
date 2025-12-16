use crate::ide::completion::{CompletionItemOrderable, CompletionRelevance};
use crate::lang::lsp::ToLsp;
use crate::lang::{db::AnalysisDatabase, text_matching::text_matches};
use cairo_lang_defs::db::DefsGroup;
use cairo_lang_defs::plugin::MacroPlugin;
use cairo_lang_filesystem::ids::CrateId;
use cairo_lang_syntax::node::ast::Attribute;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{SyntaxNode, TypedSyntaxNode};
use lsp_types::{CompletionItem, CompletionItemKind, CompletionTextEdit, Range, TextEdit};

pub mod derive;

pub fn attribute_completions<'db>(
    db: &'db AnalysisDatabase,
    node: SyntaxNode<'db>,
    crate_id: CrateId<'db>,
) -> Vec<CompletionItemOrderable> {
    // Check if cursor is on attribute name. `#[my_a<cursor>ttr(arg1, args2: 1234)]`
    if let Some(node) = node.ancestor_of_kind(db, SyntaxKind::ExprPath)
        && let Some(attr) = node.parent_of_type::<Attribute>(db)
        && let Some(span) = node.span(db).position_in_file(db, node.stable_ptr(db).file_id(db))
        && let Some(attr_completions) = attribute_completions_ex(db, attr, span.to_lsp(), crate_id)
    {
        return attr_completions;
    }

    vec![]
}

fn attribute_completions_ex<'db>(
    db: &'db AnalysisDatabase,
    attribute: Attribute<'db>,
    span: Range,
    crate_id: CrateId<'db>,
) -> Option<Vec<CompletionItemOrderable>> {
    let plugins = db.crate_macro_plugins(crate_id);

    let attr_name = attribute.attr(db).as_syntax_node().get_text(db);

    Some(
        plugins
            .iter()
            .flat_map(|plugin_id| plugin_id.long(db).declared_attributes(db))
            .map(|name| name.to_string(db))
            .filter(|name| text_matches(name, attr_name))
            .map(|name| CompletionItemOrderable {
                item: CompletionItem {
                    label: name.clone(),
                    kind: Some(CompletionItemKind::FUNCTION),
                    text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                        range: span,
                        new_text: name,
                    })),
                    ..Default::default()
                },
                relevance: CompletionRelevance::High,
            })
            .collect(),
    )
}
