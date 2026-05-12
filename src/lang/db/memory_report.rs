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

const TOP_ENTRIES_LIMIT: usize = 10;

fn top_entries(entries: &[MemoryUsageEntry]) -> Vec<MemoryUsageEntry> {
    let mut entries = entries.to_vec();
    entries.sort_by(|left, right| {
        right
            .total_size
            .cmp(&left.total_size)
            .then_with(|| right.heap_size_of_fields.cmp(&left.heap_size_of_fields))
            .then_with(|| left.debug_name.cmp(&right.debug_name))
    });
    entries.truncate(TOP_ENTRIES_LIMIT);
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

pub(crate) fn save_memory_usage_report(
    db: &AnalysisDatabase,
    dir: &std::path::Path,
) -> std::io::Result<()> {
    let report = build_memory_usage_report(db);
    std::fs::create_dir_all(dir)?;
    let json = serde_json::to_string_pretty(&report)
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;
    std::fs::write(dir.join("latest.json"), json)?;
    std::fs::write(dir.join("latest.html"), render_html(&report))?;
    Ok(())
}

fn render_html(report: &ShowMemoryUsageResponse) -> String {
    let mut entries: Vec<(&'static str, &MemoryUsageEntry)> =
        Vec::with_capacity(report.structs.len() + report.queries.len());
    for entry in &report.structs {
        entries.push((classify_struct_kind(&entry.debug_name), entry));
    }
    for entry in &report.queries {
        entries.push(("query", entry));
    }
    entries.sort_by(|a, b| b.1.total_size.cmp(&a.1.total_size));

    let total_bytes = report.summary.totals.total_size;
    let total_count = report.summary.totals.count;

    let mut out = String::with_capacity(64 * 1024);
    out.push_str(HTML_HEAD);

    push_li_open(
        &mut out,
        "session",
        "cairols memory snapshot",
        total_bytes,
        total_count,
        &format!("{} entries", entries.len()),
        true,
    );

    out.push_str("<ul>\n");
    for (kind, entry) in entries {
        push_leaf(&mut out, kind, entry);
    }
    out.push_str("</ul>\n</details></li>\n");
    out.push_str(HTML_TAIL);
    out
}

fn classify_struct_kind(debug_name: &str) -> &'static str {
    if debug_name.ends_with("Input") { "input" } else { "struct" }
}

fn push_li_open(
    out: &mut String,
    kind: &str,
    label: &str,
    bytes: usize,
    count: usize,
    summary: &str,
    open: bool,
) {
    use std::fmt::Write;
    let label_lc = label.to_lowercase();
    let _ = write!(
        out,
        r#"<li data-label="{kind} {label_lc}" data-sort-label="{label_lc}" data-total="{bytes}" data-count="{count}"><details{open}><summary><span class="kind">{kind}</span>: {label_esc} <span class="size">{size_str}</span> <span class="count">x{count}</span><span class="summary">{summary_esc}</span></summary>
"#,
        kind = kind,
        label_lc = escape(&label_lc),
        bytes = bytes,
        count = count,
        open = if open { " open" } else { "" },
        label_esc = escape(label),
        size_str = fmt_bytes(bytes),
        summary_esc = escape(summary),
    );
}

fn push_leaf(out: &mut String, kind: &str, entry: &MemoryUsageEntry) {
    use std::fmt::Write;
    let label_lc = entry.debug_name.to_lowercase();
    let _ = write!(
        out,
        r#"<li data-label="{kind} {label_lc}" data-sort-label="{label_lc}" data-total="{total}" data-count="{count}"><div class="leaf"><span class="kind">{kind}</span>: {name_esc} <span class="size">{size_str}</span> <span class="count">x{count}</span><span class="summary">stack {stack} · heap {heap} · metadata {meta}</span></div></li>
"#,
        kind = kind,
        label_lc = escape(&label_lc),
        total = entry.total_size,
        count = entry.count,
        name_esc = escape(&entry.debug_name),
        size_str = fmt_bytes(entry.total_size),
        stack = fmt_bytes(entry.size_of_fields),
        heap = fmt_bytes(entry.heap_size_of_fields),
        meta = fmt_bytes(entry.size_of_metadata),
    );
}

fn escape(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(ch),
        }
    }
    out
}

fn fmt_bytes(bytes: usize) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = 1024.0 * 1024.0;
    const GB: f64 = 1024.0 * 1024.0 * 1024.0;
    let b = bytes as f64;
    if b >= GB {
        format!("{:.2} GB", b / GB)
    } else if b >= MB {
        format!("{:.2} MB", b / MB)
    } else if b >= KB {
        format!("{:.1} KB", b / KB)
    } else {
        format!("{} B", bytes)
    }
}

const HTML_HEAD: &str = r#"<!doctype html><html><head><meta charset="utf-8"><title>CairoLS Memory Snapshot</title><style>
body{font:13px/1.35 -apple-system,BlinkMacSystemFont,Segoe UI,sans-serif;margin:24px;background:#fafafa;color:#1f2328}
.toolbar{position:sticky;top:0;background:#fafafacc;backdrop-filter:blur(8px);padding:8px 0 14px;border-bottom:1px solid #ddd;margin-bottom:12px}
input{font:inherit;padding:5px 7px;width:min(680px,80vw)}
select{font:inherit;padding:5px 7px;margin-left:8px}
ul{list-style:none;margin:0 0 0 18px;padding:0;border-left:1px solid #ddd}
li{margin:2px 0 2px 10px}
details{padding:1px 0}
summary{cursor:pointer;white-space:nowrap}
.leaf{padding:1px 0}
.kind{font-weight:700;color:#8250df}
.size{color:#0969da;font-variant-numeric:tabular-nums}
.count{color:#57606a;font-variant-numeric:tabular-nums;margin-left:6px}
.summary{color:#475569;margin-left:10px;font-size:12px;font-weight:400;white-space:normal}
.hide{display:none}
</style></head><body><div class="toolbar"><input id="filter" placeholder="Filter by name, e.g. SemanticGroup or Result"><select id="sort"><option value="total">Sort: total size</option><option value="count">Sort: count</option><option value="name">Sort: name</option></select></div><ul class="tree">
"#;

const HTML_TAIL: &str = r#"</ul><script>
const input=document.getElementById('filter');
input.addEventListener('input',()=>{
  const q=input.value.toLowerCase();
  for (const li of document.querySelectorAll('li[data-label]')) {
    const hit=!q||li.dataset.label.includes(q);
    let childHit=false;
    if(!hit){for (const c of li.querySelectorAll('li[data-label]')) { if(c.dataset.label.includes(q)) { childHit=true; break; } }}
    li.classList.toggle('hide', !(hit||childHit));
    if(q&&(hit||childHit)) { const d=li.querySelector(':scope > details'); if(d) d.open=true; }
  }
});
const sort=document.getElementById('sort');
function sortUl(ul,mode){
  const items=[...ul.children];
  items.sort((a,b)=>{
    if(mode==='count'){const av=+(a.dataset.count||0);const bv=+(b.dataset.count||0);if(bv!==av)return bv-av;}
    if(mode==='total'){const av=+(a.dataset.total||0);const bv=+(b.dataset.total||0);if(bv!==av)return bv-av;}
    return (a.dataset.sortLabel||a.dataset.label||'').localeCompare(b.dataset.sortLabel||b.dataset.label||'');
  });
  for(const it of items) ul.appendChild(it);
}
function sortAll(){const mode=sort.value;for(const ul of document.querySelectorAll('ul')) sortUl(ul,mode);}
sort.addEventListener('change',sortAll);
sortAll();
</script></body></html>
"#;
