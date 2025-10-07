use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
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

pub fn adapter_parity_report(output: Option<PathBuf>) -> Result<()> {
    let workspace = workspace_root();
    let material_components = collect_material_components(&workspace)?;
    let joy_components = collect_joy_components(&workspace)?;

    let timestamp = Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);
    let mut doc = String::new();
    doc.push_str("# Adapter Parity\n\n");
    doc.push_str(&format!(
        "_Last updated {timestamp} via `cargo xtask parity-report`._\n\n"
    ));
    doc.push_str("The tables below enumerate which framework adapters ship for each component. ");
    doc.push_str("Material adapters are discovered by scanning the adapter modules under `crates/rustic-ui-material/src`, and the Joy rows come from the Yew-first modules declared in `crates/rustic-ui-joy/src/lib.rs`. ");
    doc.push_str("Parity is validated by the cross-adapter regression suites such as [`button_adapters.rs`](../../crates/rustic-ui-material/tests/button_adapters.rs) and [`joy_yew.rs`](../../crates/rustic-ui-material/tests/joy_yew.rs). Run `cargo xtask parity-report` after adding or removing adapters so CI can confirm this dashboard stays in sync.\n\n");

    doc.push_str("## Material adapters\n\n");
    doc.push_str(&coverage_summary(&material_components));
    doc.push_str("\n\n");
    doc.push_str(&render_table(&material_components));
    doc.push_str("\n\n");

    doc.push_str("## Joy adapters\n\n");
    doc.push_str(&coverage_summary(&joy_components));
    doc.push_str("\n\n");
    doc.push_str(&render_table(&joy_components));
    doc.push_str("\n");

    let output_path =
        output.unwrap_or_else(|| workspace.join("docs/architecture/adapter-parity.md"));
    fs::write(&output_path, doc).with_context(|| {
        format!(
            "failed to write adapter parity report to {}",
            output_path.display()
        )
    })?;

    Ok(())
}

fn collect_material_components(workspace: &Path) -> Result<Vec<ComponentCoverage>> {
    let src_dir = workspace.join("crates/rustic-ui-material/src");
    let mut components = Vec::new();
    let skip = [
        "lib.rs",
        "macros.rs",
        "render_helpers.rs",
        "style_helpers.rs",
        "telemetry.rs",
    ];

    for entry in WalkDir::new(&src_dir).max_depth(1) {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.into_path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
            continue;
        }
        let file_name = match path.file_name().and_then(|name| name.to_str()) {
            Some(name) => name,
            None => continue,
        };
        if skip.contains(&file_name) {
            continue;
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

        let name = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .map(to_title_case)
            .unwrap_or_else(|| "Unknown".to_string());
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
