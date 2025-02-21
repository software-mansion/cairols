use itertools::Itertools;
use lsp_types::{
    ClientCapabilities, GotoCapability, GotoDefinitionParams, GotoDefinitionResponse, Position,
    TextDocumentClientCapabilities, TextDocumentPositionParams, lsp_request,
};

use crate::support::cairo_project_toml::CAIRO_PROJECT_TOML_2024_07;
use crate::support::cursor::render_selections;
use crate::support::fixture::Fixture;
use crate::support::{MockClient, cursors, fixture, sandbox};

mod enums;
mod fns;
mod macros;
mod mods;
mod paths;
mod structs;
mod traits;
mod vars;

fn caps(base: ClientCapabilities) -> ClientCapabilities {
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

fn goto_definition(cairo_code: &str) -> String {
    let (cairo, cursors) = cursors(cairo_code);

    let mut test = GotoDefinitionTest::begin(fixture! {
        "cairo_project.toml" => CAIRO_PROJECT_TOML_2024_07,
        "src/lib.cairo" => cairo.clone(),
    });

    let position = cursors.caret(0);

    test.request_snapshot("src/lib.cairo", position)
}

struct GotoDefinitionTest {
    ls: MockClient,
}

impl GotoDefinitionTest {
    /// Starts goto definition testing session on a given fixture.
    fn begin(fixture: Fixture) -> Self {
        let mut ls = sandbox! {
            fixture = fixture;
            client_capabilities = caps;
        };

        ls.open_all_cairo_files_and_wait_for_project_update();

        Self { ls }
    }

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
                let path = self.ls.as_ref().url_path(&url).unwrap();
                let cairo = self.ls.as_ref().read_file(&path);
                let selections = render_selections(&cairo, &ranges);
                (path.to_string_lossy().to_string(), selections)
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
