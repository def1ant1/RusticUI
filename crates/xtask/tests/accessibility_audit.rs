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
fn accessibility_audit_passes_for_accessible_docs() -> Result<()> {
    let workspace = workspace_root();
    let config = workspace.join("crates/xtask/tests/fixtures/a11y_good.json");
    assert!(config.exists(), "fixture missing: {config:?}");

    Command::new("cargo")
        .current_dir(&workspace)
        .arg("xtask")
        .arg("accessibility-audit")
        .env("RUSTIC_UI_A11Y_CONFIG", &config)
        .assert()
        .success()
        .stdout(predicate::str::contains("issues=0"));

    Ok(())
}

#[test]
fn accessibility_audit_surfaces_findings_for_bad_docs() -> Result<()> {
    let workspace = workspace_root();
    let config = workspace.join("crates/xtask/tests/fixtures/a11y_bad.json");
    assert!(config.exists(), "fixture missing: {config:?}");

    Command::new("cargo")
        .current_dir(&workspace)
        .arg("xtask")
        .arg("accessibility-audit")
        .env("RUSTIC_UI_A11Y_CONFIG", &config)
        .assert()
        .failure()
        .stdout(predicate::str::contains("[xtask][accessibility][finding]"));

    Ok(())
}
