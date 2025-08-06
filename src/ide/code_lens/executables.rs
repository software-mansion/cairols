use crate::ide::code_lens::{
    AnnotatedNode, CodeLensBuilder, CodeLensInterface, CodeLensProvider, LSCodeLens,
    collect_functions_with_attrs, get_original_module_item_and_file, make_lens_args,
    send_execute_in_terminal,
};
use crate::lang::db::AnalysisDatabase;
use crate::lang::lsp::LsProtoGroup;
use crate::lang::lsp::ToLsp;
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
    lens: CodeLens,
    command: String,
}

pub struct ExecutableCodeLensConstructionParams<'a> {
    pub db: &'a AnalysisDatabase,
    pub url: Url,
}

pub struct ExecutableCodeLensProvider;
impl CodeLensProvider for ExecutableCodeLensProvider {
    type ConstructionParams<'a> = ExecutableCodeLensConstructionParams<'a>;
    type LensBuilder = ExecutableLensBuilder;

    fn create_lens(params: Self::ConstructionParams<'_>) -> Vec<Self::LensBuilder> {
        get_executable_code_lenses_builders(params.url, params.db).unwrap_or_default()
    }
}

pub struct ExecutableLensBuilder {
    position: Position,
    file_url: Url,
    command: String,
}

impl CodeLensBuilder for ExecutableLensBuilder {
    fn build_lens(self, index: usize) -> LSCodeLens {
        let range = Range::new(self.position, self.position);
        LSCodeLens::Executable(ExecutableCodeLens {
            lens: CodeLens {
                range,
                command: Some(Command {
                    title: String::from("▶ Execute function"),
                    command: "cairo.executeCodeLens".to_string(),
                    arguments: Some(make_lens_args(self.file_url.clone(), index)),
                }),
                data: None,
            },
            command: self.command,
        })
    }
}

impl CodeLensInterface for ExecutableCodeLens {
    fn execute(&self, file_url: Url, state: &State, notifier: &Notifier) -> Option<()> {
        let file_path = file_url.to_file_path().ok()?;
        let cwd = state.project_controller.configs_registry().manifest_dir_for_file(&file_path)?;
        send_execute_in_terminal(state, notifier, self.command.clone(), cwd);
        None
    }

    fn lens(&self) -> CodeLens {
        self.lens.clone()
    }
}

pub fn get_executable_code_lenses_builders(
    url: Url,
    db: &AnalysisDatabase,
) -> Option<Vec<ExecutableLensBuilder>> {
    let mut file_code_lenses_builders = vec![];
    let file = db.file_for_url(&url)?;
    let main_module = *db.file_modules(file).ok()?.first()?;

    let crate_id = main_module.owning_crate(db);
    let has_executable_plugin = db
        .crate_macro_plugins(crate_id)
        .iter()
        .filter_map(|plugin_id| {
            BuiltinPlugin::try_from_compiler_macro_plugin(&*plugin_id.lookup_intern(db).0)
        })
        .any(|builtin_plugin: BuiltinPlugin| builtin_plugin == BuiltinPlugin::Executable);

    if !has_executable_plugin {
        return None;
    }

    get_executable_lenses_builders_in_mod(&mut file_code_lenses_builders, db, main_module, url);
    Some(file_code_lenses_builders)
}

fn get_executable_lenses_builders_in_mod(
    file_code_lenses_builders: &mut Vec<ExecutableLensBuilder>,
    db: &AnalysisDatabase,
    module: ModuleId,
    file_url: Url,
) {
    for AnnotatedNode { full_path, attribute_ptr } in collect_executable_functions(db, module) {
        if let Some(position) = get_executable_lens_position(db, attribute_ptr) {
            file_code_lenses_builders.push(ExecutableLensBuilder {
                position,
                file_url: file_url.clone(),
                command: format!("scarb execute --executable-function {full_path}"),
            });
        }
    }

    let Ok(modules) = db.module_submodules_ids(module) else { return };
    for submodule in modules.iter().copied() {
        if db.is_submodule_inline(submodule) {
            get_executable_lenses_builders_in_mod(
                file_code_lenses_builders,
                db,
                ModuleId::Submodule(submodule),
                file_url.clone(),
            );
        }
    }
}

fn collect_executable_functions(db: &AnalysisDatabase, module: ModuleId) -> Vec<AnnotatedNode> {
    collect_functions_with_attrs(db, module, &[EXECUTABLE_ATTR])
}

fn get_executable_lens_position(
    db: &AnalysisDatabase,
    attribute_ptr: SyntaxStablePtrId,
) -> Option<Position> {
    let (original_node, original_file) = get_original_module_item_and_file(db, attribute_ptr)?;
    original_node.find_attr(db, EXECUTABLE_ATTR).map(|attribute| {
        attribute
            .as_syntax_node()
            .span(db)
            .start
            .position_in_file(db, original_file)
            .map(|position| position.to_lsp())
    })?
}
