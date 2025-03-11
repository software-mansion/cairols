use super::{analysis_context::AnalysisContext, db::AnalysisDatabase};
use crate::lang::{db::LsSemanticGroup, lsp::ToLsp};
use cairo_lang_defs::{
    db::DefsGroup,
    ids::{LanguageElementId, ModuleId},
};
use cairo_lang_syntax::node::{
    TypedStablePtr, TypedSyntaxNode, ast::ItemUse, ids::SyntaxStablePtrId,
};
use lsp_types::{Position, Range, TextEdit};
use std::fmt::Display;

pub fn new_import_edit(
    db: &AnalysisDatabase,
    ctx: &AnalysisContext,
    import_path: impl Display,
) -> Option<TextEdit> {
    let use_position = use_position(db, ctx)?;

    let mut new_text = format!("use {import_path};\n");

    if !use_position.is_sticking {
        new_text.push('\n');
    }

    Some(TextEdit { range: use_position.range(), new_text })
}

fn use_position(db: &AnalysisDatabase, ctx: &AnalysisContext) -> Option<UsePosition> {
    db.module_uses_ids(ctx.module_id)
        .ok()
        .and_then(|uses| {
            let module_main_file = db.module_main_file(ctx.module_id).ok()?;

            uses.iter()
                .find(|use_statement| {
                    use_statement.stable_ptr(db).untyped().file_id(db) == module_main_file
                })
                .map(|use_statement| {
                    use_statement
                        .stable_ptr(db)
                        .untyped()
                        .lookup(db)
                        .ancestor_of_type::<ItemUse>(db)
                        .expect("use leaf must be in use item")
                        .use_kw(db)
                        .token(db)
                        .stable_ptr()
                        .untyped()
                })
                .map(|stable_ptr| UsePosition::new(db, stable_ptr, true))
                .map(Some)
                .unwrap_or_else(|| {
                    db.module_items(ctx.module_id)
                        .ok()?
                        .iter()
                        .next()
                        .map(|item| UsePosition::new(db, item.untyped_stable_ptr(db), false))
                })
        })
        .unwrap_or_else(|| {
            let ModuleId::Submodule(submodule) = ctx.module_id else { unreachable!() };

            UsePosition::new(db, submodule.untyped_stable_ptr(db), false)
        })
}

#[derive(Debug)]
struct UsePosition {
    position: Position,
    pub is_sticking: bool,
}

impl UsePosition {
    pub fn new(db: &AnalysisDatabase, ptr: SyntaxStablePtrId, is_sticking: bool) -> Option<Self> {
        let node = ptr.lookup(db);

        Some(Self {
            position: node
                .span_start_without_trivia(db)
                .position_in_file(
                    db,
                    db.module_file(db.find_module_file_containing_node(&node)?).ok()?,
                )?
                .to_lsp(),
            is_sticking,
        })
    }

    pub fn range(&self) -> Range {
        Range::new(self.position, self.position)
    }
}
