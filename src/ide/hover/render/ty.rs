use cairo_lang_defs::ids::{ImportableId, LookupItemId};
use cairo_lang_semantic::{
    db::SemanticGroup,
    expr::inference::InferenceId,
    items::{function_with_body::SemanticExprLookup, imp::ImplLongId},
    lookup_item::LookupItemEx,
    resolve::ResolvedConcreteItem,
    substitution::SemanticRewriter,
};
use cairo_lang_syntax::node::{
    TypedSyntaxNode,
    ast::{ExprPath, PathSegment, Pattern, TerminalUnderscore},
};
use cairo_lang_utils::{LookupIntern, ordered_hash_map::OrderedHashMap};
use itertools::Itertools;

use super::super::super::markdown::fenced_code_block;
use crate::{
    ide::ty::{InferredValue, format_type},
    lang::{
        db::{AnalysisDatabase, LsSemanticGroup},
        defs::{ResolvedItem, find_definition},
    },
};

pub fn ty(
    db: &AnalysisDatabase,
    underscore: TerminalUnderscore,
    importables: &OrderedHashMap<ImportableId, String>,
) -> Option<String> {
    let lookup_items = db.collect_lookup_items_with_parent_files(underscore.as_syntax_node())?;

    let result = pattern(db, underscore.clone(), &lookup_items, importables)
        .or_else(|| path(db, underscore.clone(), &lookup_items, importables))?;

    Some(fenced_code_block(&result))
}

fn pattern(
    db: &AnalysisDatabase,
    underscore: TerminalUnderscore,
    lookup_items: &[LookupItemId],
    importables: &OrderedHashMap<ImportableId, String>,
) -> Option<String> {
    let function_id = lookup_items.first()?.function_with_body()?;
    let pattern_id = db
        .lookup_pattern_by_ptr(function_id, Pattern::Underscore(underscore).stable_ptr(db))
        .ok()?;

    Some(format_type(db, db.pattern_semantic(function_id, pattern_id).ty(), importables))
}

fn path(
    db: &AnalysisDatabase,
    underscore: TerminalUnderscore,
    lookup_items: &[LookupItemId],
    importables: &OrderedHashMap<ImportableId, String>,
) -> Option<String> {
    let path = underscore.as_syntax_node().ancestor_of_type::<ExprPath>(db)?;

    let mut segments = path.segments(db).elements(db).collect_vec();

    while matches!(segments.last(), Some(PathSegment::Missing(_))) {
        segments.pop();
    }

    let last = segments
        .iter()
        .find(|last| last.as_syntax_node().is_ancestor(db, &underscore.as_syntax_node()))?;

    let PathSegment::WithGenericArgs(generic) = last else {
        // In fact it is unreachable but better safe than sorry.
        return None;
    };

    let (i, _) = generic
        .generic_args(db)
        .generic_args(db)
        .elements(db)
        .enumerate()
        .find(|(_, arg)| arg.as_syntax_node().is_ancestor(db, &underscore.as_syntax_node()))?;
    let identifier = generic.ident(db);

    let mut resolver_data = None;
    let ResolvedItem::Concrete(concrete) =
        find_definition(db, &identifier, lookup_items, &mut resolver_data)?
    else {
        return None;
    };

    let mut resolver_data = resolver_data?.clone_with_inference_id(db, InferenceId::NoContext);

    let mut inference = resolver_data.inference_data.inference(db);

    let arg = match concrete {
        ResolvedConcreteItem::Function(func) => func.get_concrete(db).generic_args[i],
        ResolvedConcreteItem::Trait(trt) => trt.generic_args(db)[i],
        ResolvedConcreteItem::Impl(imp) => {
            let ImplLongId::Concrete(imp) = imp.lookup_intern(db) else { return None };

            imp.lookup_intern(db).generic_args[i]
        }
        _ => return None,
    };

    let result = InferredValue::try_from_generic_arg_id(inference.rewrite(arg).ok()?)?
        .format(db, importables);

    Some(result)
}
