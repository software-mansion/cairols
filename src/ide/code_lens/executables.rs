use crate::ide::code_lens::{
    AnnotatedNode, FileCodeLens, LSCodeLens, LSCodeLensInterface, collect_functions_with_attrs,
    get_original_node_and_file, make_lens_args,
};
use crate::lang::db::AnalysisDatabase;
use crate::lang::lsp::LsProtoGroup;
use crate::lang::lsp::ToLsp;
use crate::lsp::ext::{ExecuteInTerminal, ExecuteInTerminalParams};
use crate::project::builtin_plugins::BuiltinPlugin;
use crate::server::client::Notifier;
use crate::state::State;
use cairo_lang_defs::db::DefsGroup;
use cairo_lang_defs::ids::ModuleId;
use cairo_lang_executable::plugin::EXECUTABLE_ATTR;
use cairo_lang_syntax::node::TypedSyntaxNode;
use cairo_lang_syntax::node::helpers::QueryAttrs;
use cairo_lang_syntax::node::ids::SyntaxStablePtrId;
use cairo_lang_utils::LookupIntern;
use lsp_types::{CodeLens, Command, Position, Range, Url};

#[derive(PartialEq, Clone, Debug)]
pub struct ExecutableCodeLens {
    pub lens: CodeLens,
    command: String,
}

impl LSCodeLensInterface for ExecutableCodeLens {
    fn execute(&self, file_url: Url, state: &State, notifier: &Notifier) -> Option<()> {
        let file_path = file_url.to_file_path().ok()?;
        notifier.notify::<ExecuteInTerminal>(ExecuteInTerminalParams {
            cwd: state.project_controller.configs_registry().manifest_dir_for_file(&file_path)?,
            command: self.command.clone(),
        });
        None
    }

    fn get_lens(&self) -> CodeLens {
        self.lens.clone()
    }
}

pub fn push_executable_code_lenses(
    file_code_lens: &mut FileCodeLens,
    url: Url,
    db: &AnalysisDatabase,
) -> Option<()> {
    let file = db.file_for_url(&url)?;
    let main_module = *db.file_modules(file).ok()?.first()?;

    let crate_id = main_module.owning_crate(db);
    let has_executable_plugin = db
        .crate_macro_plugins(crate_id)
        .iter()
        .filter_map(|plugin_id| {
            BuiltinPlugin::try_from_compiler_macro_plugin(&*plugin_id.lookup_intern(db).0)
                .map(|builtin_plugin: BuiltinPlugin| builtin_plugin == BuiltinPlugin::Executable)
        })
        .next()
        .is_some();

    if !has_executable_plugin {
        return None;
    }

    push_executable_lenses(file_code_lens, db, main_module, url);
    Some(())
}

fn push_executable_lenses(
    file_code_lens: &mut FileCodeLens,
    db: &AnalysisDatabase,
    module: ModuleId,
    file_url: Url,
) {
    for AnnotatedNode { full_path, attribute_ptr } in collect_executable_functions(db, module) {
        if let Some(position) = get_executable_lens_position(db, attribute_ptr) {
            insert_executable_code_lens_pair(file_code_lens, &file_url, &full_path, position);
        }
    }

    let Ok(modules) = db.module_submodules_ids(module) else { return };
    for submodule in modules.iter().copied() {
        if db.is_submodule_inline(submodule) {
            push_executable_lenses(
                file_code_lens,
                db,
                ModuleId::Submodule(submodule),
                file_url.clone(),
            );
        }
    }
}

fn insert_executable_code_lens_pair(
    file_state: &mut FileCodeLens,
    file_url: &Url,
    function_full_path: &String,
    position: Position,
) {
    let range = Range::new(position, position);
    let exec_fn = LSCodeLens::Executable(ExecutableCodeLens {
        lens: CodeLens {
            range,
            command: Some(Command {
                title: String::from("▶ Execute function"),
                command: "cairo.executeCodeLens".to_string(),
                arguments: Some(make_lens_args(file_url.clone(), file_state.len())),
            }),
            data: None,
        },
        command: format!("scarb execute --executable-function {function_full_path}"),
    });
    file_state.push(exec_fn);
}

fn collect_executable_functions(db: &AnalysisDatabase, module: ModuleId) -> Vec<AnnotatedNode> {
    collect_functions_with_attrs(db, module, &[EXECUTABLE_ATTR])
}

fn get_executable_lens_position(
    db: &AnalysisDatabase,
    attribute_ptr: SyntaxStablePtrId,
) -> Option<Position> {
    let (original_node, original_file) = get_original_node_and_file(db, attribute_ptr)?;
    original_node.find_attr(db, EXECUTABLE_ATTR).map(|attribute| {
        attribute
            .as_syntax_node()
            .span(db)
            .start
            .position_in_file(db, original_file)
            .map(|position| position.to_lsp())
    })?
}
