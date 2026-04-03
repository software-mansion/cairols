use std::fs;
use std::path::PathBuf;
use std::time::Instant;

use cairo_language_server::AnalysisDatabase;
use cairo_lang_filesystem::db::FilesGroup;
use cairo_lang_filesystem::ids::{FileInput, FileLongId};
use cairo_lang_utils::Intern;
use serde::Serialize;

const DEFAULT_COUNTS: [usize; 5] = [1, 10, 100, 1000, 5000];
const DEFAULT_REPETITIONS: usize = 25;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = std::env::args().nth(1).map(PathBuf::from);
    let results = DEFAULT_COUNTS
        .into_iter()
        .map(|existing_files| measure_for_existing_files(existing_files, DEFAULT_REPETITIONS))
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

fn measure_for_existing_files(existing_files: usize, repetitions: usize) -> ProbeResult {
    let granular_editor_insert = median_of(repetitions, || {
        let mut db = AnalysisDatabase::new();
        prepopulate_editor(&mut db, existing_files);
        let new_file = file_input(&db, existing_files + 1);
        let started = Instant::now();
        db.set_editor_file_content_for_input(new_file, Some("new".into()));
        started.elapsed().as_nanos() as u64
    });

    let granular_editor_update = median_of(repetitions, || {
        let mut db = AnalysisDatabase::new();
        prepopulate_editor(&mut db, existing_files.max(1));
        let existing_file = file_input(&db, existing_files.max(1));
        let started = Instant::now();
        db.set_editor_file_content_for_input(existing_file, Some("updated".into()));
        started.elapsed().as_nanos() as u64
    });

    let granular_generated_insert = median_of(repetitions, || {
        let mut db = AnalysisDatabase::new();
        prepopulate_generated(&mut db, existing_files);
        let new_file = file_input(&db, existing_files + 1);
        let started = Instant::now();
        db.set_generated_file_content_for_input(new_file, Some("generated".into()));
        started.elapsed().as_nanos() as u64
    });

    let granular_generated_update = median_of(repetitions, || {
        let mut db = AnalysisDatabase::new();
        prepopulate_generated(&mut db, existing_files.max(1));
        let existing_file = file_input(&db, existing_files.max(1));
        let started = Instant::now();
        db.set_generated_file_content_for_input(existing_file, Some("generated-updated".into()));
        started.elapsed().as_nanos() as u64
    });

    ProbeResult {
        existing_files,
        granular_editor_insert_ns: granular_editor_insert,
        granular_editor_update_ns: granular_editor_update,
        granular_generated_insert_ns: granular_generated_insert,
        granular_generated_update_ns: granular_generated_update,
    }
}

fn prepopulate_editor(db: &mut AnalysisDatabase, count: usize) {
    for index in 0..count {
        db.set_editor_file_content_for_input(file_input(db, index), Some(format!("content-{index}").into()));
    }
}

fn prepopulate_generated(db: &mut AnalysisDatabase, count: usize) {
    for index in 0..count {
        db.set_generated_file_content_for_input(
            file_input(db, index),
            Some(format!("content-{index}").into()),
        );
    }
}

fn file_input(db: &AnalysisDatabase, index: usize) -> FileInput {
    let file_id = FileLongId::OnDisk(PathBuf::from(format!("/tmp/granular-probe-{index}.cairo")))
        .intern(db);
    db.file_input(file_id).clone()
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
    existing_files: usize,
    granular_editor_insert_ns: u64,
    granular_editor_update_ns: u64,
    granular_generated_insert_ns: u64,
    granular_generated_update_ns: u64,
}
