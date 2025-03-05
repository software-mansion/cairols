use cairo_lang_defs::ids::{FileIndex, ModuleFileId, NamedLanguageElementId};
use lsp_types::CompletionItem;

use crate::ide::completion::helpers::completion_kind::importable_completion_kind;
use crate::lang::analysis_context::AnalysisContext;
use crate::lang::db::AnalysisDatabase;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_syntax::node::TypedSyntaxNode;
use cairo_lang_syntax::node::ast::ExprPath;
use cairo_lang_syntax::node::kind::SyntaxKind;
use if_chain::if_chain;

pub fn typing_completions(db: &AnalysisDatabase, ctx: &AnalysisContext<'_>) -> Vec<CompletionItem> {
    let (importables, typed_text) = if_chain!(
        if ctx.node.ancestor_of_kind(db, SyntaxKind::Attribute).is_none();
        if let Some(importables) = db.visible_importables_from_module(ModuleFileId(ctx.module_id, FileIndex(0)));
        if let Some(typed_text_segments) = ctx.node.ancestor_of_type::<ExprPath>(db).map(|path| path.elements(db));
        if let [last] = typed_text_segments.as_slice();

        then {
            (importables, last.as_syntax_node().get_text_without_trivia(db))
        } else {
            return Default::default();
        }
    );

    importables
        .iter()
        .filter_map(|(importable, path_str)| {
            let path_segments: Vec<_> = path_str.split("::").collect();

            let last_segment = path_segments.last().expect("path to import should not be empty");

            if !last_segment.starts_with(&typed_text) {
                return None;
            }

            // TODO(#284)
            Some(CompletionItem {
                label: importable.name(db).to_string(),
                kind: Some(importable_completion_kind(*importable)),
                ..CompletionItem::default()
            })
        })
        .collect()
}
