use cairo_lang_defs::db::DefsGroup;
use cairo_lang_defs::plugin::MacroPlugin;
use cairo_lang_filesystem::ids::CrateId;
use cairo_lang_syntax::node::ast::Attribute;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{SyntaxNode, TypedSyntaxNode};
use lsp_types::{CompletionItem, CompletionItemKind, CompletionTextEdit, Range, TextEdit};

use crate::ide::completion::helpers::span::{get_empty_arglist_range, get_resultant_range};
use crate::ide::completion::{CompletionItemOrderable, CompletionRelevance};
use crate::lang::{db::AnalysisDatabase, text_matching::text_matches};

pub fn derive_completions<'db>(
    db: &'db AnalysisDatabase,
    node: SyntaxNode<'db>,
    crate_id: CrateId<'db>,
) -> Vec<CompletionItemOrderable> {
    // Check if cursor is on `#[derive(Arg1, Ar<cursor>)]` arguments list.

    if let Some(path_node) = node.ancestor_of_kind(db, SyntaxKind::ExprPath)
        && let Some(node) = path_node.parent_of_kind(db, SyntaxKind::ArgClauseUnnamed)
        && let Some(attr) = node.ancestor_of_type::<Attribute>(db)
        && let Some(span) = get_resultant_range(db, node)
        && let Some(derive_completions) =
            derive_completions_ex(db, path_node.get_text(db), attr, span, crate_id)
    {
        return derive_completions;
    }

    // Check if cursor is on `#[derive(Arg1, <cursor>)]` arguments list.

    if node.ancestor_of_kind(db, SyntaxKind::Arg).is_none()
        && let Some(attr) = node.ancestor_of_type::<Attribute>(db)
        && let Some(span) = get_empty_arglist_range(db, node)
        && let Some(derive_completions) = derive_completions_ex(db, "", attr, span, crate_id)
    {
        return derive_completions;
    }

    vec![]
}
pub fn derive_completions_ex<'db>(
    db: &'db AnalysisDatabase,
    derive_name: &str,
    attribute: Attribute<'db>,
    span: Range,
    crate_id: CrateId<'db>,
) -> Option<Vec<CompletionItemOrderable>> {
    let plugins = db.crate_macro_plugins(crate_id);

    let attr_name = attribute.attr(db).as_syntax_node().get_text(db);
    let is_derive = attr_name == "derive";

    is_derive.then(|| {
        plugins
            .iter()
            .flat_map(|id| id.long(db).declared_derives(db))
            .map(|name| name.to_string(db))
            .filter(|name| text_matches(name, derive_name))
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
            .collect()
    })
}
