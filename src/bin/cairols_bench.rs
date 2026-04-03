use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{self, Command};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use cairo_language_server::lsp::ext::ShowMemoryUsageResponse;
use cairo_language_server::testing::benchmark::BenchmarkClient;
use cairo_lang_parser::db::FILE_SYNTAX_DATA_LRU_ENV;
use cairo_lang_semantic::expr::inference::solver::CANONIC_TRAIT_SOLUTIONS_LRU_ENV;
use cairo_lang_semantic::items::free_function::FREE_FUNCTION_BODY_LRU_ENV;
use cairo_lang_semantic::items::imp::IMPL_FUNCTION_BODY_LRU_ENV;
use lsp_types::Position;
use serde::Serialize;

const DEFAULT_PROJECTS: [&str; 2] = [
    "/Users/jsmolka/Work/cairo-projects/ecosystem/alexandria",
    "/Users/jsmolka/Work/cairo-projects/ecosystem/open-zeppelin",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if let Some(target) = ChildTarget::from_args(&args)? {
        let config = Config::from_args(args)?;
        let run = run_target(&config, target.into_benchmark_target())?;
        println!("{}", serde_json::to_string(&run)?);
        return Ok(());
    }

    let config = Config::from_args(args)?;
    let started_at = unix_timestamp();
    let runs = run_targets_in_subprocesses(&config)?;

    let report = BenchmarkReport {
        started_at,
        generated_at: unix_timestamp(),
        config,
        runs,
    };
    let report_json = serde_json::to_string_pretty(&report)?;

    if let Some(output) = &report.config.output {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(output, report_json)?;
    } else {
        println!("{report_json}");
    }

    Ok(())
}

fn run_targets_in_subprocesses(
    config: &Config,
) -> Result<Vec<ProjectBenchmarkRun>, Box<dyn std::error::Error>> {
    let mut runs = Vec::new();

    for project_root in &config.project_roots {
        for target in benchmark_targets(project_root, config.package_manifest_count)? {
            let output = Command::new(std::env::current_exe()?)
                .args(config.child_args_for(&target))
                .output()?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(format!(
                    "benchmark subprocess failed for {}:\n{}",
                    target.manifest.display(),
                    stderr.trim()
                )
                .into());
            }

            let run = serde_json::from_slice::<ProjectBenchmarkRun>(&output.stdout)?;
            runs.push(run);
        }
    }

    Ok(runs)
}

fn run_target(config: &Config, target: BenchmarkTarget) -> Result<ProjectBenchmarkRun, Box<dyn std::error::Error>> {
    eprintln!(
        "benchmarking {} at {}",
        target.project_name,
        target.manifest.display()
    );
    let representative_files = representative_files(&target.root, config.hot_files)?;
    if representative_files.is_empty() {
        return Err(format!("no Cairo files found under {}", target.root.display()).into());
    }
    let hot_files = representative_files
        .iter()
        .take(config.edit_files.min(representative_files.len()))
        .cloned()
        .collect::<Vec<_>>();

    set_optional_env(FILE_SYNTAX_DATA_LRU_ENV, config.file_syntax_data_lru);
    set_optional_env(FREE_FUNCTION_BODY_LRU_ENV, config.free_function_body_lru);
    set_optional_env(IMPL_FUNCTION_BODY_LRU_ENV, config.impl_function_body_lru);
    set_optional_env(CANONIC_TRAIT_SOLUTIONS_LRU_ENV, config.canonic_trait_solutions_lru);

    let mut client = BenchmarkClient::start(target.root.clone(), None);
    let startup_memory = client.memory_usage();
    let startup_metrics = rss_metrics();

    let mut checkpoints = Vec::new();
    checkpoints.push(snapshot("startup", &startup_memory, startup_metrics));

    eprintln!("  cold open");
    let cold_open = measure_phase(|| {
        client.open(&representative_files[0]);
        client.wait_for_project_update();
    });
    checkpoints.push(snapshot(
        "afterProjectLoad",
        &client.dump_benchmark_snapshot("afterProjectLoad").memory,
        rss_metrics(),
    ));

    eprintln!("  first diagnostics");
    let diagnostics_phase = measure_phase(|| {
        for file in representative_files.iter().skip(1) {
            client.open(file);
        }
        client.wait_for_diagnostics_generation();
    });
    checkpoints.push(snapshot(
        "afterFirstDiagnostics",
        &client.dump_benchmark_snapshot("afterFirstDiagnostics").memory,
        rss_metrics(),
    ));

    eprintln!("  edit loop");
    let edit_loop = run_edit_loop(&mut client, &hot_files, config.edit_iterations);
    checkpoints.push(snapshot(
        "afterEditLoop",
        &client.dump_benchmark_snapshot("afterEditLoop").memory,
        rss_metrics(),
    ));

    std::thread::sleep(config.idle_duration);
    checkpoints.push(snapshot(
        "afterIdle",
        &client.dump_benchmark_snapshot("afterIdle").memory,
        rss_metrics(),
    ));

    eprintln!("  mixed loop");
    let mixed_loop = run_mixed_loop(&mut client, &representative_files, config.mixed_rounds);
    checkpoints.push(snapshot(
        "afterMixedLoop",
        &client.dump_benchmark_snapshot("afterMixedLoop").memory,
        rss_metrics(),
    ));

    eprintln!("  forced swap");
    let force_swap = measure_phase(|| {
        let _ = client.force_database_swap();
        let _ = client.wait_for_database_swap();
        let _ = client.wait_for_diagnostics_generation();
    });
    checkpoints.push(snapshot(
        "afterForcedSwap",
        &client.dump_benchmark_snapshot("afterForcedSwap").memory,
        rss_metrics(),
    ));

    Ok(ProjectBenchmarkRun {
        project: target.project_name,
        root: target.root,
        manifest: target.manifest,
        representative_files,
        checkpoints,
        scenarios: ScenarioMetrics {
            cold_open,
            first_diagnostics: diagnostics_phase,
            edit_loop,
            mixed_loop,
            force_swap,
        },
    })
}

fn run_edit_loop(
    client: &mut BenchmarkClient,
    files: &[PathBuf],
    iterations: usize,
) -> PhaseMetrics {
    let started = Instant::now();
    for iteration in 0..iterations {
        let file = &files[iteration % files.len()];
        eprintln!("    edit iteration {} on {}", iteration + 1, file.display());
        let original = client.read_file(file);
        let edited = if iteration % 2 == 0 {
            format!("{original}\n// cairols bench iteration {iteration}")
        } else {
            format!("{original}\n")
        };
        eprintln!("      applying edit");
        client.change(file, edited);
        client.save(file);
        let _ = client.wait_for_diagnostics_generation();
        eprintln!("      restoring original content");
        client.change(file, original);
        client.save(file);
        let _ = client.wait_for_diagnostics_generation();
    }
    PhaseMetrics { duration_ms: started.elapsed().as_millis() as u64 }
}

fn run_mixed_loop(
    client: &mut BenchmarkClient,
    files: &[PathBuf],
    rounds: usize,
) -> MixedPhaseMetrics {
    let mut timings = BTreeMap::new();
    let total_started = Instant::now();

    for round in 0..rounds {
        let file = &files[round % files.len()];
        let text = client.read_file(file);
        let position = benchmark_position(&text);
        eprintln!("    mixed round {} on {}", round + 1, file.display());

        time_request(&mut timings, "hover", || {
            eprintln!("      request hover");
            let _ = client.request_hover(file, position);
        });
        time_request(&mut timings, "gotoDefinition", || {
            eprintln!("      request gotoDefinition");
            let _ = client.request_goto_definition(file, position);
        });
        time_request(&mut timings, "completion", || {
            eprintln!("      request completion");
            let _ = client.request_completion(file, position);
        });
        time_request(&mut timings, "references", || {
            eprintln!("      request references");
            let _ = client.request_references(file, position);
        });
    }

    MixedPhaseMetrics {
        duration_ms: total_started.elapsed().as_millis() as u64,
        requests: timings,
    }
}

fn time_request(
    timings: &mut BTreeMap<String, RequestTiming>,
    request_name: &str,
    action: impl FnOnce(),
) {
    let started = Instant::now();
    action();
    let elapsed_ms = started.elapsed().as_millis() as u64;
    let timing = timings.entry(request_name.to_string()).or_default();
    timing.count += 1;
    timing.total_duration_ms += elapsed_ms;
    timing.max_duration_ms = timing.max_duration_ms.max(elapsed_ms);
}

fn benchmark_targets(
    project_root: &Path,
    package_manifest_count: usize,
) -> io::Result<Vec<BenchmarkTarget>> {
    let root_manifest = project_root.join("Scarb.toml");
    let mut manifests = Vec::new();
    if root_manifest.exists() {
        manifests.push(root_manifest);
    }

    let packages_dir = project_root.join("packages");
    if packages_dir.is_dir() {
        let mut package_manifests = fs::read_dir(packages_dir)?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path().join("Scarb.toml"))
            .filter(|manifest| manifest.exists())
            .collect::<Vec<_>>();
        package_manifests.sort();
        package_manifests.truncate(package_manifest_count);
        manifests.extend(package_manifests);
    }

    Ok(manifests
        .into_iter()
        .map(|manifest| BenchmarkTarget {
            project_name: project_root
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("project")
                .to_string(),
            root: manifest.parent().expect("manifest must have parent").to_path_buf(),
            manifest,
        })
        .collect())
}

fn representative_files(root: &Path, hot_files: usize) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    collect_cairo_files(root, &mut files)?;
    files.sort_by(|left, right| {
        fs::metadata(right)
            .and_then(|meta| Ok(meta.len()))
            .unwrap_or(0)
            .cmp(&fs::metadata(left).and_then(|meta| Ok(meta.len())).unwrap_or(0))
            .then_with(|| left.cmp(right))
    });
    files.truncate(hot_files);
    Ok(files)
}

fn collect_cairo_files(dir: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.file_name().and_then(|name| name.to_str()) == Some("target") {
            continue;
        }
        if entry.file_type()?.is_dir() {
            collect_cairo_files(&path, files)?;
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("cairo") {
            files.push(path);
        }
    }
    Ok(())
}

fn benchmark_position(text: &str) -> Position {
    for (line_idx, line) in text.lines().enumerate() {
        if let Some((column, _token)) = first_identifier(line) {
            return Position::new(line_idx as u32, column as u32);
        }
    }
    Position::new(0, 0)
}

fn first_identifier(line: &str) -> Option<(usize, &str)> {
    let mut start = None;
    for (index, ch) in line.char_indices() {
        if start.is_none() && (ch == '_' || ch.is_ascii_alphabetic()) {
            start = Some(index);
        } else if let Some(begin) = start
            && !(ch == '_' || ch.is_ascii_alphanumeric())
        {
            return Some((begin, &line[begin..index]));
        }
    }
    start.map(|begin| (begin, &line[begin..]))
}

fn measure_phase(action: impl FnOnce()) -> PhaseMetrics {
    let started = Instant::now();
    action();
    PhaseMetrics { duration_ms: started.elapsed().as_millis() as u64 }
}

fn snapshot(label: &str, memory: &ShowMemoryUsageResponse, rss: RssMetrics) -> Checkpoint {
    Checkpoint { label: label.to_string(), rss, memory: memory.clone() }
}

fn set_optional_env(name: &str, value: Option<usize>) {
    if let Some(value) = value {
        unsafe { env::set_var(name, value.to_string()) };
    } else {
        unsafe { env::remove_var(name) };
    }
}

fn rss_metrics() -> RssMetrics {
    RssMetrics { current_rss_bytes: current_rss_bytes(), peak_rss_bytes: peak_rss_bytes() }
}

fn current_rss_bytes() -> u64 {
    let pid = process::id().to_string();
    let output = Command::new("ps")
        .args(["-o", "rss=", "-p", &pid])
        .output()
        .ok()
        .filter(|output| output.status.success());

    output
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .and_then(|text| text.trim().parse::<u64>().ok())
        .map(|kilobytes| kilobytes * 1024)
        .unwrap_or(0)
}

#[cfg(target_vendor = "apple")]
fn peak_rss_bytes() -> u64 {
    unsafe {
        let mut usage = std::mem::MaybeUninit::<libc::rusage>::uninit();
        if libc::getrusage(libc::RUSAGE_SELF, usage.as_mut_ptr()) == 0 {
            usage.assume_init().ru_maxrss as u64
        } else {
            0
        }
    }
}

#[cfg(not(target_vendor = "apple"))]
fn peak_rss_bytes() -> u64 {
    unsafe {
        let mut usage = std::mem::MaybeUninit::<libc::rusage>::uninit();
        if libc::getrusage(libc::RUSAGE_SELF, usage.as_mut_ptr()) == 0 {
            usage.assume_init().ru_maxrss as u64 * 1024
        } else {
            0
        }
    }
}

fn unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time must be after unix epoch")
        .as_secs()
}

#[derive(Debug, Clone, Serialize)]
struct BenchmarkReport {
    started_at: u64,
    generated_at: u64,
    config: Config,
    runs: Vec<ProjectBenchmarkRun>,
}

#[derive(Debug, Clone, Serialize)]
struct ProjectBenchmarkRun {
    project: String,
    root: PathBuf,
    manifest: PathBuf,
    representative_files: Vec<PathBuf>,
    checkpoints: Vec<Checkpoint>,
    scenarios: ScenarioMetrics,
}

impl<'de> serde::Deserialize<'de> for ProjectBenchmarkRun {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct Inner {
            project: String,
            root: PathBuf,
            manifest: PathBuf,
            representative_files: Vec<PathBuf>,
            checkpoints: Vec<Checkpoint>,
            scenarios: ScenarioMetrics,
        }

        let inner = Inner::deserialize(deserializer)?;
        Ok(Self {
            project: inner.project,
            root: inner.root,
            manifest: inner.manifest,
            representative_files: inner.representative_files,
            checkpoints: inner.checkpoints,
            scenarios: inner.scenarios,
        })
    }
}

#[derive(Debug, Clone, Serialize, serde::Deserialize)]
struct Checkpoint {
    label: String,
    rss: RssMetrics,
    memory: ShowMemoryUsageResponse,
}

#[derive(Debug, Clone, Copy, Serialize, serde::Deserialize)]
struct RssMetrics {
    current_rss_bytes: u64,
    peak_rss_bytes: u64,
}

#[derive(Debug, Clone, Serialize, serde::Deserialize)]
struct ScenarioMetrics {
    cold_open: PhaseMetrics,
    first_diagnostics: PhaseMetrics,
    edit_loop: PhaseMetrics,
    mixed_loop: MixedPhaseMetrics,
    force_swap: PhaseMetrics,
}

#[derive(Debug, Clone, Copy, Serialize, serde::Deserialize)]
struct PhaseMetrics {
    duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, serde::Deserialize)]
struct MixedPhaseMetrics {
    duration_ms: u64,
    requests: BTreeMap<String, RequestTiming>,
}

#[derive(Debug, Clone, Default, Serialize, serde::Deserialize)]
struct RequestTiming {
    count: u64,
    total_duration_ms: u64,
    max_duration_ms: u64,
}

#[derive(Debug, Clone, Serialize)]
struct BenchmarkTarget {
    project_name: String,
    root: PathBuf,
    manifest: PathBuf,
}

#[derive(Debug, Clone, Serialize)]
struct Config {
    project_roots: Vec<PathBuf>,
    output: Option<PathBuf>,
    package_manifest_count: usize,
    hot_files: usize,
    edit_files: usize,
    edit_iterations: usize,
    mixed_rounds: usize,
    file_syntax_data_lru: Option<usize>,
    free_function_body_lru: Option<usize>,
    impl_function_body_lru: Option<usize>,
    canonic_trait_solutions_lru: Option<usize>,
    idle_duration_ms: u64,
    #[serde(skip_serializing)]
    idle_duration: Duration,
}

impl Config {
    fn from_args(args: Vec<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut project_roots = DEFAULT_PROJECTS.iter().map(PathBuf::from).collect::<Vec<_>>();
        let mut output = None;
        let mut package_manifest_count = 2;
        let mut hot_files = 8;
        let mut edit_files = 3;
        let mut edit_iterations = 20;
        let mut mixed_rounds = 8;
        let mut file_syntax_data_lru = Some(0);
        let mut free_function_body_lru = Some(0);
        let mut impl_function_body_lru = Some(0);
        let mut canonic_trait_solutions_lru = Some(0);
        let mut idle_duration_ms = 1_000;

        let mut iter = args.into_iter();
        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "--child-project-name" | "--child-root" | "--child-manifest" => {
                    let _ = iter.next().ok_or(format!("missing value for {arg}"))?;
                }
                "--project-root" => {
                    project_roots.clear();
                    project_roots.push(PathBuf::from(iter.next().ok_or("missing value for --project-root")?));
                }
                "--add-project-root" => {
                    project_roots.push(PathBuf::from(iter.next().ok_or("missing value for --add-project-root")?));
                }
                "--output" => {
                    output = Some(PathBuf::from(iter.next().ok_or("missing value for --output")?));
                }
                "--package-manifests" => {
                    package_manifest_count = iter
                        .next()
                        .ok_or("missing value for --package-manifests")?
                        .parse()?;
                }
                "--hot-files" => {
                    hot_files = iter.next().ok_or("missing value for --hot-files")?.parse()?;
                }
                "--edit-files" => {
                    edit_files = iter.next().ok_or("missing value for --edit-files")?.parse()?;
                }
                "--edit-iterations" => {
                    edit_iterations = iter
                        .next()
                        .ok_or("missing value for --edit-iterations")?
                        .parse()?;
                }
                "--mixed-rounds" => {
                    mixed_rounds = iter.next().ok_or("missing value for --mixed-rounds")?.parse()?;
                }
                "--file-syntax-data-lru" => {
                    file_syntax_data_lru = Some(
                        iter.next()
                            .ok_or("missing value for --file-syntax-data-lru")?
                            .parse()?,
                    );
                }
                "--free-function-body-lru" => {
                    free_function_body_lru = Some(
                        iter.next()
                            .ok_or("missing value for --free-function-body-lru")?
                            .parse()?,
                    );
                }
                "--impl-function-body-lru" => {
                    impl_function_body_lru = Some(
                        iter.next()
                            .ok_or("missing value for --impl-function-body-lru")?
                            .parse()?,
                    );
                }
                "--canonic-trait-solutions-lru" => {
                    canonic_trait_solutions_lru = Some(
                        iter.next()
                            .ok_or("missing value for --canonic-trait-solutions-lru")?
                            .parse()?,
                    );
                }
                "--idle-ms" => {
                    idle_duration_ms = iter.next().ok_or("missing value for --idle-ms")?.parse()?;
                }
                other => return Err(format!("unknown argument: {other}").into()),
            }
        }

        Ok(Self {
            project_roots,
            output,
            package_manifest_count,
            hot_files,
            edit_files,
            edit_iterations,
            mixed_rounds,
            file_syntax_data_lru,
            free_function_body_lru,
            impl_function_body_lru,
            canonic_trait_solutions_lru,
            idle_duration_ms,
            idle_duration: Duration::from_millis(idle_duration_ms),
        })
    }

    fn child_args_for(&self, target: &BenchmarkTarget) -> Vec<String> {
        let mut args = vec![
            "--child-project-name".to_string(),
            target.project_name.clone(),
            "--child-root".to_string(),
            target.root.display().to_string(),
            "--child-manifest".to_string(),
            target.manifest.display().to_string(),
            "--package-manifests".to_string(),
            self.package_manifest_count.to_string(),
            "--hot-files".to_string(),
            self.hot_files.to_string(),
            "--edit-files".to_string(),
            self.edit_files.to_string(),
            "--edit-iterations".to_string(),
            self.edit_iterations.to_string(),
            "--mixed-rounds".to_string(),
            self.mixed_rounds.to_string(),
            "--idle-ms".to_string(),
            self.idle_duration_ms.to_string(),
        ];

        if let Some(capacity) = self.file_syntax_data_lru {
            args.push("--file-syntax-data-lru".to_string());
            args.push(capacity.to_string());
        }
        if let Some(capacity) = self.free_function_body_lru {
            args.push("--free-function-body-lru".to_string());
            args.push(capacity.to_string());
        }
        if let Some(capacity) = self.impl_function_body_lru {
            args.push("--impl-function-body-lru".to_string());
            args.push(capacity.to_string());
        }
        if let Some(capacity) = self.canonic_trait_solutions_lru {
            args.push("--canonic-trait-solutions-lru".to_string());
            args.push(capacity.to_string());
        }

        args
    }
}

#[derive(Debug, Clone)]
struct ChildTarget {
    project_name: String,
    root: PathBuf,
    manifest: PathBuf,
}

impl ChildTarget {
    fn from_args(args: &[String]) -> Result<Option<Self>, Box<dyn std::error::Error>> {
        let mut project_name = None;
        let mut root = None;
        let mut manifest = None;

        let mut iter = args.iter();
        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "--child-project-name" => {
                    project_name = Some(iter.next().ok_or("missing value for --child-project-name")?.clone());
                }
                "--child-root" => {
                    root = Some(PathBuf::from(iter.next().ok_or("missing value for --child-root")?));
                }
                "--child-manifest" => {
                    manifest =
                        Some(PathBuf::from(iter.next().ok_or("missing value for --child-manifest")?));
                }
                _ => {}
            }
        }

        match (project_name, root, manifest) {
            (Some(project_name), Some(root), Some(manifest)) => {
                Ok(Some(Self { project_name, root, manifest }))
            }
            (None, None, None) => Ok(None),
            _ => Err("child benchmark target arguments are incomplete".into()),
        }
    }

    fn into_benchmark_target(self) -> BenchmarkTarget {
        BenchmarkTarget {
            project_name: self.project_name,
            root: self.root,
            manifest: self.manifest,
        }
    }
}
