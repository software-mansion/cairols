#!/usr/bin/env python3
import argparse
import csv
import json
from pathlib import Path


CHECKPOINT_ORDER = [
    "startup",
    "afterProjectLoad",
    "afterFirstDiagnostics",
    "afterEditLoop",
    "afterIdle",
    "afterMixedLoop",
    "afterForcedSwap",
]


def load_run(path: Path):
    data = json.loads(path.read_text())
    return data["runs"][0], data["config"]


def checkpoints_by_label(run):
    return {checkpoint["label"]: checkpoint for checkpoint in run["checkpoints"]}


def mb(value):
    return value / (1024 * 1024)


def top_query_count(checkpoint, debug_name):
    return sum(
        entry["count"]
        for entry in checkpoint["memory"]["topQueries"]
        if entry["debugName"] == debug_name
    )


def write_csv(path: Path, sections):
    with path.open("w", newline="") as file:
        writer = csv.writer(file)
        writer.writerow(
            [
                "dataset",
                "phase",
                "rss_mb",
                "salsa_mb",
                "function_body_count",
                "solver_count",
                "cold_open_ms",
                "first_diagnostics_ms",
                "edit_loop_ms",
                "mixed_loop_ms",
                "force_swap_ms",
            ]
        )
        for dataset_name, run in sections:
            for checkpoint in run["checkpoints"]:
                writer.writerow(
                    [
                        dataset_name,
                        checkpoint["label"],
                        round(mb(checkpoint["rss"]["current_rss_bytes"]), 3),
                        round(
                            mb(checkpoint["memory"]["summary"]["totals"]["totalSize"]),
                            3,
                        ),
                        top_query_count(
                            checkpoint,
                            "core::result::Result<cairo_lang_semantic::items::function_with_body::FunctionBodyData<'_>, cairo_lang_diagnostics::diagnostics::DiagnosticAdded>",
                        ),
                        top_query_count(
                            checkpoint,
                            "core::result::Result<cairo_lang_semantic::expr::inference::solver::SolutionSet<'_, cairo_lang_semantic::expr::inference::canonic::CanonicalImpl<'_>>, cairo_lang_semantic::expr::inference::InferenceError<'_>>",
                        ),
                        run["scenarios"]["cold_open"]["duration_ms"],
                        run["scenarios"]["first_diagnostics"]["duration_ms"],
                        run["scenarios"]["edit_loop"]["duration_ms"],
                        run["scenarios"]["mixed_loop"]["duration_ms"],
                        run["scenarios"]["force_swap"]["duration_ms"],
                    ]
                )


def svg_polyline(points, color):
    return (
        f'<polyline fill="none" stroke="{color}" stroke-width="3" '
        f'points="{" ".join(f"{x:.1f},{y:.1f}" for x, y in points)}" />'
    )


def chart_svg(title, series, labels, output: Path, y_label):
    width = 920
    height = 420
    left = 70
    right = 30
    top = 40
    bottom = 70
    plot_width = width - left - right
    plot_height = height - top - bottom
    max_y = max(max(values) for _, _, values in series)
    max_y = max(max_y, 1.0)

    def x_for(index):
        if len(labels) == 1:
            return left + plot_width / 2
        return left + (plot_width * index / (len(labels) - 1))

    def y_for(value):
        return top + plot_height - (plot_height * value / max_y)

    colors = [color for _, color, _ in series]
    lines = [
        f'<line x1="{left}" y1="{top + plot_height}" x2="{width - right}" y2="{top + plot_height}" stroke="#444" />',
        f'<line x1="{left}" y1="{top}" x2="{left}" y2="{top + plot_height}" stroke="#444" />',
        f'<text x="{width / 2}" y="24" text-anchor="middle" font-size="20">{title}</text>',
        f'<text x="20" y="{height / 2}" text-anchor="middle" font-size="14" transform="rotate(-90, 20, {height / 2})">{y_label}</text>',
    ]

    for tick in range(6):
        value = max_y * tick / 5
        y = y_for(value)
        lines.append(
            f'<line x1="{left - 5}" y1="{y:.1f}" x2="{width - right}" y2="{y:.1f}" stroke="#e5e5e5" />'
        )
        lines.append(
            f'<text x="{left - 10}" y="{y + 5:.1f}" text-anchor="end" font-size="12">{value:.0f}</text>'
        )

    for index, label in enumerate(labels):
        x = x_for(index)
        lines.append(
            f'<text x="{x:.1f}" y="{height - 30}" text-anchor="middle" font-size="12">{label}</text>'
        )

    for name, color, values in series:
        points = [(x_for(index), y_for(value)) for index, value in enumerate(values)]
        lines.append(svg_polyline(points, color))
        for x, y in points:
            lines.append(f'<circle cx="{x:.1f}" cy="{y:.1f}" r="4" fill="{color}" />')

    legend_x = width - right - 230
    legend_y = top + 10
    for index, (name, color, _) in enumerate(series):
        y = legend_y + index * 22
        lines.append(f'<rect x="{legend_x}" y="{y - 10}" width="14" height="14" fill="{color}" />')
        lines.append(f'<text x="{legend_x + 22}" y="{y + 1}" font-size="13">{name}</text>')

    output.write_text(
        "\n".join(
            [
                f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}">',
                '<rect width="100%" height="100%" fill="white" />',
                *lines,
                "</svg>",
            ]
        )
    )


def pair_summary(before_run, after_run):
    before = checkpoints_by_label(before_run)
    after = checkpoints_by_label(after_run)
    rows = []
    for label in CHECKPOINT_ORDER:
        before_checkpoint = before[label]
        after_checkpoint = after[label]
        rows.append(
            {
                "label": label,
                "before_rss_mb": mb(before_checkpoint["rss"]["current_rss_bytes"]),
                "after_rss_mb": mb(after_checkpoint["rss"]["current_rss_bytes"]),
                "before_salsa_mb": mb(
                    before_checkpoint["memory"]["summary"]["totals"]["totalSize"]
                ),
                "after_salsa_mb": mb(
                    after_checkpoint["memory"]["summary"]["totals"]["totalSize"]
                ),
            }
        )
    return rows


def scenario_summary(before_run, after_run):
    rows = []
    for key in [
        ("cold_open", "cold_open"),
        ("first_diagnostics", "first_diagnostics"),
        ("edit_loop", "edit_loop"),
        ("mixed_loop", "mixed_loop"),
        ("force_swap", "force_swap"),
    ]:
        label, scenario_key = key
        before_ms = before_run["scenarios"][scenario_key]["duration_ms"]
        after_ms = after_run["scenarios"][scenario_key]["duration_ms"]
        rows.append(
            {
                "label": label,
                "before_ms": before_ms,
                "after_ms": after_ms,
                "delta_pct": ((after_ms - before_ms) / before_ms * 100.0) if before_ms else 0.0,
            }
        )
    return rows


def render_table(rows):
    header = "| Phase | RSS Before (MB) | RSS After (MB) | Salsa Before (MB) | Salsa After (MB) |"
    divider = "| --- | ---: | ---: | ---: | ---: |"
    body = [
        "| {label} | {before_rss_mb:.1f} | {after_rss_mb:.1f} | {before_salsa_mb:.1f} | {after_salsa_mb:.1f} |".format(
            **row
        )
        for row in rows
    ]
    return "\n".join([header, divider, *body])


def render_scenario_table(rows):
    header = "| Scenario | Time Before (ms) | Time After (ms) | Delta |"
    divider = "| --- | ---: | ---: | ---: |"
    body = [
        "| {label} | {before_ms} | {after_ms} | {delta_pct:+.1f}% |".format(**row)
        for row in rows
    ]
    return "\n".join([header, divider, *body])


def sweep_rows(paths):
    rows = []
    for path in paths:
        data = json.loads(Path(path).read_text())
        checkpoint = checkpoints_by_label(data["runs"][0])["afterMixedLoop"]
        rows.append(
            {
                "lru": data["config"]["file_syntax_data_lru"],
                "rss_mb": mb(checkpoint["rss"]["current_rss_bytes"]),
                "salsa_mb": mb(checkpoint["memory"]["summary"]["totals"]["totalSize"]),
                "syntax_data_count": top_query_count(
                    checkpoint, "cairo_lang_parser::db::SyntaxData<'_>"
                ),
            }
        )
    rows.sort(key=lambda row: row["lru"])
    return rows


def render_sweep_table(rows):
    header = "| `file_syntax_data` LRU | `afterMixedLoop` RSS (MB) | `afterMixedLoop` Salsa (MB) | `SyntaxData` count |"
    divider = "| ---: | ---: | ---: | ---: |"
    body = [
        "| {lru} | {rss_mb:.1f} | {salsa_mb:.1f} | {syntax_data_count} |".format(**row)
        for row in rows
    ]
    return "\n".join([header, divider, *body])


def render_config_block(label, config):
    return "\n".join(
        [
            f"### {label}",
            "",
            f"- `file_syntax_data` LRU: `{config['file_syntax_data_lru']}`",
            f"- `free_function_body` LRU: `{config['free_function_body_lru']}`",
            f"- `impl_function_body` LRU: `{config['impl_function_body_lru']}`",
            f"- `canonic_trait_solutions` LRU: `{config['canonic_trait_solutions_lru']}`",
            f"- hot files: `{config['hot_files']}`",
            f"- edit files: `{config['edit_files']}`",
            f"- edit iterations: `{config['edit_iterations']}`",
            f"- mixed rounds: `{config['mixed_rounds']}`",
            f"- idle duration: `{config['idle_duration_ms']} ms`",
        ]
    )


def write_report(
    path: Path,
    alex_rows,
    oz_rows,
    alex_scenarios,
    oz_scenarios,
    sweep,
    before_config,
    after_config,
):
    path.write_text(
        "\n".join(
            [
                "# CairoLS Memory Investigation Report",
                "",
                "## How To Read This Report",
                "",
                "- `Before` and `After` are the two benchmark configurations being compared in a section.",
                "- In the latency-aware reports, `Before` usually means the `lru = 0` baseline and `After` means the candidate tuned configuration.",
                "- `RSS` means `Resident Set Size`: the amount of process memory currently resident in RAM, as reported by the operating system.",
                "- `Salsa` means the total memory that Salsa itself reports for tracked structs and query storage.",
                "- `Phase` rows are checkpoint snapshots taken during one benchmark run.",
                "- `Scenario` rows are timed workloads measured over the whole run, such as opening a project or running an edit loop.",
                "",
                "## Checkpoints",
                "",
                "- `startup`: immediately after CairoLS starts, before loading the benchmark project.",
                "- `afterProjectLoad`: after the first representative file is opened and project loading finishes.",
                "- `afterFirstDiagnostics`: after the remaining representative files are opened and the first diagnostics wave settles.",
                "- `afterEditLoop`: after repeated `didChange` and `didSave` edits on the hot files.",
                "- `afterIdle`: after waiting idle for the configured idle window, without more edits.",
                "- `afterMixedLoop`: after interactive requests such as hover, goto definition, completion, and references.",
                "- `afterForcedSwap`: after forcing a full Salsa database swap and waiting for diagnostics to settle again.",
                "",
                "## Scenarios",
                "",
                "- `cold_open`: time to open the first representative file and finish project loading.",
                "- `first_diagnostics`: time to open the rest of the representative files and wait for the initial diagnostics pass.",
                "- `edit_loop`: time for the configured repeated edit/save/revert cycle on the selected hot files.",
                "- `mixed_loop`: time for the configured interactive LSP request mix.",
                "- `force_swap`: time to force a full DB swap and wait until the server settles again.",
                "",
                "## Compared Configurations",
                "",
                render_config_block("Before", before_config),
                "",
                render_config_block("After", after_config),
                "",
                "## Alexandria",
                "",
                render_table(alex_rows),
                "",
                "![Alexandria checkpoints](alexandria_checkpoint_comparison.svg)",
                "",
                render_scenario_table(alex_scenarios),
                "",
                "## OpenZeppelin",
                "",
                render_table(oz_rows),
                "",
                "![OpenZeppelin checkpoints](openzeppelin_checkpoint_comparison.svg)",
                "",
                render_scenario_table(oz_scenarios),
                "",
                "## Parser LRU Sweep",
                "",
                render_sweep_table(sweep),
                "",
                "![Parser LRU sweep](alexandria_parser_lru_sweep.svg)",
                "",
                "## Notes",
                "",
                "- The biggest residual contributors after parser bounding are semantic function-body queries and canonical trait-solver results.",
                "- Forced DB swaps drop Salsa-reported memory close to zero, while RSS only partially follows, which suggests allocator retention on top of true cache growth.",
                "- Time efficiency is treated as a parallel goal: any future LRU candidate should be compared against the `0` baseline for both memory and phase latency.",
                "",
            ]
        )
    )


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--alex-before", required=True)
    parser.add_argument("--alex-after", required=True)
    parser.add_argument("--oz-before", required=True)
    parser.add_argument("--oz-after", required=True)
    parser.add_argument("--sweep", action="append", required=True)
    parser.add_argument("--out-dir", required=True)
    args = parser.parse_args()

    out_dir = Path(args.out_dir)
    out_dir.mkdir(parents=True, exist_ok=True)

    alex_before, before_config = load_run(Path(args.alex_before))
    alex_after, after_config = load_run(Path(args.alex_after))
    oz_before, _ = load_run(Path(args.oz_before))
    oz_after, _ = load_run(Path(args.oz_after))

    write_csv(
        out_dir / "checkpoint_summary.csv",
        [
            ("alexandria_before", alex_before),
            ("alexandria_after", alex_after),
            ("openzeppelin_before", oz_before),
            ("openzeppelin_after", oz_after),
        ],
    )

    alex_rows = pair_summary(alex_before, alex_after)
    oz_rows = pair_summary(oz_before, oz_after)
    alex_scenarios = scenario_summary(alex_before, alex_after)
    oz_scenarios = scenario_summary(oz_before, oz_after)
    sweep = sweep_rows(args.sweep)

    chart_svg(
        "Alexandria RSS by Phase",
        [
            ("before", "#a33a2b", [row["before_rss_mb"] for row in alex_rows]),
            ("after", "#206095", [row["after_rss_mb"] for row in alex_rows]),
        ],
        CHECKPOINT_ORDER,
        out_dir / "alexandria_checkpoint_comparison.svg",
        "RSS (MB)",
    )
    chart_svg(
        "OpenZeppelin RSS by Phase",
        [
            ("before", "#a33a2b", [row["before_rss_mb"] for row in oz_rows]),
            ("after", "#206095", [row["after_rss_mb"] for row in oz_rows]),
        ],
        CHECKPOINT_ORDER,
        out_dir / "openzeppelin_checkpoint_comparison.svg",
        "RSS (MB)",
    )
    chart_svg(
        "Alexandria Parser LRU Sweep",
        [
            ("rss", "#206095", [row["rss_mb"] for row in sweep]),
            ("salsa", "#7a9e0f", [row["salsa_mb"] for row in sweep]),
        ],
        [str(row["lru"]) for row in sweep],
        out_dir / "alexandria_parser_lru_sweep.svg",
        "MB",
    )

    write_report(
        out_dir / "memory_report.md",
        alex_rows,
        oz_rows,
        alex_scenarios,
        oz_scenarios,
        sweep,
        before_config,
        after_config,
    )


if __name__ == "__main__":
    main()
