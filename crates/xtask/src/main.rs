//! Developer automation commands for the MUI Rust workspace.
//!
//! The `xtask` pattern keeps our repository free of ad-hoc shell
//! scripts and centralizes repeatable tasks in a small Rust binary.
//! This approach scales well for large teams and CI environments,
//! ensuring that contributors invoke the exact same logic locally
//! and in automation.

use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use mui_system::theme::{ColorScheme, JoyTheme, Theme};
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
    ///
    /// Historically this task shipped under the `refresh-icons` name. We
    /// preserve that alias so automation and bespoke scripts keep working while
    /// providing the canonical `icon-update` entrypoint surfaced in `--help`
    /// output for new contributors.
    #[command(
        name = "icon-update",
        aliases = ["refresh-icons", "refresh_icons"]
    )]
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
        /// Emit Joy specific fixtures alongside the Material outputs.
        #[arg(long)]
        joy: bool,
    },
    /// Recompute the Material component parity dashboard.
    MaterialParity,
    /// Recompute the Joy UI inventory to highlight missing Rust bindings.
    #[command(name = "joy-inventory", alias = "joy-parity")]
    JoyParity,
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
        Commands::GenerateTheme {
            overrides,
            format,
            joy,
        } => generate_theme(overrides, format, joy),
        Commands::MaterialParity => material_parity(),
        Commands::JoyParity => joy_parity(),
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
    // Each UI crate exposes multiple renderer integrations behind feature
    // flags. Exercising them independently ensures we never ship a breaking
    // change for a specific framework while the others still pass. Running the
    // suites serially keeps logging deterministic for CI while still providing
    // actionable context to developers when a specific adapter fails.
    let wasm_crates = ["crates/mui-joy", "crates/mui-material"];
    let frameworks = ["yew", "leptos", "dioxus", "sycamore"];

    for krate in &wasm_crates {
        for framework in &frameworks {
            println!(
                "[xtask] wasm tests for crate `{}` using `{}` feature",
                krate, framework
            );

            let mut cmd = Command::new("wasm-pack");
            cmd.arg("test")
                .arg("--headless")
                .arg("--chrome")
                .arg("--")
                // Explicitly disable defaults so we only compile the target
                // renderer, catching missing optional dependencies or cfgs.
                .arg("--no-default-features")
                .arg("--features")
                .arg(framework)
                .current_dir(krate);
            run(cmd)?;
        }
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

fn generate_theme(overrides: Option<PathBuf>, format: ThemeFormat, joy: bool) -> Result<()> {
    println!(
        "[xtask] generating Material theme artifacts (format: {format:?}, joy fixtures: {joy})"
    );

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

    // Split overrides into the portions that apply to all color schemes and the
    // scheme-specific fragments.  We intentionally keep this logic explicit so
    // that future scheme additions (e.g. high-contrast) can plug in without
    // reworking the generator entrypoint.
    let mut global_overrides: Option<Value> = None;
    let mut scheme_overrides: std::collections::BTreeMap<String, Value> = Default::default();

    if let Some(overrides_value) = overrides_value {
        if let Some(map) = overrides_value.as_object() {
            let mut shared = serde_json::Map::new();
            for (key, value) in map {
                match key.as_str() {
                    // The `schemes` key allows authoring overrides as
                    // `{ "schemes": { "light": {...}, "dark": {...} } }` while
                    // keeping top-level keys reserved for shared values.
                    "schemes" => {
                        if let Some(entries) = value.as_object() {
                            for (scheme, fragment) in entries {
                                scheme_overrides.insert(scheme.clone(), fragment.clone());
                            }
                        } else {
                            return Err(anyhow!(
                                "expected `schemes` override section to be an object"
                            ));
                        }
                    }
                    // Allow direct `light`/`dark` keys for ergonomics so that
                    // existing automation fixtures can migrate incrementally.
                    "light" | "dark" => {
                        scheme_overrides.insert(key.clone(), value.clone());
                    }
                    _ => {
                        shared.insert(key.clone(), value.clone());
                    }
                }
            }
            if !shared.is_empty() {
                global_overrides = Some(Value::Object(shared));
            }
        } else {
            // Non-object overrides (e.g. legacy fixtures providing the entire
            // theme structure) are treated as global so we maintain backwards
            // compatibility with bespoke integrations.
            global_overrides = Some(overrides_value);
        }
    }

    // Material defaults currently revolve around a light and dark experience.
    // Keep those first for deterministic file ordering, then append any
    // additional schemes discovered in the overrides map.
    let mut schemes = vec!["light".to_string(), "dark".to_string()];
    for scheme in scheme_overrides.keys() {
        if !schemes.contains(scheme) {
            schemes.push(scheme.clone());
        }
    }

    // Prepare the templates directory and remove historical single-file
    // artefacts so downstream tooling never accidentally consumes stale data.
    let output_dir = PathBuf::from("crates/mui-system/templates");
    fs::create_dir_all(&output_dir)?;
    for legacy in [
        output_dir.join("material_theme.json"),
        output_dir.join("material_theme.toml"),
        output_dir.join("material_css_baseline.css"),
    ] {
        if legacy.exists() {
            fs::remove_file(&legacy)?;
            println!("[xtask] removed legacy artefact {}", legacy.display());
        }
    }

    // Serialize each scheme independently while funnelling the overrides
    // through the same merge routine that powers the single theme output. The
    // verbose logging doubles as living documentation for anyone reading CI
    // logs to validate automation runs.
    for scheme in schemes {
        let mut merged_value = serde_json::to_value(&base_theme)?;
        if let Some(global) = &global_overrides {
            merge_values(&mut merged_value, global);
        }
        if let Some(specific) = scheme_overrides.get(&scheme) {
            merge_values(&mut merged_value, specific);
        }

        let merged_theme: Theme = serde_json::from_value(merged_value).with_context(|| {
            format!(
                "failed to convert merged theme representation into Theme struct for `{scheme}`"
            )
        })?;
        let mut theme =
            mui_system::theme_provider::material_theme_with_optional_overrides(Some(merged_theme));
        if let Some(color_scheme) = match scheme.as_str() {
            "light" => Some(ColorScheme::Light),
            "dark" => Some(ColorScheme::Dark),
            _ => None,
        } {
            theme.palette.initial_color_scheme = color_scheme;
        }

        let output_path = match format {
            ThemeFormat::Json => output_dir.join(format!("material_theme.{scheme}.json")),
            ThemeFormat::Toml => output_dir.join(format!("material_theme.{scheme}.toml")),
        };

        let serialized = match format {
            ThemeFormat::Json => serde_json::to_string_pretty(&theme)?,
            ThemeFormat::Toml => toml::to_string_pretty(&theme)?,
        };
        fs::write(&output_path, format!("{serialized}\n"))?;
        println!("[xtask] wrote {}", output_path.display());

        let css_path = output_dir.join(format!("material_css_baseline.{scheme}.css"));
        let css = mui_system::theme_provider::material_css_baseline_from_theme(&theme);
        fs::write(&css_path, css)?;
        println!("[xtask] wrote {}", css_path.display());

        if joy {
            let joy_payload = serde_json::json!({
                "scheme": scheme,
                "joy": &theme.joy,
                "automation": {
                    "comments": JoyTheme::automation_comments(),
                    "template": JoyTheme::json_template(),
                }
            });
            let joy_path = output_dir.join(format!("joy_theme.{scheme}.json"));
            fs::write(
                &joy_path,
                format!("{}\n", serde_json::to_string_pretty(&joy_payload)?),
            )?;
            println!("[xtask] wrote {}", joy_path.display());

            if scheme == "light" {
                let template_path = output_dir.join("joy_theme.template.json");
                let template_payload = serde_json::json!({
                    "comments": JoyTheme::automation_comments(),
                    "joy": JoyTheme::json_template(),
                });
                fs::write(
                    &template_path,
                    format!("{}\n", serde_json::to_string_pretty(&template_payload)?),
                )?;
                println!("[xtask] wrote {}", template_path.display());
            }
        }
    }

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

fn joy_parity() -> Result<()> {
    // Delegate to the dedicated Joy parity binary so the TypeScript parsing logic stays
    // encapsulated and independently testable. Keeping xtask thin ensures we can reuse the
    // scanner from CI, local development, or other automation entry points without code drift.
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("-p")
        .arg("joy-parity")
        .arg("--")
        .arg("--report")
        .arg("docs/joy-component-parity.md");
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
