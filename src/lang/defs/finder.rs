use cairo_lang_defs::db::DefsGroup;
use cairo_lang_defs::ids::{
    EnumLongId, GenericTypeId, LanguageElementId, LocalVarLongId, LookupItemId, MemberId, ModuleId,
    NamedLanguageElementId, StructLongId, SubmoduleLongId, TraitItemId, VarId,
};
use cairo_lang_diagnostics::ToOption;
use cairo_lang_parser::db::ParserGroup;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::items::function_with_body::SemanticExprLookup;
use cairo_lang_semantic::items::functions::{GenericFunctionId, ImplGenericFunctionId};
use cairo_lang_semantic::items::generics::generic_params_to_args;
use cairo_lang_semantic::items::imp::ImplLongId;
use cairo_lang_semantic::lookup_item::LookupItemEx;
use cairo_lang_semantic::resolve::{ResolvedConcreteItem, ResolvedGenericItem};
use cairo_lang_semantic::{ConcreteTraitLongId, Expr, TypeLongId, Variant};
use cairo_lang_syntax::node::ids::SyntaxStablePtrId;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{Terminal, TypedStablePtr, TypedSyntaxNode, ast};
use cairo_lang_utils::{Intern, LookupIntern, Upcast};
use tracing::error;

use crate::lang::db::{AnalysisDatabase, LsSemanticGroup};
use crate::lang::syntax::SyntaxNodeExt;

/// Either [`ResolvedGenericItem`], [`ResolvedConcreteItem`] or [`MemberId`].
pub enum ResolvedItem {
    Generic(ResolvedGenericItem),
    Concrete(ResolvedConcreteItem),
    Member(MemberId),
}

pub fn find_definition(
    db: &AnalysisDatabase,
    identifier: &ast::TerminalIdentifier,
    lookup_items: &[LookupItemId],
) -> Option<(ResolvedItem, SyntaxStablePtrId)> {
    // The lookup_resolved_*_item_by_ptr queries tend to sometimes return an encompassing item
    // instead of actually resolving the identifier.
    // The following series of heuristics resolve the identifier in alternative ways for such cases.

    if let Some(parent) = identifier.as_syntax_node().parent() {
        if parent.kind(db) == SyntaxKind::ItemModule {
            let Some(containing_module_file_id) = db.find_module_file_containing_node(&parent)
            else {
                error!("`find_definition` failed: could not find module");
                return None;
            };

            let submodule_id = SubmoduleLongId(
                containing_module_file_id,
                ast::ItemModule::from_syntax_node(db, parent).stable_ptr(),
            )
            .intern(db);
            let item = ResolvedGenericItem::Module(ModuleId::Submodule(submodule_id));
            return Some((
                ResolvedItem::Generic(item.clone()),
                resolved_generic_item_def(db, item)?,
            ));
        }
    }

    if let Some(member_id) = try_extract_member(db, identifier, lookup_items)
        .or_else(|| try_extract_member_from_constructor(db, identifier, lookup_items))
        .or_else(|| try_extract_member_declaration(db, identifier))
    {
        return Some((
            ResolvedItem::Member(member_id),
            member_id.stable_ptr(db).lookup(db).name(db).stable_ptr().untyped(),
        ));
    }

    if let Some(variant) = try_extract_variant_declaration(db, identifier) {
        let item = ResolvedGenericItem::Variant(variant);
        return Some((ResolvedItem::Generic(item.clone()), resolved_generic_item_def(db, item)?));
    }

    if let Some(var_id) = try_extract_variable_declaration(db, identifier, lookup_items) {
        let item = ResolvedGenericItem::Variable(var_id);
        return Some((ResolvedItem::Generic(item.clone()), resolved_generic_item_def(db, item)?));
    }

    for &lookup_item_id in lookup_items {
        if let Some(item) =
            db.lookup_resolved_generic_item_by_ptr(lookup_item_id, identifier.stable_ptr())
        {
            return Some((
                ResolvedItem::Generic(item.clone()),
                resolved_generic_item_def(db, item)?,
            ));
        }

        if let Some(item) =
            db.lookup_resolved_concrete_item_by_ptr(lookup_item_id, identifier.stable_ptr())
        {
            let stable_ptr = resolved_concrete_item_def(db.upcast(), item.clone())?;
            return Some((ResolvedItem::Concrete(item), stable_ptr));
        }
    }

    // FIXME(mkaput): This logic always kicks in if we're finding definition of undefined symbol
    //   which is very wrong in such cases.
    let item = match lookup_items.first().copied()? {
        LookupItemId::ModuleItem(item) => {
            ResolvedGenericItem::from_module_item(db, item).to_option()?
        }
        LookupItemId::TraitItem(trait_item) => {
            if let TraitItemId::Function(trait_function_id) = trait_item {
                let parent_trait = trait_item.trait_id(db);
                let generic_parameters = db.trait_generic_params(parent_trait).to_option()?;
                let concrete_trait = ConcreteTraitLongId {
                    trait_id: parent_trait,
                    generic_args: generic_params_to_args(&generic_parameters, db),
                };
                let concrete_trait = db.intern_concrete_trait(concrete_trait);

                ResolvedGenericItem::GenericFunction(GenericFunctionId::Impl(
                    ImplGenericFunctionId {
                        impl_id: ImplLongId::SelfImpl(concrete_trait).intern(db),
                        function: trait_function_id,
                    },
                ))
            } else {
                ResolvedGenericItem::Trait(trait_item.trait_id(db))
            }
        }
        LookupItemId::ImplItem(impl_item) => ResolvedGenericItem::Impl(impl_item.impl_def_id(db)),
    };

    Some((ResolvedItem::Generic(item.clone()), resolved_generic_item_def(db, item)?))
}

/// Extracts [`MemberId`] if the [`TerminalIdentifier`] is used as a struct member
/// in [`ast::ExprStructCtorCall`].
fn try_extract_member_from_constructor(
    db: &AnalysisDatabase,
    identifier: &ast::TerminalIdentifier,
    lookup_items: &[LookupItemId],
) -> Option<MemberId> {
    let function_id = lookup_items.first()?.function_with_body()?;

    let identifier_node = identifier.as_syntax_node();

    let constructor_expr = identifier_node.ancestor_of_type::<ast::ExprStructCtorCall>(db)?;
    let constructor_expr_id =
        db.lookup_expr_by_ptr(function_id, constructor_expr.stable_ptr().into()).ok()?;

    let Expr::StructCtor(constructor_expr_semantic) =
        db.expr_semantic(function_id, constructor_expr_id)
    else {
        return None;
    };

    let struct_member = identifier_node.ancestor_of_type::<ast::StructArgSingle>(db)?;

    let struct_member_name =
        struct_member.identifier(db).as_syntax_node().get_text_without_trivia(db);

    constructor_expr_semantic
        .members
        .iter()
        .find_map(|(id, _)| struct_member_name.eq(id.name(db).as_str()).then_some(*id))
}

/// Extracts [`MemberId`] if the [`TerminalIdentifier`] points to
/// right-hand side of access member expression e.g., to `xyz` in `self.xyz`.
fn try_extract_member(
    db: &AnalysisDatabase,
    identifier: &ast::TerminalIdentifier,
    lookup_items: &[LookupItemId],
) -> Option<MemberId> {
    let syntax_node = identifier.as_syntax_node();
    let binary_expr = syntax_node.ancestor_of_type::<ast::ExprBinary>(db)?;

    let function_with_body = lookup_items.first()?.function_with_body()?;

    let expr_id =
        db.lookup_expr_by_ptr(function_with_body, binary_expr.stable_ptr().into()).ok()?;
    let semantic_expr = db.expr_semantic(function_with_body, expr_id);

    if let Expr::MemberAccess(expr_member_access) = semantic_expr {
        let pointer_to_rhs = binary_expr.rhs(db).stable_ptr().untyped();

        let mut current_node = syntax_node;
        // Check if the terminal identifier points to a member, not a struct variable.
        while pointer_to_rhs != current_node.stable_ptr() {
            // If we found the node with the binary expression, then we're sure we won't find the
            // node with the member.
            if current_node.stable_ptr() == binary_expr.stable_ptr().untyped() {
                return None;
            }
            current_node = current_node.parent().unwrap();
        }

        Some(expr_member_access.member)
    } else {
        None
    }
}

/// Extracts [`MemberId`] if the [`TerminalIdentifier`] is a name of member in struct declaration.
fn try_extract_member_declaration(
    db: &AnalysisDatabase,
    identifier: &ast::TerminalIdentifier,
) -> Option<MemberId> {
    let member = identifier.as_syntax_node().parent()?.cast::<ast::Member>(db)?;
    assert_eq!(member.name(db), *identifier);
    let item_struct = member.as_syntax_node().ancestor_of_type::<ast::ItemStruct>(db)?;
    let struct_id = StructLongId(
        db.find_module_file_containing_node(&item_struct.as_syntax_node())?,
        item_struct.stable_ptr(),
    )
    .intern(db);
    let struct_members = db.struct_members(struct_id).ok()?;
    let member_semantic = struct_members.get(&member.name(db).text(db))?;
    Some(member_semantic.id)
}

/// Extracts [`VariantId`] if the [`TerminalIdentifier`] is a name of variant in enum declaration.
fn try_extract_variant_declaration(
    db: &AnalysisDatabase,
    identifier: &ast::TerminalIdentifier,
) -> Option<Variant> {
    let variant = identifier.as_syntax_node().ancestor_of_type::<ast::Variant>(db)?;
    assert_eq!(variant.name(db), *identifier);
    let item_enum = variant.as_syntax_node().ancestor_of_type::<ast::ItemEnum>(db)?;
    let enum_id = EnumLongId(
        db.find_module_file_containing_node(&item_enum.as_syntax_node())?,
        item_enum.stable_ptr(),
    )
    .intern(db);
    let enum_variants = db.enum_variants(enum_id).ok()?;
    let variant_id = *enum_variants.get(&variant.name(db).text(db))?;
    let variant = db.variant_semantic(enum_id, variant_id).ok()?;
    Some(variant)
}

/// Lookups if the identifier is a declaration of a variable/param in one of the lookup items.
///
/// Declaration identifiers aren't kept in `ResolvedData`, which is searched for by
/// `lookup_resolved_generic_item_by_ptr` and `lookup_resolved_concrete_item_by_ptr`.
/// Therefore, we have to look for these ourselves.
fn try_extract_variable_declaration(
    db: &AnalysisDatabase,
    identifier: &ast::TerminalIdentifier,
    lookup_items: &[LookupItemId],
) -> Option<VarId> {
    let function_id = lookup_items.first()?.function_with_body()?;

    // Look at function parameters.
    if let Some(param) = identifier.as_syntax_node().ancestor_of_kind(db, SyntaxKind::Param) {
        // Closures have different semantic model structures than regular functions.
        let params = if let Some(expr_closure_ast) = param.ancestor_of_type::<ast::ExprClosure>(db)
        {
            let expr_id =
                db.lookup_expr_by_ptr(function_id, expr_closure_ast.stable_ptr().into()).ok()?;
            let Expr::ExprClosure(expr_closure_semantic) = db.expr_semantic(function_id, expr_id)
            else {
                unreachable!("expected semantic for ast::ExprClosure to be Expr::ExprClosure");
            };
            expr_closure_semantic.params
        } else {
            let signature = db.function_with_body_signature(function_id).ok()?;
            signature.params
        };

        if let Some(param) =
            params.into_iter().find(|param| param.stable_ptr == identifier.stable_ptr())
        {
            return Some(VarId::Param(param.id));
        }
    }

    // Look at patterns in the function body.
    if let Some(pattern) = identifier.as_syntax_node().ancestor_of_type::<ast::Pattern>(db) {
        // Bail out if the pattern happens to not exist in the semantic model.
        // We don't need semantics for returning, though, due to the way how local variables are
        // identified there, so we're happily ignoring the result value.
        db.lookup_pattern_by_ptr(function_id, pattern.stable_ptr()).ok()?;

        return Some(VarId::Local(
            LocalVarLongId(function_id.module_file_id(db), identifier.stable_ptr()).intern(db),
        ));
    }

    None
}

fn resolved_concrete_item_def(
    db: &AnalysisDatabase,
    item: ResolvedConcreteItem,
) -> Option<SyntaxStablePtrId> {
    match item {
        ResolvedConcreteItem::Type(ty) => {
            if let TypeLongId::GenericParameter(param) = ty.lookup_intern(db) {
                Some(param.untyped_stable_ptr(db))
            } else {
                None
            }
        }
        ResolvedConcreteItem::Impl(imp) => {
            if let ImplLongId::GenericParameter(param) = imp.lookup_intern(db) {
                Some(param.untyped_stable_ptr(db))
            } else {
                None
            }
        }
        _ => None,
    }
}

fn resolved_generic_item_def(
    db: &AnalysisDatabase,
    item: ResolvedGenericItem,
) -> Option<SyntaxStablePtrId> {
    Some(match item {
        ResolvedGenericItem::GenericConstant(item) => {
            item.stable_ptr(db).lookup(db).name(db).stable_ptr().untyped()
        }

        ResolvedGenericItem::Module(module_id) => {
            match module_id {
                ModuleId::CrateRoot(_) => {
                    // For crate root files (src/lib.cairo), the definition node is the file itself.
                    let module_file = db.module_main_file(module_id).ok()?;
                    let file_syntax = db.file_module_syntax(module_file).ok()?;
                    file_syntax.as_syntax_node().stable_ptr()
                }
                ModuleId::Submodule(submodule_id) => {
                    // For submodules, the definition node is the identifier in `mod <ident> .*`.
                    submodule_id.stable_ptr(db).lookup(db).name(db).stable_ptr().untyped()
                }
            }
        }

        ResolvedGenericItem::GenericFunction(item) => {
            let declaration: ast::FunctionDeclaration = match item {
                GenericFunctionId::Free(id) => id.stable_ptr(db).lookup(db).declaration(db),
                GenericFunctionId::Extern(id) => id.stable_ptr(db).lookup(db).declaration(db),
                GenericFunctionId::Impl(id) => match id.impl_function(db) {
                    Ok(Some(id)) => id.stable_ptr(db).lookup(db).declaration(db),
                    // It is possible (Marek didn't find out how it happens) that we hop into a
                    // situation where concrete impl is not inferred yet, so we can't find the
                    // declaration. Fall back to trait function in such cases.
                    _ => id.function.stable_ptr(db).lookup(db).declaration(db),
                },
            };
            declaration.name(db).stable_ptr().untyped()
        }

        ResolvedGenericItem::GenericType(generic_type) => match generic_type {
            GenericTypeId::Struct(struct_id) => {
                struct_id.stable_ptr(db).lookup(db).name(db).stable_ptr().untyped()
            }
            GenericTypeId::Enum(enum_id) => {
                enum_id.stable_ptr(db).lookup(db).name(db).stable_ptr().untyped()
            }
            GenericTypeId::Extern(extern_type_id) => {
                extern_type_id.stable_ptr(db).lookup(db).name(db).stable_ptr().untyped()
            }
        },

        ResolvedGenericItem::GenericTypeAlias(type_alias) => {
            type_alias.stable_ptr(db).lookup(db).name(db).stable_ptr().untyped()
        }

        ResolvedGenericItem::GenericImplAlias(impl_alias) => {
            impl_alias.stable_ptr(db).lookup(db).name(db).stable_ptr().untyped()
        }

        ResolvedGenericItem::Variant(variant) => {
            variant.id.stable_ptr(db).lookup(db).name(db).stable_ptr().untyped()
        }

        ResolvedGenericItem::Trait(trt) => {
            trt.stable_ptr(db).lookup(db).name(db).stable_ptr().untyped()
        }

        ResolvedGenericItem::Impl(imp) => {
            imp.stable_ptr(db).lookup(db).name(db).stable_ptr().untyped()
        }

        ResolvedGenericItem::Variable(var) => match var {
            VarId::Param(param) => param.stable_ptr(db).lookup(db).name(db).stable_ptr().untyped(),
            VarId::Local(var) => var.untyped_stable_ptr(db),
            VarId::Item(item) => item.name_stable_ptr(db),
        },
    })
}
