//! Integration coverage for the curated example automation pipeline.
//!
//! These tests stub the underlying `cargo` binary so we can validate the
//! argument flow without compiling the entire workspace during `cargo test`.
//! The same entry point is exercised in CI without the stub to produce real
//! native and wasm artifacts.

use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use serde::Deserialize;
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

struct ManifestBackup {
    original: PathBuf,
    backup: PathBuf,
}

impl ManifestBackup {
    fn new(original: &Path) -> std::io::Result<Self> {
        let backup = original.with_extension("toml.bak");
        fs::rename(original, &backup)?;
        Ok(Self {
            original: original.to_path_buf(),
            backup,
        })
    }
}

impl Drop for ManifestBackup {
    fn drop(&mut self) {
        if self.backup.exists() {
            if let Err(error) = fs::rename(&self.backup, &self.original) {
                eprintln!(
                    "failed to restore manifest from {} to {}: {}",
                    self.backup.display(),
                    self.original.display(),
                    error
                );
            }
        }
    }
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

#[test]
fn selection_controls_manifest_gaps_fail_loudly() -> Result<()> {
    let workspace = workspace_root();
    let leptos_manifest = workspace.join("examples/selection-controls-leptos/Cargo.toml");

    let mut cmd = Command::cargo_bin("xtask")?;
    cmd.current_dir(&workspace)
        .arg("examples")
        .arg("selection-controls");

    if leptos_manifest.exists() {
        let yew_manifest = workspace.join("examples/selection-controls-yew/Cargo.toml");
        assert!(
            yew_manifest.exists(),
            "selection controls yew manifest should exist before temporary relocation"
        );

        let _backup = ManifestBackup::new(&yew_manifest)?;

        cmd.assert().failure().stderr(predicate::str::contains(
            "selection control example `selection-controls-yew` manifest missing",
        ));
    } else {
        cmd.assert().failure().stderr(predicate::str::contains(
            "selection control example `selection-controls-leptos` manifest missing",
        ));
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
struct HarnessEvent {
    framework: String,
    selectors: Vec<HarnessSelector>,
    status: String,
}

#[derive(Debug, Deserialize)]
struct HarnessSelector {
    key: String,
    #[serde(rename = "automation_id")]
    automation_id: String,
}

#[test]
fn selection_controls_rust_harness_covers_full_matrix() -> Result<()> {
    let workspace = workspace_root();
    let audit_dir = tempdir()?;
    let audit_path = audit_dir.path().join("audit.jsonl");

    let mut cmd = Command::cargo_bin("xtask")?;
    cmd.current_dir(&workspace)
        .arg("selection-controls")
        .arg("--skip-rust")
        .env("RUSTICUI_SELECTION_CONTROLS_SKIP_SERVER", "1")
        .env("RUSTICUI_SELECTION_CONTROLS_SKIP_BROWSER", "1")
        .env("RUSTICUI_SELECTION_CONTROLS_AUDIT_LOG", &audit_path);

    cmd.assert().success();

    let log = fs::read_to_string(&audit_path)?;
    let mut frameworks = Vec::new();
    for line in log.lines() {
        let event: HarnessEvent = serde_json::from_str(line)?;
        frameworks.push(event.framework.clone());
        assert_eq!(
            event.status, "skip-server",
            "dry runs should short-circuit before launching Chrome"
        );

        // Every selector should carry a fully qualified automation identifier so downstream
        // analytics and telemetry probes can attach without guessing prefixes.
        for selector in event.selectors {
            assert!(
                selector
                    .automation_id
                    .starts_with("automation.selection-controls."),
                "unexpected automation identifier: {}",
                selector.automation_id
            );
            assert!(!selector.key.is_empty());
        }
    }

    frameworks.sort();
    let expected = vec![
        "dioxus".to_string(),
        "react".to_string(),
        "sycamore".to_string(),
        "yew".to_string(),
    ];
    assert_eq!(frameworks, expected);

    Ok(())
}
