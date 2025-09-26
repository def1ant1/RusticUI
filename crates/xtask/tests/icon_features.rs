//! Integration coverage for the icon feature manifest generator. The scenarios
//! emulate contributors adding a brand new icon set to ensure our automation
//! rewrites `Cargo.toml` deterministically and logs actionable context for CI.

use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const START_MARKER: &str = "# BEGIN AUTO-GENERATED ICON FEATURES -- DO NOT EDIT.";
const END_MARKER: &str = "# END AUTO-GENERATED ICON FEATURES";

/// Captures and restores a file so repository state remains pristine after tests.
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

/// Removes a directory created for test isolation.
struct DirGuard {
    path: PathBuf,
    existed: bool,
}

impl DirGuard {
    fn new(path: PathBuf) -> Self {
        let existed = path.exists();
        Self { path, existed }
    }
}

impl Drop for DirGuard {
    fn drop(&mut self) {
        if self.existed {
            return;
        }
        if let Err(error) = fs::remove_dir_all(&self.path) {
            if error.kind() != io::ErrorKind::NotFound {
                eprintln!(
                    "[test] failed to remove fixture directory {}: {error}",
                    self.path.display()
                );
            }
        }
    }
}

/// Computes the workspace root using the same logic as the production binary.
fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("xtask is nested two levels below the workspace root")
        .to_path_buf()
}

/// Extracts the auto-generated feature block from the manifest for assertions.
fn feature_block(manifest: &str) -> &str {
    let start = manifest
        .find(START_MARKER)
        .expect("manifest should contain start marker");
    let end = manifest
        .find(END_MARKER)
        .expect("manifest should contain end marker");
    &manifest[start..end + END_MARKER.len()]
}

#[test]
fn new_icon_sets_update_the_manifest_deterministically() -> Result<()> {
    let workspace = workspace_root();
    let manifest_path = workspace.join("crates/rustic-ui-icons/Cargo.toml");
    let icons_root = workspace.join("crates/rustic-ui-icons/icons");
    let fixture_set = icons_root.join("__xtask_fixture");

    assert!(
        !fixture_set.exists(),
        "fixture set should not collide with real icon sets"
    );
    fs::create_dir_all(&fixture_set)?;
    let _dir_guard = DirGuard::new(fixture_set.clone());

    // Author the icons in reverse alphabetical order to ensure the generator
    // performs its own sorting and does not depend on filesystem iteration
    // semantics.
    fs::write(fixture_set.join("zeta.svg"), "<svg></svg>")?;
    fs::write(fixture_set.join("alpha.svg"), "<svg></svg>")?;

    let snapshot = FileSnapshot::new(manifest_path.clone());

    // Execute the generator through Cargo the same way the xtask pipeline does.
    let mut cmd = Command::new("cargo");
    let assertion = cmd
        .current_dir(&workspace)
        .arg("run")
        .arg("-p")
        .arg("mui-icons")
        .arg("--bin")
        .arg("update_features")
        .assert()
        .success()
        .stdout(predicate::str::contains("__xtask_fixture"))
        .stdout(predicate::str::contains(
            "manifest features refreshed successfully",
        ));

    let stdout = String::from_utf8_lossy(&assertion.get_output().stdout);
    assert!(
        stdout.contains("[mui-icons] regenerating feature manifest"),
        "generator output should provide context for maintainers"
    );

    let manifest_after_first_run = fs::read_to_string(&manifest_path)?;
    let block = feature_block(&manifest_after_first_run);

    assert!(
        block.contains("\"set-__xtask_fixture\""),
        "all-icons aggregate should reference the new set"
    );
    assert!(
        block.contains("set-__xtask_fixture = ["),
        "set feature should be emitted for the fixture"
    );
    let alpha_index = block
        .find("icon-__xtask_fixture-alpha")
        .expect("alpha icon should be present");
    let zeta_index = block
        .find("icon-__xtask_fixture-zeta")
        .expect("zeta icon should be present");
    assert!(
        alpha_index < zeta_index,
        "icons must be sorted alphabetically"
    );

    // Running the generator again without changes should be a no-op.
    let mut second = Command::new("cargo");
    second
        .current_dir(&workspace)
        .arg("run")
        .arg("-p")
        .arg("mui-icons")
        .arg("--bin")
        .arg("update_features")
        .assert()
        .success()
        .stdout(predicate::str::contains("already up to date"));

    let manifest_after_second_run = fs::read_to_string(&manifest_path)?;
    assert_eq!(
        manifest_after_first_run, manifest_after_second_run,
        "second invocation should not rewrite the manifest"
    );

    drop(snapshot);
    Ok(())
}
