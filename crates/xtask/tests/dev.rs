//! Integration tests for the `cargo xtask dev` hot-reload harness.

use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use std::path::{Path, PathBuf};

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("xtask is nested two levels below the workspace root")
        .to_path_buf()
}

#[test]
fn dry_run_reports_planned_commands() -> Result<()> {
    let workspace = workspace_root();
    let mut cmd = Command::new("cargo");
    cmd.current_dir(&workspace)
        .arg("run")
        .arg("-p")
        .arg("xtask")
        .arg("--")
        .arg("dev")
        .arg("--dry-run")
        .arg("--docs-port")
        .arg("3200")
        .arg("--gallery-port")
        .arg("3100");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "pnpm --dir docs run dev -- --hostname 127.0.0.1 --port 3200",
        ))
        .stdout(predicate::str::contains(
            "cargo run -p rustic-docs --bin rustic-docs-server --features ssr",
        ));

    Ok(())
}

#[test]
fn skipping_both_processes_fails() -> Result<()> {
    let workspace = workspace_root();
    let mut cmd = Command::new("cargo");
    cmd.current_dir(&workspace)
        .arg("run")
        .arg("-p")
        .arg("xtask")
        .arg("--")
        .arg("dev")
        .arg("--skip-docs")
        .arg("--skip-gallery")
        .arg("--dry-run");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("cannot skip both"));

    Ok(())
}
