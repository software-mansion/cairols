use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use semver::Version;
use toml_edit::{DocumentMut, InlineTable, Value};
use xshell::{Shell, cmd};

use crate::set_version;

/// Update `cairo-lang-*` crates in a proper way.
#[derive(Parser)]
pub struct Args {
    #[command(flatten)]
    spec: Spec,

    /// Do not edit any files, just inform what would be done.
    #[arg(long, default_value_t = false)]
    dry_run: bool,
}

#[derive(clap::Args, Clone)]
#[group(required = true, multiple = true)]
struct Spec {
    /// Source the compiler from crates.io and use a specific version.
    version: Option<Version>,

    /// Source the compiler from the GitHub repository and use a specific commit/ref.
    #[arg(short, long, conflicts_with = "branch")]
    rev: Option<String>,

    /// Source the compiler from the GitHub repository and use a specific branch.
    #[arg(short, long)]
    branch: Option<String>,

    /// Source the compiler from a local filesystem.
    ///
    /// This is useful for local development, but avoid commiting this to the repository.
    #[arg(short, long, conflicts_with_all = ["rev", "branch"])]
    path: Option<PathBuf>,
}

pub fn main(args: Args) -> Result<()> {
    let sh = Shell::new()?;

    let mut cargo_toml = sh.read_file("Cargo.toml")?.parse::<DocumentMut>()?;

    patch(&mut cargo_toml, "dependencies", &args);
    patch(&mut cargo_toml, "dev-dependencies", &args);

    if !args.dry_run {
        sh.write_file("Cargo.toml", cargo_toml.to_string())?;

        cmd!(sh, "cargo fetch").run()?;

        eprintln!("$ cargo xtask set-version");
        set_version::main(Default::default())?;
    }

    Ok(())
}

fn patch(cargo_toml: &mut DocumentMut, table: &str, args: &Args) {
    let deps = &mut cargo_toml[table].as_table_mut().unwrap();

    for (dep_name, dep) in deps
        .iter_mut()
        .filter(|(key, _)| key.get().starts_with("cairo-lang-") && key != "cairo-lang-macro")
    {
        let dep_name = dep_name.get();
        let dep = dep.as_value_mut().unwrap();

        // Start with expanded form: { version = "X" }
        let mut new_dep = InlineTable::new();

        if let Some(version) = &args.spec.version {
            new_dep.insert("version", version.to_string().into());
        }

        // Add a Git branch or revision reference if requested.
        if args.spec.rev.is_some() || args.spec.branch.is_some() {
            new_dep.insert("git", "https://github.com/starkware-libs/cairo".into());
        }

        if let Some(branch) = &args.spec.branch {
            new_dep.insert("branch", branch.as_str().into());
        }

        if let Some(rev) = &args.spec.rev {
            new_dep.insert("rev", rev.as_str().into());
        }

        // Add local path reference if requested.
        // For local path sources, Cargo is not looking for crates recursively therefore,
        // we need to manually provide full paths to Cairo workspace member crates.
        if let Some(path) = &args.spec.path {
            new_dep.insert(
                "path",
                path.join("crates").join(dep_name).to_string_lossy().into_owned().into(),
            );
        }

        // Sometimes we might specify extra features. Let's preserve these.
        if let Some(dep) = dep.as_inline_table() {
            if let Some(features) = dep.get("features") {
                new_dep.insert("features", features.clone());
            }
        }

        // Simplify { version = "X" } to "X" if possible.
        let new_dep: Value = if new_dep.len() == 1 {
            new_dep.remove("version").unwrap_or_else(|| new_dep.into())
        } else {
            new_dep.into()
        };

        *dep = new_dep;
    }

    deps.fmt();
    deps.sort_values();

    eprintln!("[{table}]");
    for (key, dep) in deps.iter().filter(|(key, _)| key.starts_with("cairo-lang-")) {
        eprintln!("{key} = {dep}");
    }
}
