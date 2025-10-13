use cairo_lang_defs::ids::{LookupItemId, ModuleId};
use cairo_lang_semantic::expr::inference::InferenceId;
use cairo_lang_semantic::lookup_item::HasResolverData;
use cairo_lang_semantic::resolve::Resolver;
use cairo_lang_syntax::node::SyntaxNode;

use super::db::{AnalysisDatabase, LsSemanticGroup};

pub struct AnalysisContext<'db> {
    pub node: SyntaxNode<'db>,
    pub module_id: ModuleId<'db>,
    pub lookup_item_id: Option<LookupItemId<'db>>,
    resolver: Resolver<'db>,
}

impl<'db> AnalysisContext<'db> {
    pub fn from_node(
        db: &'db AnalysisDatabase,
        node: SyntaxNode<'db>,
    ) -> Option<AnalysisContext<'db>> {
        let module_id = db.find_module_containing_node(node)?;
        let lookup_item_id = db.find_lookup_item(node);

        let resolver = match lookup_item_id.and_then(|item| item.resolver_data(db).ok()) {
            Some(item) => {
                Resolver::with_data(db, item.clone_with_inference_id(db, InferenceId::NoContext))
            }
            None => Resolver::new(db, module_id, InferenceId::NoContext),
        };

        Some(Self { node, module_id, lookup_item_id, resolver })
    }

    pub fn resolver(&self, db: &'db AnalysisDatabase) -> Resolver<'db> {
        // There is no other way to clone resolver.
        Resolver::with_data(db, self.resolver.clone_with_inference_id(db, InferenceId::NoContext))
    }
}
