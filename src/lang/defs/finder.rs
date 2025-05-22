use crate::lang::db::{AnalysisDatabase, LsSemanticGroup};
use cairo_lang_defs::db::DefsGroup;
use cairo_lang_defs::ids::{
    EnumLongId, GenericTypeId, ImplDefLongId, ImplItemId, LanguageElementId, LookupItemId,
    MemberId, ModuleId, NamedLanguageElementId, StructLongId, SubmoduleLongId, TraitItemId, VarId,
};
use cairo_lang_parser::db::ParserGroup;
use cairo_lang_semantic::db::{SemanticGroup, get_resolver_data_options};
use cairo_lang_semantic::diagnostic::{NotFoundItemType, SemanticDiagnostics};
use cairo_lang_semantic::expr::inference::InferenceId;
use cairo_lang_semantic::expr::pattern::QueryPatternVariablesFromDb;
use cairo_lang_semantic::items::function_with_body::SemanticExprLookup;
use cairo_lang_semantic::items::functions::GenericFunctionId;
use cairo_lang_semantic::items::imp::ImplLongId;
use cairo_lang_semantic::lookup_item::LookupItemEx;
use cairo_lang_semantic::resolve::ResolvedGenericItem::TraitItem;
use cairo_lang_semantic::resolve::{
    AsSegments, ResolvedConcreteItem, ResolvedGenericItem, Resolver, ResolverData,
};
use cairo_lang_semantic::substitution::SemanticRewriter;
use cairo_lang_semantic::{ConcreteImplId, Expr, TypeLongId};
use cairo_lang_syntax::node::ast::TerminalIdentifier;
use cairo_lang_syntax::node::helpers::{GetIdentifier, HasName};
use cairo_lang_syntax::node::ids::SyntaxStablePtrId;
use cairo_lang_syntax::node::{SyntaxNode, Terminal, TypedStablePtr, TypedSyntaxNode, ast};
use cairo_lang_utils::smol_str::SmolStr;
use cairo_lang_utils::{Intern, LookupIntern};
use if_chain::if_chain;
use itertools::Itertools;

/// A language element that can be a result of name resolution performed by CairoLS.
///
/// This is a strict superset of things Cairo compiler does name resolution for.
/// CairoLS tries to cover all navigation scenarios, while the compiler doesn't have to,
/// therefore, we need to add some extra layer of code over it.
/// As an example, the compiler never resolves to generic associated trait items
/// because it is coded in such a way, 🤷.
#[derive(Debug)]
pub enum ResolvedItem {
    // Compiler-handled cases.
    Generic(ResolvedGenericItem),
    Concrete(ResolvedConcreteItem),

    // CairoLS-specific additions.
    Member(MemberId),
    ImplItem(ImplItemId),
    ExprInlineMacro(SmolStr),
}

pub fn find_definition(
    db: &AnalysisDatabase,
    identifier: &ast::TerminalIdentifier,
    lookup_items: &[LookupItemId],
    resolver_data: &mut Option<ResolverData>,
) -> Option<ResolvedItem> {
    try_inline_macro(db, identifier)
        .or_else(|| try_submodule_name(db, identifier))
        .or_else(|| try_member(db, identifier, lookup_items))
        .or_else(|| try_member_from_constructor(db, identifier, lookup_items))
        .or_else(|| try_member_declaration(db, identifier))
        .or_else(|| try_variant_declaration(db, identifier))
        .or_else(|| try_variable_declaration(db, identifier, lookup_items))
        .or_else(|| try_impl_item_usages(db, identifier, lookup_items))
        .or_else(|| try_concrete_type_or_impl(db, identifier, lookup_items))
        .or_else(|| lookup_resolved_items(db, identifier, lookup_items, resolver_data))
        .or_else(|| lookup_item_name(db, identifier, lookup_items))
}

pub fn find_declaration(
    db: &AnalysisDatabase,
    identifier: &ast::TerminalIdentifier,
    lookup_items: &[LookupItemId],
    resolver_data: &mut Option<ResolverData>,
) -> Option<ResolvedItem> {
    let def = find_definition(db, identifier, lookup_items, resolver_data)?;
    let decl = try_impl_items(db, &TerminalIdentifier::cast(db, def.definition_node(db)?)?);

    decl.or(Some(def))
}

/// Tries to find a trait's impl item via trait's item usage, if we're on using its' path (ExprPath) in code.
/// This needs to be done because resolver skips a step where a associated type/impl is bound to a specific impl,
/// and resolves the aforementioned path to the bound type/impl directly.
fn try_impl_item_usages(
    db: &AnalysisDatabase,
    identifier: &TerminalIdentifier,
    lookup_items: &[LookupItemId],
) -> Option<ResolvedItem> {
    // Find if we're on ExprPath
    let path_item_segments =
        identifier.as_syntax_node().ancestor_of_type::<ast::ExprPath>(db)?.to_segments(db);

    // Snip off the path after the identifier we're on
    let path_item_segments_cloned: Vec<_> = path_item_segments
        .iter()
        .take_while_inclusive(|segment| segment.identifier(db) != identifier.text(db))
        .cloned()
        .collect();

    // The last element is the name
    let (associated_item_name_candidate, impl_prefix_candidate) =
        path_item_segments_cloned.split_last()?;
    let impl_prefix_candidate = impl_prefix_candidate.to_vec();
    if impl_prefix_candidate.is_empty() {
        return None;
    }

    let module_file_id = db.find_module_file_containing_node(&identifier.as_syntax_node())?;

    let try_find_impl_id = || {
        for &lookup_item_id in lookup_items {
            let mut resolver = Resolver::new(
                db,
                module_file_id,
                InferenceId::LookupItemDeclaration(lookup_item_id),
            );

            let diags = &mut SemanticDiagnostics::default();
            if let Ok(ResolvedConcreteItem::Impl(impl_id)) = resolver.resolve_concrete_path(
                diags,
                impl_prefix_candidate.clone(),
                NotFoundItemType::Impl,
            ) {
                return Some(impl_id);
            }
        }
        None
    };
    let impl_id = try_find_impl_id()?;

    let ImplLongId::Concrete(concrete_impl_id) = db.lookup_intern_impl(impl_id) else {
        return None;
    };

    let concrete_impl_long_id = concrete_impl_id.lookup_intern(db);
    let item = db
        .impl_item_by_name(
            concrete_impl_long_id.impl_def_id,
            associated_item_name_candidate
                .as_syntax_node()
                .get_text_without_trivia(db)
                .parse()
                .unwrap(),
        )
        .ok()??;
    Some(ResolvedItem::ImplItem(item))
}

fn try_inline_macro(
    db: &AnalysisDatabase,
    identifier: &ast::TerminalIdentifier,
) -> Option<ResolvedItem> {
    if_chain!(
        if let Some(macro_call) = identifier.as_syntax_node().ancestor_of_type::<ast::ExprInlineMacro>(db);
        let path_elements = macro_call.path(db).segments(db).elements(db);
        if let Some(macro_name) = path_elements.last();
        if macro_name.identifier(db) == identifier.text(db);

        then {
            Some(ResolvedItem::ExprInlineMacro(
                macro_call.path(db).as_syntax_node().get_text_without_trivia(db).into(),
            ))
        } else {
            None
        }
    )
}

/// Resolve elements of `impl`s to trait definitions.
fn try_impl_items(
    db: &AnalysisDatabase,
    identifier: &ast::TerminalIdentifier,
) -> Option<ResolvedItem> {
    let Some(item_impl) = &identifier.as_syntax_node().ancestor_of_type::<ast::ItemImpl>(db) else {
        return None;
    };
    let long_id = ImplDefLongId(
        db.find_module_file_containing_node(&identifier.as_syntax_node())?,
        item_impl.stable_ptr(db),
    )
    .intern(db);
    let trait_id = db.impl_def_concrete_trait(long_id).ok()?.trait_id(db);

    if let Some(function) =
        identifier.as_syntax_node().parent_of_type::<ast::FunctionDeclaration>(db)
    {
        let function_name = function.name(db);
        if &function_name == identifier {
            let function_name = function_name.text(db);
            let function = db.trait_function_by_name(trait_id, function_name).ok()??;
            return Some(ResolvedItem::Generic(TraitItem(TraitItemId::Function(function))));
        }
    }

    if let Some(constant) = identifier.as_syntax_node().ancestor_of_type::<ast::ItemConstant>(db) {
        let constant_name = constant.name(db);
        if &constant_name == identifier {
            let constant_name = constant_name.text(db);
            let constant = db.trait_constant_by_name(trait_id, constant_name).ok()??;
            return Some(ResolvedItem::Generic(TraitItem(TraitItemId::Constant(constant))));
        }
    }

    if let Some(associated_type) =
        identifier.as_syntax_node().ancestor_of_type::<ast::ItemTypeAlias>(db)
    {
        let associated_type_name = associated_type.name(db);
        if &associated_type_name == identifier {
            let associated_type_name = associated_type_name.text(db);
            let associated_type = db.trait_type_by_name(trait_id, associated_type_name).ok()??;
            return Some(ResolvedItem::Generic(TraitItem(TraitItemId::Type(associated_type))));
        }
    }

    if let Some(associated_impl) =
        identifier.as_syntax_node().ancestor_of_type::<ast::ItemImplAlias>(db)
    {
        let associated_impl_name = associated_impl.name(db);
        if &associated_impl_name == identifier {
            let associated_impl_name = associated_impl_name.text(db);
            let associated_impl = db.trait_impl_by_name(trait_id, associated_impl_name).ok()??;
            return Some(ResolvedItem::Generic(TraitItem(TraitItemId::Impl(associated_impl))));
        }
    }

    None
}

/// Resolve `mod <ident>` syntax.
fn try_submodule_name(
    db: &AnalysisDatabase,
    identifier: &ast::TerminalIdentifier,
) -> Option<ResolvedItem> {
    let item_module = identifier
        .as_syntax_node()
        .parent_of_type::<ast::ItemModule>(db)
        .filter(|item_module| item_module.name(db) == *identifier)?;
    let containing_module_file_id =
        db.find_module_file_containing_node(&item_module.as_syntax_node())?;
    let submodule_id =
        SubmoduleLongId(containing_module_file_id, item_module.stable_ptr(db)).intern(db);
    Some(ResolvedItem::Generic(ResolvedGenericItem::Module(ModuleId::Submodule(submodule_id))))
}

/// Resolve `let _ = Struct = { <ident>: ... }` syntax.
fn try_member_from_constructor(
    db: &AnalysisDatabase,
    identifier: &ast::TerminalIdentifier,
    lookup_items: &[LookupItemId],
) -> Option<ResolvedItem> {
    let function_id = lookup_items.first()?.function_with_body()?;

    let identifier_node = identifier.as_syntax_node();

    let constructor_expr = identifier_node.ancestor_of_type::<ast::ExprStructCtorCall>(db)?;
    let constructor_expr_id =
        db.lookup_expr_by_ptr(function_id, constructor_expr.stable_ptr(db).into()).ok()?;

    let Expr::StructCtor(constructor_expr_semantic) =
        db.expr_semantic(function_id, constructor_expr_id)
    else {
        return None;
    };

    let struct_member = ast::StructArgSingle::cast(db, identifier_node.parent(db)?)?;

    let struct_member_name =
        struct_member.identifier(db).as_syntax_node().get_text_without_trivia(db);

    let member_id = constructor_expr_semantic
        .members
        .iter()
        .find_map(|(id, _)| struct_member_name.eq(id.name(db).as_str()).then_some(*id))?;

    Some(ResolvedItem::Member(member_id))
}

/// Resolve the right-hand side of access member expression e.g. `self.<ident>`.
fn try_member(
    db: &AnalysisDatabase,
    identifier: &ast::TerminalIdentifier,
    lookup_items: &[LookupItemId],
) -> Option<ResolvedItem> {
    let syntax_node = identifier.as_syntax_node();
    let binary_expr = syntax_node.ancestor_of_type::<ast::ExprBinary>(db)?;

    let function_with_body = lookup_items.first()?.function_with_body()?;

    let expr_id =
        db.lookup_expr_by_ptr(function_with_body, binary_expr.stable_ptr(db).into()).ok()?;
    let semantic_expr = db.expr_semantic(function_with_body, expr_id);

    let Expr::MemberAccess(expr_member_access) = semantic_expr else { return None };

    let pointer_to_rhs = binary_expr.rhs(db).stable_ptr(db).untyped();

    let mut current_node = syntax_node;
    // Check if the terminal identifier points to a member, not a struct variable.
    while pointer_to_rhs != current_node.stable_ptr(db) {
        // If we found the node with the binary expression, then we're sure we won't find the
        // node with the member.
        if current_node.stable_ptr(db) == binary_expr.stable_ptr(db).untyped() {
            return None;
        }
        current_node = current_node.parent(db).unwrap();
    }

    let member_id = expr_member_access.member;
    Some(ResolvedItem::Member(member_id))
}

/// Resolve `struct Foo { <ident>: ... }` syntax.
fn try_member_declaration(
    db: &AnalysisDatabase,
    identifier: &ast::TerminalIdentifier,
) -> Option<ResolvedItem> {
    let member = identifier
        .as_syntax_node()
        .parent_of_type::<ast::Member>(db)
        .filter(|member| member.name(db) == *identifier)?;
    let item_struct = member.as_syntax_node().ancestor_of_type::<ast::ItemStruct>(db)?;
    let struct_id = StructLongId(
        db.find_module_file_containing_node(&item_struct.as_syntax_node())?,
        item_struct.stable_ptr(db),
    )
    .intern(db);
    let struct_members = db.struct_members(struct_id).ok()?;
    let member_id = struct_members.get(&member.name(db).text(db))?.id;
    Some(ResolvedItem::Member(member_id))
}

/// Resolve `enum Foo { <ident> }` syntax.
fn try_variant_declaration(
    db: &AnalysisDatabase,
    identifier: &ast::TerminalIdentifier,
) -> Option<ResolvedItem> {
    let variant = identifier
        .as_syntax_node()
        .ancestor_of_type::<ast::Variant>(db)
        .filter(|variant| variant.name(db) == *identifier)?;
    let item_enum = variant.as_syntax_node().ancestor_of_type::<ast::ItemEnum>(db)?;
    let enum_id = EnumLongId(
        db.find_module_file_containing_node(&item_enum.as_syntax_node())?,
        item_enum.stable_ptr(db),
    )
    .intern(db);
    let enum_variants = db.enum_variants(enum_id).ok()?;
    let variant_id = *enum_variants.get(&variant.name(db).text(db))?;
    let variant = db.variant_semantic(enum_id, variant_id).ok()?;
    Some(ResolvedItem::Generic(ResolvedGenericItem::Variant(variant)))
}

/// Lookups if the identifier is a declaration of a variable/param in one of the lookup items.
///
/// Declaration identifiers aren't kept in `ResolvedData`, which is searched for by
/// `lookup_resolved_generic_item_by_ptr` and `lookup_resolved_concrete_item_by_ptr`.
/// Therefore, we have to look for these ourselves.
fn try_variable_declaration(
    db: &AnalysisDatabase,
    identifier: &ast::TerminalIdentifier,
    lookup_items: &[LookupItemId],
) -> Option<ResolvedItem> {
    let function_id = lookup_items.first()?.function_with_body()?;

    // Look at function parameters.
    if let Some(param) = identifier
        .as_syntax_node()
        .parent_of_type::<ast::Param>(db)
        .filter(|param| param.name(db) == *identifier)
    {
        // Closures have different semantic model structures than regular functions.
        let params = if let Some(expr_closure_ast) =
            param.as_syntax_node().ancestor_of_type::<ast::ExprClosure>(db)
        {
            let expr_id =
                db.lookup_expr_by_ptr(function_id, expr_closure_ast.stable_ptr(db).into()).ok()?;
            let Expr::ExprClosure(expr_closure_semantic) = db.expr_semantic(function_id, expr_id)
            else {
                // Break in case Expr::Missing was here.
                return None;
            };
            expr_closure_semantic.params
        } else {
            let signature = db.function_with_body_signature(function_id).ok()?;
            signature.params
        };

        if let Some(param) =
            params.into_iter().find(|param| param.stable_ptr == identifier.stable_ptr(db))
        {
            let var_id = VarId::Param(param.id);
            return Some(ResolvedItem::Generic(ResolvedGenericItem::Variable(var_id)));
        }
    }

    // Look at identifier patterns in the function body.
    if let Some(pattern_ast) = identifier.as_syntax_node().ancestor_of_type::<ast::Pattern>(db) {
        let pattern_id = db.lookup_pattern_by_ptr(function_id, pattern_ast.stable_ptr(db)).ok()?;
        let pattern = db.pattern_semantic(function_id, pattern_id);
        let pattern_variable = pattern
            .variables(&QueryPatternVariablesFromDb(db, function_id))
            .into_iter()
            .find(|var| var.name == identifier.text(db))?;
        let var_id = VarId::Local(pattern_variable.var.id);
        return Some(ResolvedItem::Generic(ResolvedGenericItem::Variable(var_id)));
    }

    None
}

/// Resolves concrete type identifiers and impl identifiers in type-level expressions and type annotations.
/// In particular, handles **type and impl aliases** which require special care.
fn try_concrete_type_or_impl(
    db: &AnalysisDatabase,
    identifier: &ast::TerminalIdentifier,
    lookup_items: &[LookupItemId],
) -> Option<ResolvedItem> {
    let ptr = identifier.stable_ptr(db);
    let module_file_id = db.find_module_file_containing_node(&identifier.as_syntax_node())?;

    for &lookup_item_id in lookup_items {
        let resolved_generic = db.lookup_resolved_generic_item_by_ptr(lookup_item_id, ptr);

        // If the type obviously resolves to an alias, return immedietely.
        // This happens only in the `use` path.
        match resolved_generic {
            Some(item @ ResolvedGenericItem::GenericTypeAlias(_)) => {
                return Some(ResolvedItem::Generic(item));
            }
            Some(item @ ResolvedGenericItem::GenericImplAlias(_)) => {
                return Some(ResolvedItem::Generic(item));
            }
            _ => (),
        }

        // Compiler resolves the type/impl aliases recursively, until it reaches a concrete type.
        // If we call [`SemanticGroup::resolve_concrete_item_by_ptr`] on the identifier,
        // we will receive the aliased type/impl instead of the alias.
        //
        // To avoid this, we try resolving the identifier as a path, which is handled differently by the resolver.
        // This allows us to reach the definition of the alias.

        let type_path_segments =
            identifier.as_syntax_node().ancestor_of_type::<ast::ExprPath>(db)?.to_segments(db);

        if type_path_segments
            .last()
            .is_none_or(|segment| segment.identifier(db) != identifier.text(db))
        {
            return None;
        }

        let mut resolver =
            Resolver::new(db, module_file_id, InferenceId::LookupItemDeclaration(lookup_item_id));

        // Concrete types and type/impl aliases resolve correctly when interpreted as paths.
        let resolved_item = resolver
            .resolve_generic_path_with_args(
                &mut SemanticDiagnostics::default(),
                type_path_segments,
                NotFoundItemType::Identifier,
            )
            .ok()?;

        match &resolved_item {
            ResolvedGenericItem::GenericType(_)
            | ResolvedGenericItem::GenericTypeAlias(_)
            | ResolvedGenericItem::Impl(_)
            | ResolvedGenericItem::GenericImplAlias(_) => {
                return Some(ResolvedItem::Generic(resolved_item));
            }
            _ => (),
        }
    }

    None
}

/// Lookups for the identifier in compiler's `lookup_resolved_*_item_by_ptr` queries.
fn lookup_resolved_items(
    db: &AnalysisDatabase,
    identifier: &ast::TerminalIdentifier,
    lookup_items: &[LookupItemId],
    resolver_data: &mut Option<ResolverData>,
) -> Option<ResolvedItem> {
    let ptr = identifier.stable_ptr(db);

    for &lookup_item_id in lookup_items {
        match db.lookup_resolved_concrete_item_by_ptr(lookup_item_id, ptr) {
            None |
            // Const cannot be easily converted into generic from concrete so return it unresolved.
            Some(ResolvedConcreteItem::Constant(_)) => {
                if let Some(item) = db.lookup_resolved_generic_item_by_ptr(lookup_item_id, ptr) {
                    return Some(ResolvedItem::Generic(item));
                }
            }
            Some(item) => {
                // Mostly copied from `db.lookup_resolved_concrete_item_by_ptr`.
                *resolver_data = get_resolver_data_options(lookup_item_id, db)
                    .into_iter()
                    .find(|resolver_data| resolver_data.resolved_items.concrete.contains_key(&ptr))
                    .map(|data| data.clone_with_inference_id(db, InferenceId::NoContext));
                // We infer the impl function here manually, since it cannot be handled via resolver directly.
                // This would default to generic function later, which we don't want to happen if we can infer it.
                return try_infer_impl_function(db, resolver_data, item.clone()).or(Some(ResolvedItem::Concrete(item)));
            }
        }
    }
    None
}

/// Tries to redirect from the usage of the trait function to a concrete impl in user code
fn try_infer_impl_function(
    db: &AnalysisDatabase,
    resolver_data: &mut Option<ResolverData>,
    item: ResolvedConcreteItem,
) -> Option<ResolvedItem> {
    if let ResolvedConcreteItem::Function(function_id) = item {
        let concrete_fn = function_id.get_concrete(db);
        if let GenericFunctionId::Impl(impl_generic_function_id) = concrete_fn.generic_function {
            let impl_long_id = impl_generic_function_id.impl_id.lookup_intern(db);
            let concrete_impl_id = rewrite_impl(db, impl_long_id, resolver_data)?;
            let impl_func_id = concrete_impl_id
                .get_impl_function(db, impl_generic_function_id.function)
                .ok()??;

            return Some(ResolvedItem::ImplItem(ImplItemId::Function(impl_func_id)));
        }
    }
    None
}

/// Tries to rewrite trait function usage to a concrete function of an impl
fn rewrite_impl(
    db: &AnalysisDatabase,
    impl_long_id: ImplLongId,
    resolver_data: &mut Option<ResolverData>,
) -> Option<ConcreteImplId> {
    if let Some(resolver_data) = resolver_data {
        let mut inference = resolver_data.inference_data.inference(db);
        let rewritten = inference.rewrite(impl_long_id).ok()?;
        if let ImplLongId::Concrete(concrete_impl_id) = rewritten {
            return Some(concrete_impl_id);
        }
    }
    None
}

fn lookup_item_name(
    db: &AnalysisDatabase,
    identifier: &ast::TerminalIdentifier,
    lookup_items: &[LookupItemId],
) -> Option<ResolvedItem> {
    let lookup_item = lookup_items.first().copied()?;

    if lookup_item.name_identifier(db).stable_ptr(db) != identifier.stable_ptr(db) {
        return None;
    }

    ResolvedItem::from_lookup_item(db, lookup_item)
}

impl ResolvedItem {
    /// Finds a stable pointer to the syntax node which defines this resolved item.
    pub fn definition_stable_ptr(&self, db: &AnalysisDatabase) -> Option<SyntaxStablePtrId> {
        // TIP: This is a var so that highlighting exit points in IDEs of this function is usable.
        let stable_ptr = match self {
            // Concrete items.
            ResolvedItem::Concrete(concrete_item @ ResolvedConcreteItem::Type(ty)) => {
                if let TypeLongId::GenericParameter(param) = ty.lookup_intern(db) {
                    param.untyped_stable_ptr(db)
                } else {
                    // Try convert into generic and call definition_stable_ptr recursively.
                    Self::Generic(concrete_item.generic(db)?).definition_stable_ptr(db)?
                }
            }

            ResolvedItem::Concrete(concrete_item @ ResolvedConcreteItem::Impl(imp)) => {
                if let ImplLongId::GenericParameter(param) = imp.lookup_intern(db) {
                    param.untyped_stable_ptr(db)
                } else {
                    // Try convert into generic and call definition_stable_ptr recursively.
                    Self::Generic(concrete_item.generic(db)?).definition_stable_ptr(db)?
                }
            }

            ResolvedItem::Concrete(concrete_item) => {
                // Try convert into generic and call definition_stable_ptr recursively.
                Self::Generic(concrete_item.generic(db)?).definition_stable_ptr(db)?
            }

            // Generic items.
            ResolvedItem::Generic(ResolvedGenericItem::GenericConstant(item)) => {
                item.stable_ptr(db).lookup(db).name(db).stable_ptr(db).untyped()
            }

            ResolvedItem::Generic(ResolvedGenericItem::Module(module_id)) => {
                match module_id {
                    ModuleId::CrateRoot(_) => {
                        // For crate root files (src/lib.cairo), the definition node is the file
                        // itself.
                        let module_file = db.module_main_file(*module_id).ok()?;
                        let file_syntax = db.file_module_syntax(module_file).ok()?;
                        file_syntax.as_syntax_node().stable_ptr(db)
                    }
                    ModuleId::Submodule(submodule_id) => {
                        // For submodules, the definition node is the identifier in `mod <ident>
                        // .*`.
                        submodule_id.stable_ptr(db).lookup(db).name(db).stable_ptr(db).untyped()
                    }
                }
            }

            ResolvedItem::Generic(ResolvedGenericItem::GenericFunction(item)) => {
                let declaration: ast::FunctionDeclaration = match item {
                    GenericFunctionId::Free(id) => id.stable_ptr(db).lookup(db).declaration(db),
                    GenericFunctionId::Extern(id) => id.stable_ptr(db).lookup(db).declaration(db),
                    GenericFunctionId::Impl(id) => match id.impl_function(db) {
                        Ok(Some(id)) => id.stable_ptr(db).lookup(db).declaration(db),
                        // It is possible (Marek didn't find out how it happens) that we hop into
                        // a situation where concrete impl is not inferred yet, so we can't find the
                        // declaration. Fall back to trait function in such cases.
                        _ => id.function.stable_ptr(db).lookup(db).declaration(db),
                    },
                };
                declaration.name(db).stable_ptr(db).untyped()
            }

            ResolvedItem::Generic(ResolvedGenericItem::GenericType(generic_type)) => {
                match generic_type {
                    GenericTypeId::Struct(struct_id) => {
                        struct_id.stable_ptr(db).lookup(db).name(db).stable_ptr(db).untyped()
                    }
                    GenericTypeId::Enum(enum_id) => {
                        enum_id.stable_ptr(db).lookup(db).name(db).stable_ptr(db).untyped()
                    }
                    GenericTypeId::Extern(extern_type_id) => {
                        extern_type_id.stable_ptr(db).lookup(db).name(db).stable_ptr(db).untyped()
                    }
                }
            }

            ResolvedItem::Generic(ResolvedGenericItem::GenericTypeAlias(type_alias)) => {
                type_alias.stable_ptr(db).lookup(db).name(db).stable_ptr(db).untyped()
            }

            ResolvedItem::Generic(ResolvedGenericItem::GenericImplAlias(impl_alias)) => {
                impl_alias.stable_ptr(db).lookup(db).name(db).stable_ptr(db).untyped()
            }

            ResolvedItem::Generic(ResolvedGenericItem::Variant(variant)) => {
                variant.id.stable_ptr(db).lookup(db).name(db).stable_ptr(db).untyped()
            }

            ResolvedItem::Generic(ResolvedGenericItem::Trait(trt)) => {
                trt.stable_ptr(db).lookup(db).name(db).stable_ptr(db).untyped()
            }

            ResolvedItem::Generic(ResolvedGenericItem::Impl(imp)) => {
                imp.stable_ptr(db).lookup(db).name(db).stable_ptr(db).untyped()
            }

            ResolvedItem::Generic(ResolvedGenericItem::Variable(var)) => match var {
                VarId::Param(param) => {
                    param.stable_ptr(db).lookup(db).name(db).stable_ptr(db).untyped()
                }
                VarId::Local(var) => var.untyped_stable_ptr(db),
                VarId::Item(item) => item.name_stable_ptr(db),
            },

            ResolvedItem::Generic(ResolvedGenericItem::TraitItem(trait_item)) => match trait_item {
                TraitItemId::Function(trait_function) => {
                    trait_function.stable_ptr(db).lookup(db).name(db).stable_ptr(db).untyped()
                }
                TraitItemId::Type(trait_type) => {
                    trait_type.stable_ptr(db).lookup(db).name(db).stable_ptr(db).untyped()
                }
                TraitItemId::Constant(trait_constant) => {
                    trait_constant.stable_ptr(db).lookup(db).name(db).stable_ptr(db).untyped()
                }
                TraitItemId::Impl(trait_impl) => {
                    trait_impl.stable_ptr(db).lookup(db).name(db).stable_ptr(db).untyped()
                }
            },

            ResolvedItem::Generic(ResolvedGenericItem::Macro(_))
            | ResolvedItem::ExprInlineMacro(_) => return None,

            // Other variants.
            ResolvedItem::Member(member_id) => {
                member_id.stable_ptr(db).lookup(db).name(db).stable_ptr(db).untyped()
            }

            ResolvedItem::ImplItem(impl_item) => match impl_item {
                ImplItemId::Function(impl_function) => {
                    impl_function.stable_ptr(db).lookup(db).name(db).stable_ptr(db).untyped()
                }
                ImplItemId::Type(impl_type) => {
                    impl_type.stable_ptr(db).lookup(db).name(db).stable_ptr(db).untyped()
                }
                ImplItemId::Constant(impl_constant) => {
                    impl_constant.stable_ptr(db).lookup(db).name(db).stable_ptr(db).untyped()
                }
                ImplItemId::Impl(impl_impl) => {
                    impl_impl.stable_ptr(db).lookup(db).name(db).stable_ptr(db).untyped()
                }
            },
        };
        Some(stable_ptr)
    }

    /// Finds and returns the syntax node corresponding to the definition of the resolved item.
    pub fn definition_node(&self, db: &AnalysisDatabase) -> Option<SyntaxNode> {
        self.definition_stable_ptr(db).map(|stable_ptr| stable_ptr.lookup(db))
    }

    /// Re-wraps a [`LookupItemId`] into the corresponding [`ResolvedItem`].
    fn from_lookup_item(db: &dyn SemanticGroup, lookup_item: LookupItemId) -> Option<Self> {
        match lookup_item {
            LookupItemId::ModuleItem(module_item) => {
                ResolvedGenericItem::from_module_item(db, module_item).ok().map(Self::Generic)
            }
            LookupItemId::TraitItem(trait_item) => {
                Some(Self::Generic(ResolvedGenericItem::TraitItem(trait_item)))
            }
            LookupItemId::ImplItem(impl_item) => Some(Self::ImplItem(impl_item)),
        }
    }
}
