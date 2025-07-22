use crate::config::Config;
use crate::ide::code_lens::{
    AnnotatedNode, CodeLensProvider, FileCodeLens, LSCodeLens, LensOwner, PLAIN_EXECUTABLES,
    collect_functions_with_attrs, get_original_node_and_file,
};
use crate::lang::db::AnalysisDatabase;
use crate::lang::lsp::LsProtoGroup;
use crate::lang::lsp::ToLsp;
use crate::lsp::ext::{ExecuteInTerminal, ExecuteInTerminalParams};
use crate::server::client::Notifier;
use crate::state::State;
use cairo_lang_defs::db::DefsGroup;
use cairo_lang_defs::ids::ModuleId;
use cairo_lang_executable::plugin::EXECUTABLE_ATTR;
use cairo_lang_syntax::node::TypedSyntaxNode;
use cairo_lang_syntax::node::helpers::QueryAttrs;
use cairo_lang_syntax::node::ids::SyntaxStablePtrId;
use lsp_types::{CodeLens, Command, Position, Range, Url};
use scarb_metadata::CompilationUnitMetadata;
use serde_json::Value;
use std::collections::HashMap;

#[derive(PartialEq)]
struct ExecutableLensPayload {
    command: String,
}

#[derive(PartialEq)]
pub struct ExecutableCodeLens {
    lens: CodeLens,
    payload: ExecutableLensPayload,
}

impl LensOwner for ExecutableCodeLens {
    fn get_lens(&self) -> CodeLens {
        self.lens.clone()
    }
}

pub struct ExecutableCodeLensProvider;

impl CodeLensProvider for ExecutableCodeLensProvider {
    type LensOwner = ExecutableCodeLens;
    fn calculate_code_lens(
        &self,
        url: Url,
        db: &AnalysisDatabase,
        _config: &Config,
        compilation_units: Vec<CompilationUnitMetadata>,
    ) -> Option<FileCodeLens> {
        let file = db.file_for_url(&url)?;
        let main_module = *db.file_modules(file).ok()?.first()?;

        let mut result = HashMap::new();
        collect_executable_lenses(db, main_module, url, &mut result, compilation_units);

        Some(result)
    }

    fn execute_code_lens(
        &self,
        state: &State,
        notifier: Notifier,
        url: Url,
        code_lens: &ExecutableCodeLens,
    ) -> Option<()> {
        let file_path = url.to_file_path().ok()?;
        notifier.notify::<ExecuteInTerminal>(ExecuteInTerminalParams {
            cwd: state.project_controller.configs_registry().manifest_dir_for_file(&file_path)?,
            command: code_lens.payload.command.clone(),
        });
        None
    }
}

fn collect_executable_lenses(
    db: &AnalysisDatabase,
    module: ModuleId,
    file_url: Url,
    file_state: &mut FileCodeLens,
    compilation_units: Vec<CompilationUnitMetadata>,
) {
    for AnnotatedNode { full_path, attribute_ptr } in collect_executable_functions(db, module) {
        let Some(command) = get_scarb_execute_command_for(&full_path, compilation_units.clone())
        else {
            continue;
        };

        if let Some(position) = get_executable_lens_position(db, attribute_ptr) {
            let code_lens = make_executable_code_lens(&file_url, &full_path, position);
            file_state.insert(
                full_path,
                LSCodeLens::Executable(ExecutableCodeLens {
                    lens: code_lens,
                    payload: ExecutableLensPayload { command },
                }),
            );
        }
    }
}

fn get_scarb_execute_command_for(
    function_full_path: &String,
    compilation_units: Vec<CompilationUnitMetadata>,
) -> Option<String> {
    if precise_cu_available(function_full_path, compilation_units.clone()) {
        return Some(format!("scarb execute --executable-function {function_full_path}"));
    }

    if let Some(executable_crate_name) = get_executable_crate_name(compilation_units.clone()) {
        return Some(format!("scarb execute -p {executable_crate_name}"));
    }

    None
}

fn get_executable_crate_name(compilation_units: Vec<CompilationUnitMetadata>) -> Option<String> {
    compilation_units.iter().find_map(|cu| {
        if cu.target.kind == "executable" {
            let params_obj = cu.target.params.as_object()?;
            if params_obj.get("function").is_some() {
                return None;
            };
            Some(cu.target.name.clone())
        } else {
            None
        }
    })
}

fn precise_cu_available(
    function_full_path: &String,
    compilation_units: Vec<CompilationUnitMetadata>,
) -> bool {
    compilation_units.iter().any(|cu| {
        if cu.target.kind == "executable" {
            let Some(params_obj) = cu.target.params.as_object() else { return false };
            let Some(func) = params_obj.get("function") else { return false };
            let Some(func_name) = func.as_str() else { return false };
            func_name == function_full_path
        } else {
            false
        }
    })
}

fn make_executable_code_lens(
    file_url: &Url,
    function_full_path: &String,
    position: Position,
) -> CodeLens {
    let command = Command {
        title: String::from("▶ Run"),
        command: "cairo.executeCodeLens".to_string(),
        arguments: Some(vec![
            Value::String(String::from(function_full_path)),
            Value::String(file_url.to_string()),
        ]),
    };
    let range = Range::new(position, position);
    CodeLens { range, command: Some(command), data: None }
}

fn collect_executable_functions(db: &AnalysisDatabase, module: ModuleId) -> Vec<AnnotatedNode> {
    collect_functions_with_attrs(db, module, Vec::from(PLAIN_EXECUTABLES))
}

fn get_executable_lens_position(
    db: &AnalysisDatabase,
    function_ptr: SyntaxStablePtrId,
) -> Option<Position> {
    let (original_node, original_file) = get_original_node_and_file(db, function_ptr)?;
    original_node
        .find_attr(db, EXECUTABLE_ATTR)
        .map(|attr| attr.as_syntax_node().span_start_without_trivia(db))
        .map(|span| span.position_in_file(db, original_file))?
        .map(|position| position.to_lsp())
}
