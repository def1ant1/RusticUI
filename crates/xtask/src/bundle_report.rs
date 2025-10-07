use crate::{relative_display, run, workspace_root};
use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Utc};
use clap::Args;
use serde::Serialize;
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

/// Command line arguments for the bundle-size reporter.
#[derive(Args, Debug, Clone)]
pub struct BundleReportArgs {
    /// Directory receiving the machine-readable exports.
    #[arg(long, value_name = "PATH")]
    pub out_dir: Option<PathBuf>,

    /// Target directory used for compilation. Defaults to `target/bundle-report`.
    #[arg(long, value_name = "PATH")]
    pub target_dir: Option<PathBuf>,
}

/// Run the bundle-size reporter and emit Markdown + JSON summaries.
pub(crate) fn bundle_report(args: BundleReportArgs) -> Result<()> {
    let workspace = workspace_root();
    let target_dir = args
        .target_dir
        .clone()
        .unwrap_or_else(|| workspace.join("target").join("bundle-report"));
    fs::create_dir_all(&target_dir).with_context(|| {
        format!(
            "failed to prepare bundle-report target directory at {}",
            target_dir.display()
        )
    })?;

    let out_dir = args
        .out_dir
        .clone()
        .unwrap_or_else(|| workspace.join("test-results").join("bundle-size"));
    fs::create_dir_all(&out_dir).with_context(|| {
        format!(
            "failed to prepare bundle-report output directory at {}",
            out_dir.display()
        )
    })?;

    let docs_dir = workspace.join("docs").join("performance");
    fs::create_dir_all(&docs_dir).with_context(|| {
        format!(
            "failed to prepare documentation directory at {}",
            docs_dir.display()
        )
    })?;

    let now = SystemTime::now();
    let generated_at: DateTime<Utc> = now.into();
    let generated_at_unix = now.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();

    let scenarios = scenarios();
    let mut measurements = Vec::new();

    println!(
        "[xtask][bundle-report] compiling {} feature matrices",
        scenarios.len()
    );

    for spec in &scenarios {
        let measurement = compile_scenario(&workspace, &target_dir, spec)?;
        measurements.push(measurement);
    }

    measurements.sort_by(|a, b| {
        (a.crate_name.clone(), a.display_name.clone())
            .cmp(&(b.crate_name.clone(), b.display_name.clone()))
    });

    let mut baselines = BTreeMap::new();
    for measurement in &measurements {
        if measurement.is_baseline {
            baselines.insert(measurement.crate_name.clone(), measurement.size_bytes);
        }
    }

    let mut enriched = Vec::new();
    for mut measurement in measurements {
        if let Some(base) = baselines.get(&measurement.crate_name) {
            let delta = measurement.size_bytes as i64 - *base as i64;
            let delta_kib = delta as f64 / 1024.0;
            let delta_percent = if *base == 0 {
                None
            } else {
                Some((measurement.size_bytes as f64 / *base as f64 - 1.0) * 100.0)
            };

            measurement.delta_bytes = Some(delta);
            measurement.delta_kib = Some(delta_kib);
            measurement.delta_percent = delta_percent;
        }

        enriched.push(measurement);
    }

    let report = BundleReport {
        generated_at: generated_at.to_rfc3339(),
        generated_at_unix,
        target_dir: relative_display(&workspace, &target_dir),
        scenarios: enriched,
        notes: default_notes(),
    };

    let json = serde_json::to_string_pretty(&report)?;
    let json_path = out_dir.join("bundle-report.json");
    fs::write(&json_path, json.as_bytes()).with_context(|| {
        format!(
            "failed to write bundle-report JSON to {}",
            json_path.display()
        )
    })?;
    println!(
        "[xtask][bundle-report] wrote JSON summary to {}",
        relative_display(&workspace, &json_path)
    );

    let markdown = render_markdown(&report);
    let markdown_path = out_dir.join("bundle-report.md");
    fs::write(&markdown_path, markdown.as_bytes()).with_context(|| {
        format!(
            "failed to write bundle-report Markdown to {}",
            markdown_path.display()
        )
    })?;
    println!(
        "[xtask][bundle-report] wrote Markdown summary to {}",
        relative_display(&workspace, &markdown_path)
    );

    let docs_markdown_path = docs_dir.join("bundle-costs.md");
    fs::write(&docs_markdown_path, markdown.as_bytes()).with_context(|| {
        format!(
            "failed to update documentation at {}",
            docs_markdown_path.display()
        )
    })?;
    println!(
        "[xtask][bundle-report] refreshed docs at {}",
        relative_display(&workspace, &docs_markdown_path)
    );

    let docs_json_path = docs_dir.join("bundle-costs.json");
    fs::write(&docs_json_path, json.as_bytes()).with_context(|| {
        format!(
            "failed to update documentation JSON at {}",
            docs_json_path.display()
        )
    })?;

    Ok(())
}

fn compile_scenario(
    workspace: &Path,
    target_dir: &Path,
    spec: &BundleScenario,
) -> Result<ScenarioMeasurement> {
    println!(
        "[xtask][bundle-report] measuring {} ({})",
        spec.display_name, spec.command_summary
    );
    let mut cmd = Command::new("cargo");
    cmd.current_dir(workspace)
        .arg("build")
        .arg("--release")
        .arg("--package")
        .arg(spec.crate_name)
        .arg("--target-dir")
        .arg(target_dir);

    if !spec.default_features {
        cmd.arg("--no-default-features");
    }

    if !spec.features.is_empty() {
        cmd.arg("--features");
        cmd.arg(spec.features.join(","));
    }

    run(cmd)?;

    let artifact = find_rlib(target_dir, spec.crate_name)?;
    let size_bytes = artifact
        .metadata()
        .with_context(|| format!("failed to read metadata for {}", artifact.display()))?
        .len();

    let artifact_display = relative_display(workspace, &artifact);

    Ok(ScenarioMeasurement {
        id: spec.id.to_string(),
        crate_name: spec.crate_name.to_string(),
        display_name: spec.display_name.to_string(),
        description: spec.description.to_string(),
        default_features: spec.default_features,
        features: spec.features.iter().map(|f| f.to_string()).collect(),
        feature_label: spec.feature_label.to_string(),
        command_summary: spec.command_summary.to_string(),
        is_baseline: spec.is_baseline,
        size_bytes,
        size_kib: size_bytes as f64 / 1024.0,
        artifact: artifact_display,
        delta_bytes: None,
        delta_kib: None,
        delta_percent: None,
    })
}

fn find_rlib(target_dir: &Path, crate_name: &str) -> Result<PathBuf> {
    let deps_dir = target_dir.join("release").join("deps");
    let prefix = format!("lib{}", crate_name.replace('-', "_"));
    let mut candidates = Vec::new();

    for entry in fs::read_dir(&deps_dir).with_context(|| {
        format!(
            "failed to list compiled artifacts in {}",
            deps_dir.display()
        )
    })? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("rlib") {
            continue;
        }
        if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
            if !file_name.starts_with(&prefix) {
                continue;
            }
        }

        let modified = entry
            .metadata()
            .and_then(|meta| meta.modified())
            .unwrap_or(UNIX_EPOCH);
        candidates.push((modified, path));
    }

    candidates.sort_by_key(|(modified, _)| *modified);

    let (_, path) = candidates
        .pop()
        .ok_or_else(|| anyhow!("compiled artifact for {crate_name} not found"))?;
    Ok(path)
}

fn render_markdown(report: &BundleReport) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "# RusticUI bundle cost report");
    out.push('\n');
    let _ = writeln!(
        out,
        "Generated at {} (unix: {}).",
        report.generated_at, report.generated_at_unix
    );
    out.push('\n');
    let _ = writeln!(
        out,
        "Artifacts compiled with release profile under `{}`.",
        report.target_dir
    );
    out.push('\n');

    let _ = writeln!(
        out,
        "| Scenario | Crate | Features | Size (KiB) | Δ KiB | Δ % | Artifact | Notes |"
    );
    let _ = writeln!(
        out,
        "|----------|-------|----------|-----------:|------:|----:|----------|-------|"
    );

    for scenario in &report.scenarios {
        let delta_kib = scenario
            .delta_kib
            .map(|value| format!("{value:.2}"))
            .unwrap_or_else(|| "0.00".into());
        let delta_percent = scenario
            .delta_percent
            .map(|value| format!("{value:.2}"))
            .unwrap_or_else(|| "0.00".into());

        let _ = writeln!(
            out,
            "| {} | {} | {} | {:.2} | {} | {} | `{}` | {} |",
            scenario.display_name,
            scenario.crate_name,
            scenario.feature_label,
            scenario.size_kib,
            delta_kib,
            delta_percent,
            scenario.artifact,
            scenario.description
        );
    }

    out.push('\n');
    let _ = writeln!(out, "## Methodology");
    out.push('\n');
    for note in &report.notes {
        let _ = writeln!(out, "- {}", note);
    }

    out
}

fn default_notes() -> Vec<String> {
    vec![
        "Sizes capture release-mode .rlib artifacts compiled on the CI host triple.".into(),
        "Run `cargo xtask bundle-report` to refresh the data before shipping feature-flag changes.".into(),
        "The generated Markdown feeds docs/performance/bundle-costs.md so engineering docs stay in sync with telemetry.".into(),
        "Baseline measurements enable the `forms` feature because headless/material crates reference shared form utilities even wh
en other flags are disabled.".into(),
    ]
}

#[derive(Debug, Clone)]
struct BundleScenario {
    id: &'static str,
    crate_name: &'static str,
    display_name: &'static str,
    description: &'static str,
    default_features: bool,
    features: &'static [&'static str],
    feature_label: &'static str,
    command_summary: &'static str,
    is_baseline: bool,
}

#[derive(Debug, Serialize, Clone)]
struct ScenarioMeasurement {
    id: String,
    crate_name: String,
    display_name: String,
    description: String,
    default_features: bool,
    features: Vec<String>,
    feature_label: String,
    command_summary: String,
    is_baseline: bool,
    size_bytes: u64,
    size_kib: f64,
    artifact: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    delta_bytes: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delta_kib: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delta_percent: Option<f64>,
}

#[derive(Debug, Serialize)]
struct BundleReport {
    generated_at: String,
    generated_at_unix: u64,
    target_dir: String,
    scenarios: Vec<ScenarioMeasurement>,
    notes: Vec<String>,
}

fn scenarios() -> Vec<BundleScenario> {
    const FORMS_ONLY: &[&str] = &["forms"];
    const FORMS_FEEDBACK: &[&str] = &["forms", "feedback"];
    const FORMS_PROGRESS: &[&str] = &["forms", "progress"];
    const FULL_OPTIONAL: &[&str] = &["forms", "feedback", "progress"];

    vec![
        BundleScenario {
            id: "headless-forms",
            crate_name: "rustic-ui-headless",
            display_name: "Headless (forms core)",
            description: "Baseline surface with form controls enabled. Modules like select/text-field require this feature to compile.",
            default_features: false,
            features: FORMS_ONLY,
            feature_label: "forms",
            command_summary: "cargo build -p rustic-ui-headless --release --no-default-features --features forms",
            is_baseline: true,
        },
        BundleScenario {
            id: "headless-feedback",
            crate_name: "rustic-ui-headless",
            display_name: "Headless (forms + feedback)",
            description: "Adds snackbar, rating and feedback primitives.",
            default_features: false,
            features: FORMS_FEEDBACK,
            feature_label: "forms, feedback",
            command_summary:
                "cargo build -p rustic-ui-headless --release --no-default-features --features forms,feedback",
            is_baseline: false,
        },
        BundleScenario {
            id: "headless-progress",
            crate_name: "rustic-ui-headless",
            display_name: "Headless (forms + progress)",
            description: "Includes determinate and indeterminate progress indicators.",
            default_features: false,
            features: FORMS_PROGRESS,
            feature_label: "forms, progress",
            command_summary:
                "cargo build -p rustic-ui-headless --release --no-default-features --features forms,progress",
            is_baseline: false,
        },
        BundleScenario {
            id: "headless-default",
            crate_name: "rustic-ui-headless",
            display_name: "Headless (default)",
            description: "Full optional surface mirroring the published crate defaults.",
            default_features: true,
            features: FULL_OPTIONAL,
            feature_label: "default features",
            command_summary: "cargo build -p rustic-ui-headless --release",
            is_baseline: false,
        },
        BundleScenario {
            id: "material-forms",
            crate_name: "rustic-ui-material",
            display_name: "Material (forms core)",
            description:
                "Baseline Material renderers require form controls; this matches the smallest supported feature matrix.",
            default_features: false,
            features: FORMS_ONLY,
            feature_label: "forms",
            command_summary:
                "cargo build -p rustic-ui-material --release --no-default-features --features forms",
            is_baseline: true,
        },
        BundleScenario {
            id: "material-feedback",
            crate_name: "rustic-ui-material",
            display_name: "Material (forms + feedback)",
            description: "Activates alert, backdrop and snackbar renderers.",
            default_features: false,
            features: FORMS_FEEDBACK,
            feature_label: "forms, feedback",
            command_summary:
                "cargo build -p rustic-ui-material --release --no-default-features --features forms,feedback",
            is_baseline: false,
        },
        BundleScenario {
            id: "material-progress",
            crate_name: "rustic-ui-material",
            display_name: "Material (forms + progress)",
            description: "Adds linear/circular progress components and skeleton loaders.",
            default_features: false,
            features: FORMS_PROGRESS,
            feature_label: "forms, progress",
            command_summary:
                "cargo build -p rustic-ui-material --release --no-default-features --features forms,progress",
            is_baseline: false,
        },
        BundleScenario {
            id: "material-default",
            crate_name: "rustic-ui-material",
            display_name: "Material (default)",
            description: "Full optional surface mirroring the published crate defaults.",
            default_features: true,
            features: FULL_OPTIONAL,
            feature_label: "default features",
            command_summary: "cargo build -p rustic-ui-material --release",
            is_baseline: false,
        },
    ]
}
