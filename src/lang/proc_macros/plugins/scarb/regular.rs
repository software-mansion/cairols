use cairo_lang_defs::patcher::{PatchBuilder, RewriteNode};
use cairo_lang_defs::plugin::{PluginDiagnostic, PluginGeneratedFile, PluginResult};
use cairo_lang_filesystem::db::Edition;
use cairo_lang_filesystem::ids::{CodeMapping, CodeOrigin};
use std::collections::HashSet;

use super::into_cairo_diagnostics;
use crate::lang::db::AnalysisDatabase;
use crate::lang::proc_macros::db::{get_attribute_expansion, get_derive_expansion};
use crate::lang::proc_macros::plugins::scarb::child_nodes::{
    ChildNodesWithoutAttributes, ItemWithAttributes,
};
use crate::lang::proc_macros::plugins::scarb::conversion::{
    CallSiteLocation, code_mapping_from_proc_macro_server,
};
use crate::lang::proc_macros::plugins::scarb::types::{
    AdaptedCodeMapping, AdaptedDiagnostic, AdaptedTokenStream, ExpandableAttrLocation,
    TokenStreamBuilder,
};
use cairo_lang_filesystem::span::{TextOffset, TextSpan as CairoTextSpan};

use cairo_lang_macro::{AllocationContext, TextSpan, TokenStream, TokenStreamMetadata};
use cairo_lang_syntax::attribute::structured::{AttributeArgVariant, AttributeStructurize};
use cairo_lang_syntax::node::ast::{
    self, Expr, ImplItem, MaybeImplBody, MaybeTraitBody, PathSegment,
};
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::helpers::QueryAttrs;
use cairo_lang_syntax::node::ids::SyntaxStablePtrId;
use cairo_lang_syntax::node::{SyntaxNode, Terminal, TypedStablePtr, TypedSyntaxNode};
use convert_case::{Case, Casing};
use itertools::Itertools;
use scarb_proc_macro_server_types::methods::ProcMacroResult;
use scarb_proc_macro_server_types::methods::expand::{ExpandAttributeParams, ExpandDeriveParams};
use scarb_proc_macro_server_types::scope::ProcMacroScope;
use scarb_stable_hash::StableHasher;

use cairo_lang_utils::smol_str::{SmolStr, ToSmolStr};

const DERIVE_ATTR: &str = "derive";

/// Copied from: <https://github.com/software-mansion/scarb/blob/4e81d1c4498137f80e211c6e2c6a5a6de01c66f2/scarb/src/compiler/plugin/proc_macro/host.rs#L893>
/// Modified scarb code with replaced dylib calls in favour of [`ProcMacroGroup`] calls. Also
/// removed `aux_data`.
pub fn macro_generate_code(
    db: &AnalysisDatabase,
    expansion_context: ProcMacroScope,
    item_ast: ast::ModuleItem,
    defined_attributes: &[String],
    defined_derives: &[String],
    metadata: &cairo_lang_defs::plugin::MacroPluginMetadata<'_>,
) -> PluginResult {
    let stream_metadata = calculate_metadata(db, item_ast.clone(), metadata.edition);

    // Handle inner functions.
    if let InnerAttrExpansionResult::Some(result) =
        expand_inner_attr(db, expansion_context.clone(), defined_attributes, item_ast.clone())
    {
        return result.into();
    }

    // Expand first attribute.
    // Note that we only expand the first attribute, as we assume that the rest of the attributes
    // will be handled by a subsequent call to this function.
    let ctx = AllocationContext::default();
    let (input, body) = parse_attribute(db, Vec::from(defined_attributes), item_ast.clone(), &ctx);

    if let Some(result) = match input {
        AttrExpansionFound::Last(AttrExpansionArgs {
            name,
            args,
            call_site,
            attribute_location,
        }) => Some((name, args, call_site, attribute_location, true)),
        AttrExpansionFound::Some(AttrExpansionArgs {
            name,
            args,
            call_site,
            attribute_location,
        }) => Some((name, args, call_site, attribute_location, false)),
        AttrExpansionFound::None => None,
    }
    .map(|(name, args, call_site, attribute_location, last)| {
        let token_stream = body.with_metadata(stream_metadata.clone());
        expand_attribute(
            db,
            expansion_context.clone(),
            last,
            token_stream,
            item_ast.as_syntax_node(),
            AttrExpansionArgs { name, call_site, args, attribute_location },
        )
    }) {
        return result.into();
    }

    // Expand all derives.
    // Note that all proc macro attributes should be already expanded at this point.
    if let Some(result) = expand_derives(
        db,
        expansion_context.clone(),
        defined_derives,
        item_ast.clone(),
        stream_metadata.clone(),
    ) {
        return result;
    }

    // No expansions can be applied.
    PluginResult { code: None, diagnostics: Vec::new(), remove_original_item: false }
}

fn expand_inner_attr(
    db: &AnalysisDatabase,
    expansion_context: ProcMacroScope,
    defined_attributes: &[String],
    item_ast: ast::ModuleItem,
) -> InnerAttrExpansionResult {
    let mut context = InnerAttrExpansionContext::new(db, &item_ast);
    let mut used_attr_names: HashSet<String> = Default::default();
    let mut all_none = true;
    let ctx = AllocationContext::default();
    let item_start_offset = item_ast.as_syntax_node().span(db).start;

    match item_ast.clone() {
        ast::ModuleItem::Trait(trait_ast) => {
            context.add_node(trait_ast.attributes(db).as_syntax_node());
            context.add_node(trait_ast.visibility(db).as_syntax_node());
            context.add_node(trait_ast.trait_kw(db).as_syntax_node());
            context.add_node(trait_ast.name(db).as_syntax_node());
            context.add_node(trait_ast.generic_params(db).as_syntax_node());

            // Parser attributes for inner functions.
            match trait_ast.body(db) {
                MaybeTraitBody::None(terminal) => {
                    context.add_node(terminal.as_syntax_node());
                    InnerAttrExpansionResult::None
                }
                MaybeTraitBody::Some(body) => {
                    context.add_node(body.lbrace(db).as_syntax_node());

                    let item_list = body.items(db);
                    for item in item_list.elements(db) {
                        let ast::TraitItem::Function(func) = item else {
                            context.add_node(item.as_syntax_node());
                            continue;
                        };

                        let mut token_stream_builder = TokenStreamBuilder::new(db);
                        let attrs = func.attributes(db).elements(db).collect_vec();
                        let found = parse_attrs(
                            db,
                            defined_attributes,
                            &mut token_stream_builder,
                            attrs,
                            item_start_offset,
                            &ctx,
                        );
                        if let Some(name) = found.as_name() {
                            used_attr_names.insert(name);
                        }
                        token_stream_builder.add_node(func.declaration(db).as_syntax_node());
                        token_stream_builder.add_node(func.body(db).as_syntax_node());
                        let token_stream = token_stream_builder.build(&ctx);

                        let token_stream = found.adapt_token_stream(token_stream);
                        all_none = all_none
                            && do_expand_inner_attr(
                                db,
                                &mut context,
                                expansion_context.clone(),
                                found,
                                &func,
                                token_stream,
                            );
                    }

                    context.add_node(body.rbrace(db).as_syntax_node());

                    if all_none {
                        InnerAttrExpansionResult::None
                    } else {
                        InnerAttrExpansionResult::Some(
                            context.into_result(used_attr_names.into_iter().collect()),
                        )
                    }
                }
            }
        }

        ast::ModuleItem::Impl(impl_ast) => {
            context.add_node(impl_ast.attributes(db).as_syntax_node());
            context.add_node(impl_ast.visibility(db).as_syntax_node());
            context.add_node(impl_ast.impl_kw(db).as_syntax_node());
            context.add_node(impl_ast.name(db).as_syntax_node());
            context.add_node(impl_ast.generic_params(db).as_syntax_node());
            context.add_node(impl_ast.of_kw(db).as_syntax_node());
            context.add_node(impl_ast.trait_path(db).as_syntax_node());

            match impl_ast.body(db) {
                MaybeImplBody::None(terminal) => {
                    context.add_node(terminal.as_syntax_node());
                    InnerAttrExpansionResult::None
                }
                MaybeImplBody::Some(body) => {
                    context.add_node(body.lbrace(db).as_syntax_node());

                    let items = body.items(db);
                    for item in items.elements(db) {
                        let ImplItem::Function(func) = item else {
                            context.add_node(item.as_syntax_node());
                            continue;
                        };

                        let mut token_stream_builder = TokenStreamBuilder::new(db);
                        let attrs = func.attributes(db).elements(db).collect_vec();
                        let found = parse_attrs(
                            db,
                            defined_attributes,
                            &mut token_stream_builder,
                            attrs,
                            item_start_offset,
                            &ctx,
                        );
                        if let Some(name) = found.as_name() {
                            used_attr_names.insert(name);
                        }
                        token_stream_builder.add_node(func.visibility(db).as_syntax_node());
                        token_stream_builder.add_node(func.declaration(db).as_syntax_node());
                        token_stream_builder.add_node(func.body(db).as_syntax_node());
                        let token_stream = token_stream_builder.build(&ctx);
                        let token_stream = found.adapt_token_stream(token_stream);
                        all_none = all_none
                            && do_expand_inner_attr(
                                db,
                                &mut context,
                                expansion_context.clone(),
                                found,
                                &func,
                                token_stream,
                            );
                    }

                    context.add_node(body.rbrace(db).as_syntax_node());

                    if all_none {
                        InnerAttrExpansionResult::None
                    } else {
                        InnerAttrExpansionResult::Some(
                            context.into_result(used_attr_names.into_iter().collect()),
                        )
                    }
                }
            }
        }
        _ => InnerAttrExpansionResult::None,
    }
}

fn do_expand_inner_attr(
    db: &AnalysisDatabase,
    context: &mut InnerAttrExpansionContext,
    expansion_context: ProcMacroScope,
    found: AttrExpansionFound,
    func: &impl TypedSyntaxNode,
    token_stream: AdaptedTokenStream,
) -> bool {
    let mut all_none = true;
    let input = match found {
        AttrExpansionFound::Last(input) => {
            all_none = false;
            input
        }
        AttrExpansionFound::Some(input) => {
            all_none = false;
            input
        }
        AttrExpansionFound::None => {
            context.add_node(func.as_syntax_node());
            return all_none;
        }
    };

    let result = get_attribute_expansion(
        db,
        ExpandAttributeParams {
            context: expansion_context,
            attr: input.name.clone(),
            args: input.args.clone(),
            item: TokenStream::from(token_stream.clone()),
            adapted_call_site: input.attribute_location.adapted_call_site().into(),
        },
    );

    if result.code_mappings.is_some() {
        // v2 logic.
        context.register_result_metadata_v2(db, &input, token_stream.to_string(), result.clone());
    } else {
        // v1 logic.
        context.register_result_metadata_v1(
            result.token_stream.to_string(),
            func.as_syntax_node().span(db),
        );
    }

    all_none
}

struct InnerAttrExpansionContext<'a> {
    // Metadata returned for expansions.
    diagnostics: Vec<PluginDiagnostic>,
    any_changed: bool,
    item_builder: PatchBuilder<'a>,
}

impl<'a> InnerAttrExpansionContext<'a> {
    pub fn new(db: &'a dyn SyntaxGroup, item_ast: &'a ast::ModuleItem) -> Self {
        Self {
            diagnostics: Vec::new(),
            any_changed: false,
            item_builder: PatchBuilder::new(db, item_ast),
        }
    }

    pub fn add_node(&mut self, node: SyntaxNode) {
        self.item_builder.add_node(node);
    }

    fn register_diagnotics(
        &mut self,
        db: &dyn SyntaxGroup,
        diagnostics: Vec<AdaptedDiagnostic>,
        stable_ptr: SyntaxStablePtrId,
    ) {
        let diagnostics = diagnostics.into_iter().map(Into::into).collect();
        self.diagnostics.extend(into_cairo_diagnostics(db, diagnostics, stable_ptr));
    }

    pub fn register_result_metadata_v2(
        &mut self,
        db: &dyn SyntaxGroup,
        input: &AttrExpansionArgs,
        original: String,
        result: ProcMacroResult,
    ) {
        let expanded = result.token_stream.to_string();
        let changed = expanded.as_str() != original;

        let diagnostics = input.attribute_location.adapt_diagnostics(result.diagnostics);
        self.register_diagnotics(db, diagnostics, input.call_site.stable_ptr);

        self.any_changed = self.any_changed || changed;

        let code_mappings = result.code_mappings.unwrap_or_default();
        let code_mappings =
            code_mappings.into_iter().map(code_mapping_from_proc_macro_server).collect();
        let adapted_code_mappings = input.attribute_location.adapt_code_mappings(code_mappings);
        self.item_builder.add_modified(rewrite_node_patch_from_expansion_result(
            adapted_code_mappings,
            result.token_stream.to_string(),
        ));
    }

    pub fn register_result_metadata_v1(&mut self, result: String, origin_span: CairoTextSpan) {
        self.item_builder.add_modified(RewriteNode::Mapped {
            origin: origin_span,
            node: Box::new(RewriteNode::Text(result)),
        });
    }

    pub fn into_result(self, attr_names: Vec<String>) -> AttributePluginResult {
        let msg = if attr_names.len() == 1 {
            "the attribute macro"
        } else {
            "one of the attribute macros"
        };
        let derive_names = attr_names.iter().join("`, `");
        let note = format!("this error originates in {msg}: `{derive_names}`");

        AttributePluginResult::new()
            .with_remove_original_item(true)
            .with_plugin_diagnostics(self.diagnostics)
            .with_generated_file(
                AttributeGeneratedFile::from_patch_builder("proc_attr_inner", self.item_builder)
                    .with_diagnostics_note(note),
            )
    }
}

fn rewrite_node_patch_from_expansion_result(
    code_mappings: Vec<AdaptedCodeMapping>,
    expanded: String,
) -> RewriteNode {
    let code_mappings = code_mappings.into_iter().map(Into::into).collect_vec();
    RewriteNode::TextAndMapping(expanded, code_mappings)
}

enum InnerAttrExpansionResult {
    None,
    Some(AttributePluginResult),
}

pub enum AttrExpansionFound {
    Some(AttrExpansionArgs),
    Last(AttrExpansionArgs),
    None,
}

pub struct AttrExpansionArgs {
    pub name: String,
    pub args: TokenStream,
    pub call_site: CallSiteLocation,
    pub attribute_location: ExpandableAttrLocation,
}

impl AttrExpansionFound {
    pub fn as_name(&self) -> Option<String> {
        match self {
            AttrExpansionFound::Some(AttrExpansionArgs { name, .. })
            | AttrExpansionFound::Last(AttrExpansionArgs { name, .. }) => Some(name.clone()),
            AttrExpansionFound::None => None,
        }
    }
}

/// Find first attribute procedural macros that should be expanded.
///
/// Remove the attribute from the code.
pub(crate) fn parse_attribute(
    db: &dyn SyntaxGroup,
    defined_attributes: Vec<String>,
    item_ast: ast::ModuleItem,
    ctx: &AllocationContext,
) -> (AttrExpansionFound, AdaptedTokenStream) {
    let mut token_stream_builder = TokenStreamBuilder::new(db);
    let input = match item_ast.clone() {
        ast::ModuleItem::Trait(ast) => {
            parse_item(&ast, db, &mut token_stream_builder, ctx, defined_attributes)
        }
        ast::ModuleItem::Impl(ast) => {
            parse_item(&ast, db, &mut token_stream_builder, ctx, defined_attributes)
        }
        ast::ModuleItem::Module(ast) => {
            parse_item(&ast, db, &mut token_stream_builder, ctx, defined_attributes)
        }
        ast::ModuleItem::FreeFunction(ast) => {
            parse_item(&ast, db, &mut token_stream_builder, ctx, defined_attributes)
        }
        ast::ModuleItem::ExternFunction(ast) => {
            parse_item(&ast, db, &mut token_stream_builder, ctx, defined_attributes)
        }
        ast::ModuleItem::ExternType(ast) => {
            parse_item(&ast, db, &mut token_stream_builder, ctx, defined_attributes)
        }
        ast::ModuleItem::Struct(ast) => {
            parse_item(&ast, db, &mut token_stream_builder, ctx, defined_attributes)
        }
        ast::ModuleItem::Enum(ast) => {
            parse_item(&ast, db, &mut token_stream_builder, ctx, defined_attributes)
        }
        ast::ModuleItem::Constant(ast) => {
            parse_item(&ast, db, &mut token_stream_builder, ctx, defined_attributes)
        }
        ast::ModuleItem::Use(ast) => {
            parse_item(&ast, db, &mut token_stream_builder, ctx, defined_attributes)
        }
        ast::ModuleItem::ImplAlias(ast) => {
            parse_item(&ast, db, &mut token_stream_builder, ctx, defined_attributes)
        }
        ast::ModuleItem::TypeAlias(ast) => {
            parse_item(&ast, db, &mut token_stream_builder, ctx, defined_attributes)
        }
        // The items below are not supported.
        ast::ModuleItem::HeaderDoc(_) => AttrExpansionFound::None,
        ast::ModuleItem::Missing(_) => AttrExpansionFound::None,
        ast::ModuleItem::MacroDeclaration(_) => AttrExpansionFound::None,
        ast::ModuleItem::InlineMacro(_) => AttrExpansionFound::None,
    };
    let token_stream = input.adapt_token_stream(token_stream_builder.build(ctx));
    (input, token_stream)
}

fn parse_item<T: ItemWithAttributes + ChildNodesWithoutAttributes>(
    ast: &T,
    db: &dyn SyntaxGroup,
    token_stream_builder: &mut TokenStreamBuilder<'_>,
    ctx: &AllocationContext,
    defined_attributes: Vec<String>,
) -> AttrExpansionFound {
    let span = ast.span_with_trivia(db);
    let attrs = ast.item_attributes(db);
    let expansion =
        parse_attrs(db, &defined_attributes, token_stream_builder, attrs, span.start, ctx);
    token_stream_builder.extend(ast.child_nodes_without_attributes(db));
    expansion
}

fn parse_attrs(
    db: &dyn SyntaxGroup,
    defined_attributes: &[String],
    builder: &mut TokenStreamBuilder<'_>,
    item_attrs: Vec<ast::Attribute>,
    item_start_offset: TextOffset,
    ctx: &AllocationContext,
) -> AttrExpansionFound {
    // This function parses attributes of the item,
    // checking if those attributes correspond to a procedural macro that should be fired.
    // The proc macro attribute found is removed from attributes list,
    // while other attributes are appended to the `PathBuilder` passed as an argument.

    // Note this function does not affect the executable attributes,
    // as it only pulls `ExpansionKind::Attr` from the plugin.
    // This means that executable attributes will neither be removed from the item,
    // nor will they cause the item to be rewritten.
    let mut expansion = None;
    let mut last = true;
    for attr in item_attrs {
        // We ensure that this flag is changed *after* the expansion is found.
        if last {
            let structured_attr = attr.clone().structurize(db);
            let found = defined_attributes.contains(&structured_attr.id.into());
            if found {
                if expansion.is_none() {
                    let mut args_builder = TokenStreamBuilder::new(db);
                    args_builder.add_node(attr.arguments(db).as_syntax_node());
                    let args = args_builder.build(ctx);
                    expansion = Some(AttrExpansionArgs {
                        name: attr.attr(db).as_syntax_node().get_text_without_trivia(db),
                        args,
                        call_site: CallSiteLocation::new(&attr, db),
                        attribute_location: ExpandableAttrLocation::new(
                            &attr,
                            item_start_offset,
                            db,
                        ),
                    });
                    // Do not add the attribute for found expansion.
                    continue;
                } else {
                    last = false;
                }
            }
        }
        builder.add_node(attr.as_syntax_node());
    }
    match (expansion, last) {
        (Some(args), true) => AttrExpansionFound::Last(args),
        (Some(args), false) => AttrExpansionFound::Some(args),
        (None, _) => AttrExpansionFound::None,
    }
}

/// Handle `#[derive(...)]` attribute.
///
/// Returns a list of expansions that this plugin should apply.
fn parse_derive(
    db: &dyn SyntaxGroup,
    defined_derives: &[String],
    item_ast: ast::ModuleItem,
) -> Vec<(String, CallSiteLocation)> {
    let attrs = match item_ast {
        ast::ModuleItem::Struct(struct_ast) => {
            Some(struct_ast.query_attr(db, DERIVE_ATTR).collect_vec())
        }
        ast::ModuleItem::Enum(enum_ast) => Some(enum_ast.query_attr(db, DERIVE_ATTR).collect_vec()),
        _ => None,
    };

    attrs
        .unwrap_or_default()
        .iter()
        .map(|attr| attr.clone().structurize(db))
        .flat_map(|attr| attr.args.into_iter())
        .filter_map(|attr| {
            let AttributeArgVariant::Unnamed(value) = attr.clone().variant else {
                return None;
            };
            let Expr::Path(path) = value else {
                return None;
            };
            let path = path.segments(db).elements(db);
            let path = path.last()?;
            let PathSegment::Simple(segment) = path else {
                return None;
            };
            let ident = segment.ident(db);
            let value = ident.text(db).to_string();

            let matching_derive = defined_derives
                .iter()
                .find(|derive| derive.to_case(Case::Pascal) == value)
                .cloned()?;

            Some((matching_derive, CallSiteLocation::new(&segment, db)))
        })
        .collect()
}

fn expand_derives(
    db: &AnalysisDatabase,
    expansion_context: ProcMacroScope,
    defined_derives: &[String],
    item_ast: ast::ModuleItem,
    stream_metadata: TokenStreamMetadata,
) -> Option<PluginResult> {
    let mut token_stream_builder = TokenStreamBuilder::new(db);
    token_stream_builder.add_node(item_ast.as_syntax_node());
    token_stream_builder.with_metadata(stream_metadata.clone());
    let ctx = AllocationContext::default();
    let token_stream = token_stream_builder.build(&ctx);

    // All derives to be applied.
    let derives = parse_derive(db, defined_derives, item_ast.clone());

    if derives.is_empty() {
        // No derives found - returning early.
        return None;
    }

    let stable_ptr = derives[0].1.stable_ptr;
    let span_db = stable_ptr.lookup(db).span(db);
    let call_site = TextSpan { start: span_db.start.as_u32(), end: span_db.end.as_u32() };

    let derive_names: Vec<String> = derives.into_iter().map(|a| a.0).collect();
    // region: Modified scarb code
    let result = get_derive_expansion(
        db,
        ExpandDeriveParams {
            context: expansion_context,
            derives: derive_names.clone(),
            item: token_stream,
            call_site,
        },
    );
    // endregion

    Some(PluginResult {
        code: if result.token_stream.is_empty() {
            None
        } else {
            let content = result.token_stream.to_string();
            let msg = if derive_names.len() == 1 {
                "the derive macro"
            } else {
                "one of the derive macros"
            };
            let derive_names = derive_names.iter().join("`, `");
            let note = format!("this error originates in {msg}: `{derive_names}`");

            let code_mappings = result
                .code_mappings
                .map(|x| x.into_iter().map(code_mapping_from_proc_macro_server).collect())
                .unwrap_or_default();

            Some(PluginGeneratedFile {
                name: "proc_macro_derive".into(),
                code_mappings,
                content,
                aux_data: None,
                diagnostics_note: Some(note),
            })
        },
        diagnostics: into_cairo_diagnostics(db, result.diagnostics, stable_ptr),
        // Note that we don't remove the original item here, unlike for attributes.
        // We do not add the original code to the generated file either.
        remove_original_item: false,
    })
}

#[allow(clippy::too_many_arguments)]
fn expand_attribute(
    db: &AnalysisDatabase,
    expansion_context: ProcMacroScope,
    last: bool,
    token_stream: AdaptedTokenStream,
    original_node: SyntaxNode,
    input: AttrExpansionArgs,
) -> AttributePluginResult {
    // region: Modified scarb code
    let result = get_attribute_expansion(
        db,
        ExpandAttributeParams {
            context: expansion_context,
            args: input.args.clone(),
            attr: input.name.clone(),
            item: token_stream.clone().into(),
            adapted_call_site: input.attribute_location.adapted_call_site().into(),
        },
    );
    // endregion
    let diagnostics =
        input.attribute_location.adapt_diagnostics(result.diagnostics).into_iter().collect();

    // Handle token stream.
    if result.token_stream.is_empty() {
        // Remove original code
        return AttributePluginResult::new().with_remove_original_item(true).with_diagnostics(
            db,
            input.call_site.stable_ptr,
            diagnostics,
        );
    }

    // This is a minor optimization.
    // If the expanded macro attribute is the only one that will be expanded by `ProcMacroHost`
    // in this `generate_code` call (i.e. all the other macro attributes has been expanded by
    // previous calls), and the expansion did not produce any changes, we can skip rewriting the
    // expanded node by simply returning no generated code, and leaving the original item as is.
    // However, if we have other macro attributes to expand, we must rewrite the node even if no
    // changes have been produced, so that we can parse the attributes once again and expand them.
    // In essence, `code: None, remove_original_item: false` means `ProcMacroHost` will not be
    // called again for this AST item.
    // This optimization limits the number of generated nodes a bit.
    if last && token_stream.to_string() == result.token_stream.to_string() {
        return AttributePluginResult::new().with_diagnostics(
            db,
            input.call_site.stable_ptr,
            diagnostics,
        );
    }

    let file_name = format!("proc_macro_{}", input.name);
    let content = result.token_stream.to_string();
    let mappings: Vec<CodeMapping> = result
        .code_mappings
        .map(|mappings| {
            input
                .attribute_location
                .adapt_code_mappings(
                    mappings.into_iter().map(code_mapping_from_proc_macro_server).collect(),
                )
                .into_iter()
                .map(Into::into)
                .collect()
        })
        .unwrap_or_else(|| {
            vec![CodeMapping {
                origin: CodeOrigin::Span(original_node.span_without_trivia(db)),
                span: CairoTextSpan::from_str(&content),
            }]
        });

    AttributePluginResult::new()
        .with_remove_original_item(true)
        .with_diagnostics(db, input.call_site.stable_ptr, diagnostics)
        .with_generated_file(
            AttributeGeneratedFile::new(file_name)
                .with_content(content)
                .with_code_mappings(mappings)
                .with_diagnostics_note(format!(
                    "this error originates in the attribute macro: `{}`",
                    input.name
                )),
        )
}

fn calculate_metadata(
    db: &dyn SyntaxGroup,
    item_ast: ast::ModuleItem,
    edition: Edition,
) -> TokenStreamMetadata {
    fn short_hash(hashable: impl std::hash::Hash) -> String {
        let mut hasher = StableHasher::new();
        hashable.hash(&mut hasher);
        hasher.finish_as_short_hash()
    }

    let stable_ptr = item_ast.clone().stable_ptr(db).untyped();
    let file_path = stable_ptr.file_id(db).full_path(db);
    let file_id = short_hash(file_path.clone());
    let edition = serde_json::to_value(edition).unwrap();
    TokenStreamMetadata::new(file_path, file_id, edition)
}

#[derive(Default)]
pub struct AttributePluginResult {
    diagnostics: Vec<PluginDiagnostic>,
    remove_original_item: bool,
    code: Option<PluginGeneratedFile>,
}

impl AttributePluginResult {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_diagnostics(
        mut self,
        db: &dyn SyntaxGroup,
        call_site_stable_ptr: SyntaxStablePtrId,
        diagnostics: Vec<AdaptedDiagnostic>,
    ) -> Self {
        let diagnostics = diagnostics.into_iter().map(Into::into).collect();
        self.diagnostics = into_cairo_diagnostics(db, diagnostics, call_site_stable_ptr);
        self
    }

    pub fn with_plugin_diagnostics(mut self, diagnostics: Vec<PluginDiagnostic>) -> Self {
        self.diagnostics = diagnostics;
        self
    }

    pub fn with_remove_original_item(mut self, remove: bool) -> Self {
        self.remove_original_item = remove;
        self
    }

    pub fn with_generated_file(mut self, generated_file: AttributeGeneratedFile) -> Self {
        self.code = Some(generated_file.into());
        self
    }
}

impl From<AttributePluginResult> for PluginResult {
    fn from(value: AttributePluginResult) -> Self {
        PluginResult {
            diagnostics: value.diagnostics,
            remove_original_item: value.remove_original_item,
            code: value.code,
        }
    }
}

pub struct AttributeGeneratedFile {
    name: SmolStr,
    content: String,
    code_mappings: Vec<CodeMapping>,
    diagnostics_note: Option<String>,
}

impl AttributeGeneratedFile {
    pub fn new(name: impl ToSmolStr) -> Self {
        Self {
            name: name.to_smolstr(),
            content: Default::default(),
            code_mappings: Default::default(),
            diagnostics_note: Default::default(),
        }
    }

    pub fn from_patch_builder(name: impl ToSmolStr, item_builder: PatchBuilder<'_>) -> Self {
        let (expanded, mut code_mappings) = item_builder.build();
        // PatchBuilder::build() adds additional mapping at the end,
        // which wraps the whole outputted code.
        // We remove it, so we can properly translate locations spanning multiple token spans.
        code_mappings.pop();
        Self {
            name: name.to_smolstr(),
            content: expanded,
            code_mappings,
            diagnostics_note: Default::default(),
        }
    }

    pub fn with_content(mut self, content: impl ToString) -> Self {
        self.content = content.to_string();
        self
    }

    pub fn with_code_mappings(mut self, code_mappings: Vec<CodeMapping>) -> Self {
        self.code_mappings = code_mappings;
        self
    }

    pub fn with_diagnostics_note(mut self, diagnostics_note: impl ToString) -> Self {
        self.diagnostics_note = Some(diagnostics_note.to_string());
        self
    }
}

impl From<AttributeGeneratedFile> for PluginGeneratedFile {
    fn from(value: AttributeGeneratedFile) -> Self {
        PluginGeneratedFile {
            name: value.name,
            content: value.content,
            code_mappings: value.code_mappings,
            aux_data: None,
            diagnostics_note: value.diagnostics_note,
        }
    }
}
