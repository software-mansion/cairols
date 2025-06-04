use super::{markdown::fenced_code_block, ty::format_type};
use crate::lang::{
    db::{AnalysisDatabase, LsSemanticGroup},
    lsp::{LsProtoGroup, ToCairo, ToLsp},
};
use cairo_lang_filesystem::ids::FileId;
use cairo_lang_parser::db::ParserGroup;
use cairo_lang_semantic::{
    db::SemanticGroup,
    expr::inference::InferenceId,
    lookup_item::{HasResolverData, LookupItemEx},
    substitution::SemanticRewriter,
};
use cairo_lang_syntax::node::{
    SyntaxNode, TypedStablePtr, TypedSyntaxNode,
    ast::{OptionTypeClause, StatementLet},
};
use lsp_types::{
    InlayHint, InlayHintKind, InlayHintLabel, InlayHintLabelPart, InlayHintLabelPartTooltip,
    InlayHintParams, MarkupContent, MarkupKind,
};
use types::find_underscores;

mod types;

pub fn inlay_hints(db: &AnalysisDatabase, params: InlayHintParams) -> Option<Vec<InlayHint>> {
    let file = db.file_for_url(&params.text_document.uri)?;
    let range = params.range.to_cairo().offset_in_file(db, file)?;

    let syntax = db.file_syntax(file).ok()?;

    let mut result = vec![];

    let importables =
        db.visible_importables_from_module(db.find_module_file_containing_node(syntax)?)?;

    for let_statement in syntax
        .descendants(db)
        .filter(|node| range.contains(node.span_without_trivia(db)))
        .filter_map(|node| StatementLet::cast(db, node))
    {
        let Some(lookup_item) = db.find_lookup_item(let_statement.as_syntax_node()) else {
            continue;
        };
        let Some(func) = lookup_item.function_with_body() else {
            continue;
        };

        let Some(body) = db.function_body(func).ok() else {
            continue;
        };

        let mut inference_data = lookup_item
            .resolver_data(db)
            .ok()?
            .inference_data
            .clone_with_inference_id(db, InferenceId::NoContext);
        let mut inference = inference_data.inference(db);

        for (_id, pat) in &body.arenas.patterns {
            if let_statement.pattern(db) != pat.stable_ptr().lookup(db) {
                continue;
            }

            let type_clause = match let_statement.type_clause(db) {
                OptionTypeClause::Empty(_) => None,
                OptionTypeClause::TypeClause(type_clause) => Some(type_clause.ty(db)),
            };

            for var in pat.variables(&body.arenas.patterns) {
                let pattern_node = var.stable_ptr.0.lookup(db);

                let ty = inference.rewrite(var.var.ty).ok()?;

                if let Some(type_clause) = type_clause.clone() {
                    for (underscore, inferred_ty) in find_underscores(db, type_clause, ty) {
                        let type_string = inferred_ty.format(db, &importables);

                        let tooltip = fenced_code_block(&type_string);

                        result.extend(var_type_inlay_hint(
                            db,
                            file,
                            underscore.as_syntax_node(),
                            type_string,
                            tooltip,
                        ));
                    }
                } else {
                    let type_string = format_type(db, ty, &importables);
                    let tooltip = fenced_code_block(&type_string);

                    result.extend(var_type_inlay_hint(
                        db,
                        file,
                        pattern_node,
                        type_string,
                        tooltip,
                    ));
                };
            }
        }
    }

    Some(result)
}

fn var_type_inlay_hint(
    db: &AnalysisDatabase,
    file: FileId,
    node: SyntaxNode,
    type_string: String,
    tooltip: String,
) -> Option<InlayHint> {
    Some(InlayHint {
        position: node.span_without_trivia(db).position_in_file(db, file)?.end.to_lsp(),
        label: InlayHintLabel::LabelParts(vec![
            InlayHintLabelPart { value: ": ".to_string(), tooltip: None, ..Default::default() },
            InlayHintLabelPart {
                value: type_string.clone(),
                tooltip: Some(InlayHintLabelPartTooltip::MarkupContent(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: tooltip,
                })),
                ..Default::default()
            },
        ]),
        kind: Some(InlayHintKind::TYPE),
        text_edits: None,
        tooltip: None,
        padding_left: None,
        padding_right: None,
        data: None,
    })
}
