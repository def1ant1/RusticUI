//! Tests for the multi-suite coverage dashboard aggregator.
//!
//! The fixtures simulate grcov, Vitest, accessibility, and Playwright outputs so we
//! can exercise the aggregation logic without invoking heavyweight external
//! tooling during `cargo test`.

use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("xtask is nested two levels below the workspace root")
        .to_path_buf()
}

#[test]
fn coverage_report_succeeds_with_fixtures() -> Result<()> {
    let workspace = workspace_root();
    let out_dir = tempfile::tempdir()?;
    let mut cmd = Command::new("cargo");
    cmd.current_dir(&workspace)
        .arg("run")
        .arg("-p")
        .arg("xtask")
        .arg("--")
        .arg("coverage-report")
        .arg("--fixtures")
        .arg(
            workspace
                .join("crates/xtask/tests/fixtures/coverage/success")
                .display()
                .to_string(),
        )
        .arg("--out-dir")
        .arg(out_dir.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("coverage dashboard"));

    let markdown = out_dir.path().join("coverage-report.md");
    let contents = fs::read_to_string(&markdown)?;
    assert!(contents.contains("RusticUI coverage dashboard"));
    assert!(contents.contains("✅"));

    Ok(())
}

#[test]
fn coverage_report_fails_when_typescript_missing() -> Result<()> {
    let workspace = workspace_root();
    let mut cmd = Command::new("cargo");
    cmd.current_dir(&workspace)
        .arg("run")
        .arg("-p")
        .arg("xtask")
        .arg("--")
        .arg("coverage-report")
        .arg("--fixtures")
        .arg(
            workspace
                .join("crates/xtask/tests/fixtures/coverage/missing_ts")
                .display()
                .to_string(),
        )
        .arg("--out-dir")
        .arg(workspace.join("target/coverage-test/missing-ts"));

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("TypeScript automation"));

    Ok(())
}

#[test]
fn coverage_report_fails_on_low_rust_coverage() -> Result<()> {
    let workspace = workspace_root();
    let mut cmd = Command::new("cargo");
    cmd.current_dir(&workspace)
        .arg("run")
        .arg("-p")
        .arg("xtask")
        .arg("--")
        .arg("coverage-report")
        .arg("--fixtures")
        .arg(
            workspace
                .join("crates/xtask/tests/fixtures/coverage/rust_regression")
                .display()
                .to_string(),
        )
        .arg("--out-dir")
        .arg(workspace.join("target/coverage-test/rust-regression"));

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Rust workspace"));

    Ok(())
}
