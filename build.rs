use std::{path::Path, process::Command};

fn main() {
    commit_info();
}

/// Modified from https://github.com/software-mansion/scarb/blob/465992a6470746a74a1fa9017d81fc77814a89a0/utils/scarb-build-metadata/build.rs#L12-L35
fn commit_info() {
    if !Path::new("./.git").exists() {
        return;
    }
    println!("cargo:rerun-if-changed=./.git/index");
    let output = match Command::new("git")
        .arg("log")
        .arg("-1")
        .arg("--date=short")
        .arg("--format=%H %h %cd")
        .arg("--abbrev=9")
        .output()
    {
        Ok(output) if output.status.success() => output,
        _ => return,
    };
    let stdout = String::from_utf8(output.stdout).unwrap();
    let mut parts = stdout.split_whitespace();
    let mut next = || parts.next().unwrap();
    println!("cargo:rustc-env=LS_COMMIT_HASH={}", next());
    println!("cargo:rustc-env=LS_COMMIT_SHORT_HASH={}", next());
    println!("cargo:rustc-env=LS_COMMIT_DATE={}", next())
}
