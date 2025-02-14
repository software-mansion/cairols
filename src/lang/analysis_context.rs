use cairo_lang_defs::ids::{LookupItemId, ModuleId};
use cairo_lang_semantic::expr::inference::InferenceId;
use cairo_lang_semantic::lookup_item::HasResolverData;
use cairo_lang_semantic::resolve::Resolver;
use cairo_lang_syntax::node::SyntaxNode;

use super::db::{AnalysisDatabase, LsSemanticGroup};

pub struct AnalysisContext<'db> {
    pub node: SyntaxNode,
    pub module_id: ModuleId,
    pub lookup_item_id: Option<LookupItemId>,
    resolver: Resolver<'db>,
}

impl<'db> AnalysisContext<'db> {
    pub fn from_node(db: &'db AnalysisDatabase, node: SyntaxNode) -> Option<AnalysisContext<'db>> {
        let module_file_id = db.find_module_file_containing_node(&node)?;
        let lookup_item_id = db.find_lookup_item(&node);

        let resolver = match lookup_item_id {
            Some(item) => {
                let data = item
                    .resolver_data(db)
                    .ok()?
                    .clone_with_inference_id(db, InferenceId::NoContext);

                Resolver::with_data(db, data)
            }
            None => Resolver::new(db, module_file_id, InferenceId::NoContext),
        };

        let module_id = module_file_id.0;

        Some(Self { node, module_id, lookup_item_id, resolver })
    }

    pub fn resolver(&self, db: &'db AnalysisDatabase) -> Resolver<'db> {
        // There is no other way to clone resolver.
        Resolver::with_data(db, self.resolver.clone_with_inference_id(db, InferenceId::NoContext))
    }
}
