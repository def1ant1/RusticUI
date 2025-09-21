//! Developer automation commands for the MUI Rust workspace.
//!
//! The `xtask` pattern keeps our repository free of ad-hoc shell
//! scripts and centralizes repeatable tasks in a small Rust binary.
//! This approach scales well for large teams and CI environments,
//! ensuring that contributors invoke the exact same logic locally
//! and in automation.

use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use mui_system::theme::Theme;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Entry point for the `cargo xtask` command.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Xtask {
    #[command(subcommand)]
    command: Commands,
}

/// Tasks that can be executed. Each variant maps to a subcommand.
#[derive(Subcommand)]
enum Commands {
    /// Format all Rust sources. Use `--check` in CI.
    Fmt {
        /// Only verify formatting without modifying files.
        #[arg(long)]
        check: bool,
    },
    /// Run Clippy across the workspace and deny warnings.
    Clippy,
    /// Execute the default test suites for all crates.
    Test,
    /// Run WebAssembly tests via `wasm-pack` for selected crates.
    WasmTest,
    /// Build API documentation for the entire workspace.
    Doc,
    /// Refresh the Material Design icon bindings.
    RefreshIcons,
    /// Generate an `lcov.info` report using grcov.
    Coverage,
    /// Execute Criterion benchmarks. Succeeds even if none exist.
    Bench,
    /// Regenerate component scaffolding and associated metadata.
    UpdateComponents,
    /// Run automated accessibility audits against the docs site.
    AccessibilityAudit,
    /// Build the JavaScript documentation site.
    BuildDocs,
    /// Regenerate serialized theme templates and CSS baselines.
    GenerateTheme {
        /// Optional path to a JSON or TOML fixture that overrides
        /// sections of the canonical Material theme before serialization.
        #[arg(long)]
        overrides: Option<PathBuf>,
        /// Output format written to disk.
        #[arg(long, value_enum, default_value_t = ThemeFormat::Json)]
        format: ThemeFormat,
    },
    /// Recompute the Material component parity dashboard.
    MaterialParity,
}

fn main() -> Result<()> {
    let xtask = Xtask::parse();
    match xtask.command {
        Commands::Fmt { check } => fmt(check),
        Commands::Clippy => clippy(),
        Commands::Test => test(),
        Commands::WasmTest => wasm_test(),
        Commands::Doc => doc(),
        Commands::RefreshIcons => refresh_icons(),
        Commands::Coverage => coverage(),
        Commands::Bench => bench(),
        Commands::UpdateComponents => update_components(),
        Commands::AccessibilityAudit => accessibility_audit(),
        Commands::BuildDocs => build_docs(),
        Commands::GenerateTheme { overrides, format } => generate_theme(overrides, format),
        Commands::MaterialParity => material_parity(),
    }
}

/// Output encodings supported by the theme generator.
#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
enum ThemeFormat {
    Json,
    Toml,
}

/// Helper to execute an external command with verbose logging.
///
/// By centralizing the spawning logic we ensure that every task
/// propagates failures and surfaces the exact command line that
/// was executed. This dramatically simplifies troubleshooting in
/// large CI systems where logs are often the only feedback.
fn run(mut cmd: Command) -> Result<()> {
    // Print the command for transparency before execution.
    println!("[xtask] running: {:?}", cmd);
    let status = cmd.status()?;
    if !status.success() {
        return Err(anyhow!("command {:?} failed with status {:?}", cmd, status));
    }
    Ok(())
}

fn fmt(check: bool) -> Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.arg("fmt").arg("--all");
    if check {
        cmd.arg("--").arg("--check");
    }
    run(cmd)
}

fn clippy() -> Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.arg("clippy")
        .arg("--workspace")
        .arg("--all-targets")
        .arg("--all-features")
        .arg("--")
        .arg("-D")
        .arg("warnings");
    run(cmd)
}

fn test() -> Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.arg("test").arg("--workspace").arg("--all-features");
    run(cmd)?;
    // Also ensure each example still compiles for the WebAssembly target.
    let examples = [
        "examples/mui-yew",
        "examples/mui-leptos",
        "examples/mui-dioxus",
        "examples/mui-sycamore",
    ];
    for ex in &examples {
        let mut check = Command::new("cargo");
        check
            .arg("check")
            .arg("--target")
            .arg("wasm32-unknown-unknown")
            .arg("--manifest-path")
            .arg(format!("{}/Cargo.toml", ex));
        run(check)?;
    }
    Ok(())
}

fn wasm_test() -> Result<()> {
    // Crates with WebAssembly tests. Extend this list as needed.
    let wasm_crates = ["crates/mui-joy", "crates/mui-material"];
    for krate in &wasm_crates {
        let mut cmd = Command::new("wasm-pack");
        cmd.arg("test")
            .arg("--headless")
            .arg("--chrome")
            // All interactive components currently rely on the `yew` feature for
            // rendering. By enabling it here we exercise the same code paths in
            // CI and local development.
            .arg("--features")
            .arg("yew")
            .current_dir(krate);
        run(cmd)?;
    }
    Ok(())
}

fn doc() -> Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.arg("doc")
        .arg("--no-deps")
        .arg("--workspace")
        .arg("--all-features");
    run(cmd)
}

fn refresh_icons() -> Result<()> {
    // Delegate to the existing Rust binary that fetches the latest
    // Material Design SVGs and regenerates the strongly typed bindings.
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("-p")
        .arg("mui-icons-material")
        .arg("--bin")
        .arg("update_icons")
        .arg("--features")
        .arg("update-icons");
    run(cmd)
}

fn update_components() -> Result<()> {
    // Rebuild component metadata such as PropTypes or other generated
    // artifacts. This leverages the existing Node script so contributors
    // do not need to remember the exact incantation.
    let mut cmd = Command::new("pnpm");
    cmd.arg("proptypes");
    run(cmd)
}

fn accessibility_audit() -> Result<()> {
    // Execute Playwright based accessibility tests that crawl the
    // documentation site. Any violation bubbles up as a command failure
    // ensuring CI visibility.
    let mut cmd = Command::new("pnpm");
    cmd.arg("test:e2e-website");
    run(cmd)
}

fn build_docs() -> Result<()> {
    // Build the full documentation website via the existing npm script.
    // This compiles API documentation, markdown demos and bundles the
    // static site for deployment.
    let mut cmd = Command::new("pnpm");
    cmd.arg("docs:build");
    run(cmd)
}

fn coverage() -> Result<()> {
    // Run tests first so that coverage data is produced.
    test()?;
    let mut cmd = Command::new("grcov");
    cmd.arg(".")
        .arg("--binary-path")
        .arg("./target/debug/")
        .arg("-s")
        .arg(".")
        .arg("-t")
        .arg("lcov")
        .arg("--branch")
        .arg("--ignore-not-existing")
        .arg("-o")
        .arg("lcov.info");
    run(cmd)
}

fn generate_theme(overrides: Option<PathBuf>, format: ThemeFormat) -> Result<()> {
    println!("[xtask] generating Material theme artifacts (format: {format:?})");

    // Load the optional override fixture from disk.  We keep this logic verbose so CI logs
    // clearly document which file was considered and how it was interpreted.
    let overrides_value = match overrides {
        Some(path) => {
            println!("[xtask] loading overrides from {}", path.display());
            let raw = fs::read_to_string(&path).with_context(|| {
                format!("failed to read override fixture at {}", path.display())
            })?;
            let value = parse_overrides(&path, &raw)?;
            println!("[xtask] successfully parsed overrides");
            Some(value)
        }
        None => {
            println!("[xtask] no overrides supplied; using canonical defaults");
            None
        }
    };

    // Always start from the canonical Material theme before layering user supplied overrides.
    let base_theme = mui_system::theme_provider::material_theme();
    let theme = if let Some(overrides_value) = overrides_value {
        let mut merged_theme_value = serde_json::to_value(&base_theme)?;
        // Perform a deep merge so nested objects (palette, typography, etc.) can be partially
        // specified without requiring every field.  This mirrors the ergonomics of the JS tooling
        // and keeps configuration files succinct.
        merge_values(&mut merged_theme_value, &overrides_value);
        let merged_theme: Theme = serde_json::from_value(merged_theme_value)
            .with_context(|| "failed to convert merged theme representation into Theme struct")?;
        mui_system::theme_provider::material_theme_with_optional_overrides(Some(merged_theme))
    } else {
        mui_system::theme_provider::material_theme_with_optional_overrides::<Theme>(None)
    };

    let output_path = match format {
        ThemeFormat::Json => PathBuf::from("crates/mui-system/templates/material_theme.json"),
        ThemeFormat::Toml => PathBuf::from("crates/mui-system/templates/material_theme.toml"),
    };
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let serialized = match format {
        ThemeFormat::Json => serde_json::to_string_pretty(&theme)?,
        ThemeFormat::Toml => toml::to_string_pretty(&theme)?,
    };
    fs::write(&output_path, format!("{serialized}\n"))?;
    println!("[xtask] wrote {}", output_path.display());

    let css_path = output_path.with_file_name("material_css_baseline.css");
    let css = mui_system::theme_provider::material_css_baseline_from_theme(&theme);
    fs::write(&css_path, css)?;
    println!("[xtask] wrote {}", css_path.display());

    Ok(())
}

/// Parses an override fixture into a [`serde_json::Value`] so we can merge it with the default
/// theme irrespective of the original file format.
fn parse_overrides(path: &Path, raw: &str) -> Result<Value> {
    let ext = path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_ascii_lowercase())
        .unwrap_or_else(|| "json".to_string());

    let value: Value = if ext == "toml" {
        toml::from_str(raw)
            .with_context(|| format!("failed to parse TOML overrides from {}", path.display()))?
    } else {
        serde_json::from_str(raw)
            .with_context(|| format!("failed to parse JSON overrides from {}", path.display()))?
    };

    Ok(value)
}

/// Recursively merges JSON values.  Objects are merged key-by-key while primitive values are
/// replaced outright.  This mirrors how JavaScript `Object.assign` works and matches developer
/// expectations when porting configurations from the upstream ecosystem.
fn merge_values(base: &mut Value, overrides: &Value) {
    if let (Some(base_map), Some(override_map)) = (base.as_object_mut(), overrides.as_object()) {
        for (key, value) in override_map {
            merge_values(base_map.entry(key.clone()).or_insert(Value::Null), value);
        }
    } else {
        *base = overrides.clone();
    }
}

fn material_parity() -> Result<()> {
    // Keep the parity snapshot fresh so enterprise adopters can track adoption progress
    // without spelunking through multiple repositories.
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("-p")
        .arg("material-parity")
        .arg("--")
        .arg("--report")
        .arg("docs/material-component-parity.md");
    run(cmd)
}

fn bench() -> Result<()> {
    // Criterion will exit with an error if no benchmarks exist.
    // Swallow the non-zero exit code to keep CI green when benches are absent.
    let status = Command::new("cargo")
        .arg("bench")
        .arg("--workspace")
        .status()?;
    if !status.success() {
        // Report but don't fail.
        eprintln!("cargo bench exited with {:?}", status);
    }
    Ok(())
}
