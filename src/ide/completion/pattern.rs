use if_chain::if_chain;
use lsp_types::{CompletionItem, CompletionItemKind};

use crate::lang::db::AnalysisDatabase;
use crate::lang::text_matching::text_matches;
use crate::{
    ide::completion::path::path_prefix_completions, lang::analysis_context::AnalysisContext,
};
use cairo_lang_defs::ids::GenericTypeId;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::diagnostic::NotFoundItemType;
use cairo_lang_semantic::resolve::ResolvedGenericItem;
use cairo_lang_syntax::node::{
    Token, TypedSyntaxNode,
    ast::{PatternEnum, PatternIdentifier, PatternStruct, PatternStructParam},
};
use std::collections::HashSet;

pub fn struct_pattern_completions(
    db: &AnalysisDatabase,
    ctx: &AnalysisContext<'_>,
) -> Vec<CompletionItem> {
    let (all_members, existing_members, typed) = if_chain!(
        if let Some(pattern) = ctx.node.ancestor_of_type::<PatternStruct>(db);
        if let typed = ctx.node.ancestor_of_type::<PatternIdentifier>(db).filter(|ident| {
            ident.as_syntax_node().parent().and_then(|p| p.parent())
                == Some(pattern.as_syntax_node())
        });
        if let Ok(ResolvedGenericItem::GenericType(GenericTypeId::Struct(struct_item))) =
            ctx.resolver(db).resolve_generic_path(
                &mut Default::default(),
                &pattern.path(db),
                NotFoundItemType::Type,
                None,
            );
        if let Ok(all_members) = db.struct_members(struct_item);

        then {
            (all_members, pattern.params(db).elements(db), typed)
        } else {
            return Default::default()
        }
    );

    let existing_members: HashSet<_> = existing_members
        .into_iter()
        .filter_map(|member| match member {
            PatternStructParam::Single(ident) => Some(ident.name(db).token(db).text(db)),
            PatternStructParam::WithExpr(params) => Some(params.name(db).token(db).text(db)),
            PatternStructParam::Tail(_) => None,
        })
        .collect();

    let typed = typed.map(|ident| ident.name(db).token(db).text(db)).unwrap_or_default();

    all_members
        .keys()
        .filter(|member| !existing_members.contains(member.as_str()))
        .filter(|member| text_matches(member, &typed))
        .map(|member| CompletionItem {
            label: member.to_string(),
            kind: Some(CompletionItemKind::VARIABLE),
            ..CompletionItem::default()
        })
        .collect()
}

pub fn enum_pattern_completions(
    db: &AnalysisDatabase,
    ctx: &AnalysisContext<'_>,
) -> Vec<CompletionItem> {
    if_chain!(
        if let Some(pattern) = ctx.node.ancestor_of_type::<PatternEnum>(db);
        let path = pattern.path(db);
        let mut segments = path.elements(db);
        let _ = {
            // If there is tail (ie. some::path::) last segment will be of type missing, remove it.
            if path.has_tail(db) {
                segments.pop();
            }
        };
        if let Some(result) = path_prefix_completions(db, ctx, segments);

        then {
            result
        } else {
            Default::default()
        }
    )
}
