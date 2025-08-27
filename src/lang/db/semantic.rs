use std::collections::{HashSet, VecDeque};
use std::sync::Arc;

use cairo_lang_defs::db::get_all_path_leaves;
use cairo_lang_defs::ids::{
    ConstantLongId, EnumLongId, ExternFunctionLongId, ExternTypeLongId, FileIndex,
    FreeFunctionLongId, ImplAliasLongId, ImplConstantDefLongId, ImplDefLongId, ImplFunctionLongId,
    ImplItemId, LanguageElementId, LookupItemId, ModuleFileId, ModuleId, ModuleItemId,
    ModuleTypeAliasLongId, StructLongId, TraitConstantLongId, TraitFunctionLongId, TraitImplLongId,
    TraitItemId, TraitLongId, TraitTypeLongId, UseLongId, VarId,
};
use cairo_lang_diagnostics::Maybe;
use cairo_lang_filesystem::db::{get_parent_and_mapping, translate_location};
use cairo_lang_filesystem::ids::{CodeOrigin, FileId, FileKind, FileLongId};
use cairo_lang_parser::db::ParserGroup;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::expr::pattern::QueryPatternVariablesFromDb;
use cairo_lang_semantic::items::function_with_body::SemanticExprLookup;
use cairo_lang_semantic::lookup_item::LookupItemEx;
use cairo_lang_semantic::{Binding, GenericParam};
use cairo_lang_syntax::node::helpers::GetIdentifier;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{SyntaxNode, TypedSyntaxNode, ast};
use cairo_lang_utils::ordered_hash_set::OrderedHashSet;
use cairo_lang_utils::{Intern, Upcast};

use crate::lang::db::SyntaxNodeExt;

use super::LsSyntaxGroup;

#[cairo_lang_proc_macros::query_group(LsSemanticDatabase)]
pub trait LsSemanticGroup:
    SemanticGroup + for<'db> Upcast<'db, dyn SemanticGroup> + LsSyntaxGroup
{
    /// Returns a [`LookupItemId`] corresponding to the node or its first parent all the way up to
    /// syntax root in the file.
    ///
    /// This method is a shortcut for getting the first item out of `collect_lookup_items_leaf`.
    /// Returns `None` if there is missing data in the compiler database.
    fn find_lookup_item<'db>(&'db self, node: SyntaxNode<'db>) -> Option<LookupItemId<'db>>;

    /// Returns [`LookupItemId`]s corresponding to the node or its first parent all the way up to
    /// syntax root in the file.
    ///
    /// Returns `None` if there is missing data in the compiler database.
    /// It is not expected for this function to return `Some([])`, but do not assume this.
    fn collect_lookup_items_leaf<'db>(
        &'db self,
        node: SyntaxNode<'db>,
    ) -> Option<Vec<LookupItemId<'db>>>;

    /// Calls [`LsSemanticGroup::collect_lookup_items`] on provided node and all parent files that this node is mapping to.
    fn collect_lookup_items_with_parent_files<'db>(
        &'db self,
        node: SyntaxNode<'db>,
    ) -> Option<Vec<LookupItemId<'db>>>;

    /// Finds the corresponding node in the parent file for a given node using code mappings.
    /// The method aims to locate the most expansive node in the parent file that maps to the provided node.
    fn corresponding_node_in_parent_file<'db>(
        &'db self,
        node: SyntaxNode<'db>,
    ) -> Option<SyntaxNode<'db>>;

    /// Returns [`LookupItemId`]s corresponding to the node and its parents all the way up to syntax
    /// root in the file.
    ///
    /// Returns `None` if there is missing data in the compiler database.
    /// It is not expected for this function to return `Some([])`, but do not assume this.
    fn collect_lookup_items<'db>(
        &'db self,
        node: SyntaxNode<'db>,
    ) -> Option<Vec<LookupItemId<'db>>>;

    /// Returns a [`ModuleFileId`] containing the node.
    fn find_module_file_containing_node<'db>(
        &'db self,
        node: SyntaxNode<'db>,
    ) -> Option<ModuleFileId<'db>>;

    /// Finds a [`ModuleId`] containing the node.
    fn find_module_containing_node<'db>(&'db self, node: SyntaxNode<'db>) -> Option<ModuleId<'db>>;

    /// Reverse lookups [`VarId`] to get a [`Binding`] associated with it.
    ///
    /// While [`VarId`] is basically an ID of a [`Binding`],
    /// no mapping from the former to the latter is being maintained in [`SemanticGroup`].
    /// This forces us to perform elaborate reverse lookups,
    /// which makes life here much harder than needed.
    fn lookup_binding<'db>(&'db self, var_id: VarId<'db>) -> Option<Binding<'db>>;

    /// Collects `file` and all its descendants together with modules from all these files.
    ///
    /// **CAVEAT**: it does not collect descendant files that come from inline macros - it will when
    /// the compiler moves inline macros resolving to [`DefsGroup`].
    fn file_and_subfiles_with_corresponding_modules<'db>(
        &'db self,
        file: FileId<'db>,
    ) -> Option<(HashSet<FileId<'db>>, HashSet<ModuleId<'db>>)>;

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
    fn get_node_resultants<'db>(&'db self, node: SyntaxNode<'db>) -> Option<Vec<SyntaxNode<'db>>>;

    fn find_generated_nodes<'db>(
        &'db self,
        node_descendant_files: Arc<[FileId<'db>]>,
        node: SyntaxNode<'db>,
    ) -> OrderedHashSet<SyntaxNode<'db>>;

    /// Returns generic parameters defined by the item.
    /// Items that do not introduce any generics (use statements, modules etc.) yield an empty vector.
    fn item_generic_params<'db>(
        &'db self,
        lookup_item: LookupItemId<'db>,
    ) -> Vec<GenericParam<'db>>;
}

fn find_lookup_item<'db>(
    db: &'db dyn LsSemanticGroup,
    node: SyntaxNode<'db>,
) -> Option<LookupItemId<'db>> {
    db.collect_lookup_items_leaf(node)?.into_iter().next()
}

fn collect_lookup_items_leaf<'db>(
    db: &'db dyn LsSemanticGroup,
    node: SyntaxNode<'db>,
) -> Option<Vec<LookupItemId<'db>>> {
    let module_file_id = db.find_module_file_containing_node(node)?;

    node.ancestors_with_self(db).find_map(|node| lookup_item_from_ast(db, module_file_id, node))
}

fn collect_lookup_items_with_parent_files<'db>(
    db: &'db dyn LsSemanticGroup,
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

fn corresponding_node_in_parent_file<'db>(
    db: &'db dyn LsSemanticGroup,
    node: SyntaxNode<'db>,
) -> Option<SyntaxNode<'db>> {
    let (parent, mappings) = get_parent_and_mapping(db, node.stable_ptr(db).file_id(db))?;

    let span_in_parent = translate_location(&mappings, node.span(db))?;

    db.widest_node_within_span(parent, span_in_parent)
}

fn collect_lookup_items<'db>(
    db: &'db dyn LsSemanticGroup,
    node: SyntaxNode<'db>,
) -> Option<Vec<LookupItemId<'db>>> {
    let module_file_id = db.find_module_file_containing_node(node)?;

    Some(
        node.ancestors_with_self(db)
            .flat_map(|node| lookup_item_from_ast(db, module_file_id, node).unwrap_or_default())
            .collect(),
    )
}

fn find_module_file_containing_node<'db>(
    db: &'db dyn LsSemanticGroup,
    node: SyntaxNode<'db>,
) -> Option<ModuleFileId<'db>> {
    let module_id = db.find_module_containing_node(node)?;
    let file = node.stable_ptr(db).file_id(db);

    let i = db
        .module_files(module_id)
        .map(|files| {
            files.iter().enumerate().find(|(_, f)| **f == file).map(|(i, _)| i).unwrap_or_default()
        })
        .unwrap_or_default();

    let file_index = FileIndex(i);
    Some(ModuleFileId(module_id, file_index))
}

fn find_module_containing_node<'db>(
    db: &'db dyn LsSemanticGroup,
    node: SyntaxNode<'db>,
) -> Option<ModuleId<'db>> {
    // Get the main module of the main file containing the node.
    // The node may be located in a virtual file of a submodule.
    let main_module = {
        // Get the file where the node is located.
        // This might be a virtual file generated by a compiler plugin.
        let node_file_id = node.stable_ptr(db).file_id(db);

        // Get the root module of a file containing the node.
        db.file_modules(node_file_id).ok()?.iter().copied().next()?
    };

    // Get the stack (bottom-up) of submodule names in the file containing the node, in the main
    // module, that lead to the node.
    node.ancestors(db)
        .filter(|node| node.kind(db) == SyntaxKind::ItemModule)
        .map(|node| {
            ast::ItemModule::from_syntax_node(db, node).stable_ptr(db).name_green(db).identifier(db)
        })
        // Buffer the stack to get DoubleEndedIterator.
        .collect::<Vec<_>>()
        .into_iter()
        // And get id of the (sub)module containing the node by traversing this stack top-down.
        .try_rfold(main_module, |module, name| {
            let ModuleItemId::Submodule(submodule) =
                db.module_item_by_name(module, name.into()).ok()??
            else {
                return None;
            };
            Some(ModuleId::Submodule(submodule))
        })
}

fn lookup_binding<'db>(db: &'db dyn LsSemanticGroup, var_id: VarId<'db>) -> Option<Binding<'db>> {
    match var_id {
        VarId::Param(param_id) => {
            // Get param's syntax node.
            let param = param_id.untyped_stable_ptr(db).lookup(db);

            // Get the function which contains the variable/parameter.
            let function_id = db.find_lookup_item(param)?.function_with_body()?;

            // Get function signature.
            let signature = db.function_with_body_signature(function_id).ok()?;

            // Find the binding in the function's signature.
            signature.params.into_iter().find(|p| p.id == param_id).map(Into::into)
        }

        VarId::Local(local_var_id) => {
            // Get the Pattern syntax node which defines the variable.
            let identifier = local_var_id.untyped_stable_ptr(db).lookup(db);
            let pattern = identifier.ancestor_of_type::<ast::Pattern>(db)?;

            // Get the function which contains the variable/parameter.
            let function_id =
                db.find_lookup_item(pattern.as_syntax_node())?.function_with_body()?;

            // Get the semantic model for the pattern.
            let semantic_db: &dyn SemanticGroup = db.upcast();
            let pattern = semantic_db.pattern_semantic(
                function_id,
                db.lookup_pattern_by_ptr(function_id, pattern.stable_ptr(db)).ok()?,
            );

            // Extract the binding from the found pattern.
            let binding = pattern
                .variables(&QueryPatternVariablesFromDb(db.upcast(), function_id))
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

fn file_and_subfiles_with_corresponding_modules<'db>(
    db: &'db dyn LsSemanticGroup,
    file: FileId<'db>,
) -> Option<(HashSet<FileId<'db>>, HashSet<ModuleId<'db>>)> {
    let mut modules: HashSet<_> = db.file_modules(file).ok()?.iter().copied().collect();
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
        for file_id in db.module_files(module_id).ok()?.iter() {
            if files.insert(*file_id) {
                for module_id in db.file_modules(*file_id).ok()?.iter() {
                    if modules.insert(*module_id) {
                        modules_queue.push_back(*module_id);
                    }
                }
            }
        }
    }
    Some((files, modules))
}

#[tracing::instrument(skip_all)]
fn get_node_resultants<'db>(
    db: &'db dyn LsSemanticGroup,
    node: SyntaxNode<'db>,
) -> Option<Vec<SyntaxNode<'db>>> {
    let main_file = node.stable_ptr(db).file_id(db);

    let (mut files, _) = db.file_and_subfiles_with_corresponding_modules(main_file)?;

    files.remove(&main_file);

    let files: Arc<[FileId]> = files.into_iter().collect();
    let resultants = db.find_generated_nodes(files, node);

    Some(resultants.into_iter().collect())
}

/// If the ast node is a lookup item, return corresponding ids. Otherwise, returns `None`.
/// See [LookupItemId<'db>].
fn lookup_item_from_ast<'db>(
    db: &'db dyn SemanticGroup,
    module_file_id: ModuleFileId<'db>,
    node: SyntaxNode<'db>,
) -> Option<Vec<LookupItemId<'db>>> {
    let syntax_db = db;

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

#[tracing::instrument(skip_all)]
/// See [`LsSemanticGroup::get_node_resultants`].
fn find_generated_nodes<'db>(
    db: &'db dyn LsSemanticGroup,
    node_descendant_files: Arc<[FileId<'db>]>,
    node: SyntaxNode<'db>,
) -> OrderedHashSet<SyntaxNode<'db>> {
    let start_file = node.stable_ptr(db).file_id(db);

    let mut result = OrderedHashSet::default();

    let mut is_replaced = false;

    for file in node_descendant_files.iter().cloned() {
        let Some((parent, mappings)) = get_parent_and_mapping(db, file) else {
            continue;
        };

        if parent != start_file {
            continue;
        }

        let Ok(file_syntax) = file_syntax(db, file) else {
            continue;
        };

        let mappings: Vec<_> = mappings
            .iter()
            .filter(|mapping| match mapping.origin {
                CodeOrigin::CallSite(_) => true,
                CodeOrigin::Start(start) => start == node.span(db).start,
                CodeOrigin::Span(span) => node.span(db).contains(span),
            })
            .cloned()
            .collect();
        if mappings.is_empty() {
            continue;
        }

        let is_replacing_og_item = match file.long(db) {
            FileLongId::Virtual(vfs) => vfs.original_item_removed,
            FileLongId::External(id) => db.ext_as_virtual(*id).original_item_removed,
            _ => unreachable!(),
        };

        let mut new_nodes: OrderedHashSet<_> = Default::default();

        for mapping in &mappings {
            file_syntax.lookup_offset(db, mapping.span.start).for_each_terminal(db, |terminal| {
                // Skip end of the file terminal, which is also a syntax tree leaf.
                // As `ModuleItemList` and `TerminalEndOfFile` have the same parent,
                // which is the `SyntaxFile`, so we don't want to take the `SyntaxFile`
                // as an additional resultant.
                if terminal.kind(db) != SyntaxKind::TerminalEndOfFile {
                    let nodes: Vec<_> = terminal
                        .ancestors_with_self(db)
                        .map_while(|new_node| {
                            translate_location(&mappings, new_node.span(db))
                                .map(|span_in_parent| (new_node, span_in_parent))
                        })
                        .take_while(|(_, span_in_parent)| node.span(db).contains(*span_in_parent))
                        .collect();

                    if let Some((last_node, _)) = nodes.last().cloned() {
                        let (new_node, _) = nodes
                            .into_iter()
                            .rev()
                            .take_while(|(node, _)| node.span(db) == last_node.span(db))
                            .last()
                            .unwrap();

                        new_nodes.insert(new_node);
                    }
                }
            });
        }

        // If there is no node found, don't mark it as potentially replaced.
        if !new_nodes.is_empty() {
            is_replaced = is_replaced || is_replacing_og_item;
        }

        for new_node in new_nodes {
            result.extend(find_generated_nodes(db, Arc::clone(&node_descendant_files), new_node));
        }
    }

    if !is_replaced {
        result.insert(node);
    }

    result
}

pub fn file_syntax<'db>(db: &'db dyn ParserGroup, file: FileId<'db>) -> Maybe<SyntaxNode<'db>> {
    match file.kind(db) {
        FileKind::Expr => db.file_expr_syntax(file).map(|a| a.as_syntax_node()),
        FileKind::Module => db.file_module_syntax(file).map(|a| a.as_syntax_node()),
        FileKind::StatementList => db.file_statement_list_syntax(file).map(|a| a.as_syntax_node()),
    }
}

fn item_generic_params<'db>(
    db: &'db dyn SemanticGroup,
    lookup_item: LookupItemId<'db>,
) -> Vec<GenericParam<'db>> {
    match lookup_item {
        LookupItemId::ModuleItem(module_item_id) => match module_item_id {
            ModuleItemId::FreeFunction(free_function_id) => {
                db.free_function_generic_params(free_function_id)
            }
            ModuleItemId::Struct(struct_id) => db.struct_generic_params(struct_id),
            ModuleItemId::Enum(enum_id) => db.enum_generic_params(enum_id),
            ModuleItemId::TypeAlias(module_type_alias_id) => {
                db.module_type_alias_generic_params(module_type_alias_id)
            }
            ModuleItemId::ImplAlias(impl_alias_id) => db.impl_alias_generic_params(impl_alias_id),
            ModuleItemId::Trait(trait_id) => db.trait_generic_params(trait_id),
            ModuleItemId::Impl(impl_def_id) => db.impl_def_generic_params(impl_def_id),
            ModuleItemId::ExternType(extern_type_id) => {
                db.extern_type_declaration_generic_params(extern_type_id)
            }
            ModuleItemId::ExternFunction(extern_function_id) => {
                db.extern_function_declaration_generic_params(extern_function_id)
            }
            _ => return vec![],
        },
        LookupItemId::TraitItem(trait_item_id) => match trait_item_id {
            TraitItemId::Function(trait_function_id) => {
                db.trait_function_generic_params(trait_function_id)
            }
            TraitItemId::Type(trait_type_id) => db.trait_type_generic_params(trait_type_id),
            _ => return vec![],
        },
        LookupItemId::ImplItem(impl_item_id) => match impl_item_id {
            ImplItemId::Function(impl_function_id) => {
                db.impl_function_generic_params(impl_function_id)
            }
            ImplItemId::Type(impl_type_def_id) => db.impl_type_def_generic_params(impl_type_def_id),
            _ => return vec![],
        },
    }
    .into_iter()
    .flatten()
    .collect()
}
