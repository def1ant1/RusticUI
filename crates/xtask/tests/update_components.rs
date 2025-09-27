use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::Value;
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

#[test]
fn update_components_generates_manifest_from_fixture() -> Result<()> {
    let workspace = workspace_root();
    let config = workspace.join("crates/xtask/tests/fixtures/component_config.json");
    assert!(config.exists(), "fixture missing: {config:?}");

    let temp = tempdir()?;
    let out_dir = temp.path().join("component-metadata");

    let mut cmd = Command::new("cargo");
    let assertion = cmd
        .current_dir(&workspace)
        .arg("xtask")
        .arg("update-components")
        .env("RUSTIC_UI_COMPONENT_CONFIG", &config)
        .env("RUSTIC_UI_COMPONENT_OUT_DIR", &out_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("packages=1"))
        .stdout(predicate::str::contains("components=1"))
        .stdout(predicate::str::contains("props=3"));

    let stdout = String::from_utf8_lossy(&assertion.get_output().stdout);
    assert!(
        stdout.contains("[xtask][update-components]"),
        "expected log output to document the manifest path"
    );

    let manifest_path = out_dir.join("component-metadata.json");
    assert!(
        manifest_path.exists(),
        "manifest missing: {manifest_path:?}"
    );

    let manifest_json = fs::read_to_string(&manifest_path)?;
    let manifest: Value = serde_json::from_str(&manifest_json)?;
    assert_eq!(
        manifest["schema"].as_str(),
        Some("rustic-ui/component-metadata@v1"),
        "schema should advertise the Rust-native manifest"
    );

    let packages = manifest["packages"]
        .as_array()
        .expect("packages array present");
    assert_eq!(packages.len(), 1, "expected exactly one package in fixture");

    let package = &packages[0];
    assert_eq!(
        package["package"].as_str(),
        Some("rustic-ui-fixture"),
        "package identifier should match fixture configuration"
    );
    assert!(
        package["packages"]
            .as_array()
            .expect("packages list present")
            .iter()
            .any(|value| value == "rustic-ui-fixture"),
        "package metadata should include the RusticUI identifier"
    );
    assert!(
        package["legacy_packages"]
            .as_array()
            .expect("legacy packages present")
            .iter()
            .any(|value| value == "@mui/fixture"),
        "legacy metadata should be preserved for archives"
    );

    let components = package["components"]
        .as_array()
        .expect("components array present");
    assert_eq!(components.len(), 1, "expected a single component entry");

    let component = &components[0];
    assert_eq!(component["component"].as_str(), Some("Button"));
    let interfaces = component["interfaces"]
        .as_array()
        .expect("interfaces array present");
    assert_eq!(interfaces.len(), 1, "expected a single interface entry");
    assert_eq!(
        interfaces[0]["interface"].as_str(),
        Some("ButtonOwnProps"),
        "interface name should mirror the source declaration"
    );

    let props = interfaces[0]["props"].as_array().expect("props present");
    assert_eq!(props.len(), 3, "expected three props from the fixture");
    assert_eq!(props[0]["name"], Value::String("color".into()));
    assert_eq!(props[0]["optional"].as_bool(), Some(true));
    assert_eq!(
        props[0]["type"],
        Value::String("'primary' | 'secondary'".into())
    );

    assert_eq!(props[1]["name"], Value::String("label".into()));
    assert_eq!(props[1]["optional"].as_bool(), Some(false));
    assert_eq!(props[1]["type"], Value::String("string".into()));

    assert_eq!(props[2]["name"], Value::String("onClick".into()));
    assert_eq!(props[2]["optional"].as_bool(), Some(true));
    assert_eq!(props[2]["type"].as_str(), Some("function"));

    Ok(())
}
