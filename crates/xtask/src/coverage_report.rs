use crate::accessibility;
use crate::relative_display;
use crate::workspace_root;
use anyhow::{anyhow, bail, Context, Result};
use chrono::{DateTime, Utc};
use clap::Args;
use quick_xml::events::Event;
use quick_xml::Reader;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// Command-line flags for the coverage report aggregator.
///
/// The defaults intentionally match the automation layout used in CI so
/// maintainers can simply run `cargo xtask coverage-report` locally without
/// remembering extra arguments.  Hidden options exist exclusively for the
/// test harness where we hydrate deterministic fixtures instead of executing
/// heavyweight Playwright suites during `cargo test`.
#[derive(Args, Debug, Clone)]
pub struct CoverageReportArgs {
    /// Override the directory that receives the JSON/Markdown exports.
    #[arg(long, value_name = "PATH")]
    pub out_dir: Option<PathBuf>,

    /// Inject fixture data instead of running the real pipelines.  This keeps
    /// unit tests fast while still exercising the aggregation logic.
    #[arg(long, hide = true, value_name = "PATH")]
    pub fixtures: Option<PathBuf>,
}

/// Entry point invoked by the CLI dispatcher.
pub(crate) fn coverage_report(args: CoverageReportArgs) -> Result<()> {
    let workspace = workspace_root();
    let data_source = if let Some(fixtures) = args.fixtures.clone() {
        CoverageDataSource::Fixtures(fixtures)
    } else {
        CoverageDataSource::Workspace(workspace.clone())
    };

    let out_dir = args
        .out_dir
        .clone()
        .unwrap_or_else(|| workspace.join("test-results").join("coverage"));
    fs::create_dir_all(&out_dir).with_context(|| {
        format!(
            "failed to create coverage report output directory at {}",
            out_dir.display()
        )
    })?;

    let now = SystemTime::now();
    let timestamp = now.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
    let generated_at: DateTime<Utc> = now.into();

    let mut suites = Vec::new();
    suites.push(collect_rust_coverage(&data_source)?);
    suites.push(collect_typescript_coverage(&data_source)?);
    suites.push(collect_accessibility_signal(&data_source)?);
    suites.push(collect_visual_regressions(&data_source)?);
    suites.push(collect_adapter_visual_regressions(&data_source)?);

    let report = CoverageReport {
        generated_at: generated_at.to_rfc3339(),
        generated_at_unix: timestamp,
        suites,
        notes: default_notes(),
    };

    let json_path = out_dir.join("coverage-report.json");
    write_json(&json_path, &report)?;
    println!(
        "[xtask][coverage-report] wrote machine-readable summary to {}",
        relative_display(&workspace, &json_path)
    );

    let markdown_path = out_dir.join("coverage-report.md");
    write_markdown(&markdown_path, &report, &workspace)?;
    println!(
        "[xtask][coverage-report] wrote coverage dashboard to {}",
        relative_display(&workspace, &markdown_path)
    );

    let failures: Vec<_> = report
        .suites
        .iter()
        .filter(|suite| suite.status != SuiteStatus::Passed)
        .map(|suite| format!("{}: {:?}", suite.name, suite.status))
        .collect();

    if failures.is_empty() {
        Ok(())
    } else {
        Err(anyhow!(
            "coverage regression detected: {}",
            failures.join(", ")
        ))
    }
}

/// Captures whether the aggregator should read from the real workspace or a
/// deterministic fixture directory used during testing.
#[derive(Debug, Clone)]
enum CoverageDataSource {
    Workspace(PathBuf),
    Fixtures(PathBuf),
}

impl CoverageDataSource {
    fn resolve<P: AsRef<Path>>(&self, relative: P) -> PathBuf {
        match self {
            CoverageDataSource::Workspace(root) => root.join(relative.as_ref()),
            CoverageDataSource::Fixtures(root) => root.join(relative.as_ref()),
        }
    }
}

/// Canonical thresholds used across CI and local validation.  Keeping them in
/// constants makes it straightforward to update when the team raises the bar.
const RUST_LINE_THRESHOLD: f64 = 75.0;
const RUST_BRANCH_THRESHOLD: f64 = 60.0;
const TS_PASS_RATE_THRESHOLD: f64 = 97.5;

/// Structured JSON payload persisted to `test-results/coverage/coverage-report.json`.
#[derive(Debug, Serialize)]
struct CoverageReport {
    generated_at: String,
    generated_at_unix: u64,
    suites: Vec<SuiteReport>,
    notes: Vec<String>,
}

#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum SuiteStatus {
    Passed,
    Failed,
    Skipped,
}

#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum SuiteCategory {
    Unit,
    Integration,
    Accessibility,
    Visual,
}

#[derive(Debug, Serialize)]
struct SuiteReport {
    name: String,
    kind: String,
    category: SuiteCategory,
    status: SuiteStatus,
    #[serde(flatten)]
    metrics: SuiteMetrics,
    details: Vec<String>,
    artifacts: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(tag = "metric_type", rename_all = "snake_case")]
enum SuiteMetrics {
    Coverage {
        line_rate: Option<f64>,
        branch_rate: Option<f64>,
        line_threshold: Option<f64>,
        branch_threshold: Option<f64>,
    },
    PassRate {
        pass_rate: f64,
        total: u64,
        passed: u64,
        failed: u64,
        skipped: u64,
        minimum_pass_rate: f64,
    },
    Accessibility {
        files_scanned: usize,
        issues: usize,
        require_zero_issues: bool,
    },
    VisualRegression {
        snapshots: u64,
        differences: u64,
        updated: u64,
        skipped: u64,
        require_zero_differences: bool,
    },
}

impl SuiteCategory {
    fn label(&self) -> &'static str {
        match self {
            SuiteCategory::Unit => "Unit",
            SuiteCategory::Integration => "Integration",
            SuiteCategory::Accessibility => "Accessibility",
            SuiteCategory::Visual => "Visual Snapshot",
        }
    }
}

fn collect_rust_coverage(source: &CoverageDataSource) -> Result<SuiteReport> {
    let path = source.resolve(Path::new("lcov.info"));
    if !path.exists() {
        return Ok(SuiteReport {
            name: "Rust workspace".into(),
            kind: "rust".into(),
            category: SuiteCategory::Integration,
            status: SuiteStatus::Skipped,
            metrics: SuiteMetrics::Coverage {
                line_rate: None,
                branch_rate: None,
                line_threshold: Some(RUST_LINE_THRESHOLD),
                branch_threshold: Some(RUST_BRANCH_THRESHOLD),
            },
            details: vec![
                "`lcov.info` not found; ensure `cargo xtask coverage` ran beforehand".into(),
            ],
            artifacts: vec![path.display().to_string()],
        });
    }

    let raw = fs::read_to_string(&path)
        .with_context(|| format!("failed to read Rust coverage report at {}", path.display()))?;
    let (lines_total, lines_covered, branches_total, branches_covered) = parse_lcov(&raw)?;

    if lines_total == 0 {
        bail!("lcov report at {} contained zero lines", path.display());
    }

    let line_rate = percentage(lines_covered, lines_total);
    let branch_rate = if branches_total == 0 {
        None
    } else {
        Some(percentage(branches_covered, branches_total))
    };

    let mut details = vec![format!(
        "processed {} Rust source lines across grcov instrumentation",
        lines_total
    )];
    if let Some(branches_total) = non_zero(branches_total) {
        details.push(format!(
            "branch analysis considered {} edges",
            branches_total
        ));
    }

    let status = if line_rate < RUST_LINE_THRESHOLD
        || branch_rate
            .map(|rate| rate < RUST_BRANCH_THRESHOLD)
            .unwrap_or(false)
    {
        SuiteStatus::Failed
    } else {
        SuiteStatus::Passed
    };

    Ok(SuiteReport {
        name: "Rust workspace".into(),
        kind: "rust".into(),
        category: SuiteCategory::Integration,
        status,
        metrics: SuiteMetrics::Coverage {
            line_rate: Some(line_rate),
            branch_rate,
            line_threshold: Some(RUST_LINE_THRESHOLD),
            branch_threshold: Some(RUST_BRANCH_THRESHOLD),
        },
        details,
        artifacts: vec![path.display().to_string()],
    })
}

fn collect_typescript_coverage(source: &CoverageDataSource) -> Result<SuiteReport> {
    let path = source.resolve(Path::new("test-results").join("junit.xml"));
    if !path.exists() {
        return Ok(SuiteReport {
            name: "TypeScript automation".into(),
            kind: "typescript".into(),
            category: SuiteCategory::Unit,
            status: SuiteStatus::Skipped,
            metrics: SuiteMetrics::PassRate {
                pass_rate: 0.0,
                total: 0,
                passed: 0,
                failed: 0,
                skipped: 0,
                minimum_pass_rate: TS_PASS_RATE_THRESHOLD,
            },
            details: vec![
                "Vitest/JUnit summary missing. Run the docs + packages TS suites before aggregating.".into(),
            ],
            artifacts: vec![path.display().to_string()],
        });
    }

    let summary = parse_junit_summary(&path)?;
    if summary.tests == 0 {
        bail!(
            "test summary at {} did not record any tests",
            path.display()
        );
    }
    let executed = summary.tests.saturating_sub(summary.skipped);
    if executed == 0 {
        return Ok(SuiteReport {
            name: "TypeScript automation".into(),
            kind: "typescript".into(),
            category: SuiteCategory::Unit,
            status: SuiteStatus::Skipped,
            metrics: SuiteMetrics::PassRate {
                pass_rate: 0.0,
                total: summary.tests,
                passed: 0,
                failed: summary.failures + summary.errors,
                skipped: summary.skipped,
                minimum_pass_rate: TS_PASS_RATE_THRESHOLD,
            },
            details: vec!["All suites were marked as skipped in JUnit output".into()],
            artifacts: vec![path.display().to_string()],
        });
    }

    let failures = summary.failures + summary.errors;
    let passed = executed.saturating_sub(failures);
    let pass_rate = percentage(passed, executed);
    let status = if pass_rate < TS_PASS_RATE_THRESHOLD {
        SuiteStatus::Failed
    } else {
        SuiteStatus::Passed
    };

    Ok(SuiteReport {
        name: "TypeScript automation".into(),
        kind: "typescript".into(),
        category: SuiteCategory::Unit,
        status,
        metrics: SuiteMetrics::PassRate {
            pass_rate,
            total: summary.tests,
            passed,
            failed: failures,
            skipped: summary.skipped,
            minimum_pass_rate: TS_PASS_RATE_THRESHOLD,
        },
        details: vec![format!(
            "aggregated {} test cases across Vitest/Karma/Playwright pipelines",
            summary.tests
        )],
        artifacts: vec![path.display().to_string()],
    })
}

fn collect_accessibility_signal(source: &CoverageDataSource) -> Result<SuiteReport> {
    match source {
        CoverageDataSource::Fixtures(root) => {
            let path = root.join("accessibility.json");
            if !path.exists() {
                return Ok(SuiteReport {
                    name: "Accessibility audits".into(),
                    kind: "accessibility".into(),
                    category: SuiteCategory::Accessibility,
                    status: SuiteStatus::Skipped,
                    metrics: SuiteMetrics::Accessibility {
                        files_scanned: 0,
                        issues: 0,
                        require_zero_issues: true,
                    },
                    details: vec![
                        "accessibility fixture missing; run the Markdown sweeps to populate coverage".into(),
                    ],
                    artifacts: vec![path.display().to_string()],
                });
            }
            let raw = fs::read_to_string(&path).with_context(|| {
                format!(
                    "failed to read accessibility fixture summary from {}",
                    path.display()
                )
            })?;
            let summary: accessibility_fixture::FixtureAccessibilitySummary =
                serde_json::from_str(&raw)?;
            let status = if summary.issues == 0 {
                SuiteStatus::Passed
            } else {
                SuiteStatus::Failed
            };
            Ok(SuiteReport {
                name: "Accessibility audits".into(),
                kind: "accessibility".into(),
                category: SuiteCategory::Accessibility,
                status,
                metrics: SuiteMetrics::Accessibility {
                    files_scanned: summary.files_scanned,
                    issues: summary.issues,
                    require_zero_issues: true,
                },
                details: summary.notes,
                artifacts: vec![path.display().to_string()],
            })
        }
        CoverageDataSource::Workspace(root) => {
            let summary = accessibility::run(root, accessibility::AuditMode::Standard, None)?;
            let status = if summary.issues.is_empty() {
                SuiteStatus::Passed
            } else {
                SuiteStatus::Failed
            };
            let mut details = vec![format!(
                "scanned {} markdown entries for alt-text + heading coverage",
                summary.files_scanned
            )];
            if !summary.issues.is_empty() {
                for finding in &summary.issues {
                    details.push(format!(
                        "{} :: {}",
                        relative_display(root, &finding.path),
                        finding.message
                    ));
                }
            }
            Ok(SuiteReport {
                name: "Accessibility audits".into(),
                kind: "accessibility".into(),
                category: SuiteCategory::Accessibility,
                status,
                metrics: SuiteMetrics::Accessibility {
                    files_scanned: summary.files_scanned,
                    issues: summary.issues.len(),
                    require_zero_issues: true,
                },
                details,
                artifacts: vec!["Generated programmatically".into()],
            })
        }
    }
}

fn collect_visual_regressions(source: &CoverageDataSource) -> Result<SuiteReport> {
    let path = source.resolve(Path::new("test-results").join("visual-regressions.json"));
    if !path.exists() {
        return Ok(SuiteReport {
            name: "Visual regression snapshots".into(),
            kind: "visual_regression".into(),
            category: SuiteCategory::Visual,
            status: SuiteStatus::Skipped,
            metrics: SuiteMetrics::VisualRegression {
                snapshots: 0,
                differences: 0,
                updated: 0,
                skipped: 0,
                require_zero_differences: true,
            },
            details: vec![
                "visual regression summary missing; ensure Playwright pipeline produced JSON output".into(),
            ],
            artifacts: vec![path.display().to_string()],
        });
    }

    let raw = fs::read_to_string(&path).with_context(|| {
        format!(
            "failed to read visual regression summary at {}",
            path.display()
        )
    })?;
    let summary: VisualSummary = serde_json::from_str(&raw)?;

    if summary.snapshots == 0 {
        bail!(
            "visual regression summary at {} recorded zero snapshots",
            path.display()
        );
    }

    let status = if summary.differences == 0 {
        SuiteStatus::Passed
    } else {
        SuiteStatus::Failed
    };

    Ok(SuiteReport {
        name: "Visual regression snapshots".into(),
        kind: "visual_regression".into(),
        category: SuiteCategory::Visual,
        status,
        metrics: SuiteMetrics::VisualRegression {
            snapshots: summary.snapshots,
            differences: summary.differences,
            updated: summary.updated,
            skipped: summary.skipped,
            require_zero_differences: true,
        },
        details: summary.notes,
        artifacts: vec![path.display().to_string()],
    })
}

fn collect_adapter_visual_regressions(source: &CoverageDataSource) -> Result<SuiteReport> {
    let path = source.resolve(Path::new("test-results").join("visual-regressions-adapters.json"));
    if !path.exists() {
        return Ok(SuiteReport {
            name: "Adapter visual regression snapshots".into(),
            kind: "visual_regression".into(),
            category: SuiteCategory::Visual,
            status: SuiteStatus::Skipped,
            metrics: SuiteMetrics::VisualRegression {
                snapshots: 0,
                differences: 0,
                updated: 0,
                skipped: 0,
                require_zero_differences: true,
            },
            details: vec![
                "adapter visual regression summary missing; ensure adapter Storybook pipeline produced JSON output".into(),
            ],
            artifacts: vec![path.display().to_string()],
        });
    }

    let raw = fs::read_to_string(&path).with_context(|| {
        format!(
            "failed to read adapter visual regression summary at {}",
            path.display()
        )
    })?;
    let summary: VisualSummary = serde_json::from_str(&raw)?;

    if summary.snapshots == 0 {
        bail!(
            "adapter visual regression summary at {} recorded zero snapshots",
            path.display()
        );
    }

    let status = if summary.differences == 0 {
        SuiteStatus::Passed
    } else {
        SuiteStatus::Failed
    };

    Ok(SuiteReport {
        name: "Adapter visual regression snapshots".into(),
        kind: "visual_regression".into(),
        category: SuiteCategory::Visual,
        status,
        metrics: SuiteMetrics::VisualRegression {
            snapshots: summary.snapshots,
            differences: summary.differences,
            updated: summary.updated,
            skipped: summary.skipped,
            require_zero_differences: true,
        },
        details: summary.notes,
        artifacts: vec![path.display().to_string()],
    })
}

fn write_json(path: &Path, report: &CoverageReport) -> Result<()> {
    let file = File::create(path)
        .with_context(|| format!("failed to open {} for JSON output", path.display()))?;
    serde_json::to_writer_pretty(file, report).with_context(|| {
        format!(
            "failed to serialise coverage report into {}",
            path.display()
        )
    })
}

fn write_markdown(path: &Path, report: &CoverageReport, workspace: &Path) -> Result<()> {
    let mut file = File::create(path)
        .with_context(|| format!("failed to open {} for Markdown output", path.display()))?;

    writeln!(
        file,
        "# RusticUI coverage dashboard\n\nGenerated at {} (unix: {}).\n",
        report.generated_at, report.generated_at_unix
    )?;

    writeln!(
        file,
        "| Suite | Track | Discipline | Key metrics | Thresholds | Notes |\n| --- | --- | --- | --- | --- | --- |"
    )?;

    for suite in &report.suites {
        let status_emoji = match suite.status {
            SuiteStatus::Passed => "✅",
            SuiteStatus::Failed => "❌",
            SuiteStatus::Skipped => "⚠️",
        };
        let metrics = summarise_metrics(&suite.metrics);
        let thresholds = summarise_thresholds(&suite.metrics);
        let notes = suite.details.join("<br />");
        writeln!(
            file,
            "| {} {} | {} | {} | {} | {} | {} |",
            status_emoji,
            suite.name,
            suite.kind,
            suite.category.label(),
            metrics,
            thresholds,
            notes
        )?;
    }

    if !report.notes.is_empty() {
        writeln!(file, "\n## Automation notes\n")?;
        for note in &report.notes {
            writeln!(file, "- {}", note)?;
        }
    }

    writeln!(
        file,
        "\nArtifacts stored relative to the workspace root ({}).",
        workspace.display()
    )?;

    Ok(())
}

fn summarise_metrics(metrics: &SuiteMetrics) -> String {
    match metrics {
        SuiteMetrics::Coverage {
            line_rate,
            branch_rate,
            ..
        } => format!(
            "line: {}% / branch: {}%",
            display_percent(*line_rate),
            branch_rate
                .map(|rate| display_percent(Some(rate)))
                .unwrap_or_else(|| "n/a".into())
        ),
        SuiteMetrics::PassRate {
            pass_rate,
            passed,
            failed,
            skipped,
            ..
        } => format!(
            "{}% pass ({} passed / {} failed / {} skipped)",
            display_percent(Some(*pass_rate)),
            passed,
            failed,
            skipped
        ),
        SuiteMetrics::Accessibility {
            files_scanned,
            issues,
            ..
        } => format!("{} files scanned / {} issues", files_scanned, issues),
        SuiteMetrics::VisualRegression {
            snapshots,
            differences,
            skipped,
            ..
        } => format!(
            "{} snapshots / {} diffs / {} skipped",
            snapshots, differences, skipped
        ),
    }
}

fn summarise_thresholds(metrics: &SuiteMetrics) -> String {
    match metrics {
        SuiteMetrics::Coverage {
            line_threshold,
            branch_threshold,
            ..
        } => format!(
            "line ≥ {}% / branch ≥ {}%",
            display_percent(*line_threshold),
            display_percent(*branch_threshold)
        ),
        SuiteMetrics::PassRate {
            minimum_pass_rate, ..
        } => format!("pass-rate ≥ {}%", display_percent(Some(*minimum_pass_rate))),
        SuiteMetrics::Accessibility { .. } => "zero issues".into(),
        SuiteMetrics::VisualRegression { .. } => "zero diffs".into(),
    }
}

fn display_percent(value: Option<f64>) -> String {
    value
        .map(|v| format!("{:.2}", v))
        .unwrap_or_else(|| "0.00".into())
}

fn percentage(covered: u64, total: u64) -> f64 {
    if total == 0 {
        0.0
    } else {
        (covered as f64 / total as f64) * 100.0
    }
}

fn non_zero(value: u64) -> Option<u64> {
    if value == 0 {
        None
    } else {
        Some(value)
    }
}

fn parse_lcov(raw: &str) -> Result<(u64, u64, u64, u64)> {
    let mut lines_total = 0;
    let mut lines_covered = 0;
    let mut branches_total = 0;
    let mut branches_covered = 0;

    for line in raw.lines() {
        if let Some(rest) = line.strip_prefix("DA:") {
            let mut parts = rest.split(',');
            let _line_number = parts.next().ok_or_else(|| anyhow!("malformed DA record"))?;
            let count = parts
                .next()
                .ok_or_else(|| anyhow!("malformed DA counter"))?
                .parse::<u64>()?;
            lines_total += 1;
            if count > 0 {
                lines_covered += 1;
            }
        } else if let Some(rest) = line.strip_prefix("BRDA:") {
            let mut parts = rest.split(',');
            let _line_number = parts.next();
            let _block = parts.next();
            let _branch = parts.next();
            let hits = parts
                .next()
                .ok_or_else(|| anyhow!("malformed BRDA record"))?;
            if hits != "-" {
                branches_total += 1;
                if hits.parse::<u64>()? > 0 {
                    branches_covered += 1;
                }
            }
        }
    }

    Ok((lines_total, lines_covered, branches_total, branches_covered))
}

fn parse_junit_summary(path: &Path) -> Result<JunitSummary> {
    let mut reader = Reader::from_file(path)
        .with_context(|| format!("failed to open {} for reading", path.display()))?;
    reader.trim_text(true);
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(elem) | Event::Empty(elem) => {
                let name = elem.name();
                if name.as_ref() == b"testsuites" || name.as_ref() == b"testsuite" {
                    let mut summary = JunitSummary::default();
                    for attr in elem.attributes() {
                        let attr = attr?;
                        match attr.key.as_ref() {
                            b"tests" => {
                                summary.tests = parse_attr_u64(attr.value.as_ref(), "tests")?
                            }
                            b"failures" => {
                                summary.failures = parse_attr_u64(attr.value.as_ref(), "failures")?
                            }
                            b"errors" => {
                                summary.errors = parse_attr_u64(attr.value.as_ref(), "errors")?
                            }
                            b"skipped" => {
                                summary.skipped = parse_attr_u64(attr.value.as_ref(), "skipped")?
                            }
                            _ => {}
                        }
                    }
                    return Ok(summary);
                }
            }
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    Err(anyhow!(
        "JUnit summary not found in {}. Ensure the test reporter produced <testsuites>.",
        path.display()
    ))
}

fn parse_attr_u64(raw: &[u8], label: &str) -> Result<u64> {
    let text = std::str::from_utf8(raw)
        .with_context(|| format!("failed to parse {} attribute as UTF-8", label))?;
    Ok(text.parse::<u64>().with_context(|| {
        format!(
            "failed to parse {} attribute as integer (value: {})",
            label, text
        )
    })?)
}

fn default_notes() -> Vec<String> {
    vec![
        "Rust metrics are sourced from grcov; regenerate via `cargo xtask coverage`.".into(),
        "TypeScript pass-rate derives from the Vitest/Karma junit.xml written to test-results/.".into(),
        "Accessibility sweeps reuse `cargo xtask accessibility-audit` to guarantee markdown hygiene.".into(),
        "Playwright visual regressions must export test-results/visual-regressions.json (see docs/testing/coverage-overview.md)."
            .into(),
        "Adapter Storybook snapshots must export test-results/visual-regressions-adapters.json (see docs/testing/visual-regressions.md)."
            .into(),
        "Bundle-size deltas come from `cargo xtask bundle-report`; see docs/performance/bundle-costs.md for the rendered table."
            .into(),
    ]
}

#[derive(Debug, Default)]
struct JunitSummary {
    tests: u64,
    failures: u64,
    errors: u64,
    skipped: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct VisualSummary {
    snapshots: u64,
    differences: u64,
    updated: u64,
    skipped: u64,
    #[serde(default)]
    notes: Vec<String>,
}

mod accessibility_fixture {
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    pub struct FixtureAccessibilitySummary {
        pub files_scanned: usize,
        pub issues: usize,
        #[serde(default)]
        pub notes: Vec<String>,
    }
}
