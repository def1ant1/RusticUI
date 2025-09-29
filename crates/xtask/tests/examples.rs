//! Integration coverage for the curated example automation pipeline.
//!
//! These tests stub the underlying `cargo` binary so we can validate the
//! argument flow without compiling the entire workspace during `cargo test`.
//! The same entry point is exercised in CI without the stub to produce real
//! native and wasm artifacts.

use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::tempdir;

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("xtask is nested two levels below the workspace root")
        .to_path_buf()
}

#[cfg(unix)]
#[test]
fn layout_examples_invoke_cargo_for_native_and_wasm() -> Result<()> {
    use std::os::unix::fs::PermissionsExt;

    let workspace = workspace_root();
    let stub_dir = tempdir()?;
    let log_path = stub_dir.path().join("cargo-invocations.log");
    let cargo_stub = stub_dir.path().join("cargo");
    let script = format!(
        "#!/bin/sh\nprintf '%s\\n' \"$*\" >> \"{}\"\n",
        log_path.display()
    );
    fs::write(&cargo_stub, script)?;
    let mut perms = fs::metadata(&cargo_stub)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&cargo_stub, perms)?;

    let mut cmd = Command::cargo_bin("xtask")?;
    let path_env = std::env::var("PATH").unwrap_or_default();
    cmd.current_dir(&workspace).arg("examples").env(
        "PATH",
        format!("{}:{}", stub_dir.path().display(), path_env),
    );

    cmd.assert().success();

    let log = fs::read_to_string(&log_path)?;
    let lines: Vec<&str> = log.lines().collect();
    assert_eq!(lines.len(), 4, "expected two targets per layout example");

    // Ensure the first pair targets the Leptos demo without a wasm target flag.
    assert!(lines[0].contains("layout-box-leptos"));
    assert!(!lines[0].contains("--target"));
    assert!(lines[1].contains("layout-box-leptos"));
    assert!(lines[1].contains("--target wasm32-unknown-unknown"));

    // And the second pair should focus on the Yew layout gallery.
    assert!(lines[2].contains("layout-grid-yew"));
    assert!(!lines[2].contains("--target"));
    assert!(lines[3].contains("layout-grid-yew"));
    assert!(lines[3].contains("--target wasm32-unknown-unknown"));

    Ok(())
}

#[test]
fn examples_conflict_release_and_profile() -> Result<()> {
    let workspace = workspace_root();
    let mut cmd = Command::cargo_bin("xtask")?;
    cmd.current_dir(&workspace)
        .arg("examples")
        .arg("--release")
        .arg("--profile")
        .arg("ci");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("cannot be used with"));

    Ok(())
}
