//! Smoke tests that assert the CLI help output reflects the RusticUI
//! branding and directories referenced by automation tasks.
//!
//! The assertions here intentionally operate on `--help` output so we avoid
//! invoking heavyweight commands (like Playwright) during `cargo test` while
//! still guaranteeing that contributors receive accurate documentation.

use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use std::path::{Path, PathBuf};

/// Mirrors the runtime helper so the tests execute in the workspace root.
fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("xtask is nested two levels below the workspace root")
        .to_path_buf()
}

#[test]
fn root_help_mentions_rustic_branding() -> Result<()> {
    let workspace = workspace_root();
    let mut cmd = Command::new("cargo");
    cmd.current_dir(&workspace)
        .arg("run")
        .arg("-p")
        .arg("xtask")
        .arg("--")
        .arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Rust-first automation"))
        .stdout(predicate::str::contains("Rustic icon bindings"));

    Ok(())
}

#[test]
fn test_help_lists_joy_examples() -> Result<()> {
    let workspace = workspace_root();
    let mut cmd = Command::new("cargo");
    cmd.current_dir(&workspace)
        .arg("run")
        .arg("-p")
        .arg("xtask")
        .arg("--")
        .arg("test")
        .arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("examples/joy-yew"))
        .stdout(predicate::str::contains("examples/joy-leptos"));

    Ok(())
}

#[test]
fn nightly_accessibility_help_mentions_env_toggle() -> Result<()> {
    let workspace = workspace_root();
    let mut cmd = Command::new("cargo");
    cmd.current_dir(&workspace)
        .arg("run")
        .arg("-p")
        .arg("xtask")
        .arg("--")
        .arg("accessibility-nightly")
        .arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("RUSTIC_UI_A11Y_MODE"));

    Ok(())
}
