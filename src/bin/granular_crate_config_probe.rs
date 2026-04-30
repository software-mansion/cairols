use std::fs;
use std::path::PathBuf;
use std::time::Instant;

use cairo_language_server::AnalysisDatabase;
use cairo_lang_filesystem::db::{CrateConfigurationInput, CrateSettings};
use cairo_lang_filesystem::ids::{CrateInput, DirectoryInput};
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;
use serde::Serialize;

const DEFAULT_COUNTS: [usize; 5] = [1, 10, 100, 1000, 5000];
const DEFAULT_REPETITIONS: usize = 25;

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
        let configs = build_crate_config_map(existing_crates);
        let started = Instant::now();
        let mut configs = configs.clone();
        configs.insert(crate_input(existing_crates + 1), crate_config_input(existing_crates + 1));
        started.elapsed().as_nanos() as u64
    });

    let aggregate_update = median_of(repetitions, || {
        let configs = build_crate_config_map(existing_crates.max(1));
        let started = Instant::now();
        let mut configs = configs.clone();
        configs.insert(
            crate_input(existing_crates.max(1)),
            crate_config_input(existing_crates.max(1) + 10_000),
        );
        started.elapsed().as_nanos() as u64
    });

    let granular_set_insert = median_of(repetitions, || {
        let mut db = AnalysisDatabase::new();
        prepopulate_granular(&mut db, existing_crates);
        let started = Instant::now();
        db.set_crate_config_for_input(
            crate_input(existing_crates + 1),
            Some(crate_config_input(existing_crates + 1)),
        );
        started.elapsed().as_nanos() as u64
    });

    let granular_set_update = median_of(repetitions, || {
        let mut db = AnalysisDatabase::new();
        prepopulate_granular(&mut db, existing_crates.max(1));
        let started = Instant::now();
        db.set_crate_config_for_input(
            crate_input(existing_crates.max(1)),
            Some(crate_config_input(existing_crates.max(1) + 10_000)),
        );
        started.elapsed().as_nanos() as u64
    });

    let granular_sync_insert = median_of(repetitions, || {
        let mut db = AnalysisDatabase::new();
        prepopulate_granular(&mut db, existing_crates);
        let started = Instant::now();
        db.sync_crate_configs(build_crate_config_map(existing_crates + 1));
        started.elapsed().as_nanos() as u64
    });

    let granular_sync_update = median_of(repetitions, || {
        let mut db = AnalysisDatabase::new();
        prepopulate_granular(&mut db, existing_crates.max(1));
        let mut desired = build_crate_config_map(existing_crates.max(1));
        desired.insert(
            crate_input(existing_crates.max(1)),
            crate_config_input(existing_crates.max(1) + 10_000),
        );
        let started = Instant::now();
        db.sync_crate_configs(desired);
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

fn prepopulate_granular(db: &mut AnalysisDatabase, count: usize) {
    db.sync_crate_configs(build_crate_config_map(count));
}

fn build_crate_config_map(count: usize) -> OrderedHashMap<CrateInput, CrateConfigurationInput> {
    (0..count).map(|index| (crate_input(index), crate_config_input(index))).collect()
}

fn crate_input(index: usize) -> CrateInput {
    CrateInput::Real {
        name: format!("crate_{index}"),
        discriminator: Some(format!("disc_{index}")),
    }
}

fn crate_config_input(index: usize) -> CrateConfigurationInput {
    CrateConfigurationInput {
        root: DirectoryInput::Real(PathBuf::from(format!("/tmp/granular-crate-config-{index}"))),
        settings: CrateSettings::default(),
        cache_file: None,
    }
}

fn median_of(samples: usize, mut f: impl FnMut() -> u64) -> u64 {
    let mut values = (0..samples).map(|_| f()).collect::<Vec<_>>();
    values.sort_unstable();
    values[values.len() / 2]
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
