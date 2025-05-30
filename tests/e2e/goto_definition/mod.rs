use itertools::Itertools;
use lsp_types::request::GotoDefinition;
use lsp_types::{
    ClientCapabilities, GotoCapability, GotoDefinitionParams, GotoDefinitionResponse, Position,
    TextDocumentClientCapabilities, TextDocumentPositionParams, lsp_request,
};

use crate::support::MockClient;
use crate::support::cursor::{Cursors, render_selections, render_selections_relevant_lines};
use crate::support::scarb::scarb_core_path;
use crate::support::transform::Transformer;

mod consts;
mod enums;
mod fns;
mod macros;
// FIXME: may not work bcs module
mod mods;
mod paths;
mod structs;
mod trait_impls;
mod traits;
// FIXME: may not work bcs module
mod types;
mod vars;

impl Transformer for GotoDefinition {
    fn capabilities(base: ClientCapabilities) -> ClientCapabilities {
        ClientCapabilities {
            text_document: base.text_document.or_else(Default::default).map(|it| {
                TextDocumentClientCapabilities {
                    definition: Some(GotoCapability {
                        dynamic_registration: Some(false),
                        link_support: None,
                    }),
                    ..it
                }
            }),
            ..base
        }
    }

    fn transform(ls: MockClient, cursors: Cursors) -> String {
        let position = cursors.assert_single_caret();

        let mut test = GotoDefinitionTest { ls };
        test.request_snapshot("src/lib.cairo", position)
    }
}

struct GotoDefinitionTest {
    ls: MockClient,
}

impl GotoDefinitionTest {
    /// Sends `textDocument/definition` request at given position in a given file and returns
    /// a list of target fixture file paths (relative) and rendered selections in these.
    fn request(
        &mut self,
        path: &str,
        position: Position,
    ) -> Result<Vec<(String, String)>, &'static str> {
        let code_action_params = GotoDefinitionParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: self.ls.doc_id(path),
                position,
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        };
        let response =
            self.ls.send_request::<lsp_request!("textDocument/definition")>(code_action_params);

        let locations = match response {
            Some(GotoDefinitionResponse::Scalar(location)) => {
                vec![location]
            }
            Some(GotoDefinitionResponse::Array(locations)) => locations,
            Some(GotoDefinitionResponse::Link(_)) => {
                panic!("unexpected GotoDefinitionResponse::Link");
            }
            None => {
                return Err("none response");
            }
        };

        Ok(locations
            .into_iter()
            .map(|location| (location.uri, location.range))
            .into_group_map()
            .into_iter()
            .map(|(url, ranges)| {
                let path = self
                    .ls
                    .fixture
                    .url_path(&url)
                    .unwrap_or_else(|_| url.to_file_path().unwrap())
                    .to_string_lossy()
                    .to_string();

                let cairo = self.ls.fixture.read_file(&path);

                let core_path = scarb_core_path().to_string_lossy().to_string();

                // Files from corelib do not belong to the test fixture
                // and need to be handled separately.
                if path.contains(&core_path) {
                    let item_path_relative_to_core_src = path.strip_prefix(&core_path).unwrap();
                    let item_path_relative_to_cache =
                        format!("core/src{item_path_relative_to_core_src}");

                    // Show only lines containing selections.
                    let selections = render_selections_relevant_lines(&cairo, &ranges);
                    (item_path_relative_to_cache, selections)
                } else {
                    let selections = render_selections(&cairo, &ranges);
                    (path, selections)
                }
            })
            .collect())
    }

    /// Same as [`GotoDefinitionTest::request`] but produces a report string
    /// which is useful for snapshot testing.
    fn request_snapshot(&mut self, path: &str, position: Position) -> String {
        let result = match self.request(path, position) {
            Ok(result) => result,
            Err(err) => return err.into(),
        };

        let show_header = result.len() != 1 || result[0].0 != path;

        let mut report = String::new();
        for (path, selections) in result {
            if show_header {
                report.push_str(&format!("// → {path}\n"));
            }
            report.push_str(&selections);
        }
        report
    }
}
