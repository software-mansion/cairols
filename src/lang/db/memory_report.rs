use std::collections::BTreeMap;

use salsa::{Database, IngredientInfo};

use crate::lang::db::AnalysisDatabase;


use crate::lsp::ext::{
    MemoryUsageEntry, MemoryUsageLayerSummary, MemoryUsageSummary, MemoryUsageTotals,
    ShowMemoryUsageResponse,
};

pub(crate) fn build_memory_usage_report(db: &dyn Database) -> ShowMemoryUsageResponse {
    let memory_usage = db.memory_usage();

    let structs = memory_usage.structs.into_iter().map(entry_from_info).collect::<Vec<_>>();
    let queries =
        memory_usage.queries.into_values().map(entry_from_info).collect::<Vec<_>>();

    let top_structs = top_entries(&structs);
    let top_queries = top_entries(&queries);
    let summary = build_summary(structs.iter().chain(queries.iter()));

    ShowMemoryUsageResponse { summary, structs, queries, top_structs, top_queries }
}

fn top_entries(entries: &[MemoryUsageEntry]) -> Vec<MemoryUsageEntry> {
    let mut entries = entries.to_vec();
    entries.sort_by(|left, right| {
        right
            .total_size
            .cmp(&left.total_size)
            .then_with(|| right.heap_size_of_fields.cmp(&left.heap_size_of_fields))
            .then_with(|| left.debug_name.cmp(&right.debug_name))
    });
    entries.truncate(10);
    entries
}

fn build_summary<'a>(
    entries: impl IntoIterator<Item = &'a MemoryUsageEntry>,
) -> MemoryUsageSummary {
    let mut totals = MemoryUsageTotals::default();
    let mut by_layer = BTreeMap::<String, MemoryUsageTotals>::new();

    for entry in entries {
        accumulate(&mut totals, entry);
        accumulate(by_layer.entry(entry.layer.clone()).or_default(), entry);
    }

    MemoryUsageSummary {
        totals,
        by_layer: by_layer
            .into_iter()
            .map(|(layer, totals)| MemoryUsageLayerSummary { layer, totals })
            .collect(),
    }
}

fn accumulate(target: &mut MemoryUsageTotals, entry: &MemoryUsageEntry) {
    target.count += entry.count;
    target.size_of_metadata += entry.size_of_metadata;
    target.size_of_fields += entry.size_of_fields;
    target.heap_size_of_fields += entry.heap_size_of_fields;
    target.total_size += entry.total_size;
}

fn entry_from_info(info: IngredientInfo) -> MemoryUsageEntry {
    let debug_name = info.debug_name().to_string();
    let size_of_metadata = info.size_of_metadata();
    let size_of_fields = info.size_of_fields();
    let heap_size_of_fields = info.heap_size_of_fields().unwrap_or_default();

    MemoryUsageEntry {
        layer: infer_layer(&debug_name).to_string(),
        debug_name,
        count: info.count(),
        size_of_metadata,
        size_of_fields,
        heap_size_of_fields,
        total_size: size_of_metadata + size_of_fields + heap_size_of_fields,
    }
}

fn infer_layer(debug_name: &str) -> &'static str {
    if debug_name.contains("cairo_lang_filesystem") {
        "filesystem"
    } else if debug_name.contains("cairo_lang_parser") || debug_name.contains("cairo_lang_syntax")
    {
        "parser"
    } else if debug_name.contains("cairo_lang_defs") {
        "defs"
    } else if debug_name.contains("cairo_lang_semantic") {
        "semantic"
    } else if debug_name.contains("cairo_lang_lowering") {
        "lowering"
    } else if debug_name.contains("cairo_lang_sierra_generator")
        || debug_name.contains("cairo_lang_sierra")
    {
        "sierra"
    } else if debug_name.contains("cairo_language_common") {
        "language-common"
    } else if debug_name.contains("cairo_lint") {
        "linter"
    } else if debug_name.contains("cairo_language_server") || debug_name.contains("cairols") {
        "cairols"
    } else {
        "other"
    }
}

pub(crate) fn print_memory_usage_report(db: &AnalysisDatabase) {
    use std::process;

    let report = build_memory_usage_report(db);
    let fmt_mb = |n: usize| format!("{:.2} MB", n as f64 / 1_048_576.0);

    let current_rss = {
        let pid = process::id().to_string();
        std::process::Command::new("ps")
            .args(["-o", "rss=", "-p", &pid])
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .and_then(|s| s.trim().parse::<usize>().ok())
            .map(|kb| kb * 1024)
            .unwrap_or(0)
    };

    let peak_rss = unsafe {
        let mut usage = std::mem::MaybeUninit::<libc::rusage>::uninit();
        if libc::getrusage(libc::RUSAGE_SELF, usage.as_mut_ptr()) == 0 {
            #[cfg(target_vendor = "apple")]
            { usage.assume_init().ru_maxrss as usize }
            #[cfg(not(target_vendor = "apple"))]
            { usage.assume_init().ru_maxrss as usize * 1024 }
        } else {
            0
        }
    };

    let totals = &report.summary.totals;
    eprintln!("=== Salsa memory report ===");
    eprintln!("RSS current: {}  peak: {}", fmt_mb(current_rss), fmt_mb(peak_rss));
    eprintln!(
        "Salsa total: count={} stack={} heap={} total={}",
        totals.count,
        fmt_mb(totals.size_of_fields),
        fmt_mb(totals.heap_size_of_fields),
        fmt_mb(totals.total_size),
    );

    eprintln!("\n--- top queries ---");
    eprintln!("{:<60} {:>8} {:>10} {:>10}", "name", "count", "stack", "heap");
    for e in &report.top_queries {
        eprintln!(
            "{:<60} {:>8} {:>10} {:>10}",
            e.debug_name, e.count, fmt_mb(e.size_of_fields), fmt_mb(e.heap_size_of_fields),
        );
    }

    eprintln!("\n--- top structs ---");
    eprintln!("{:<60} {:>8} {:>10} {:>10}", "name", "count", "stack", "heap");
    for e in &report.top_structs {
        eprintln!(
            "{:<60} {:>8} {:>10} {:>10}",
            e.debug_name, e.count, fmt_mb(e.size_of_fields), fmt_mb(e.heap_size_of_fields),
        );
    }

    eprintln!("\n--- by layer ---");
    for layer in &report.summary.by_layer {
        eprintln!(
            "{:<20} stack={} heap={} total={}",
            layer.layer,
            fmt_mb(layer.totals.size_of_fields),
            fmt_mb(layer.totals.heap_size_of_fields),
            fmt_mb(layer.totals.total_size),
        );
    }
}
