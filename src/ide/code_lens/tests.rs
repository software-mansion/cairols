use std::ops::Not;

use cairo_lang_defs::db::DefsGroup;
use cairo_lang_defs::ids::FreeFunctionLongId;
use cairo_lang_defs::ids::ModuleId;
use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_defs::ids::SubmoduleLongId;
use cairo_lang_defs::ids::TopLevelLanguageElementId;
use cairo_lang_defs::plugin::MacroPlugin;
use cairo_lang_filesystem::db::FilesGroup;
use cairo_lang_filesystem::db::get_originating_location;
use cairo_lang_filesystem::ids::CrateId;
use cairo_lang_filesystem::ids::SpanInFile;
use cairo_lang_filesystem::span::TextPositionSpan;
use cairo_lang_syntax::node::ids::SyntaxStablePtrId;
use cairo_lang_syntax::node::{TypedStablePtr, TypedSyntaxNode, ast::ModuleItem};
use cairo_lang_test_plugin::TestPlugin;
use cairo_lang_utils::Intern;
use cairo_language_common::CommonGroup;
use lsp_types::{CodeLens, Command, Range, Url};

use super::{
    AnnotatedNode, CodeLensBuilder, CodeLensInterface, CodeLensProvider, LSCodeLens,
    collect_functions_with_attrs, make_lens_args, send_execute_in_terminal,
};
use crate::config::{Config, TestRunner};
use crate::lang::db::AnalysisDatabase;
use crate::lang::db::LsSemanticGroup;
use crate::lang::db::LsSyntaxGroup;
use crate::lang::lsp::ToLsp;
use crate::lang::lsp::{LsProtoGroup, ToCairo};
use crate::server::client::Notifier;
use crate::state::State;

#[derive(PartialEq, Clone, Debug)]
pub struct TestCodeLens {
    lens: CodeLens,
    full_path: String,
}

impl CodeLensInterface for TestCodeLens {
    fn execute(&self, file_url: Url, state: &State, notifier: &Notifier) -> Option<()> {
        let db = &state.db;

        let file = db.file_for_url(&file_url)?;
        let file_path = file_url.to_file_path().ok()?;
        let span = TextPositionSpan::offset_in_file(self.lens.range.to_cairo(), db, file)?;
        // When creating range we have to use [`SyntaxNode::span_without_trivia`] to place lens next to attribute
        let node = db.widest_node_within_span_without_trivia(file, span)?;

        let (full_qualified_path, module_id) =
            db.get_node_resultants(node).as_ref()?.iter().find_map(|resultant| {
                let module_item = resultant
                    .ancestors_with_self(db)
                    .find_map(|node| ModuleItem::cast(db, node))?;
                let module_id = db.find_module_containing_node(module_item.as_syntax_node())?;
                let path = TestFullQualifiedPath::new(db, module_item, module_id)?;

                // Find resultant that produced this codelens earlier by comparing full paths
                (sanitize_test_case_name(path.as_ref()) == self.full_path)
                    .then_some((path, module_id))
            })?;

        let command = state.config.test_runner.command(
            full_qualified_path,
            AvailableTestRunners::new(db, module_id.owning_crate(db))?,
            &state.config.run_test_command,
        )?;
        let cwd = state.project_controller.configs_registry().manifest_dir_for_file(&file_path)?;
        send_execute_in_terminal(state, notifier, command, cwd);
        Some(())
    }

    fn lens(&self) -> CodeLens {
        self.lens.clone()
    }
}

pub struct TestCodeLensBuilder {
    full_path: String,
    title: String,
    range: Range,
    file_url: Url,
}

impl TestCodeLensBuilder {
    fn new(range: Range, full_path: String, file_url: Url, is_plural: bool) -> Self {
        let mut title = "▶ Run test".to_string();

        if is_plural {
            title.push('s');
        }

        Self { full_path: sanitize_test_case_name(&full_path), title, file_url, range }
    }
}

impl CodeLensBuilder for TestCodeLensBuilder {
    fn build_lens(self, index: usize) -> LSCodeLens {
        let command = Command {
            title: self.title,
            command: "cairo.executeCodeLens".to_string(),
            arguments: Some(make_lens_args(self.file_url.clone(), index)),
        };

        LSCodeLens::Test(TestCodeLens {
            lens: CodeLens { range: self.range, command: Some(command), data: None },
            full_path: self.full_path,
        })
    }
}

pub struct TestCodeLensProvider;
impl CodeLensProvider for TestCodeLensProvider {
    type ConstructionParams<'a> = TestCodeLensConstructionParams<'a>;
    type LensBuilder = TestCodeLensBuilder;

    fn create_lens(params: Self::ConstructionParams<'_>) -> Vec<Self::LensBuilder> {
        get_test_code_lenses_builders(params.url, params.db, &params.config).unwrap_or_default()
    }
}

pub struct TestCodeLensConstructionParams<'a> {
    pub url: Url,
    pub db: &'a AnalysisDatabase,
    pub config: Config,
}

pub fn get_test_code_lenses_builders(
    url: Url,
    db: &AnalysisDatabase,
    config: &Config,
) -> Option<Vec<TestCodeLensBuilder>> {
    let mut file_code_lens = vec![];
    let file = db.file_for_url(&url)?;

    let main_module = *db.file_modules(file).ok()?.first()?;

    let is_runner_available = config
        .test_runner
        .command(
            TestFullQualifiedPath::Function(String::new()), // We can substitute with anything here.
            AvailableTestRunners::new(db, main_module.owning_crate(db))?,
            &config.run_test_command,
        )
        .is_some();

    if is_runner_available {
        collect_test_lenses(&mut file_code_lens, db, main_module, url);
    }

    Some(file_code_lens)
}

enum TestFullQualifiedPath {
    Function(String),
    Module(String),
}

impl TestFullQualifiedPath {
    fn cairo_test_command(&self) -> String {
        format!("scarb cairo-test --filter {}", self.as_ref())
    }

    fn snforge_command(&self) -> String {
        match self {
            TestFullQualifiedPath::Function(path) => {
                format!("snforge test {path} --exact", path = sanitize_test_case_name(path))
            }
            TestFullQualifiedPath::Module(path) => {
                format!("snforge test {path}", path = sanitize_test_case_name(path))
            }
        }
    }
}

impl AsRef<str> for TestFullQualifiedPath {
    fn as_ref(&self) -> &str {
        match self {
            TestFullQualifiedPath::Function(path) | TestFullQualifiedPath::Module(path) => path,
        }
    }
}

impl TestFullQualifiedPath {
    pub fn new(
        db: &AnalysisDatabase,
        module_item: ModuleItem,
        module_id: ModuleId,
    ) -> Option<Self> {
        match module_item {
            ModuleItem::FreeFunction(function_with_body) => {
                let path = ModuleItemId::FreeFunction(
                    FreeFunctionLongId(module_id, function_with_body.stable_ptr(db)).intern(db),
                )
                .full_path(db);

                Some(TestFullQualifiedPath::Function(path))
            }
            ModuleItem::Module(item_module) => {
                let path = ModuleItemId::Submodule(
                    SubmoduleLongId(module_id, item_module.stable_ptr(db)).intern(db),
                )
                .full_path(db);

                Some(TestFullQualifiedPath::Module(path))
            }
            _ => None,
        }
    }
}

struct AvailableTestRunners {
    cairo_test: bool,
    snforge: bool,
}

impl AvailableTestRunners {
    pub fn new<'db>(db: &'db AnalysisDatabase, crate_id: CrateId<'db>) -> Option<Self> {
        let cairo_test = db.crate_macro_plugins(crate_id).iter().any(|plugin_id| {
            plugin_id.long(db).plugin_type_id() == TestPlugin::default().plugin_type_id()
        });

        // This will not work with crate renames in `Scarb.toml`, but there is no better way to do this now.
        let snforge = db.crate_config(crate_id)?.settings.dependencies.contains_key("snforge_std");

        Some(Self { cairo_test, snforge })
    }
}

impl TestRunner {
    fn command(
        &self,
        test_path: TestFullQualifiedPath,
        available_runners: AvailableTestRunners,
        custom_command: &str,
    ) -> Option<String> {
        match self {
            Self::Auto => match (available_runners.cairo_test, available_runners.snforge) {
                (true, false) => Some(test_path.cairo_test_command()),
                (false, true) => Some(test_path.snforge_command()),
                _ => None,
            },
            Self::CairoTest if available_runners.cairo_test => Some(test_path.cairo_test_command()),
            Self::Snforge if available_runners.snforge => Some(test_path.snforge_command()),
            Self::Custom => Some(custom_command.replace("{{TEST_PATH}}", test_path.as_ref())),
            _ => None,
        }
    }
}

const TEST_EXECUTABLES: [&str; 2] = ["test", "snforge_internal_test_executable"];
fn collect_test_functions<'db>(
    db: &'db AnalysisDatabase,
    module: ModuleId<'db>,
) -> Vec<AnnotatedNode<'db>> {
    collect_functions_with_attrs(db, module, &TEST_EXECUTABLES)
}

fn collect_test_lenses<'db>(
    file_code_lens: &mut Vec<TestCodeLensBuilder>,
    db: &'db AnalysisDatabase,
    module: ModuleId<'db>,
    file_url: Url,
) {
    for node in collect_test_functions(db, module) {
        maybe_push_code_lens(
            db,
            file_code_lens,
            |range, full_path| TestCodeLensBuilder::new(range, full_path, file_url.clone(), false),
            node,
        );
    }

    let Ok(modules) = db.module_submodules_ids(module) else { return };

    for submodule in modules.iter().copied() {
        let is_inline = db.is_submodule_inline(submodule);

        let has_tests = if is_inline {
            let tests_count = file_code_lens.len();

            collect_test_lenses(
                file_code_lens,
                db,
                ModuleId::Submodule(submodule),
                file_url.clone(),
            );

            // Append mod only if it contains tests.
            tests_count != file_code_lens.len()
        } else {
            has_any_test(db, ModuleId::Submodule(submodule))
        };

        if has_tests {
            let ptr = submodule.stable_ptr(db).lookup(db).module_kw(db).stable_ptr(db).untyped();

            let full_path = submodule.full_path(db);
            let module_node = AnnotatedNode { full_path: full_path.clone(), attribute_ptr: ptr };
            maybe_push_code_lens(
                db,
                file_code_lens,
                |range, full_path| {
                    TestCodeLensBuilder::new(range, full_path, file_url.clone(), true)
                },
                module_node,
            );
        }
    }
}

fn has_any_test<'db>(db: &'db AnalysisDatabase, module: ModuleId<'db>) -> bool {
    if collect_test_functions(db, module).is_empty().not() {
        return true;
    }

    let Ok(modules) = db.module_submodules_ids(module) else { return false };

    modules.iter().copied().map(ModuleId::Submodule).any(|submodule| {
        collect_test_functions(db, submodule).is_empty().not() || has_any_test(db, submodule)
    })
}

fn get_test_lens_range<'db>(
    db: &'db AnalysisDatabase,
    ptr: SyntaxStablePtrId<'db>,
) -> Option<Range> {
    // Use [`SyntaxNode::span_without_trivia`] to place lens next to attribute
    let SpanInFile { file_id, span } = get_originating_location(
        db,
        SpanInFile { file_id: ptr.file_id(db), span: ptr.lookup(db).span_without_trivia(db) },
        None,
    );

    span.position_in_file(db, file_id).map(|position| position.to_lsp())
}

fn maybe_push_code_lens(
    db: &AnalysisDatabase,
    file_state: &mut Vec<TestCodeLensBuilder>,
    make_code_lens: impl FnOnce(Range, String) -> TestCodeLensBuilder,
    annotated_function: AnnotatedNode,
) {
    let AnnotatedNode { attribute_ptr, full_path } = annotated_function;
    if let Some(range) = get_test_lens_range(db, attribute_ptr) {
        let lens_builder = make_code_lens(range, full_path);

        file_state.push(lens_builder)
    }
}

// Copied from starknet-foundry
fn sanitize_test_case_name(name: &str) -> String {
    // Test names generated by `#[test]` and `#[fuzzer]` macros contain internal suffixes
    name.replace("__test_generated", "")
        .replace("__fuzzer_generated", "")
        .replace("__snforge_internal_test_generated", "")
}
