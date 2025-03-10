// A trick to ignore the proc macro tests until new Scarb nigtly is available in CI.
// TODO(#453): Unignore
#[cfg(not(feature = "test_proc_macros"))]
mod custom;
#[cfg(not(feature = "test_proc_macros"))]
mod snforge;
