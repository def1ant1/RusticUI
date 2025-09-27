//! Regression tests covering the new bundle-oriented xtask subcommands.
//!
//! The legacy npm distribution shipped prebuilt ZIPs and manifests that CI pipelines consumed.
//! These tests guarantee the Rust-first automation keeps emitting the same machine-readable
//! contracts so enterprise adopters can flip between ecosystems without rewriting pipelines.

use anyhow::Result;
use assert_cmd::Command;
use serde_json::Value;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use tempfile::tempdir;
use walkdir::WalkDir;

#[derive(Debug)]
struct FileSnapshot {
    path: PathBuf,
    original_bytes: Option<Vec<u8>>,
}

impl FileSnapshot {
    fn new(path: PathBuf) -> Self {
        let original_bytes = fs::read(&path).ok();
        Self {
            path,
            original_bytes,
        }
    }

    fn restore(&self) {
        match &self.original_bytes {
            Some(bytes) => {
                if let Err(error) = fs::write(&self.path, bytes) {
                    eprintln!("[test] failed to restore {}: {error}", self.path.display());
                }
            }
            None => match fs::remove_file(&self.path) {
                Ok(()) => {}
                Err(error) if error.kind() == io::ErrorKind::NotFound => {}
                Err(error) => {
                    eprintln!(
                        "[test] failed to remove {} during cleanup: {error}",
                        self.path.display()
                    );
                }
            },
        }
    }
}

impl Drop for FileSnapshot {
    fn drop(&mut self) {
        self.restore();
    }
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("xtask is nested two levels below the workspace root")
        .to_path_buf()
}

#[test]
fn icons_bundle_emits_single_manifest_and_unique_entries() -> Result<()> {
    let workspace = workspace_root();
    let fixture_dir = workspace.join("crates/rustic-ui-icons/icons/__xtask_fixture");
    if fixture_dir.exists() {
        fs::remove_dir_all(&fixture_dir)?;
    }
    let temp = tempdir()?;
    let out_dir = temp.path().join("icons-artifacts");

    let mut cmd = Command::new("cargo");
    cmd.current_dir(&workspace)
        .arg("xtask")
        .arg("icons-bundle")
        .arg("--out-dir")
        .arg(&out_dir);

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("[xtask][icons-bundle] summary="));

    let bundle_dir = out_dir.join("icons");
    let manifest_path = bundle_dir.join("icons.manifest.json");
    assert!(
        manifest_path.exists(),
        "manifest missing at {manifest_path:?}"
    );

    let manifest_json = fs::read_to_string(&manifest_path)?;
    let manifest: Value = serde_json::from_str(&manifest_json)?;
    assert_eq!(manifest["bundle"], Value::String("icons".into()));
    assert_eq!(
        manifest["metadata"]["bundle_kind"].as_str(),
        Some("icon-assets")
    );

    let entries = manifest["entries"]
        .as_array()
        .expect("entries array present");
    assert!(
        !entries.is_empty(),
        "expected at least one SVG payload in the manifest"
    );
    let unique_paths: HashSet<_> = entries
        .iter()
        .map(|entry| entry["relative_path"].as_str().expect("path string"))
        .collect();
    assert_eq!(
        entries.len(),
        unique_paths.len(),
        "duplicate entries detected"
    );

    for entry in entries {
        let legacy_packages = entry["metadata"]["legacy_packages"]
            .as_array()
            .expect("legacy package list present");
        assert!(
            legacy_packages
                .iter()
                .any(|value| value == "@mui/icons-material"),
            "entry missing legacy package metadata"
        );
        assert_eq!(entry["media_type"].as_str(), Some("image/svg+xml"));
    }

    if fixture_dir.exists() {
        fs::remove_dir_all(&fixture_dir)?;
    }

    Ok(())
}

#[test]
fn themes_bundle_emits_css_and_json_with_overrides() -> Result<()> {
    let workspace = workspace_root();
    let temp = tempdir()?;
    let out_dir = temp.path().join("themes-artifacts");
    let overrides = workspace.join("crates/xtask/tests/fixtures/material_overrides.json");
    assert!(overrides.exists(), "fixture missing: {overrides:?}");

    let mut cmd = Command::new("cargo");
    cmd.current_dir(&workspace)
        .arg("xtask")
        .arg("themes-bundle")
        .arg("--overrides")
        .arg(&overrides)
        .arg("--joy")
        .arg("--out-dir")
        .arg(&out_dir);

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("[xtask][themes-bundle] summary="));

    let bundle_dir = out_dir.join("themes");
    let templates_dir = workspace.join("crates/rustic-ui-system/templates");
    let _snapshots: Vec<FileSnapshot> = WalkDir::new(&templates_dir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| FileSnapshot::new(entry.path().to_path_buf()))
        .collect();
    let manifest_path = bundle_dir.join("themes.manifest.json");
    assert!(
        manifest_path.exists(),
        "manifest missing: {manifest_path:?}"
    );

    let manifest_json = fs::read_to_string(&manifest_path)?;
    let manifest: Value = serde_json::from_str(&manifest_json)?;
    assert_eq!(manifest["bundle"], Value::String("themes".into()));
    assert_eq!(
        manifest["metadata"]["bundle_kind"].as_str(),
        Some("theme-assets")
    );
    assert_eq!(manifest["metadata"]["joy"].as_bool(), Some(true));

    if let Some(Value::String(path)) = manifest["metadata"].get("overrides") {
        assert!(
            path.ends_with("crates/xtask/tests/fixtures/material_overrides.json"),
            "manifest should reference the overrides fixture"
        );
    } else {
        panic!("manifest should include the overrides path metadata");
    }

    let entries = manifest["entries"]
        .as_array()
        .expect("entries array present");
    assert!(entries.len() >= 4, "expected multiple theme outputs");
    let unique_paths: HashSet<_> = entries
        .iter()
        .map(|entry| entry["relative_path"].as_str().expect("path string"))
        .collect();
    assert_eq!(
        entries.len(),
        unique_paths.len(),
        "duplicate entries detected"
    );
    assert!(
        entries
            .iter()
            .any(|entry| entry["media_type"].as_str() == Some("text/css")),
        "CSS baselines must be present in the bundle"
    );
    assert!(
        entries
            .iter()
            .any(|entry| entry["media_type"].as_str() == Some("application/json")),
        "JSON payloads must be present in the bundle"
    );

    Ok(())
}
