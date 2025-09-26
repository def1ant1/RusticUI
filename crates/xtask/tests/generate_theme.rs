//! Integration tests exercising the `cargo xtask generate-theme` automation pipeline.
//!
//! These tests spin up the command end-to-end to guarantee that future
//! refactors keep producing artifacts compatible with large enterprise
//! consumers that depend on our serialized themes and CSS baselines.

use anyhow::Result;
use assert_cmd::Command;
use serde_json::Value;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Captures the original state of a generated file so we can restore it once the
/// integration test completes.
///
/// We snapshot the bytes instead of merely tracking a boolean because the xtask
/// pipeline is expected to rewrite the files on every invocation. Restoring the
/// precise contents keeps `git status` clean for developers running the tests
/// locally and mirrors how CI should leave the repository.
#[derive(Debug)]
struct FileSnapshot {
    path: PathBuf,
    original_bytes: Option<Vec<u8>>,
}

impl FileSnapshot {
    /// Takes a best-effort snapshot of the file located at `path`.
    fn new(path: PathBuf) -> Self {
        let original_bytes = fs::read(&path).ok();
        Self {
            path,
            original_bytes,
        }
    }

    /// Restores the file to its original state.
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

/// Helper returning the repository root so we can invoke `cargo xtask` with the same
/// working directory as human contributors.
fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("xtask is nested two levels below the workspace root")
        .to_path_buf()
}

/// Ensures that `cargo xtask generate-theme` keeps producing both light and dark
/// artifacts when fixture overrides are supplied. The command should log helpful
/// context for maintainers, and the generated JSON + CSS outputs must reflect the
/// merged tokens.
#[test]
fn generates_light_and_dark_artifacts_with_fixtures() -> Result<()> {
    let workspace = workspace_root();
    let fixtures = workspace.join("crates/xtask/tests/fixtures");
    let overrides = fixtures.join("material_overrides.json");
    assert!(overrides.exists(), "fixture missing: {overrides:?}");

    let templates_dir = workspace.join("crates/rustic-ui-system/templates");
    let expected_outputs = [
        templates_dir.join("material_theme.light.json"),
        templates_dir.join("material_theme.dark.json"),
        templates_dir.join("material_css_baseline.light.css"),
        templates_dir.join("material_css_baseline.dark.css"),
        templates_dir.join("joy_theme.light.json"),
        templates_dir.join("joy_theme.dark.json"),
        templates_dir.join("joy_theme.template.json"),
    ];

    // Snapshot existing artifacts so the repository stays clean for local developers.
    let _snapshots: Vec<FileSnapshot> = expected_outputs
        .iter()
        .cloned()
        .map(FileSnapshot::new)
        .collect();

    // Execute the xtask command exactly like CI would.
    let mut cmd = Command::new("cargo");
    cmd.current_dir(&workspace)
        .arg("xtask")
        .arg("generate-theme")
        .arg("--overrides")
        .arg(&overrides)
        .arg("--joy");

    let assertion = cmd
        .assert()
        .success()
        .stdout(predicates::str::contains(
            "generating Material theme artifacts",
        ))
        .stdout(predicates::str::contains("loading overrides"));

    // Verify that the log output remains descriptive for maintainers who rely on CI logs.
    let stdout = String::from_utf8_lossy(&assertion.get_output().stdout);
    assert!(
        stdout
            .contains("[xtask] wrote crates/rustic-ui-system/templates/material_theme.light.json"),
        "xtask output should mention the light theme artifact"
    );
    for artifact in [
        "material_theme.light.json",
        "material_theme.dark.json",
        "joy_theme.light.json",
        "joy_theme.dark.json",
        "joy_theme.template.json",
    ] {
        assert!(
            stdout.contains(&format!(
                "[xtask] wrote crates/rustic-ui-system/templates/{artifact}"
            )),
            "xtask output should mention the {artifact} artifact"
        );
    }

    // Confirm that every artifact exists so downstream tooling can consume it.
    for path in &expected_outputs {
        assert!(path.exists(), "expected artifact missing: {path:?}");
    }

    let light_json = fs::read_to_string(&expected_outputs[0])?;
    let dark_json = fs::read_to_string(&expected_outputs[1])?;
    let light_value: Value = serde_json::from_str(&light_json)?;
    let dark_value: Value = serde_json::from_str(&dark_json)?;

    // Global typography overrides should apply to all schemes.
    assert_eq!(
        light_value
            .get("typography")
            .and_then(|t| t.get("font_family"))
            .and_then(Value::as_str),
        Some("Corporate Sans, Arial, sans-serif")
    );
    assert_eq!(
        dark_value
            .get("typography")
            .and_then(|t| t.get("font_family"))
            .and_then(Value::as_str),
        Some("Corporate Sans, Arial, sans-serif")
    );
    assert_eq!(
        dark_value
            .get("typography")
            .and_then(|t| t.get("font_weight_bold"))
            .and_then(Value::as_f64),
        Some(910.0)
    );

    // Scheme-specific palette overrides should only affect their respective outputs.
    let light_palette = light_value
        .get("palette")
        .and_then(|p| p.get("light"))
        .expect("light scheme palette should exist");
    assert_eq!(
        light_palette.get("primary").and_then(Value::as_str),
        Some("#0057b7")
    );
    let dark_palette = dark_value
        .get("palette")
        .and_then(|p| p.get("dark"))
        .expect("dark scheme palette should exist");
    assert_eq!(
        dark_palette.get("primary").and_then(Value::as_str),
        Some("#ffd700")
    );
    for key in ["success", "warning", "info"] {
        assert!(
            light_palette.get(key).is_some(),
            "light palette should expose `{key}`"
        );
        assert!(
            dark_palette.get(key).is_some(),
            "dark palette should expose `{key}`"
        );
    }

    // CSS baselines should capture typography + palette overrides so applications render correctly.
    let light_css = fs::read_to_string(&expected_outputs[2])?;
    let dark_css = fs::read_to_string(&expected_outputs[3])?;
    assert!(
        light_css.contains("font-family: Corporate Sans, Arial, sans-serif"),
        "Light CSS baseline should include the global typography override"
    );
    assert!(
        dark_css.contains("font-family: Corporate Sans, Arial, sans-serif"),
        "Dark CSS baseline should include the global typography override"
    );
    assert!(
        light_css.contains("background-color: #f4f6fb"),
        "Light CSS baseline should include the light scheme background color"
    );
    assert!(
        dark_css.contains("background-color: #020617"),
        "Dark CSS baseline should include the dark scheme background color"
    );
    assert!(
        dark_css.contains("color: #e2e8f0"),
        "Dark CSS baseline should include the dark scheme text color"
    );

    let joy_light = fs::read_to_string(&expected_outputs[4])?;
    let joy_dark = fs::read_to_string(&expected_outputs[5])?;
    let joy_template = fs::read_to_string(&expected_outputs[6])?;
    let joy_light_value: Value = serde_json::from_str(&joy_light)?;
    let joy_dark_value: Value = serde_json::from_str(&joy_dark)?;
    let joy_template_value: Value = serde_json::from_str(&joy_template)?;

    for value in [&joy_light_value, &joy_dark_value] {
        assert_eq!(
            value
                .get("joy")
                .and_then(|joy| joy.get("focus"))
                .and_then(|focus| focus.get("thickness"))
                .and_then(Value::as_u64),
            Some(2),
            "Joy fixtures should honour default focus thickness"
        );
        assert!(
            value
                .get("automation")
                .and_then(|meta| meta.get("comments"))
                .is_some(),
            "Joy fixtures should embed automation comments"
        );
    }

    assert!(
        joy_template_value
            .get("joy")
            .and_then(Value::as_object)
            .is_some(),
        "Template should expose Joy defaults"
    );

    Ok(())
}
