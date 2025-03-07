use super::{analysis_context::AnalysisContext, db::AnalysisDatabase};
use crate::lang::lsp::ToLsp;
use cairo_lang_defs::{
    db::DefsGroup,
    ids::{FileIndex, LanguageElementId, ModuleFileId, ModuleId},
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
) -> TextEdit {
    let use_position = use_position(db, ctx);

    let mut new_text = format!("use {import_path};\n");

    if !use_position.is_sticking {
        new_text.push('\n');
    }

    TextEdit { range: use_position.range(), new_text }
}

fn use_position(db: &AnalysisDatabase, ctx: &AnalysisContext) -> UsePosition {
    db.module_uses_ids(ctx.module_id)
        .ok()
        .and_then(|uses| {
            uses.iter()
                .next()
                .map(|use_statement| {
                    Some(UsePosition::new(
                        db,
                        ctx,
                        use_statement
                            .stable_ptr(db)
                            .untyped()
                            .lookup(db)
                            .ancestor_of_type::<ItemUse>(db)
                            .expect("use leaf must be in use item")
                            .use_kw(db)
                            .token(db)
                            .stable_ptr()
                            .0,
                        true,
                    ))
                })
                .unwrap_or_else(|| {
                    db.module_items(ctx.module_id)
                        .ok()?
                        .iter()
                        .next()
                        .map(|item| UsePosition::new(db, ctx, item.untyped_stable_ptr(db), false))
                })
        })
        .unwrap_or_else(|| {
            let ModuleId::Submodule(submodule) = ctx.module_id else { unreachable!() };

            UsePosition::new(db, ctx, submodule.untyped_stable_ptr(db), false)
        })
}

#[derive(Debug)]
struct UsePosition {
    position: Position,
    pub is_sticking: bool,
}

impl UsePosition {
    pub fn new(
        db: &AnalysisDatabase,
        ctx: &AnalysisContext,
        ptr: SyntaxStablePtrId,
        is_sticking: bool,
    ) -> Self {
        Self {
            position: ptr
                .lookup(db)
                .offset()
                .position_in_file(
                    db,
                    db.module_file(ModuleFileId(ctx.module_id, FileIndex(0))).unwrap(),
                )
                .unwrap()
                .to_lsp(),
            is_sticking,
        }
    }

    pub fn range(&self) -> Range {
        Range::new(self.position, self.position)
    }
}
