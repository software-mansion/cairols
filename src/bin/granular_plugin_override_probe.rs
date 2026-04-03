use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use cairo_language_server::AnalysisDatabase;
use cairo_lang_defs::db::{defs_group_input, init_defs_group};
use cairo_lang_defs::ids::{InlineMacroExprPluginLongId, MacroPluginLongId};
use cairo_lang_executable_plugin::executable_plugin_suite;
use cairo_lang_plugins::plugins::ConfigPlugin;
use cairo_lang_semantic::db::{init_semantic_group, semantic_group_input};
use cairo_lang_semantic::ids::AnalyzerPluginLongId;
use cairo_lang_semantic::inline_macros::get_default_plugin_suite;
use cairo_lang_semantic::plugin::PluginSuite;
use cairo_lang_starknet::starknet_plugin_suite;
use cairo_lang_test_plugin::test_plugin_suite;
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;
use cairo_lint::plugin::cairo_lint_allow_plugin_suite;
use itertools::Itertools;
use serde::Serialize;
use salsa::Setter;

const DEFAULT_COUNTS: [usize; 5] = [1, 10, 100, 1000, 5000];
const DEFAULT_REPETITIONS: usize = 25;

#[salsa::db]
#[derive(Clone, Default)]
struct AggregatePluginDatabase {
    storage: salsa::Storage<Self>,
}

#[salsa::db]
impl salsa::Database for AggregatePluginDatabase {}

impl AggregatePluginDatabase {
    fn new() -> Self {
        let mut db = Self::default();
        init_defs_group(&mut db);
        init_semantic_group(&mut db);
        db
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = std::env::args().nth(1).map(PathBuf::from);
    let results = DEFAULT_COUNTS
        .into_iter()
        .map(|existing_crates| measure_for_existing_crates(existing_crates, DEFAULT_REPETITIONS))
        .collect::<Vec<_>>();

    let json = serde_json::to_string_pretty(&ProbeReport {
        repetitions: DEFAULT_REPETITIONS,
        results,
    })?;

    if let Some(path) = output {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, json)?;
    } else {
        println!("{json}");
    }

    Ok(())
}

fn measure_for_existing_crates(existing_crates: usize, repetitions: usize) -> ProbeResult {
    let aggregate_insert = median_of(repetitions, || {
        let mut db = AggregatePluginDatabase::new();
        prepopulate_aggregate(&mut db, existing_crates);
        let started = Instant::now();
        let mut macro_overrides = defs_group_input(&db).macro_plugin_overrides(&db).clone().unwrap();
        let mut inline_overrides =
            defs_group_input(&db).inline_macro_plugin_overrides(&db).clone().unwrap();
        let mut analyzer_overrides =
            semantic_group_input(&db).analyzer_plugin_overrides(&db).clone().unwrap();
        let suite = plugin_suite_inputs(existing_crates + 1);
        macro_overrides.insert(crate_input(existing_crates + 1), suite.macro_plugins);
        inline_overrides.insert(crate_input(existing_crates + 1), suite.inline_macro_plugins);
        analyzer_overrides.insert(crate_input(existing_crates + 1), suite.analyzer_plugins);
        defs_group_input(&db).set_macro_plugin_overrides(&mut db).to(Some(macro_overrides));
        defs_group_input(&db).set_inline_macro_plugin_overrides(&mut db).to(Some(inline_overrides));
        semantic_group_input(&db).set_analyzer_plugin_overrides(&mut db).to(Some(analyzer_overrides));
        started.elapsed().as_nanos() as u64
    });

    let aggregate_update = median_of(repetitions, || {
        let mut db = AggregatePluginDatabase::new();
        prepopulate_aggregate(&mut db, existing_crates.max(1));
        let started = Instant::now();
        let mut macro_overrides = defs_group_input(&db).macro_plugin_overrides(&db).clone().unwrap();
        let mut inline_overrides =
            defs_group_input(&db).inline_macro_plugin_overrides(&db).clone().unwrap();
        let mut analyzer_overrides =
            semantic_group_input(&db).analyzer_plugin_overrides(&db).clone().unwrap();
        let suite = plugin_suite_inputs(existing_crates.max(1) + 10_000);
        macro_overrides.insert(crate_input(existing_crates.max(1)), suite.macro_plugins);
        inline_overrides.insert(crate_input(existing_crates.max(1)), suite.inline_macro_plugins);
        analyzer_overrides.insert(crate_input(existing_crates.max(1)), suite.analyzer_plugins);
        defs_group_input(&db).set_macro_plugin_overrides(&mut db).to(Some(macro_overrides));
        defs_group_input(&db).set_inline_macro_plugin_overrides(&mut db).to(Some(inline_overrides));
        semantic_group_input(&db).set_analyzer_plugin_overrides(&mut db).to(Some(analyzer_overrides));
        started.elapsed().as_nanos() as u64
    });

    let granular_set_insert = median_of(repetitions, || {
        let mut db = AnalysisDatabase::new();
        prepopulate_granular(&mut db, existing_crates);
        let started = Instant::now();
        db.set_override_crate_plugins_from_suite(
            crate_input(existing_crates + 1),
            plugin_suite(existing_crates + 1),
        );
        started.elapsed().as_nanos() as u64
    });

    let granular_set_update = median_of(repetitions, || {
        let mut db = AnalysisDatabase::new();
        prepopulate_granular(&mut db, existing_crates.max(1));
        let started = Instant::now();
        db.set_override_crate_plugins_from_suite(
            crate_input(existing_crates.max(1)),
            plugin_suite(existing_crates.max(1) + 10_000),
        );
        started.elapsed().as_nanos() as u64
    });

    let granular_sync_insert = median_of(repetitions, || {
        let mut db = AnalysisDatabase::new();
        prepopulate_granular(&mut db, existing_crates);
        let started = Instant::now();
        db.sync_granular_crate_plugin_suites(build_suite_map(existing_crates + 1));
        started.elapsed().as_nanos() as u64
    });

    let granular_sync_update = median_of(repetitions, || {
        let mut db = AnalysisDatabase::new();
        prepopulate_granular(&mut db, existing_crates.max(1));
        let mut desired = build_suite_map(existing_crates.max(1));
        desired.insert(
            crate_input(existing_crates.max(1)),
            plugin_suite(existing_crates.max(1) + 10_000),
        );
        let started = Instant::now();
        db.sync_granular_crate_plugin_suites(desired);
        started.elapsed().as_nanos() as u64
    });

    ProbeResult {
        existing_crates,
        aggregate_insert_ns: aggregate_insert,
        aggregate_update_ns: aggregate_update,
        granular_set_insert_ns: granular_set_insert,
        granular_set_update_ns: granular_set_update,
        granular_sync_insert_ns: granular_sync_insert,
        granular_sync_update_ns: granular_sync_update,
    }
}

fn prepopulate_aggregate(db: &mut AggregatePluginDatabase, count: usize) {
    defs_group_input(db).set_macro_plugin_overrides(db).to(Some(
        (0..count)
            .map(|index| {
                let suite = plugin_suite_inputs(index);
                (crate_input(index), suite.macro_plugins)
            })
            .collect(),
    ));
    defs_group_input(db).set_inline_macro_plugin_overrides(db).to(Some(
        (0..count)
            .map(|index| {
                let suite = plugin_suite_inputs(index);
                (crate_input(index), suite.inline_macro_plugins)
            })
            .collect(),
    ));
    semantic_group_input(db).set_analyzer_plugin_overrides(db).to(Some(
        (0..count)
            .map(|index| {
                let suite = plugin_suite_inputs(index);
                (crate_input(index), suite.analyzer_plugins)
            })
            .collect(),
    ));
}

fn prepopulate_granular(db: &mut AnalysisDatabase, count: usize) {
    db.sync_granular_crate_plugin_suites(build_suite_map(count));
}

fn build_suite_map(count: usize) -> OrderedHashMap<cairo_lang_filesystem::ids::CrateInput, PluginSuite> {
    (0..count).map(|index| (crate_input(index), plugin_suite(index))).collect()
}

fn crate_input(index: usize) -> cairo_lang_filesystem::ids::CrateInput {
    cairo_lang_filesystem::ids::CrateInput::Real {
        name: format!("crate_{index}"),
        discriminator: Some(format!("disc_{index}")),
    }
}

fn plugin_suite(index: usize) -> PluginSuite {
    let mut suite = PluginSuite {
        plugins: vec![Arc::new(ConfigPlugin::default())],
        ..Default::default()
    };
    suite.add(get_default_plugin_suite());
    suite.add(test_plugin_suite());
    suite.add(executable_plugin_suite());
    suite.add(cairo_lint_allow_plugin_suite());
    if index % 2 == 0 {
        suite.add(starknet_plugin_suite());
    }
    suite
}

fn plugin_suite_inputs(index: usize) -> PluginSuiteInputs {
    let suite = plugin_suite(index);
    PluginSuiteInputs {
        macro_plugins: Arc::from(
            suite.plugins.into_iter().map(MacroPluginLongId).collect_vec(),
        ),
        analyzer_plugins: Arc::from(
            suite.analyzer_plugins.into_iter().map(AnalyzerPluginLongId).collect_vec(),
        ),
        inline_macro_plugins: Arc::new(
            suite
                .inline_macro_plugins
                .into_iter()
                .map(|(name, plugin)| (name, InlineMacroExprPluginLongId(plugin)))
                .collect(),
        ),
    }
}

fn median_of(samples: usize, mut f: impl FnMut() -> u64) -> u64 {
    let mut values = (0..samples).map(|_| f()).collect::<Vec<_>>();
    values.sort_unstable();
    values[values.len() / 2]
}

#[derive(Debug)]
struct PluginSuiteInputs {
    macro_plugins: Arc<[MacroPluginLongId]>,
    analyzer_plugins: Arc<[AnalyzerPluginLongId]>,
    inline_macro_plugins: Arc<OrderedHashMap<String, InlineMacroExprPluginLongId>>,
}

#[derive(Debug, Serialize)]
struct ProbeReport {
    repetitions: usize,
    results: Vec<ProbeResult>,
}

#[derive(Debug, Serialize)]
struct ProbeResult {
    existing_crates: usize,
    aggregate_insert_ns: u64,
    aggregate_update_ns: u64,
    granular_set_insert_ns: u64,
    granular_set_update_ns: u64,
    granular_sync_insert_ns: u64,
    granular_sync_update_ns: u64,
}
