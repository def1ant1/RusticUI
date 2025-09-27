//! Developer automation commands for the RusticUI workspace.
//!
//! The `xtask` pattern keeps our repository free of ad-hoc shell
//! scripts and centralizes repeatable tasks in a small Rust binary.
//! This approach scales well for large teams and CI environments,
//! ensuring that contributors invoke the exact same logic locally
//! and in automation.
//!
//! The commands declared below intentionally favour a "Rust-first"
//! workflow: we hydrate design tokens from `rustic-ui-system`, drive
//! front-end automation via strongly typed binaries, and orchestrate
//! web tooling (Playwright, mdBook, etc.) through a single entry
//! point.  Enterprise adopters can wire these tasks directly into CI
//! without sprinkling custom shell scripts across repositories, while
//! contributors get consistent documentation about which crates,
//! examples, and documentation sites each task touches.

use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use rustic_ui_design_tokens::ArtifactBundleBuilder;
use rustic_ui_system::{
    theme::{ColorScheme, JoyTheme, Theme},
    theme_provider,
};
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

/// Entry point for the `cargo xtask` command.
#[derive(Parser)]
#[command(
    author,
    version,
    about = "Rust-first automation for RusticUI contributors.",
    long_about = None,
    disable_help_flag = false,
    disable_help_subcommand = true
)]
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
    ///
    /// After the workspace tests finish we compile the `joy-*` WebAssembly
    /// examples (`examples/joy-yew`, `examples/joy-leptos`, etc.) to guarantee
    /// each renderer remains compatible with the shared RusticUI APIs.
    Test,
    /// Run WebAssembly tests via `wasm-pack` for selected crates.
    ///
    /// This exercises the `rustic-ui-material` and `rustic-ui-joy` crates across
    /// every supported renderer to ensure feature flags stay in sync.
    WasmTest,
    /// Build API documentation for the entire workspace.
    Doc,
    /// Refresh the Rustic icon bindings.
    #[command(name = "icon-update")]
    RefreshIcons,
    /// Package refreshed icon assets into reproducible archives and manifests.
    #[command(name = "icons-bundle")]
    IconsBundle {
        /// Copy the generated bundle into `archives/assets/icons` for legacy consumers.
        #[arg(long)]
        compat: bool,
        /// Override the output directory used for bundle staging.
        #[arg(long = "out-dir")]
        out_dir: Option<PathBuf>,
    },
    /// Generate an `lcov.info` report using grcov.
    Coverage,
    /// Execute Criterion benchmarks. Succeeds even if none exist.
    Bench,
    /// Regenerate component scaffolding and associated metadata.
    UpdateComponents,
    /// Run automated accessibility smoke tests against the docs site.
    AccessibilityAudit,
    /// Execute the long running nightly accessibility coverage suite.
    ///
    /// The command exports `RUSTIC_UI_A11Y_MODE=nightly` so Playwright can
    /// widen its crawl. This is a dry-run friendly hook for upcoming fixtures.
    #[command(name = "accessibility-nightly")]
    AccessibilityNightly,
    /// Build the Rust-first documentation site and supporting API docs.
    BuildDocs,
    /// Regenerate RusticUI serialized theme templates and CSS baselines.
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
    /// Generate theme artifacts and wrap them in distribution-ready bundles.
    #[command(name = "themes-bundle")]
    ThemesBundle {
        /// Optional path to a JSON or TOML fixture that overrides the base theme.
        #[arg(long)]
        overrides: Option<PathBuf>,
        /// Output format written to disk before bundling.
        #[arg(long, value_enum, default_value_t = ThemeFormat::Json)]
        format: ThemeFormat,
        /// Emit Joy-specific payloads alongside the Material artifacts.
        #[arg(long)]
        joy: bool,
        /// Copy the generated bundle into `archives/assets/themes` for legacy consumers.
        #[arg(long)]
        compat: bool,
        /// Override the output directory used for bundle staging.
        #[arg(long = "out-dir")]
        out_dir: Option<PathBuf>,
    },
    /// Recompute the RusticUI Material component parity dashboard.
    MaterialParity,
    /// Recompute the RusticUI Joy inventory to highlight missing Rust bindings.
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
        Commands::IconsBundle { compat, out_dir } => icons_bundle(out_dir, compat),
        Commands::Coverage => coverage(),
        Commands::Bench => bench(),
        Commands::UpdateComponents => update_components(),
        Commands::AccessibilityAudit => accessibility_audit(),
        Commands::AccessibilityNightly => accessibility_nightly(),
        Commands::BuildDocs => build_docs(),
        Commands::GenerateTheme {
            overrides,
            format,
            joy,
        } => generate_theme(overrides, format, joy),
        Commands::ThemesBundle {
            overrides,
            format,
            joy,
            compat,
            out_dir,
        } => themes_bundle(overrides, format, joy, compat, out_dir),
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

impl ThemeFormat {
    fn as_str(&self) -> &'static str {
        match self {
            ThemeFormat::Json => "json",
            ThemeFormat::Toml => "toml",
        }
    }
}

/// Returns the workspace root so automation can run from a stable location.
///
/// Commands like `cargo run -p rustic-ui-icons` expect relative paths that are rooted
/// at the repository top-level. Computing it once keeps subsequent helpers
/// compact and avoids repeating the ancestor traversal logic.
fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("xtask is nested two levels below the workspace root")
        .to_path_buf()
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
    // We prioritise the joy-* demos because they double as regression
    // coverage for the RusticUI bindings across all supported renderers.
    let examples = [
        "examples/joy-yew",
        "examples/joy-leptos",
        "examples/joy-dioxus",
        "examples/joy-sycamore",
    ];
    for ex in &examples {
        let mut check = Command::new("cargo");
        check
            .arg("check")
            .arg("--target")
            .arg("wasm32-unknown-unknown")
            .arg("--manifest-path")
            .arg(format!("{ex}/Cargo.toml"));
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
    let wasm_crates = ["crates/rustic-ui-joy", "crates/rustic-ui-material"];
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
    let workspace = workspace_root();

    println!("[xtask] refreshing upstream Rustic icon glyphs via the managed download utility");
    // Delegate to the existing Rust binary that fetches the latest Material
    // Design SVGs and rewrites the `rustic-ui-icons-material` feature manifest.
    let mut material = Command::new("cargo");
    material
        .current_dir(&workspace)
        .arg("run")
        .arg("-p")
        .arg("rustic-ui-icons-material")
        .arg("--bin")
        .arg("update_icons")
        .arg("--features")
        .arg("update-icons");
    run(material)?;

    println!(
        "[xtask] regenerating the consolidated rustic-ui-icons feature manifest from local assets"
    );
    // Ensure the top-level `rustic-ui-icons` crate mirrors whatever assets are now on
    // disk. This keeps the multi-set workflow deterministic across CI and
    // contributor machines.
    let mut features = Command::new("cargo");
    features
        .current_dir(&workspace)
        .arg("run")
        .arg("-p")
        .arg("rustic-ui-icons")
        .arg("--bin")
        .arg("update_features");
    run(features)
}

fn icons_bundle(out_dir: Option<PathBuf>, compat: bool) -> Result<()> {
    println!("[xtask] assembling distributable RusticUI icon archives");
    if let Err(error) = refresh_icons() {
        eprintln!(
            "[xtask][icons-bundle] icon refresh failed: {error:?}. proceeding with existing assets"
        );
    }

    let workspace = workspace_root();
    let artifact_root = out_dir.unwrap_or_else(|| workspace.join("target/artifacts/icons"));
    let bundle_root = artifact_root.join("icons");
    println!("[xtask] staging icon payload in {}", bundle_root.display());

    let mut builder = ArtifactBundleBuilder::new(&bundle_root, "icons")?;
    let icon_sources = [
        (
            workspace.join("crates/rustic-ui-icons/icons/material"),
            PathBuf::from("rustic-ui-icons/material"),
            "rustic-ui-icons-material",
        ),
        (
            workspace.join("crates/rustic-ui-icons-material/material-icons"),
            PathBuf::from("rustic-ui-icons-material"),
            "rustic-ui-icons-material-sys",
        ),
    ];

    for (source, relative_root, label) in icon_sources {
        if !source.exists() {
            println!(
                "[xtask][icons-bundle] skipping missing source {}",
                source.display()
            );
            continue;
        }
        builder.ingest_directory(
            &source,
            &relative_root,
            "icon-svg",
            "image/svg+xml",
            move |path| {
                json!({
                    "legacy_packages": ["@mui/icons-material"],
                    "icon_family": label,
                    "file_stem": path
                        .file_stem()
                        .and_then(|stem| stem.to_str())
                        .unwrap_or_default(),
                })
            },
        )?;
    }

    let summary = builder.finalize(json!({
        "legacy_packages": ["@mui/icons-material"],
        "bundle_kind": "icon-assets",
        "schema": "rustic-ui-design-tokens/v1",
    }))?;

    let summary_payload = json!({
        "bundle": "icons",
        "manifest": relative_display(&workspace, &summary.manifest),
        "archives": summary
            .archives
            .iter()
            .map(|path| relative_display(&workspace, path))
            .collect::<Vec<_>>(),
        "entries": summary.entries.len(),
        "legacy_packages": ["@mui/icons-material"],
    });
    println!(
        "[xtask][icons-bundle] summary={}",
        serde_json::to_string(&summary_payload)?
    );

    if compat {
        let destination = workspace.join("archives/assets/icons");
        let synced = summary.sync_to(&destination)?;
        println!(
            "[xtask][icons-bundle] compat-sync={}",
            relative_display(&workspace, &synced)
        );
    }

    Ok(())
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
    // Execute the fast Playwright based accessibility smoke tests that crawl
    // the primary documentation entry points. Any violation bubbles up as a
    // command failure ensuring CI visibility for release gating.
    let mut cmd = Command::new("pnpm");
    cmd.arg("test:e2e-website");
    run(cmd)
}

fn accessibility_nightly() -> Result<()> {
    // Delegate to the same Playwright test suite but toggle the extended
    // coverage mode so nightly jobs exercise every section of the content
    // tree. The `RUSTIC_UI_A11Y_MODE` variable is consumed by upcoming
    // Playwright fixtures that widen the crawl scope.
    println!("[xtask] running nightly accessibility sweeps with extended Playwright coverage");
    let mut cmd = Command::new("pnpm");
    cmd.arg("test:e2e-website");
    cmd.env("RUSTIC_UI_A11Y_MODE", "nightly");
    run(cmd)
}

fn build_docs() -> Result<()> {
    // Compose the Rust documentation experience by first generating API docs
    // (consumed through mdBook `include_str!` snippets) and then building the
    // rendered book. Splitting the steps keeps CI logs actionable and makes it
    // obvious which phase fails when new chapters land.
    println!("[xtask] generating workspace API docs so the mdBook embeds stay in sync");
    doc()?;

    let workspace = workspace_root();
    let book_dir = workspace.join("docs/rust-book");
    println!(
        "[xtask] building the Rust-first documentation book via mdBook at {}",
        book_dir.display()
    );

    let mut cmd = Command::new("mdbook");
    cmd.arg("build").arg(&book_dir);
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
    let base_theme: Theme = Theme::default();

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
    let output_dir = PathBuf::from("crates/rustic-ui-system/templates");
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
            if let Some(map) = specific.as_object() {
                let mut scoped: serde_json::Map<String, Value> = serde_json::Map::new();
                for (key, value) in map {
                    if key == "palette" {
                        if let Some(palette_map) = value.as_object() {
                            let mut palette_wrapper = serde_json::Map::new();
                            palette_wrapper
                                .insert(scheme.clone(), Value::Object(palette_map.clone()));
                            scoped.insert(key.clone(), Value::Object(palette_wrapper));
                        } else {
                            scoped.insert(key.clone(), value.clone());
                        }
                    } else {
                        scoped.insert(key.clone(), value.clone());
                    }
                }
                merge_values(&mut merged_value, &Value::Object(scoped));
            } else {
                merge_values(&mut merged_value, specific);
            }
        }

        let merged_theme: Theme = serde_json::from_value(merged_value).with_context(|| {
            format!(
                "failed to convert merged theme representation into Theme struct for `{scheme}`"
            )
        })?;
        let mut theme = merged_theme;
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
        let css = theme_provider::material_css_baseline_from_theme(&theme);
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

fn themes_bundle(
    overrides: Option<PathBuf>,
    format: ThemeFormat,
    joy: bool,
    compat: bool,
    out_dir: Option<PathBuf>,
) -> Result<()> {
    println!(
        "[xtask] preparing themed asset bundle (format: {}, joy fixtures: {joy})",
        format.as_str()
    );
    let overrides_snapshot = overrides.clone();
    generate_theme(overrides, format, joy)?;

    let workspace = workspace_root();
    let artifact_root = out_dir.unwrap_or_else(|| workspace.join("target/artifacts/themes"));
    let bundle_root = artifact_root.join("themes");
    println!("[xtask] staging theme payload in {}", bundle_root.display());

    let templates_dir = workspace.join("crates/rustic-ui-system/templates");
    let mut builder = ArtifactBundleBuilder::new(&bundle_root, "themes")?;
    for entry in WalkDir::new(&templates_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if !entry.file_type().is_file() {
            continue;
        }

        let file_name = entry.file_name().to_string_lossy();
        let relative = Path::new("templates").join(
            entry.path().strip_prefix(&templates_dir).with_context(|| {
                format!(
                    "failed to compute relative path for template {}",
                    entry.path().display()
                )
            })?,
        );

        let extension = entry
            .path()
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");

        let scheme = scheme_from_filename(&file_name);
        let (kind, media_type, metadata) = if file_name.starts_with("material_theme") {
            (
                format!("material-theme-{extension}"),
                manifest_media_type(extension),
                json!({
                    "legacy_packages": ["@mui/material", "@mui/system"],
                    "scheme": scheme,
                    "format": extension,
                }),
            )
        } else if file_name.starts_with("material_css_baseline") {
            (
                "material-css-baseline".to_string(),
                "text/css",
                json!({
                    "legacy_packages": ["@mui/material", "@mui/system"],
                    "scheme": scheme,
                    "format": "css",
                }),
            )
        } else if file_name.starts_with("joy_theme") {
            (
                "joy-theme-json".to_string(),
                "application/json",
                json!({
                    "legacy_packages": ["@mui/joy"],
                    "scheme": scheme,
                    "format": extension,
                }),
            )
        } else {
            continue;
        };

        builder.ingest_file(entry.path(), &relative, kind, media_type, metadata)?;
    }

    let override_path = overrides_snapshot
        .as_ref()
        .map(|path| relative_display(&workspace, path));
    let summary = builder.finalize(json!({
        "legacy_packages": ["@mui/material", "@mui/system"],
        "bundle_kind": "theme-assets",
        "schema": "rustic-ui-design-tokens/v1",
        "format": format.as_str(),
        "joy": joy,
        "overrides": override_path,
    }))?;

    let summary_payload = json!({
        "bundle": "themes",
        "manifest": relative_display(&workspace, &summary.manifest),
        "archives": summary
            .archives
            .iter()
            .map(|path| relative_display(&workspace, path))
            .collect::<Vec<_>>(),
        "entries": summary.entries.len(),
        "format": format.as_str(),
        "joy": joy,
    });
    println!(
        "[xtask][themes-bundle] summary={}",
        serde_json::to_string(&summary_payload)?
    );

    if compat {
        let destination = workspace.join("archives/assets/themes");
        let synced = summary.sync_to(&destination)?;
        println!(
            "[xtask][themes-bundle] compat-sync={}",
            relative_display(&workspace, &synced)
        );
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

fn scheme_from_filename(file_name: &str) -> String {
    let mut parts = file_name.split('.');
    let _prefix = parts.next();
    parts.next().unwrap_or("default").to_string()
}

fn manifest_media_type(extension: &str) -> &'static str {
    match extension {
        "json" => "application/json",
        "toml" => "application/toml",
        "css" => "text/css",
        _ => "application/octet-stream",
    }
}

fn relative_display(root: &Path, target: &Path) -> String {
    target
        .strip_prefix(root)
        .map(|relative| relative.display().to_string())
        .unwrap_or_else(|_| target.display().to_string())
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
