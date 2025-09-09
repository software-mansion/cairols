use std::collections::HashSet;

use cairo_lang_defs::ids::GenericTypeId;
use cairo_lang_semantic::diagnostic::NotFoundItemType;
use cairo_lang_semantic::resolve::ResolvedGenericItem;
use cairo_lang_semantic::{db::SemanticGroup, resolve::ResolutionContext};
use cairo_lang_syntax::node::{
    Token, TypedSyntaxNode,
    ast::{PatternEnum, PatternIdentifier, PatternStruct, PatternStructParam},
};
use itertools::Itertools;
use lsp_types::{CompletionItem, CompletionItemKind};

use crate::ide::completion::CompletionItemOrderable;
use crate::lang::db::AnalysisDatabase;
use crate::lang::text_matching::text_matches;
use crate::{
    ide::completion::path::path_prefix_completions, lang::analysis_context::AnalysisContext,
};

pub fn struct_pattern_completions<'db>(
    db: &'db AnalysisDatabase,
    ctx: &AnalysisContext<'db>,
) -> Vec<CompletionItemOrderable> {
    let (all_members, existing_members, typed) = if let Some(pattern) =
        ctx.node.ancestor_of_type::<PatternStruct>(db)
        && let typed = ctx.node.ancestor_of_type::<PatternIdentifier>(db).filter(|ident| {
            ident.as_syntax_node().parent(db).and_then(|p| p.parent(db))
                == Some(pattern.as_syntax_node())
        })
        && let Ok(ResolvedGenericItem::GenericType(GenericTypeId::Struct(struct_item))) =
            ctx.resolver(db).resolve_generic_path(
                &mut Default::default(),
                &pattern.path(db),
                NotFoundItemType::Type,
                ResolutionContext::Default,
            )
        && let Ok(all_members) = db.struct_members(struct_item)
    {
        (all_members, pattern.params(db).elements(db), typed)
    } else {
        return Default::default();
    };

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
        .filter(|member| !existing_members.contains(&***member))
        .filter(|member| text_matches(&***member, typed))
        .map(|member| CompletionItem {
            label: member.to_string(),
            kind: Some(CompletionItemKind::VARIABLE),
            ..CompletionItem::default()
        })
        .collect()
}

pub fn enum_pattern_completions<'db>(
    db: &'db AnalysisDatabase,
    ctx: &AnalysisContext<'db>,
) -> Vec<CompletionItemOrderable> {
    if let Some(pattern) = ctx.node.ancestor_of_type::<PatternEnum>(db)
        && let path = pattern.path(db)
        && let mut segments = path.segments(db).elements(db).collect_vec()
        && let _ = {
            // If there is tail (ie. some::path::) last segment will be of type missing, remove it.
            if path.segments(db).has_tail(db) {
                segments.pop();
            }
        }
        && let Some(result) = path_prefix_completions(db, ctx, segments)
    {
        result
    } else {
        Default::default()
    }
}
