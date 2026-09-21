use cairo_lang_macro::{AllocationContext, ProcMacroResult, TextSpan, TokenStream};
use salsa::Database;
use scarb_proc_macro_host::{Expansion, ExpansionId, ExpansionKind, ProcMacroBackend};
use scarb_proc_macro_server_types::methods::defined_macros::{
    CompilationUnitComponentMacros, MacroWithHash,
};
use scarb_proc_macro_server_types::methods::expand::{
    ExpandAttributeParams, ExpandDeriveParams, ExpandInlineMacroParams,
};
use scarb_proc_macro_server_types::scope::ProcMacroScope;

use crate::lang::proc_macros::db::{
    get_attribute_expansion, get_derive_expansion, get_inline_macros_expansion,
};

/// Identifies a single expansion of a single macro advertised by the proc-macro-server.
///
/// The fingerprint is the hash of the plugin package providing the macro. It is part of every
/// expansion cache key, so that a rebuilt macro invalidates the results it produced earlier.
#[derive(Clone, Debug)]
pub struct PmsMacroId {
    expansion: Expansion,
    fingerprint: u64,
}

impl ExpansionId for PmsMacroId {
    fn expansion(&self) -> &Expansion {
        &self.expansion
    }
}

/// Expands procedural macros by asking `scarb proc-macro-server`.
///
/// Expansion is asynchronous: a request the server has not answered yet resolves to a placeholder,
/// and the analysis is redone once the response lands in the database. See
/// [`crate::lang::proc_macros::db`] for the request and caching logic.
#[derive(Debug)]
pub struct PmsBackend {
    scope: ProcMacroScope,
    source_packages: Vec<String>,
    expansions: Vec<PmsMacroId>,
}

impl PmsBackend {
    pub fn new(scope: ProcMacroScope, macros: CompilationUnitComponentMacros) -> Self {
        let CompilationUnitComponentMacros {
            attributes,
            inline_macros,
            derives,
            executables,
            debug_info,
            ..
        } = macros;

        let named = |macros: Vec<MacroWithHash>, kind: ExpansionKind| {
            macros.into_iter().map(move |MacroWithHash { name, cairo_name, hash }| PmsMacroId {
                expansion: Expansion {
                    expansion_name: name.into(),
                    cairo_name: cairo_name.into(),
                    kind: kind.clone(),
                },
                fingerprint: hash,
            })
        };

        // Executable attributes only mark code for later processing and are never expanded, so
        // they need no fingerprint.
        let executables = executables.into_iter().map(|name| PmsMacroId {
            expansion: Expansion {
                expansion_name: name.clone().into(),
                cairo_name: name.into(),
                kind: ExpansionKind::Executable,
            },
            fingerprint: 0,
        });

        let expansions = named(attributes, ExpansionKind::Attr)
            .chain(named(inline_macros, ExpansionKind::Inline))
            .chain(named(derives, ExpansionKind::Derive))
            .chain(executables)
            .collect();

        Self { scope, source_packages: debug_info.source_packages, expansions }
    }

    /// Serialized ids of the Scarb packages that define these macros. Shown by the crate
    /// introspection view.
    pub fn source_packages(&self) -> &[String] {
        &self.source_packages
    }
}

impl ProcMacroBackend for PmsBackend {
    type Id = PmsMacroId;
    /// Auxiliary data is only consumed by Scarb at the end of a compilation, so the language
    /// server neither requests nor stores it.
    type AuxData = ();

    fn expansions(&self) -> Vec<PmsMacroId> {
        self.expansions.clone()
    }

    fn expand(
        &self,
        db: &dyn Database,
        id: &PmsMacroId,
        call_site: TextSpan,
        args: TokenStream,
        item: TokenStream,
    ) -> ProcMacroResult {
        let context = self.scope.clone();
        let name = id.expansion.expansion_name.to_string();
        let fingerprint = id.fingerprint;

        let result = match id.expansion.kind {
            ExpansionKind::Attr => get_attribute_expansion(
                db,
                ExpandAttributeParams {
                    context,
                    attr: name,
                    args,
                    item,
                    adapted_call_site: call_site,
                },
                fingerprint,
            ),
            ExpansionKind::Derive => get_derive_expansion(
                db,
                ExpandDeriveParams { context, derive: name, item, call_site },
                fingerprint,
            ),
            // The host hands inline macro arguments over as the item.
            ExpansionKind::Inline => get_inline_macros_expansion(
                db,
                ExpandInlineMacroParams { context, name, args: item, call_site },
                fingerprint,
            ),
            ExpansionKind::Executable => {
                unreachable!("executable attributes are not registered as expansions")
            }
        };

        // The cached result is plain data; the host works with the macro api token stream.
        let ctx = AllocationContext::default();
        ProcMacroResult {
            token_stream: result.token_stream.to_token_stream(&ctx),
            aux_data: None,
            diagnostics: result.diagnostics,
            full_path_markers: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use scarb_proc_macro_host::ExpansionQuery;
    use scarb_proc_macro_server_types::methods::defined_macros::DebugInfo;
    use scarb_proc_macro_server_types::scope::{CompilationUnitComponent, Workspace};

    use super::*;

    fn macro_with_hash(name: &str, cairo_name: &str, hash: u64) -> MacroWithHash {
        MacroWithHash { name: name.to_string(), cairo_name: cairo_name.to_string(), hash }
    }

    fn backend() -> PmsBackend {
        let scope = ProcMacroScope {
            workspace: Workspace { manifest_path: PathBuf::from("/tmp/Scarb.toml") },
            component: CompilationUnitComponent {
                name: "test_package".to_string(),
                discriminator: None,
            },
        };
        PmsBackend::new(
            scope,
            CompilationUnitComponentMacros {
                component: CompilationUnitComponent {
                    name: "test_package".to_string(),
                    discriminator: None,
                },
                attributes: vec![macro_with_hash("some_attr", "some_attr", 1)],
                inline_macros: vec![macro_with_hash("some_inline", "some_inline", 2)],
                derives: vec![macro_with_hash("some_derive", "SomeDerive", 3)],
                executables: vec!["some_executable".to_string()],
                debug_info: DebugInfo { source_packages: vec!["some_package".to_string()] },
            },
        )
    }

    #[test]
    fn derives_are_declared_under_the_cairo_name_reported_by_the_server() {
        // Cairo code writes derives as `#[derive(SomeDerive)]`. The server decides that name, so
        // the language server never has to repeat Scarb's casing rules.
        assert_eq!(backend().declared_derives(), vec!["SomeDerive".to_string()]);
    }

    #[test]
    fn attributes_and_executables_are_both_declared_as_attributes() {
        // The compiler has to know about executable attributes and the full path marker so it
        // does not report them as unknown, even though they are never expanded.
        let backend = backend();
        assert_eq!(
            backend.declared_attributes(),
            vec![
                "some_attr".to_string(),
                "some_executable".to_string(),
                scarb_proc_macro_host::FULL_PATH_MARKER_KEY.to_string(),
            ]
        );
        assert_eq!(backend.executable_attributes(), vec!["some_executable".to_string()]);
    }

    #[test]
    fn inline_macros_are_listed_for_plugin_registration() {
        let backend = backend();
        let inline = backend.inline_macros();
        assert_eq!(inline.len(), 1);
        assert_eq!(inline[0].expansion.cairo_name.as_str(), "some_inline");
        assert_eq!(inline[0].fingerprint, 2);
    }

    #[test]
    fn expansions_are_found_by_the_name_used_in_cairo_code() {
        let backend = backend();

        let found = backend
            .find_expansion(&ExpansionQuery::with_cairo_name("SomeDerive", ExpansionKind::Derive))
            .expect("derive should be found by its Cairo name");
        // The request sent to the server uses the expansion name, not the Cairo one.
        assert_eq!(found.expansion.expansion_name.as_str(), "some_derive");
        assert_eq!(found.fingerprint, 3);

        assert!(
            backend
                .find_expansion(&ExpansionQuery::with_cairo_name(
                    "some_derive",
                    ExpansionKind::Derive
                ))
                .is_none(),
            "the snake case name is not what Cairo code writes"
        );
    }

    #[test]
    fn expansions_of_a_different_kind_do_not_match() {
        let backend = backend();
        assert!(
            backend
                .find_expansion(&ExpansionQuery::with_cairo_name(
                    "some_attr",
                    ExpansionKind::Inline
                ))
                .is_none()
        );
    }

    #[test]
    fn source_packages_are_reported_for_introspection() {
        assert_eq!(backend().source_packages(), ["some_package".to_string()]);
    }
}
