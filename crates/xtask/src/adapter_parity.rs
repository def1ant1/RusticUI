use std::collections::BTreeSet;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use chrono::{SecondsFormat, Utc};
use regex::Regex;
use walkdir::WalkDir;

use crate::workspace_root;

const FRAMEWORKS: [(&str, &str); 5] = [
    ("react", "React"),
    ("yew", "Yew"),
    ("leptos", "Leptos"),
    ("dioxus", "Dioxus"),
    ("sycamore", "Sycamore"),
];

#[derive(Debug, Clone)]
struct ComponentCoverage {
    name: String,
    frameworks: BTreeSet<String>,
}

pub fn adapter_parity_report(output: Option<PathBuf>, check: bool) -> Result<()> {
    let workspace = workspace_root();
    let material_components = collect_material_components(&workspace)?;
    let joy_components = collect_joy_components(&workspace)?;

    let output_path =
        output.unwrap_or_else(|| workspace.join("docs/architecture/adapter-parity.md"));

    let current_timestamp = Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);
    let existing = if check {
        Some(fs::read_to_string(&output_path).unwrap_or_default())
    } else {
        None
    };
    let timestamp = existing
        .as_ref()
        .and_then(|doc| extract_timestamp(doc))
        .unwrap_or(current_timestamp);
    let doc = build_report(timestamp, &material_components, &joy_components);

    if check {
        // The `--check` flag is wired into CI so contributors cannot forget to
        // refresh the parity dashboard after adding or removing adapters. We
        // read the current file (if any) and compare it to the freshly
        // generated payload without mutating the workspace. A mismatch produces
        // a targeted error message that points developers at the automated
        // regeneration command rather than leaving them to guess why CI failed.
        if existing.unwrap_or_default() != doc {
            bail!(
                "adapter parity dashboard is stale; run `cargo xtask parity-report` and commit the refreshed docs"
            );
        }
        return Ok(());
    }

    fs::write(&output_path, doc).with_context(|| {
        format!(
            "failed to write adapter parity report to {}",
            output_path.display()
        )
    })?;

    Ok(())
}

fn build_report(
    timestamp: String,
    material_components: &[ComponentCoverage],
    joy_components: &[ComponentCoverage],
) -> String {
    let mut doc = String::new();
    doc.push_str("# Adapter Parity\n\n");
    doc.push_str(&format!(
        "_Last updated {timestamp} via `cargo xtask parity-report`._\n\n"
    ));
    doc.push_str("The tables below enumerate which framework adapters ship for each component. ");
    doc.push_str("Material adapters are discovered by scanning the adapter modules under `crates/rustic-ui-material/src`, and the Joy rows come from the Yew-first modules declared in `crates/rustic-ui-joy/src/lib.rs`. ");
    doc.push_str("Parity is validated by the cross-adapter regression suites such as [`button_adapters.rs`](../../crates/rustic-ui-material/tests/button_adapters.rs) and [`joy_yew.rs`](../../crates/rustic-ui-material/tests/joy_yew.rs). Run `cargo xtask parity-report` after adding or removing adapters, and `cargo xtask parity-report --check` in CI, so this dashboard stays in sync.\n\n");

    doc.push_str("## Material adapters\n\n");
    doc.push_str(&coverage_summary(material_components));
    doc.push_str("\n\n");
    doc.push_str(&render_table(material_components));
    doc.push_str("\n\n");

    doc.push_str("## Joy adapters\n\n");
    doc.push_str(&coverage_summary(joy_components));
    doc.push_str("\n\n");
    doc.push_str(&render_table(joy_components));
    doc.push_str("\n");

    doc
}

fn extract_timestamp(doc: &str) -> Option<String> {
    let regex = Regex::new(r"^_Last updated ([^ ]+) via `cargo xtask parity-report`\._$").ok()?;
    doc.lines()
        .find_map(|line| regex.captures(line).map(|caps| caps[1].to_string()))
}

fn collect_material_components(workspace: &Path) -> Result<Vec<ComponentCoverage>> {
    let src_dir = workspace.join("crates/rustic-ui-material/src");
    let mut components = Vec::new();
    let skip = [
        "lib",
        "macros",
        "render_helpers",
        "style_helpers",
        "telemetry",
    ];

    for entry in WalkDir::new(&src_dir).max_depth(2) {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.into_path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
            continue;
        }
        let parent = match path.parent() {
            Some(parent) => parent,
            None => continue,
        };
        let is_root_file = parent == src_dir;
        let is_mod_rs = path
            .file_name()
            .and_then(OsStr::to_str)
            .map(|name| name == "mod.rs")
            .unwrap_or(false);

        if !is_root_file && !is_mod_rs {
            // Skip helper submodules (`box/layout.rs`) so we only count the
            // primary component entry points. Those helpers are accounted for
            // indirectly via the `mod.rs` dispatcher.
            continue;
        }

        let component_key = if is_root_file {
            match path.file_stem().and_then(|stem| stem.to_str()) {
                Some(stem) => stem.to_owned(),
                None => continue,
            }
        } else {
            match parent.file_name().and_then(|segment| segment.to_str()) {
                Some(segment) => segment.to_owned(),
                None => continue,
            }
        };

        if skip.iter().any(|item| *item == component_key.as_str()) {
            continue;
        }

        // Only accept `mod.rs` files that sit directly under the src directory so
        // nested implementation modules like `accordion/details.rs` do not show up
        // as standalone components.
        if is_mod_rs {
            if parent.parent().map(|p| p != src_dir).unwrap_or(true) {
                continue;
            }
        }

        let content = fs::read_to_string(&path).with_context(|| {
            format!(
                "failed to read material component source {}",
                path.display()
            )
        })?;
        let mut frameworks = BTreeSet::new();
        for (slug, _) in FRAMEWORKS {
            let marker = format!("pub mod {slug}");
            if content.contains(&marker) {
                frameworks.insert(slug.to_string());
            }
        }

        if frameworks.is_empty() {
            continue;
        }

        let name = to_title_case(&component_key);
        components.push(ComponentCoverage { name, frameworks });
    }

    components.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(components)
}

fn collect_joy_components(workspace: &Path) -> Result<Vec<ComponentCoverage>> {
    let lib_path = workspace.join("crates/rustic-ui-joy/src/lib.rs");
    let content = fs::read_to_string(&lib_path)
        .with_context(|| format!("failed to read joy lib {}", lib_path.display()))?;
    let module_regex = Regex::new(
        r#"(?m)#\s*\[\s*cfg\(feature\s*=\s*"yew"\s*\)\s*]\s*pub\s+mod\s+([a-z0-9_]+)\s*;"#,
    )?;
    let mut components = Vec::new();
    for capture in module_regex.captures_iter(&content) {
        let module = &capture[1];
        let name = to_title_case(module);
        let mut frameworks = BTreeSet::new();
        frameworks.insert("yew".to_string());
        components.push(ComponentCoverage { name, frameworks });
    }

    components.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(components)
}

fn coverage_summary(components: &[ComponentCoverage]) -> String {
    if components.is_empty() {
        return "_No adapters discovered._".to_string();
    }

    let total = components.len();
    let mut lines = Vec::new();
    for (slug, label) in FRAMEWORKS {
        let count = components
            .iter()
            .filter(|component| component.frameworks.contains(slug))
            .count();
        lines.push(format!("- {label} adapters: {count}/{total}"));
    }

    lines.join("\n")
}

fn render_table(components: &[ComponentCoverage]) -> String {
    if components.is_empty() {
        return "_No data available._".to_string();
    }

    let mut table = String::new();
    table.push_str("| Component | React | Yew | Leptos | Dioxus | Sycamore |\n");
    table.push_str("| --- | --- | --- | --- | --- | --- |\n");

    for component in components {
        table.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} |\n",
            component.name,
            coverage_cell(component, "react"),
            coverage_cell(component, "yew"),
            coverage_cell(component, "leptos"),
            coverage_cell(component, "dioxus"),
            coverage_cell(component, "sycamore"),
        ));
    }

    table
}

fn coverage_cell(component: &ComponentCoverage, framework: &str) -> &'static str {
    if component.frameworks.contains(framework) {
        "✅"
    } else {
        "⬜"
    }
}

fn to_title_case(value: &str) -> String {
    value
        .split('_')
        .filter(|segment| !segment.is_empty())
        .map(|segment| {
            let mut chars = segment.chars();
            match chars.next() {
                Some(first) => format!("{}{}", first.to_uppercase(), chars.as_str()),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn extract_timestamp_from_marker_line() {
        let doc = "_Last updated 2024-12-01T03:45:00Z via `cargo xtask parity-report`._";
        assert_eq!(
            extract_timestamp(doc).as_deref(),
            Some("2024-12-01T03:45:00Z")
        );
    }

    #[test]
    fn coverage_summary_reports_empty_state() {
        assert_eq!(coverage_summary(&[]), "_No adapters discovered._");
    }

    #[test]
    fn build_report_preserves_existing_timestamp_and_data() {
        let mut frameworks = BTreeSet::new();
        frameworks.insert("react".to_string());
        let timestamp = "2024-05-05T10:00:00Z".to_string();
        let doc = build_report(
            timestamp.clone(),
            &[ComponentCoverage {
                name: "Box".into(),
                frameworks: frameworks.clone(),
            }],
            &[],
        );
        assert!(doc.contains(&format!(
            "_Last updated {timestamp} via `cargo xtask parity-report`._"
        )));
        assert!(doc.contains("| Box | ✅ |"));
    }

    #[test]
    fn collect_material_components_handles_files_and_mod_rs() -> Result<()> {
        let tmp = TempDir::new()?;
        let workspace = tmp.path();
        let material_src = workspace.join("crates/rustic-ui-material/src");
        fs::create_dir_all(&material_src)?;

        // Direct file component.
        fs::write(
            material_src.join("box.rs"),
            "#[cfg(feature = \"react\")] pub mod react;\npub mod yew;",
        )?;

        // Directory with mod.rs dispatcher and helper module (should be ignored).
        let nested = material_src.join("app_bar");
        fs::create_dir_all(&nested)?;
        fs::write(
            nested.join("mod.rs"),
            "#[cfg(feature = \"dioxus\")] pub mod dioxus;",
        )?;
        fs::File::create(nested.join("react.rs"))?; // helper file ignored by scanner.

        let components = collect_material_components(workspace)?;
        assert_eq!(components.len(), 2);

        let app_bar = components
            .iter()
            .find(|component| component.name == "App Bar")
            .expect("App Bar component should be discovered");
        assert!(app_bar.frameworks.contains("dioxus"));
        assert_eq!(app_bar.frameworks.len(), 1);

        let boxed = components
            .iter()
            .find(|component| component.name == "Box")
            .expect("Box component should be discovered");
        assert!(boxed.frameworks.contains("react"));
        assert!(boxed.frameworks.contains("yew"));
        assert_eq!(boxed.frameworks.len(), 2);

        Ok(())
    }

    #[test]
    fn collect_joy_components_only_enrolls_yew_modules() -> Result<()> {
        let tmp = TempDir::new()?;
        let workspace = tmp.path();
        let joy_src = workspace.join("crates/rustic-ui-joy/src");
        fs::create_dir_all(&joy_src)?;
        let mut lib = fs::File::create(joy_src.join("lib.rs"))?;
        writeln!(
            lib,
            r#"
#[cfg(feature = "yew")]
pub mod accordion;

#[cfg(feature = "yew")]
pub mod button;

pub mod internal_only;
"#
        )?;

        let components = collect_joy_components(workspace)?;
        assert_eq!(components.len(), 2);
        assert!(components.iter().all(|component| {
            component.frameworks.len() == 1 && component.frameworks.contains("yew")
        }));
        assert!(components
            .iter()
            .any(|component| component.name == "Accordion"));
        assert!(components
            .iter()
            .any(|component| component.name == "Button"));

        Ok(())
    }
}
