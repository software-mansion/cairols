use cairo_lang_defs::db::get_all_path_leaves;
use cairo_lang_defs::ids::{
    ConstantLongId, EnumLongId, ExternFunctionLongId, ExternTypeLongId, FreeFunctionLongId,
    ImplAliasLongId, ImplConstantDefLongId, ImplDefLongId, ImplFunctionLongId, ImplItemId,
    LanguageElementId, LookupItemId, MacroDeclarationLongId, ModuleId, ModuleItemId,
    ModuleTypeAliasLongId, StructLongId, TraitConstantLongId, TraitFunctionLongId, TraitImplLongId,
    TraitItemId, TraitLongId, TraitTypeLongId, UseLongId, VarId,
};
use cairo_lang_filesystem::db::{get_parent_and_mapping, translate_location};
use cairo_lang_semantic::expr::pattern::QueryPatternVariablesFromDb;
use cairo_lang_semantic::items::enm::EnumSemantic;
use cairo_lang_semantic::items::extern_function::ExternFunctionSemantic;
use cairo_lang_semantic::items::extern_type::ExternTypeSemantic;
use cairo_lang_semantic::items::free_function::FreeFunctionSemantic;
use cairo_lang_semantic::items::function_with_body::{
    FunctionWithBodySemantic, SemanticExprLookup,
};
use cairo_lang_semantic::items::imp::ImplSemantic;
use cairo_lang_semantic::items::impl_alias::ImplAliasSemantic;
use cairo_lang_semantic::items::module_type_alias::ModuleTypeAliasSemantic;
use cairo_lang_semantic::items::structure::StructSemantic;
use cairo_lang_semantic::items::trt::TraitSemantic;
use cairo_lang_semantic::lookup_item::LookupItemEx;
use cairo_lang_semantic::lsp_helpers::LspHelpers;
use cairo_lang_semantic::{Binding, GenericParam};
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{SyntaxNode, TypedSyntaxNode, ast};
use cairo_lang_utils::Intern;
use salsa::Database;

use super::LsSyntaxGroup;

pub trait LsSemanticGroup: Database {
    /// Returns a [`LookupItemId`] corresponding to the node or its first parent all the way up to
    /// syntax root in the file.
    ///
    /// This method is a shortcut for getting the first item out of `collect_lookup_items_leaf`.
    /// Returns `None` if there is missing data in the compiler database.
    fn find_lookup_item<'db>(&'db self, node: SyntaxNode<'db>) -> Option<LookupItemId<'db>> {
        find_lookup_item(self.as_dyn_database(), (), node)
    }

    /// Returns [`LookupItemId`]s corresponding to the node or its first parent all the way up to
    /// syntax root in the file.
    ///
    /// Returns `None` if there is missing data in the compiler database.
    /// It is not expected for this function to return `Some([])`, but do not assume this.
    fn collect_lookup_items_leaf<'db>(
        &'db self,
        node: SyntaxNode<'db>,
    ) -> Option<&'db Vec<LookupItemId<'db>>> {
        collect_lookup_items_leaf(self.as_dyn_database(), (), node).as_ref()
    }

    /// Calls [`LsSemanticGroup::collect_lookup_items`] on provided node and all parent files that this node is mapping to.
    fn collect_lookup_items_with_parent_files<'db>(
        &'db self,
        node: SyntaxNode<'db>,
    ) -> Option<&'db Vec<LookupItemId<'db>>> {
        collect_lookup_items_with_parent_files(self.as_dyn_database(), (), node).as_ref()
    }

    /// Finds the corresponding node in the parent file for a given node using code mappings.
    /// The method aims to locate the most expansive node in the parent file that maps to the provided node.
    fn corresponding_node_in_parent_file<'db>(
        &'db self,
        node: SyntaxNode<'db>,
    ) -> Option<SyntaxNode<'db>> {
        corresponding_node_in_parent_file(self.as_dyn_database(), (), node)
    }

    /// Returns [`LookupItemId`]s corresponding to the node and its parents all the way up to syntax
    /// root in the file.
    ///
    /// Returns `None` if there is missing data in the compiler database.
    /// It is not expected for this function to return `Some([])`, but do not assume this.
    fn collect_lookup_items<'db>(
        &'db self,
        node: SyntaxNode<'db>,
    ) -> Option<&'db Vec<LookupItemId<'db>>> {
        collect_lookup_items(self.as_dyn_database(), (), node).as_ref()
    }

    /// Reverse lookups [`VarId`] to get a [`Binding`] associated with it.
    ///
    /// While [`VarId`] is basically an ID of a [`Binding`],
    /// no mapping from the former to the latter is being maintained in [`SemanticGroup`].
    /// This forces us to perform elaborate reverse lookups,
    /// which makes life here much harder than needed.
    fn lookup_binding<'db>(&'db self, var_id: VarId<'db>) -> Option<Binding<'db>> {
        lookup_binding(self.as_dyn_database(), (), var_id)
    }

    /// Returns generic parameters defined by the item.
    /// Items that do not introduce any generics (use statements, modules etc.) yield an empty vector.
    fn item_generic_params<'db>(
        &'db self,
        lookup_item: LookupItemId<'db>,
    ) -> &'db Vec<GenericParam<'db>> {
        item_generic_params(self.as_dyn_database(), (), lookup_item)
    }
}

impl<T: Database + ?Sized> LsSemanticGroup for T {}

#[salsa::tracked]
fn find_lookup_item<'db>(
    db: &'db dyn Database,
    _: (),
    node: SyntaxNode<'db>,
) -> Option<LookupItemId<'db>> {
    db.collect_lookup_items_leaf(node)?.first().copied()
}

#[salsa::tracked(returns(ref))]
fn collect_lookup_items_leaf<'db>(
    db: &'db dyn Database,
    _: (),
    node: SyntaxNode<'db>,
) -> Option<Vec<LookupItemId<'db>>> {
    let module_id = db.find_module_containing_node(node)?;

    node.ancestors_with_self(db).find_map(|node| lookup_item_from_ast(db, module_id, node).clone())
}

#[salsa::tracked(returns(ref))]
fn collect_lookup_items_with_parent_files<'db>(
    db: &'db dyn Database,
    _: (),
    node: SyntaxNode<'db>,
) -> Option<Vec<LookupItemId<'db>>> {
    let mut node = Some(node);
    let mut result = vec![];

    while let Some(current_node) = node {
        result.extend(db.collect_lookup_items(current_node)?);

        node = db.corresponding_node_in_parent_file(current_node);
    }

    Some(result)
}

#[salsa::tracked]
fn corresponding_node_in_parent_file<'db>(
    db: &'db dyn Database,
    _: (),
    node: SyntaxNode<'db>,
) -> Option<SyntaxNode<'db>> {
    let (parent, mappings) = get_parent_and_mapping(db, node.stable_ptr(db).file_id(db))?;

    let span_in_parent = translate_location(mappings, node.span(db))?;

    db.widest_node_within_span(parent.file_id, span_in_parent)
}

#[salsa::tracked(returns(ref))]
fn collect_lookup_items<'db>(
    db: &'db dyn Database,
    _: (),
    node: SyntaxNode<'db>,
) -> Option<Vec<LookupItemId<'db>>> {
    let module_id = db.find_module_containing_node(node)?;

    Some(
        node.ancestors_with_self(db)
            .flat_map(|node| lookup_item_from_ast(db, module_id, node).unwrap_or_default())
            .collect(),
    )
}

#[salsa::tracked]
fn lookup_binding<'db>(
    db: &'db dyn Database,
    _tracked: (),
    var_id: VarId<'db>,
) -> Option<Binding<'db>> {
    match var_id {
        VarId::Param(param_id) => {
            // Get param's syntax node.
            let param = param_id.untyped_stable_ptr(db).lookup(db);

            // Get the function which contains the variable/parameter.
            let function_id = db.find_lookup_item(param)?.function_with_body()?;

            // Get function signature.
            let signature = db.function_with_body_signature(function_id).ok()?;

            // Find the binding in the function's signature.
            signature.params.iter().find(|p| p.id == param_id).map(|p| p.clone().into())
        }

        VarId::Local(local_var_id) => {
            // Get the Pattern syntax node which defines the variable.
            let identifier = local_var_id.untyped_stable_ptr(db).lookup(db);
            let pattern = identifier.ancestor_of_type::<ast::Pattern>(db)?;

            // Get the function which contains the variable/parameter.
            let function_id =
                db.find_lookup_item(pattern.as_syntax_node())?.function_with_body()?;

            // Get the semantic model for the pattern.
            let pattern = db.pattern_semantic(
                function_id,
                db.lookup_pattern_by_ptr(function_id, pattern.stable_ptr(db)).ok()?,
            );

            // Extract the binding from the found pattern.
            let binding = pattern
                .variables(&QueryPatternVariablesFromDb(db, function_id))
                .into_iter()
                .find(|pv| pv.var.id == local_var_id)?
                .var
                .into();

            Some(binding)
        }

        VarId::Item(_stmt_item_id) => {
            // TODO(#58): Implement this.
            None
        }
    }
}

/// If the ast node is a lookup item, return corresponding ids. Otherwise, returns `None`.
/// See [LookupItemId<'db>].
fn lookup_item_from_ast<'db>(
    db: &'db dyn Database,
    module_id: ModuleId<'db>,
    node: SyntaxNode<'db>,
) -> Option<Vec<LookupItemId<'db>>> {
    let syntax_db = db;

    let is_in_impl = node.ancestor_of_kind(syntax_db, SyntaxKind::ItemImpl).is_some();

    Some(match node.kind(syntax_db) {
        SyntaxKind::ItemConstant => {
            if is_in_impl {
                vec![LookupItemId::ImplItem(ImplItemId::Constant(
                    ImplConstantDefLongId(
                        module_id,
                        ast::ItemConstant::from_syntax_node(syntax_db, node).stable_ptr(syntax_db),
                    )
                    .intern(db),
                ))]
            } else {
                vec![LookupItemId::ModuleItem(ModuleItemId::Constant(
                    ConstantLongId(
                        module_id,
                        ast::ItemConstant::from_syntax_node(db, node).stable_ptr(db),
                    )
                    .intern(db),
                ))]
            }
        }
        SyntaxKind::FunctionWithBody => {
            if is_in_impl {
                vec![LookupItemId::ImplItem(ImplItemId::Function(
                    ImplFunctionLongId(
                        module_id,
                        ast::FunctionWithBody::from_syntax_node(db, node).stable_ptr(db),
                    )
                    .intern(db),
                ))]
            } else {
                vec![LookupItemId::ModuleItem(ModuleItemId::FreeFunction(
                    FreeFunctionLongId(
                        module_id,
                        ast::FunctionWithBody::from_syntax_node(db, node).stable_ptr(db),
                    )
                    .intern(db),
                ))]
            }
        }
        SyntaxKind::ItemExternFunction => {
            vec![LookupItemId::ModuleItem(ModuleItemId::ExternFunction(
                ExternFunctionLongId(
                    module_id,
                    ast::ItemExternFunction::from_syntax_node(db, node).stable_ptr(db),
                )
                .intern(db),
            ))]
        }
        SyntaxKind::ItemExternType => vec![LookupItemId::ModuleItem(ModuleItemId::ExternType(
            ExternTypeLongId(
                module_id,
                ast::ItemExternType::from_syntax_node(db, node).stable_ptr(db),
            )
            .intern(db),
        ))],
        SyntaxKind::ItemTrait => {
            vec![LookupItemId::ModuleItem(ModuleItemId::Trait(
                TraitLongId(module_id, ast::ItemTrait::from_syntax_node(db, node).stable_ptr(db))
                    .intern(db),
            ))]
        }
        SyntaxKind::TraitItemConstant => {
            vec![LookupItemId::TraitItem(TraitItemId::Constant(
                TraitConstantLongId(
                    module_id,
                    ast::TraitItemConstant::from_syntax_node(db, node).stable_ptr(db),
                )
                .intern(db),
            ))]
        }
        SyntaxKind::TraitItemFunction => {
            vec![LookupItemId::TraitItem(TraitItemId::Function(
                TraitFunctionLongId(
                    module_id,
                    ast::TraitItemFunction::from_syntax_node(db, node).stable_ptr(db),
                )
                .intern(db),
            ))]
        }
        SyntaxKind::TraitItemImpl => {
            vec![LookupItemId::TraitItem(TraitItemId::Impl(
                TraitImplLongId(
                    module_id,
                    ast::TraitItemImpl::from_syntax_node(db, node).stable_ptr(db),
                )
                .intern(db),
            ))]
        }
        SyntaxKind::TraitItemType => {
            vec![LookupItemId::TraitItem(TraitItemId::Type(
                TraitTypeLongId(
                    module_id,
                    ast::TraitItemType::from_syntax_node(db, node).stable_ptr(db),
                )
                .intern(db),
            ))]
        }
        SyntaxKind::ItemImpl => {
            vec![LookupItemId::ModuleItem(ModuleItemId::Impl(
                ImplDefLongId(module_id, ast::ItemImpl::from_syntax_node(db, node).stable_ptr(db))
                    .intern(db),
            ))]
        }
        SyntaxKind::ItemStruct => {
            vec![LookupItemId::ModuleItem(ModuleItemId::Struct(
                StructLongId(module_id, ast::ItemStruct::from_syntax_node(db, node).stable_ptr(db))
                    .intern(db),
            ))]
        }
        SyntaxKind::ItemEnum => {
            vec![LookupItemId::ModuleItem(ModuleItemId::Enum(
                EnumLongId(module_id, ast::ItemEnum::from_syntax_node(db, node).stable_ptr(db))
                    .intern(db),
            ))]
        }
        SyntaxKind::ItemUse => {
            // Item use is not a lookup item, so we need to collect all UseLeaf, which are lookup
            // items.
            let item_use = ast::ItemUse::from_syntax_node(db, node);
            get_all_path_leaves(db, &item_use)
                .into_iter()
                .map(|leaf| {
                    let use_long_id = UseLongId(module_id, leaf.stable_ptr(syntax_db));
                    LookupItemId::ModuleItem(ModuleItemId::Use(use_long_id.intern(db)))
                })
                .collect()
        }
        SyntaxKind::ItemTypeAlias => vec![LookupItemId::ModuleItem(ModuleItemId::TypeAlias(
            ModuleTypeAliasLongId(
                module_id,
                ast::ItemTypeAlias::from_syntax_node(db, node).stable_ptr(db),
            )
            .intern(db),
        ))],
        SyntaxKind::ItemImplAlias => vec![LookupItemId::ModuleItem(ModuleItemId::ImplAlias(
            ImplAliasLongId(
                module_id,
                ast::ItemImplAlias::from_syntax_node(db, node).stable_ptr(db),
            )
            .intern(db),
        ))],
        SyntaxKind::ItemMacroDeclaration => {
            vec![LookupItemId::ModuleItem(ModuleItemId::MacroDeclaration(
                MacroDeclarationLongId(
                    module_id,
                    ast::ItemMacroDeclaration::from_syntax_node(db, node).stable_ptr(db),
                )
                .intern(db),
            ))]
        }
        _ => return None,
    })
}

#[salsa::tracked(returns(ref))]
fn item_generic_params<'db>(
    db: &'db dyn Database,
    _tracked: (),
    lookup_item: LookupItemId<'db>,
) -> Vec<GenericParam<'db>> {
    match lookup_item {
        LookupItemId::ModuleItem(module_item_id) => match module_item_id {
            ModuleItemId::FreeFunction(free_function_id) => {
                db.free_function_generic_params(free_function_id).map(|x| x.to_vec())
            }
            ModuleItemId::Struct(struct_id) => {
                db.struct_generic_params(struct_id).map(|x| x.to_vec())
            }
            ModuleItemId::Enum(enum_id) => db.enum_generic_params(enum_id).map(|x| x.to_vec()),
            ModuleItemId::TypeAlias(module_type_alias_id) => {
                db.module_type_alias_generic_params(module_type_alias_id).map(|x| x.to_vec())
            }
            ModuleItemId::ImplAlias(impl_alias_id) => {
                db.impl_alias_generic_params(impl_alias_id).map(|x| x.to_vec())
            }
            ModuleItemId::Trait(trait_id) => db.trait_generic_params(trait_id).map(|x| x.to_vec()),
            ModuleItemId::Impl(impl_def_id) => {
                db.impl_def_generic_params(impl_def_id).map(|x| x.to_vec())
            }
            ModuleItemId::ExternType(extern_type_id) => {
                db.extern_type_declaration_generic_params(extern_type_id).map(|x| x.to_vec())
            }
            ModuleItemId::ExternFunction(extern_function_id) => db
                .extern_function_declaration_generic_params(extern_function_id)
                .map(|x| x.to_vec()),
            _ => return vec![],
        },
        LookupItemId::TraitItem(trait_item_id) => match trait_item_id {
            TraitItemId::Function(trait_function_id) => {
                db.trait_function_generic_params(trait_function_id).map(|x| x.to_vec())
            }
            TraitItemId::Type(trait_type_id) => {
                db.trait_type_generic_params(trait_type_id).map(|x| x.to_vec())
            }
            _ => return vec![],
        },
        LookupItemId::ImplItem(impl_item_id) => match impl_item_id {
            ImplItemId::Function(impl_function_id) => {
                db.impl_function_generic_params(impl_function_id).map(|x| x.to_vec())
            }
            ImplItemId::Type(impl_type_def_id) => db.impl_type_def_generic_params(impl_type_def_id),
            _ => return vec![],
        },
    }
    .into_iter()
    .flatten()
    .collect()
}
