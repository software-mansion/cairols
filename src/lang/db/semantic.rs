use super::LsSyntaxGroup;
use cairo_lang_defs::db::{DefsGroup, get_all_path_leaves};
use cairo_lang_defs::ids::{
    ConstantLongId, EnumLongId, ExternFunctionLongId, ExternTypeLongId, FileIndex,
    FreeFunctionLongId, ImplAliasLongId, ImplConstantDefLongId, ImplDefLongId, ImplFunctionLongId,
    ImplItemId, LanguageElementId, LookupItemId, ModuleFileId, ModuleId, ModuleItemId,
    ModuleTypeAliasLongId, StructLongId, TraitConstantLongId, TraitFunctionLongId, TraitImplLongId,
    TraitItemId, TraitLongId, TraitTypeLongId, UseLongId, VarId,
};
use cairo_lang_filesystem::db::{get_parent_and_mapping, translate_location};
use cairo_lang_filesystem::ids::{FileId, FileLongId};
use cairo_lang_semantic::Binding;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::expr::pattern::QueryPatternVariablesFromDb;
use cairo_lang_semantic::items::function_with_body::SemanticExprLookup;
use cairo_lang_semantic::lookup_item::LookupItemEx;
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::helpers::GetIdentifier;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{SyntaxNode, TypedSyntaxNode, ast};
use cairo_lang_utils::LookupIntern;
use cairo_lang_utils::ordered_hash_set::OrderedHashSet;
use cairo_lang_utils::{Intern, Upcast};
use std::collections::{HashSet, VecDeque};

// TODO(mkaput): Make this a real Salsa query group with sensible LRU.
/// Language server-specific extensions to the semantic group.
pub trait LsSemanticGroup: Upcast<dyn SemanticGroup> + SemanticGroup + LsSyntaxGroup {
    /// Returns a [`LookupItemId`] corresponding to the node or its first parent all the way up to
    /// syntax root in the file.
    ///
    /// This method is a shortcut for getting the first item out of `collect_lookup_items_leaf`.
    /// Returns `None` if there is missing data in the compiler database.
    fn find_lookup_item(&self, node: &SyntaxNode) -> Option<LookupItemId> {
        self.collect_lookup_items_leaf(node)?.into_iter().next()
    }

    /// Returns [`LookupItemId`]s corresponding to the node or its first parent all the way up to
    /// syntax root in the file.
    ///
    /// Returns `None` if there is missing data in the compiler database.
    /// It is not expected for this function to return `Some([])`, but do not assume this.
    fn collect_lookup_items_leaf(&self, node: &SyntaxNode) -> Option<Vec<LookupItemId>> {
        let module_file_id = self.find_module_file_containing_node(node)?;

        node.ancestors_with_self(self.upcast())
            .find_map(|node| lookup_item_from_ast(self.upcast(), module_file_id, node))
    }

    /// Calls [`LsSemanticGroup::collect_lookup_items`] on provided node and all parent files that this node is mapping to.
    fn collect_lookup_items_with_parent_files(
        &self,
        node: &SyntaxNode,
    ) -> Option<Vec<LookupItemId>> {
        let mut node = Some(*node);
        let mut result = vec![];

        while let Some(current_node) = node {
            result.extend(self.collect_lookup_items(&current_node)?);

            node = self.corresponding_node_in_parent_file(&current_node);
        }

        Some(result)
    }

    /// Finds the corresponding node in the parent file for a given node using code mappings.
    /// The method aims to locate the most expansive node in the parent file that maps to the provided node.
    fn corresponding_node_in_parent_file(&self, node: &SyntaxNode) -> Option<SyntaxNode> {
        let db: &dyn SemanticGroup = self.upcast();

        let (parent, mappings) = get_parent_and_mapping(db, node.stable_ptr(db).file_id(db))?;

        let span_in_parent = translate_location(&mappings, node.span(db))?;

        self.widest_node_within_span(parent, span_in_parent)
    }

    /// Returns [`LookupItemId`]s corresponding to the node and its parents all the way up to syntax
    /// root in the file.
    ///
    /// Returns `None` if there is missing data in the compiler database.
    /// It is not expected for this function to return `Some([])`, but do not assume this.
    fn collect_lookup_items(&self, node: &SyntaxNode) -> Option<Vec<LookupItemId>> {
        let module_file_id = self.find_module_file_containing_node(node)?;

        Some(
            node.ancestors_with_self(self.upcast())
                .flat_map(|node| {
                    lookup_item_from_ast(self.upcast(), module_file_id, node).unwrap_or_default()
                })
                .collect(),
        )
    }

    /// Returns a [`ModuleFileId`] containing the node.
    fn find_module_file_containing_node(&self, node: &SyntaxNode) -> Option<ModuleFileId> {
        let module_id = self.find_module_containing_node(node)?;
        let file = node.stable_ptr(self.upcast()).file_id(self.upcast());

        let i = self
            .module_files(module_id)
            .map(|files| {
                files
                    .iter()
                    .enumerate()
                    .find(|(_, f)| **f == file)
                    .map(|(i, _)| i)
                    .unwrap_or_default()
            })
            .unwrap_or_default();

        let file_index = FileIndex(i);
        Some(ModuleFileId(module_id, file_index))
    }

    /// Finds a [`ModuleId`] containing the node.
    ///
    /// If the node is located in a virtual file generated by a compiler plugin, this method will
    /// return the (sub)module of the main, user-written file that leads to the node.
    fn find_module_containing_node(&self, node: &SyntaxNode) -> Option<ModuleId> {
        let db: &dyn DefsGroup = self.upcast();
        let semantic_db: &dyn SemanticGroup = self.upcast();
        let syntax_db: &dyn SyntaxGroup = self.upcast();

        // Get the main module of the main file that leads to the node.
        // The node may be located in a virtual file of a submodule.
        // This code attempts to get the absolute "parent" of both "module" and "file" parts.
        let main_module = {
            // Get the file where the node is located.
            // This might be a virtual file generated by a compiler plugin.
            let node_file_id = node.stable_ptr(syntax_db).file_id(syntax_db);

            // Get the root module of a file containing the node.
            let node_main_module = db.file_modules(node_file_id).ok()?.iter().copied().next()?;

            // Get the main module of the file.
            let main_file = db.module_main_file(node_main_module).ok()?;

            // Get the main module of that file.
            db.file_modules(main_file).ok()?.iter().copied().next()?
        };

        // Get the stack (bottom-up) of submodule names in the file containing the node, in the main
        // module, that lead to the node.
        node.ancestors(syntax_db)
            .filter(|node| node.kind(syntax_db) == SyntaxKind::ItemModule)
            .map(|node| {
                ast::ItemModule::from_syntax_node(syntax_db, node)
                    .stable_ptr(syntax_db)
                    .name_green(syntax_db)
                    .identifier(syntax_db)
            })
            // Buffer the stack to get DoubleEndedIterator.
            .collect::<Vec<_>>()
            .into_iter()
            // And get id of the (sub)module containing the node by traversing this stack top-down.
            .try_rfold(main_module, |module, name| {
                let ModuleItemId::Submodule(submodule) =
                    semantic_db.module_item_by_name(module, name).ok()??
                else {
                    return None;
                };
                Some(ModuleId::Submodule(submodule))
            })
    }

    /// Reverse lookups [`VarId`] to get a [`Binding`] associated with it.
    ///
    /// While [`VarId`] is basically an ID of a [`Binding`],
    /// no mapping from the former to the latter is being maintained in [`SemanticGroup`].
    /// This forces us to perform elaborate reverse lookups,
    /// which makes life here much harder than needed.
    fn lookup_binding(&self, var_id: VarId) -> Option<Binding> {
        let syntax_db: &dyn SyntaxGroup = self.upcast();

        match var_id {
            VarId::Param(param_id) => {
                // Get param's syntax node.
                let param = param_id.untyped_stable_ptr(self.upcast()).lookup(syntax_db);

                // Get the function which contains the variable/parameter.
                let function_id = self.find_lookup_item(&param)?.function_with_body()?;

                // Get function signature.
                let signature = self.function_with_body_signature(function_id).ok()?;

                // Find the binding in the function's signature.
                signature.params.into_iter().find(|p| p.id == param_id).map(Into::into)
            }

            VarId::Local(local_var_id) => {
                // Get the Pattern syntax node which defines the variable.
                let identifier = local_var_id.untyped_stable_ptr(self.upcast()).lookup(syntax_db);
                let pattern = identifier.ancestor_of_type::<ast::Pattern>(syntax_db)?;

                // Get the function which contains the variable/parameter.
                let function_id =
                    self.find_lookup_item(&pattern.as_syntax_node())?.function_with_body()?;

                // Get the semantic model for the pattern.
                let pattern = self.pattern_semantic(
                    function_id,
                    self.lookup_pattern_by_ptr(function_id, pattern.stable_ptr(self.upcast()))
                        .ok()?,
                );

                // Extract the binding from the found pattern.
                let binding = pattern
                    .variables(&QueryPatternVariablesFromDb(self.upcast(), function_id))
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

    /// Collects `file` and all its descendants together with modules from all these files.
    ///
    /// **CAVEAT**: it does not collect descendant files that come from inline macros - it will when
    /// the compiler moves inline macros resolving to [`DefsGroup`].
    fn file_and_subfiles_with_corresponding_modules(
        &self,
        file: FileId,
    ) -> Option<(HashSet<FileId>, HashSet<ModuleId>)> {
        let mut modules: HashSet<_> = self.file_modules(file).ok()?.iter().copied().collect();
        let mut files = HashSet::from([file]);
        // Collect descendants of `file`
        // and modules from all virtual files that are descendants of `file`.
        //
        // Caveat: consider a situation `file1` --(child)--> `file2` with file contents:
        // - `file1`: `mod file2_origin_module { #[file2]fn sth() {} }`
        // - `file2`: `mod mod_from_file2 { }`
        //  It is important that `file2` content contains a module.
        //
        // Problem: in this situation it is not enough to call `db.file_modules(file1_id)` since
        //  `mod_from_file2` won't be in the result of this query.
        // Solution: we can find file id of `file2`
        //  (note that we only have file id of `file1` at this point)
        //  in `db.module_files(mod_from_file1_from_which_file2_origins)`.
        //  Then we can call `db.file_modules(file2_id)` to obtain module id of `mod_from_file2`.
        //  We repeat this procedure until there is nothing more to collect.
        let mut modules_queue: VecDeque<_> = modules.iter().copied().collect();
        while let Some(module_id) = modules_queue.pop_front() {
            for file_id in self.module_files(module_id).ok()?.iter() {
                if files.insert(*file_id) {
                    for module_id in self.file_modules(*file_id).ok()?.iter() {
                        if modules.insert(*module_id) {
                            modules_queue.push_back(*module_id);
                        }
                    }
                }
            }
        }
        Some((files, modules))
    }
    /// We use the term `resultants` to refer to generated nodes that are mapped to the original node and are not deleted.
    /// Efectively (user nodes + generated nodes - removed nodes) set always contains resultants for any user defined node.
    /// Semantic data may be available only for resultants.
    ///
    /// Consider the following foundry code as an example:
    /// ```ignore
    /// #[test]
    /// #[available_gas(123)]
    /// fn test_fn(){
    /// }
    /// ```
    /// This code expands to something like:
    /// ```ignore
    /// #[available_gas(123)]
    /// fn test_fn(){
    ///     if is_config_run {
    ///         // do config check
    ///         return;
    ///     }
    /// }
    /// ```
    /// It then further transforms to:
    /// ```ignore
    /// fn test_fn(){
    ///     if is_config_run {
    ///         // do config check
    ///         set_available_gas(123);
    ///         return;
    ///     }
    /// }
    /// ```
    ///
    /// Let's label these as files 1, 2 and 3, respectively. The macros used here are attribute proc macros. They delete old code and generate new code.
    /// In this process, `test_fn` from file 1 is deleted. However, `test_fn` from file 2 is mapped to it.
    /// Therefore, we should ignore `test_fn` from file 1 as it no longer exists and
    /// should use `test_fn` from file 2. But then, `test_fn` from file 2 is effectively replaced by `test_fn` from file 3, so `test_fn` from file 2 is now deleted.
    ///
    /// In this scenario, only `test_fn` from file 3 is a resultant. Both `test_fn` from files 1 and 2 were deleted.
    ///
    /// So for input being `test_fn` from file 1, only `test_fn` from file 3 is returned
    ///
    /// Now, consider another example:
    ///
    /// The `generate_trait` macro is a builtin macro that does not remove the original code. Thus, we have the following code:
    ///
    /// ```ignore
    /// #[generate_trait]
    /// impl FooImpl for FooTrait {}
    /// ```
    /// This code generates the following:
    /// ```ignore
    /// trait FooTrait {}
    /// ```
    ///
    /// Both the original and the generated files are considered when calculating semantics, since original `FooTrait` was not removed.
    /// Additionally `FooTrait` from file 2 is mapped to `FooTrait` from file 1.
    ///
    /// Therefore for `FooTrait` from file 1, `FooTrait` from file 1 and `FooTrait` from file 2 are returned.
    fn get_node_resultants(&self, node: SyntaxNode) -> Option<Vec<SyntaxNode>> {
        let db: &dyn SemanticGroup = self.upcast();

        let main_file = node.stable_ptr(db).file_id(db);

        let (mut files, _) = self.file_and_subfiles_with_corresponding_modules(main_file)?;

        files.remove(&main_file);

        let files: Vec<_> = files.into_iter().collect();
        let resultants = find_generated_nodes(db, &files, node);

        Some(resultants.into_iter().collect())
    }
}

impl<T> LsSemanticGroup for T where T: Upcast<dyn SemanticGroup> + SemanticGroup {}

/// If the ast node is a lookup item, return corresponding ids. Otherwise, returns `None`.
/// See [LookupItemId].
fn lookup_item_from_ast(
    db: &dyn SemanticGroup,
    module_file_id: ModuleFileId,
    node: SyntaxNode,
) -> Option<Vec<LookupItemId>> {
    let syntax_db = db.upcast();

    let is_in_impl = node.ancestor_of_kind(syntax_db, SyntaxKind::ItemImpl).is_some();

    Some(match node.kind(syntax_db) {
        SyntaxKind::ItemConstant => {
            if is_in_impl {
                vec![LookupItemId::ImplItem(ImplItemId::Constant(
                    ImplConstantDefLongId(
                        module_file_id,
                        ast::ItemConstant::from_syntax_node(syntax_db, node).stable_ptr(syntax_db),
                    )
                    .intern(db),
                ))]
            } else {
                vec![LookupItemId::ModuleItem(ModuleItemId::Constant(
                    ConstantLongId(
                        module_file_id,
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
                        module_file_id,
                        ast::FunctionWithBody::from_syntax_node(db, node).stable_ptr(db),
                    )
                    .intern(db),
                ))]
            } else {
                vec![LookupItemId::ModuleItem(ModuleItemId::FreeFunction(
                    FreeFunctionLongId(
                        module_file_id,
                        ast::FunctionWithBody::from_syntax_node(db, node).stable_ptr(db),
                    )
                    .intern(db),
                ))]
            }
        }
        SyntaxKind::ItemExternFunction => {
            vec![LookupItemId::ModuleItem(ModuleItemId::ExternFunction(
                ExternFunctionLongId(
                    module_file_id,
                    ast::ItemExternFunction::from_syntax_node(db, node).stable_ptr(db),
                )
                .intern(db),
            ))]
        }
        SyntaxKind::ItemExternType => vec![LookupItemId::ModuleItem(ModuleItemId::ExternType(
            ExternTypeLongId(
                module_file_id,
                ast::ItemExternType::from_syntax_node(db, node).stable_ptr(db),
            )
            .intern(db),
        ))],
        SyntaxKind::ItemTrait => {
            vec![LookupItemId::ModuleItem(ModuleItemId::Trait(
                TraitLongId(
                    module_file_id,
                    ast::ItemTrait::from_syntax_node(db, node).stable_ptr(db),
                )
                .intern(db),
            ))]
        }
        SyntaxKind::TraitItemConstant => {
            vec![LookupItemId::TraitItem(TraitItemId::Constant(
                TraitConstantLongId(
                    module_file_id,
                    ast::TraitItemConstant::from_syntax_node(db, node).stable_ptr(db),
                )
                .intern(db),
            ))]
        }
        SyntaxKind::TraitItemFunction => {
            vec![LookupItemId::TraitItem(TraitItemId::Function(
                TraitFunctionLongId(
                    module_file_id,
                    ast::TraitItemFunction::from_syntax_node(db, node).stable_ptr(db),
                )
                .intern(db),
            ))]
        }
        SyntaxKind::TraitItemImpl => {
            vec![LookupItemId::TraitItem(TraitItemId::Impl(
                TraitImplLongId(
                    module_file_id,
                    ast::TraitItemImpl::from_syntax_node(db, node).stable_ptr(db),
                )
                .intern(db),
            ))]
        }
        SyntaxKind::TraitItemType => {
            vec![LookupItemId::TraitItem(TraitItemId::Type(
                TraitTypeLongId(
                    module_file_id,
                    ast::TraitItemType::from_syntax_node(db, node).stable_ptr(db),
                )
                .intern(db),
            ))]
        }
        SyntaxKind::ItemImpl => {
            vec![LookupItemId::ModuleItem(ModuleItemId::Impl(
                ImplDefLongId(
                    module_file_id,
                    ast::ItemImpl::from_syntax_node(db, node).stable_ptr(db),
                )
                .intern(db),
            ))]
        }
        SyntaxKind::ItemStruct => {
            vec![LookupItemId::ModuleItem(ModuleItemId::Struct(
                StructLongId(
                    module_file_id,
                    ast::ItemStruct::from_syntax_node(db, node).stable_ptr(db),
                )
                .intern(db),
            ))]
        }
        SyntaxKind::ItemEnum => {
            vec![LookupItemId::ModuleItem(ModuleItemId::Enum(
                EnumLongId(
                    module_file_id,
                    ast::ItemEnum::from_syntax_node(db, node).stable_ptr(db),
                )
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
                    let use_long_id = UseLongId(module_file_id, leaf.stable_ptr(syntax_db));
                    LookupItemId::ModuleItem(ModuleItemId::Use(use_long_id.intern(db)))
                })
                .collect()
        }
        SyntaxKind::ItemTypeAlias => vec![LookupItemId::ModuleItem(ModuleItemId::TypeAlias(
            ModuleTypeAliasLongId(
                module_file_id,
                ast::ItemTypeAlias::from_syntax_node(db, node).stable_ptr(db),
            )
            .intern(db),
        ))],
        SyntaxKind::ItemImplAlias => vec![LookupItemId::ModuleItem(ModuleItemId::ImplAlias(
            ImplAliasLongId(
                module_file_id,
                ast::ItemImplAlias::from_syntax_node(db, node).stable_ptr(db),
            )
            .intern(db),
        ))],
        _ => return None,
    })
}

/// See [`LsSemanticGroup::get_node_resultants`].
fn find_generated_nodes(
    db: &dyn SemanticGroup,
    node_descendant_files: &[FileId],
    node: SyntaxNode,
) -> OrderedHashSet<SyntaxNode> {
    let start_file = node.stable_ptr(db).file_id(db);

    let mut result = OrderedHashSet::default();

    let mut is_replaced = false;

    for &file in node_descendant_files {
        let Some((parent, mappings)) = get_parent_and_mapping(db, file) else {
            continue;
        };

        if parent != start_file {
            continue;
        }

        let Ok(file_syntax) = db.file_syntax(file) else {
            continue;
        };

        let is_replacing_og_item = match file.lookup_intern(db) {
            FileLongId::Virtual(vfs) => vfs.original_item_removed,
            FileLongId::External(id) => db.ext_as_virtual(id).original_item_removed,
            _ => unreachable!(),
        };

        for token in file_syntax.tokens(db) {
            let Some(span_in_parent) = translate_location(&mappings, token.span(db)) else {
                continue;
            };

            if !node.span(db).contains(span_in_parent) {
                continue;
            }

            is_replaced = is_replaced || is_replacing_og_item;

            result.extend(find_generated_nodes(db, node_descendant_files, token));
        }
    }

    if !is_replaced {
        result.insert(node);
    }

    result
}
