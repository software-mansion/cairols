use cairo_lang_defs::db::DefsGroup;
use cairo_lang_defs::ids::{
    EnumLongId, GenericTypeId, LanguageElementId, LookupItemId, MemberId, ModuleId,
    NamedLanguageElementId, StructLongId, SubmoduleLongId, TraitItemId, VarId,
};
use cairo_lang_diagnostics::ToOption;
use cairo_lang_parser::db::ParserGroup;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::expr::pattern::QueryPatternVariablesFromDb;
use cairo_lang_semantic::items::function_with_body::SemanticExprLookup;
use cairo_lang_semantic::items::functions::{GenericFunctionId, ImplGenericFunctionId};
use cairo_lang_semantic::items::generics::generic_params_to_args;
use cairo_lang_semantic::items::imp::ImplLongId;
use cairo_lang_semantic::lookup_item::LookupItemEx;
use cairo_lang_semantic::resolve::{ResolvedConcreteItem, ResolvedGenericItem};
use cairo_lang_semantic::{ConcreteTraitLongId, Expr, TypeLongId};
use cairo_lang_syntax::node::ids::SyntaxStablePtrId;
use cairo_lang_syntax::node::{SyntaxNode, Terminal, TypedStablePtr, TypedSyntaxNode, ast};
use cairo_lang_utils::{Intern, LookupIntern};

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
) -> Option<ResolvedItem> {
    try_submodule_name(db, identifier)
        .or_else(|| try_member(db, identifier, lookup_items))
        .or_else(|| try_member_from_constructor(db, identifier, lookup_items))
        .or_else(|| try_member_declaration(db, identifier))
        .or_else(|| try_variant_declaration(db, identifier))
        .or_else(|| try_variable_declaration(db, identifier, lookup_items))
        .or_else(|| lookup_resolved_items(db, identifier, lookup_items))
        .or_else(|| {
            // FIXME(mkaput): This logic always kicks in if we're finding definition of undefined
            //   symbol which is very wrong in such cases.
            let item = match lookup_items.first().copied()? {
                LookupItemId::ModuleItem(item) => {
                    ResolvedGenericItem::from_module_item(db, item).to_option()?
                }
                LookupItemId::TraitItem(trait_item) => {
                    if let TraitItemId::Function(trait_function_id) = trait_item {
                        let parent_trait = trait_item.trait_id(db);
                        let generic_parameters =
                            db.trait_generic_params(parent_trait).to_option()?;
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
                LookupItemId::ImplItem(impl_item) => {
                    ResolvedGenericItem::Impl(impl_item.impl_def_id(db))
                }
            };

            Some(ResolvedItem::Generic(item))
        })
}

/// Resolve `mod <ident>` syntax.
fn try_submodule_name(
    db: &AnalysisDatabase,
    identifier: &ast::TerminalIdentifier,
) -> Option<ResolvedItem> {
    let item_module = identifier.as_syntax_node().parent_of_type::<ast::ItemModule>(db)?;
    assert_eq!(item_module.name(db), *identifier);
    let containing_module_file_id =
        db.find_module_file_containing_node(&item_module.as_syntax_node())?;
    let submodule_id =
        SubmoduleLongId(containing_module_file_id, item_module.stable_ptr()).intern(db);
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
        db.lookup_expr_by_ptr(function_id, constructor_expr.stable_ptr().into()).ok()?;

    let Expr::StructCtor(constructor_expr_semantic) =
        db.expr_semantic(function_id, constructor_expr_id)
    else {
        return None;
    };

    let struct_member = identifier_node.ancestor_of_type::<ast::StructArgSingle>(db)?;

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
        db.lookup_expr_by_ptr(function_with_body, binary_expr.stable_ptr().into()).ok()?;
    let semantic_expr = db.expr_semantic(function_with_body, expr_id);

    let Expr::MemberAccess(expr_member_access) = semantic_expr else { return None };

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

    let member_id = expr_member_access.member;
    Some(ResolvedItem::Member(member_id))
}

/// Resolve `struct Foo { <ident>: ... }` syntax.
fn try_member_declaration(
    db: &AnalysisDatabase,
    identifier: &ast::TerminalIdentifier,
) -> Option<ResolvedItem> {
    let member = identifier.as_syntax_node().parent_of_type::<ast::Member>(db)?;
    assert_eq!(member.name(db), *identifier);
    let item_struct = member.as_syntax_node().ancestor_of_type::<ast::ItemStruct>(db)?;
    let struct_id = StructLongId(
        db.find_module_file_containing_node(&item_struct.as_syntax_node())?,
        item_struct.stable_ptr(),
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
    if let Some(param) = identifier.as_syntax_node().parent_of_type::<ast::Param>(db) {
        assert_eq!(param.name(db), *identifier);

        // Closures have different semantic model structures than regular functions.
        let params = if let Some(expr_closure_ast) =
            param.as_syntax_node().ancestor_of_type::<ast::ExprClosure>(db)
        {
            let expr_id =
                db.lookup_expr_by_ptr(function_id, expr_closure_ast.stable_ptr().into()).ok()?;
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
            params.into_iter().find(|param| param.stable_ptr == identifier.stable_ptr())
        {
            let var_id = VarId::Param(param.id);
            return Some(ResolvedItem::Generic(ResolvedGenericItem::Variable(var_id)));
        }
    }

    // Look at identifier patterns in the function body.
    if let Some(pattern_ast) = identifier.as_syntax_node().ancestor_of_type::<ast::Pattern>(db) {
        let pattern_id = db.lookup_pattern_by_ptr(function_id, pattern_ast.stable_ptr()).ok()?;
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

/// Lookups for the identifier in compiler's `lookup_resolved_*_item_by_ptr` queries.
fn lookup_resolved_items(
    db: &AnalysisDatabase,
    identifier: &ast::TerminalIdentifier,
    lookup_items: &[LookupItemId],
) -> Option<ResolvedItem> {
    for &lookup_item_id in lookup_items {
        if let Some(item) =
            db.lookup_resolved_generic_item_by_ptr(lookup_item_id, identifier.stable_ptr())
        {
            return Some(ResolvedItem::Generic(item));
        }

        if let Some(item) =
            db.lookup_resolved_concrete_item_by_ptr(lookup_item_id, identifier.stable_ptr())
        {
            return Some(ResolvedItem::Concrete(item));
        }
    }
    None
}

impl ResolvedItem {
    /// Finds a stable pointer to the syntax node which defines this resolved item.
    pub fn definition_stable_ptr(&self, db: &AnalysisDatabase) -> Option<SyntaxStablePtrId> {
        // TIP: This is a var so that highlighting exit points in IDEs of this function is usable.
        let stable_ptr = match self {
            // Concrete items.
            ResolvedItem::Concrete(ResolvedConcreteItem::Type(ty)) => {
                if let TypeLongId::GenericParameter(param) = ty.lookup_intern(db) {
                    param.untyped_stable_ptr(db)
                } else {
                    return None;
                }
            }

            ResolvedItem::Concrete(ResolvedConcreteItem::Impl(imp)) => {
                if let ImplLongId::GenericParameter(param) = imp.lookup_intern(db) {
                    param.untyped_stable_ptr(db)
                } else {
                    return None;
                }
            }

            ResolvedItem::Concrete(_) => return None,

            // Generic items.
            ResolvedItem::Generic(ResolvedGenericItem::GenericConstant(item)) => {
                item.stable_ptr(db).lookup(db).name(db).stable_ptr().untyped()
            }

            ResolvedItem::Generic(ResolvedGenericItem::Module(module_id)) => {
                match module_id {
                    ModuleId::CrateRoot(_) => {
                        // For crate root files (src/lib.cairo), the definition node is the file
                        // itself.
                        let module_file = db.module_main_file(*module_id).ok()?;
                        let file_syntax = db.file_module_syntax(module_file).ok()?;
                        file_syntax.as_syntax_node().stable_ptr()
                    }
                    ModuleId::Submodule(submodule_id) => {
                        // For submodules, the definition node is the identifier in `mod <ident>
                        // .*`.
                        submodule_id.stable_ptr(db).lookup(db).name(db).stable_ptr().untyped()
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
                declaration.name(db).stable_ptr().untyped()
            }

            ResolvedItem::Generic(ResolvedGenericItem::GenericType(generic_type)) => {
                match generic_type {
                    GenericTypeId::Struct(struct_id) => {
                        struct_id.stable_ptr(db).lookup(db).name(db).stable_ptr().untyped()
                    }
                    GenericTypeId::Enum(enum_id) => {
                        enum_id.stable_ptr(db).lookup(db).name(db).stable_ptr().untyped()
                    }
                    GenericTypeId::Extern(extern_type_id) => {
                        extern_type_id.stable_ptr(db).lookup(db).name(db).stable_ptr().untyped()
                    }
                }
            }

            ResolvedItem::Generic(ResolvedGenericItem::GenericTypeAlias(type_alias)) => {
                type_alias.stable_ptr(db).lookup(db).name(db).stable_ptr().untyped()
            }

            ResolvedItem::Generic(ResolvedGenericItem::GenericImplAlias(impl_alias)) => {
                impl_alias.stable_ptr(db).lookup(db).name(db).stable_ptr().untyped()
            }

            ResolvedItem::Generic(ResolvedGenericItem::Variant(variant)) => {
                variant.id.stable_ptr(db).lookup(db).name(db).stable_ptr().untyped()
            }

            ResolvedItem::Generic(ResolvedGenericItem::Trait(trt)) => {
                trt.stable_ptr(db).lookup(db).name(db).stable_ptr().untyped()
            }

            ResolvedItem::Generic(ResolvedGenericItem::Impl(imp)) => {
                imp.stable_ptr(db).lookup(db).name(db).stable_ptr().untyped()
            }

            ResolvedItem::Generic(ResolvedGenericItem::Variable(var)) => match var {
                VarId::Param(param) => {
                    param.stable_ptr(db).lookup(db).name(db).stable_ptr().untyped()
                }
                VarId::Local(var) => var.untyped_stable_ptr(db),
                VarId::Item(item) => item.name_stable_ptr(db),
            },

            // Other variants.
            ResolvedItem::Member(member_id) => {
                member_id.stable_ptr(db).lookup(db).name(db).stable_ptr().untyped()
            }
        };
        Some(stable_ptr)
    }

    /// Finds and returns the syntax node corresponding to the definition of the resolved item.
    pub fn definition_node(&self, db: &AnalysisDatabase) -> Option<SyntaxNode> {
        self.definition_stable_ptr(db).map(|stable_ptr| stable_ptr.lookup(db))
    }
}
