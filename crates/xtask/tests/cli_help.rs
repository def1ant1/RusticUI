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
        .stdout(predicate::str::contains(
            "extended nightly accessibility coverage suite",
        ))
        .stdout(predicate::str::contains("docs section"));

    Ok(())
}

#[test]
fn update_components_help_highlights_packages_and_env() -> Result<()> {
    let workspace = workspace_root();
    let mut cmd = Command::new("cargo");
    cmd.current_dir(&workspace)
        .arg("run")
        .arg("-p")
        .arg("xtask")
        .arg("--")
        .arg("update-components")
        .arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("packages"))
        .stdout(predicate::str::contains("RUSTIC_UI_COMPONENT_CONFIG"));

    Ok(())
}

#[test]
fn icons_bundle_help_mentions_schema_bridge() -> Result<()> {
    let workspace = workspace_root();
    let mut cmd = Command::new("cargo");
    cmd.current_dir(&workspace)
        .arg("run")
        .arg("-p")
        .arg("xtask")
        .arg("--")
        .arg("icons-bundle")
        .arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Rust-native bundler"))
        .stdout(predicate::str::contains("legacy_packages"));

    Ok(())
}

#[test]
fn examples_help_highlights_layout_group() -> Result<()> {
    let workspace = workspace_root();
    let mut cmd = Command::new("cargo");
    cmd.current_dir(&workspace)
        .arg("run")
        .arg("-p")
        .arg("xtask")
        .arg("--")
        .arg("examples")
        .arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("curated Rust example collections"))
        .stdout(predicate::str::contains("layout-box-leptos"))
        .stdout(predicate::str::contains("selection-controls-leptos"));

    Ok(())
}

#[test]
fn docs_build_help_mentions_cache_hint() -> Result<()> {
    let workspace = workspace_root();
    let mut cmd = Command::new("cargo");
    cmd.current_dir(&workspace)
        .arg("run")
        .arg("-p")
        .arg("xtask")
        .arg("--")
        .arg("docs-build")
        .arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("target/rustic-docs-wasm"))
        .stdout(predicate::str::contains("CARGO_TARGET_DIR"));

    Ok(())
}

#[test]
fn docs_test_help_mentions_playwright() -> Result<()> {
    let workspace = workspace_root();
    let mut cmd = Command::new("cargo");
    cmd.current_dir(&workspace)
        .arg("run")
        .arg("-p")
        .arg("xtask")
        .arg("--")
        .arg("docs-test")
        .arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("wasm-pack"))
        .stdout(predicate::str::contains("Playwright"))
        .stdout(predicate::str::contains("docs-test.log"));

    Ok(())
}

#[test]
fn docs_package_help_mentions_export_dir() -> Result<()> {
    let workspace = workspace_root();
    let mut cmd = Command::new("cargo");
    cmd.current_dir(&workspace)
        .arg("run")
        .arg("-p")
        .arg("xtask")
        .arg("--")
        .arg("docs-package")
        .arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("deploy-ready Rustic docs"))
        .stdout(predicate::str::contains("RUSTIC_DOCS_EXPORT_DIR"))
        .stdout(predicate::str::contains("CARGO_TARGET_DIR"))
        .stdout(predicate::str::contains("hashed manifest"))
        .stdout(predicate::str::contains("--dry-run"));

    Ok(())
}

#[test]
fn build_docs_help_mentions_concurrency() -> Result<()> {
    let workspace = workspace_root();
    let mut cmd = Command::new("cargo");
    cmd.current_dir(&workspace)
        .arg("run")
        .arg("-p")
        .arg("xtask")
        .arg("--")
        .arg("build-docs")
        .arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("docs::build helper"))
        .stdout(predicate::str::contains("CARGO_TARGET_DIR"))
        .stdout(predicate::str::contains("errors unchanged"));

    Ok(())
}

#[test]
fn deploy_docs_help_mentions_env_overrides() -> Result<()> {
    let workspace = workspace_root();
    let mut cmd = Command::new("cargo");
    cmd.current_dir(&workspace)
        .arg("run")
        .arg("-p")
        .arg("xtask")
        .arg("--")
        .arg("deploy-docs")
        .arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Stage API docs"))
        .stdout(predicate::str::contains("RUSTIC_UI_DEPLOY_OUTPUT"))
        .stdout(predicate::str::contains("--dry-run"));

    Ok(())
}

#[test]
fn accessibility_audit_help_mentions_config_manifest() -> Result<()> {
    let workspace = workspace_root();
    let mut cmd = Command::new("cargo");
    cmd.current_dir(&workspace)
        .arg("run")
        .arg("-p")
        .arg("xtask")
        .arg("--")
        .arg("accessibility-audit")
        .arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Markdown-first accessibility"))
        .stdout(predicate::str::contains("RUSTIC_UI_A11Y_CONFIG"));

    Ok(())
}
