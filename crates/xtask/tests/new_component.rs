//! Integration tests covering the `cargo xtask new-component` scaffolding flow.
//!
//! The tests focus on dry-run output so we avoid mutating the repository while
//! still asserting that the generator references every surface (Rust modules,
//! TypeScript adapters, docs stubs, and tests).

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
fn dry_run_lists_every_output_path() -> Result<()> {
    let workspace = workspace_root();
    let mut cmd = Command::new("cargo");
    cmd.current_dir(&workspace)
        .arg("run")
        .arg("-p")
        .arg("xtask")
        .arg("--")
        .arg("new-component")
        .arg("data-dashboard")
        .arg("--dry-run");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "crates/rustic-ui-material/src/data_dashboard.rs",
        ))
        .stdout(predicate::str::contains(
            "crates/rustic-ui-headless/src/data_dashboard.rs",
        ))
        .stdout(predicate::str::contains(
            "packages/mui-material/src/DataDashboard/RusticAdapter.tsx",
        ))
        .stdout(predicate::str::contains(
            "packages/mui-material/src/DataDashboard/RusticAdapter.stories.tsx",
        ))
        .stdout(predicate::str::contains(
            "packages/mui-material/src/DataDashboard/RusticAdapter.spec.tsx",
        ))
        .stdout(predicate::str::contains(
            "docs/src/pages/system/components/data-dashboard.mdx",
        ));

    Ok(())
}

#[test]
fn material_only_omits_headless_outputs() -> Result<()> {
    let workspace = workspace_root();
    let mut cmd = Command::new("cargo");
    cmd.current_dir(&workspace)
        .arg("run")
        .arg("-p")
        .arg("xtask")
        .arg("--")
        .arg("new-component")
        .arg("headless-free")
        .arg("--dry-run")
        .arg("--material-only");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "crates/rustic-ui-material/src/headless_free.rs",
        ))
        .stdout(predicate::str::contains(
            "packages/mui-material/src/HeadlessFree/RusticAdapter.tsx",
        ))
        .stdout(predicate::str::contains(
            "packages/mui-material/src/HeadlessFree/RusticAdapter.stories.tsx",
        ))
        .stdout(predicate::str::contains(
            "packages/mui-material/src/HeadlessFree/RusticAdapter.spec.tsx",
        ))
        .stdout(predicate::str::contains(
            "docs/src/pages/system/components/headless-free.mdx",
        ))
        .stdout(predicate::str::contains("Material Rust module"))
        .stdout(predicate::str::contains(
            "React/TypeScript adapter telemetry helper",
        ))
        .stdout(predicate::str::contains("Docs MDX stub"))
        .stdout(predicate::str::contains("templates ready"))
        .stdout(predicate::str::contains("crates/rustic-ui-headless/src/headless_free.rs").not());

    Ok(())
}

#[test]
fn conflicting_skip_flags_error() -> Result<()> {
    let workspace = workspace_root();
    let mut cmd = Command::new("cargo");
    cmd.current_dir(&workspace)
        .arg("run")
        .arg("-p")
        .arg("xtask")
        .arg("--")
        .arg("new-component")
        .arg("bad")
        .arg("--material-only")
        .arg("--headless-only")
        .arg("--dry-run");

    cmd.assert().failure().stderr(predicate::str::contains(
        "cannot be used with one or more of the other specified arguments",
    ));

    Ok(())
}
