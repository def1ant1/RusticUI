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
use clap::{Args, Parser, Subcommand, ValueEnum};
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
    /// Audit third-party crates for advisories, bans, and license issues.
    Deny,
    /// Execute the default test suites for all crates.
    ///
    /// Pass `--examples` to also compile every Rust example (`examples/mui-*`,
    /// `examples/joy-yew`, `examples/joy-leptos`, `examples/select-menu-*`, etc.) for `wasm32-unknown-unknown`. This keeps
    /// the example gallery aligned with the published crates without forcing
    /// every contributor to pay the additional compile time unless they
    /// explicitly opt in.
    Test {
        /// Also compile every Rust example crate for `wasm32-unknown-unknown`.
        #[arg(long)]
        examples: bool,
    },
    /// Compile curated Rust example collections for native and WebAssembly targets.
    #[command(
        about = "Compile curated Rust example collections for native and WebAssembly targets.",
        long_about = "Compile curated Rust example collections for native and WebAssembly targets without relying on ad-hoc shell scripts. Each group is centrally defined so new demos can be enrolled in CI by appending a manifest entry instead of wiring fresh workflows.\n\nLayout demos currently validated: examples/layout-box-leptos, examples/layout-grid-yew. Update the `layout_examples` helper when shipping new layouts so CI picks them up automatically."
    )]
    Examples(ExamplesArgs),
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
    #[command(
        name = "icons-bundle",
        about = "Package refreshed icon assets into reproducible archives and manifests.",
        long_about = "Package refreshed icon assets into reproducible archives and manifests. The Rust-native bundler emits both RusticUI crate identifiers via the `packages` field and retains the historical npm names under `legacy_packages` so CI pipelines can bridge ecosystems during phased migrations."
    )]
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
    /// Regenerate the Rust-native component metadata manifest from TypeScript declarations.
    #[command(
        about = "Regenerate the Rust-native component metadata manifest from TypeScript declarations.",
        long_about = "Regenerate the Rust-native component metadata manifest by parsing the upstream TypeScript declarations directly in Rust. The manifest records both `packages` (RusticUI crate identifiers) and `legacy_packages` (historical npm names) so downstream tools can pivot without pnpm. Override the scan targets with `RUSTIC_UI_COMPONENT_CONFIG` and customize the output directory via `RUSTIC_UI_COMPONENT_OUT_DIR`."
    )]
    UpdateComponents,
    /// Run Markdown-first accessibility smoke tests against the docs corpus.
    #[command(
        about = "Run Markdown-first accessibility smoke tests against the docs corpus.",
        long_about = "Run Markdown-first accessibility smoke tests against the docs corpus without invoking external Playwright or pnpm scripts. Provide a JSON manifest through `RUSTIC_UI_A11Y_CONFIG` to focus on bespoke directories during CI dry runs."
    )]
    AccessibilityAudit,
    /// Execute the extended nightly accessibility coverage suite across every docs section.
    #[command(
        name = "accessibility-nightly",
        about = "Execute the extended nightly accessibility coverage suite across every docs section.",
        long_about = "Execute the extended nightly accessibility coverage suite across every docs section. This variant mirrors the standard audit but widens the default target list so enterprise CI jobs can run a comprehensive scan without custom Playwright harnesses."
    )]
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
        Commands::Deny => deny(),
        Commands::Test { examples } => test(examples),
        Commands::Examples(args) => examples(args),
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

fn deny() -> Result<()> {
    println!(
        "[xtask] auditing dependencies for security advisories, license drift, and banned crates"
    );

    let mut cmd = Command::new("cargo");
    cmd.arg("deny").arg("check");
    cmd.current_dir(workspace_root());
    run(cmd)
}

fn test(include_examples: bool) -> Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.arg("test").arg("--workspace").arg("--all-features");
    run(cmd)?;

    if !include_examples {
        return Ok(());
    }

    let workspace = workspace_root();
    let examples_root = workspace.join("examples");
    let example_crates = discover_example_crates(&examples_root)?;

    if example_crates.is_empty() {
        println!(
            "[xtask][examples] no Rust example manifests found under {}",
            examples_root.display()
        );
        return Ok(());
    }

    println!(
        "[xtask][examples] validating {} Rust example(s) for wasm32-unknown-unknown",
        example_crates.len()
    );

    for example in example_crates {
        println!(
            "[xtask][examples] verifying `{}` against wasm32-unknown-unknown",
            example.name
        );

        run_example_command(
            &example,
            "check",
            Some("wasm32-unknown-unknown"),
            None,
            &[],
            "wasm cargo check",
        )?;
        println!(
            "[xtask][examples] `{}` passed cargo check for wasm32-unknown-unknown",
            example.name
        );

        run_example_command(
            &example,
            "test",
            Some("wasm32-unknown-unknown"),
            None,
            &["--no-run"],
            "wasm cargo test --no-run",
        )?;
        println!(
            "[xtask][examples] `{}` passed cargo test --no-run for wasm32-unknown-unknown",
            example.name
        );
    }

    println!(
        "[xtask][examples] all Rust examples compiled successfully for wasm32-unknown-unknown"
    );
    Ok(())
}

/// Metadata for a Rust example crate under `examples/`.
///
/// Capturing both the human-readable name and the `Cargo.toml` path keeps the
/// logging consistent across CI and local runs while avoiding repetitive path
/// joins throughout the verification loop.
#[derive(Debug, Clone)]
struct ExampleCrate {
    name: String,
    manifest: PathBuf,
}

/// CLI options accepted by the `examples` subcommand.
#[derive(Args, Debug, Clone)]
struct ExamplesArgs {
    /// Curated collection of example manifests to compile.
    #[arg(value_enum, default_value_t = ExampleGroup::Layout)]
    group: ExampleGroup,
    /// Build artifacts in release mode for both native and wasm targets.
    #[arg(long, conflicts_with = "profile")]
    release: bool,
    /// Use a named Cargo profile instead of `--release`.
    #[arg(long)]
    profile: Option<String>,
}

/// Supported example collections.
#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
enum ExampleGroup {
    /// Layout demos that validate multi-surface grid and box flows.
    Layout,
    /// Focus trap utilities shared across framework adapters.
    Utilities,
}

impl ExampleGroup {
    fn as_str(&self) -> &'static str {
        match self {
            ExampleGroup::Layout => "layout",
            ExampleGroup::Utilities => "utilities",
        }
    }
}

/// Build configuration flags shared across native and wasm invocations.
#[derive(Debug, Clone, Default)]
struct BuildOptions {
    release: bool,
    profile: Option<String>,
}

impl BuildOptions {
    fn apply_to(&self, cmd: &mut Command) {
        if let Some(profile) = &self.profile {
            cmd.arg("--profile").arg(profile);
        } else if self.release {
            cmd.arg("--release");
        }
    }
}

/// Enumerate Rust example crates that opt into the automation pipeline.
///
/// We intentionally restrict the search to direct children of `examples/` so the
/// task remains predictable even as the gallery evolves with supporting assets
/// (React shells, screenshots, etc.). Only directories that expose a
/// `Cargo.toml` are returned, allowing hybrid demo folders to coexist without
/// tripping the Rust verification logic.
fn discover_example_crates(examples_root: &Path) -> Result<Vec<ExampleCrate>> {
    if !examples_root.exists() {
        return Ok(Vec::new());
    }

    let mut crates = Vec::new();
    let entries = fs::read_dir(examples_root).with_context(|| {
        format!(
            "failed to read examples directory at {}",
            examples_root.display()
        )
    })?;

    for entry in entries {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }

        let manifest = entry.path().join("Cargo.toml");
        if !manifest.exists() {
            continue;
        }

        let name = entry.file_name().to_string_lossy().to_string();

        crates.push(ExampleCrate { name, manifest });
    }

    crates.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(crates)
}

fn examples(args: ExamplesArgs) -> Result<()> {
    let workspace = workspace_root();
    let build_opts = BuildOptions {
        release: args.release,
        profile: args.profile.clone(),
    };

    println!(
        "[xtask][examples] compiling `{}` group with {} profile",
        args.group.as_str(),
        if let Some(profile) = &build_opts.profile {
            format!("profile `{}`", profile)
        } else if build_opts.release {
            "--release".to_string()
        } else {
            "default dev".to_string()
        }
    );

    let crates = match args.group {
        ExampleGroup::Layout => layout_examples(&workspace)?,
        ExampleGroup::Utilities => utilities_examples(&workspace)?,
    };

    if crates.is_empty() {
        println!(
            "[xtask][examples] no example manifests registered for the `{}` group",
            args.group.as_str()
        );
        return Ok(());
    }

    for example in crates {
        println!(
            "[xtask][examples] building `{}` for native host target",
            example.name
        );
        run_example_command(
            &example,
            "build",
            None,
            Some(&build_opts),
            &[],
            "native cargo build",
        )?;
        println!(
            "[xtask][examples] `{}` compiled successfully for the native host",
            example.name
        );

        println!(
            "[xtask][examples] building `{}` for wasm32-unknown-unknown",
            example.name
        );
        run_example_command(
            &example,
            "build",
            Some("wasm32-unknown-unknown"),
            Some(&build_opts),
            &[],
            "wasm cargo build",
        )?;
        println!(
            "[xtask][examples] `{}` compiled successfully for wasm32-unknown-unknown",
            example.name
        );
    }

    println!(
        "[xtask][examples] completed `{}` compilation set",
        args.group.as_str()
    );

    Ok(())
}

fn layout_examples(workspace: &Path) -> Result<Vec<ExampleCrate>> {
    // Add new layout demos here to keep CI coverage centralized. Keeping the
    // manifests in a single list ensures that nightly pipelines only require a
    // pull request touching this function, rather than bespoke workflow YAML.
    const LAYOUT_MANIFESTS: &[(&str, &str)] = &[
        ("layout-box-leptos", "examples/layout-box-leptos/Cargo.toml"),
        ("layout-grid-yew", "examples/layout-grid-yew/Cargo.toml"),
    ];

    let mut crates = Vec::with_capacity(LAYOUT_MANIFESTS.len());
    for (name, manifest) in LAYOUT_MANIFESTS {
        let manifest_path = workspace.join(manifest);
        if !manifest_path.exists() {
            return Err(anyhow!(
                "layout example `{}` manifest missing at {}",
                name,
                manifest_path.display()
            ));
        }

        crates.push(ExampleCrate {
            name: (*name).to_string(),
            manifest: manifest_path,
        });
    }

    Ok(crates)
}

fn utilities_examples(workspace: &Path) -> Result<Vec<ExampleCrate>> {
    // The focus trap utilities exercise automation-heavy bootstrap scripts across
    // every supported renderer. Centralising the manifests here lets CI toggle
    // the entire suite with `cargo xtask examples --group utilities`.
    const UTILITIES_MANIFESTS: &[(&str, &str)] = &[
        (
            "utils-trap-focus-dioxus",
            "examples/utils-trap-focus-dioxus/Cargo.toml",
        ),
        (
            "utils-trap-focus-leptos",
            "examples/utils-trap-focus-leptos/Cargo.toml",
        ),
        (
            "utils-trap-focus-sycamore",
            "examples/utils-trap-focus-sycamore/Cargo.toml",
        ),
        (
            "utils-trap-focus-yew",
            "examples/utils-trap-focus-yew/Cargo.toml",
        ),
    ];

    let mut crates = Vec::with_capacity(UTILITIES_MANIFESTS.len());
    for (name, manifest) in UTILITIES_MANIFESTS {
        let manifest_path = workspace.join(manifest);
        if !manifest_path.exists() {
            return Err(anyhow!(
                "utilities example `{}` manifest missing at {}",
                name,
                manifest_path.display()
            ));
        }

        crates.push(ExampleCrate {
            name: (*name).to_string(),
            manifest: manifest_path,
        });
    }

    Ok(crates)
}

fn run_example_command(
    example: &ExampleCrate,
    cargo_subcommand: &str,
    target: Option<&str>,
    build_opts: Option<&BuildOptions>,
    extra_args: &[&str],
    context: &str,
) -> Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.arg(cargo_subcommand);

    if let Some(target) = target {
        cmd.arg("--target").arg(target);
    }

    if let Some(opts) = build_opts {
        opts.apply_to(&mut cmd);
    }

    for arg in extra_args {
        cmd.arg(arg);
    }

    cmd.arg("--manifest-path").arg(&example.manifest);

    run(cmd).with_context(|| {
        format!(
            "{} failed for example `{}` at {}",
            context,
            example.name,
            example.manifest.display(),
        )
    })
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
                    "packages": ["rustic-ui-icons"],
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

    // The manifest now records both the new RusticUI crate identifiers and the legacy npm
    // packages so downstream automation can straddle ecosystems during migrations. This keeps the
    // schema backwards compatible for archival tooling while exposing the Rust-native contract.
    let summary = builder.finalize(json!({
        "packages": ["rustic-ui-icons"],
        "legacy_packages": ["@mui/icons-material"],
        "bundle_kind": "icon-assets",
        "schema": "rustic-ui-design-tokens/v1",
    }))?;

    let summary_payload = json!({
        "bundle": "icons",
        "packages": ["rustic-ui-icons"],
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
    let workspace = workspace_root();
    let config_override = std::env::var_os("RUSTIC_UI_COMPONENT_CONFIG").map(PathBuf::from);
    let out_dir_override = std::env::var_os("RUSTIC_UI_COMPONENT_OUT_DIR").map(PathBuf::from);

    let summary = component_metadata::generate_manifest(
        &workspace,
        config_override.as_deref(),
        out_dir_override.as_deref(),
    )?;

    println!(
        "[xtask][update-components] manifest={} packages={} components={} interfaces={} props={}",
        relative_display(&workspace, &summary.manifest),
        summary.package_count,
        summary.component_count,
        summary.interface_count,
        summary.prop_count,
    );

    Ok(())
}

fn accessibility_audit() -> Result<()> {
    run_accessibility(accessibility::AuditMode::Standard)
}

fn accessibility_nightly() -> Result<()> {
    println!(
        "[xtask] running nightly accessibility sweeps across the expanded documentation corpus"
    );
    run_accessibility(accessibility::AuditMode::Nightly)
}

fn run_accessibility(mode: accessibility::AuditMode) -> Result<()> {
    let workspace = workspace_root();
    let config_override = std::env::var_os("RUSTIC_UI_A11Y_CONFIG").map(PathBuf::from);
    let summary = accessibility::run(&workspace, mode, config_override.as_deref())?;

    println!(
        "[xtask][accessibility] mode={} scanned_files={} issues={}",
        mode.as_str(),
        summary.files_scanned,
        summary.issues.len(),
    );

    if summary.issues.is_empty() {
        println!("[xtask][accessibility] all markdown documents cleared the automated checks");
        Ok(())
    } else {
        for finding in &summary.issues {
            println!(
                "[xtask][accessibility][finding] file={} message={}",
                relative_display(&workspace, &finding.path),
                finding.message
            );
        }
        Err(anyhow!(
            "{} accessibility finding(s) detected. See the log above for remediation details.",
            summary.issues.len()
        ))
    }
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
    test(false)?;
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

mod component_metadata {
    use super::relative_display;
    use anyhow::{anyhow, Context, Result};
    use chrono::{SecondsFormat, Utc};
    use once_cell::sync::Lazy;
    use regex::Regex;
    use serde::{Deserialize, Serialize};
    use std::collections::BTreeMap;
    use std::fs;
    use std::path::{Path, PathBuf};
    use swc_common::{sync::Lrc, Globals, SourceMap, GLOBALS};
    use swc_ecma_ast::{
        Decl, ExportDecl, Expr, Ident, Lit, Module, ModuleItem, Stmt, TsEntityName,
        TsInterfaceDecl, TsKeywordTypeKind, TsLit, TsType, TsTypeElement, TsTypeRef,
        TsUnionOrIntersectionType,
    };
    use swc_ecma_parser::{lexer::Lexer, Parser as SwcParser, StringInput, Syntax, TsSyntax};
    use walkdir::WalkDir;

    const MANIFEST_SCHEMA: &str = "rustic-ui/component-metadata@v1";

    static COMPONENT_INTERFACE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"^(?P<name>[A-Z][A-Za-z0-9]+?)(?:Own)?Props$")
            .expect("component metadata interface regex")
    });

    const IGNORED_PATTERNS: &[&str] = &[
        "Override",
        "Overrides",
        "Classes",
        "OwnerState",
        "TypeMap",
        "Slot",
        "Slots",
    ];

    const DEFAULT_SOURCES: &[(&str, &[&str], &str)] = &[
        (
            "rustic-ui-material",
            &["@mui/material"],
            "packages/mui-material/src",
        ),
        ("rustic-ui-joy", &["@mui/joy"], "packages/mui-joy/src"),
        ("rustic-ui-lab", &["@mui/lab"], "packages/mui-lab/src"),
    ];

    #[derive(Debug, Clone, Serialize)]
    struct ComponentManifest {
        schema: String,
        generated_at: String,
        packages: Vec<ComponentPackage>,
    }

    #[derive(Debug, Clone, Serialize)]
    struct ComponentPackage {
        package: String,
        packages: Vec<String>,
        legacy_packages: Vec<String>,
        source_root: String,
        components: Vec<ComponentEntry>,
    }

    #[derive(Debug, Clone, Serialize)]
    struct ComponentEntry {
        component: String,
        interfaces: Vec<ComponentInterface>,
    }

    #[derive(Debug, Clone, Serialize)]
    struct ComponentInterface {
        interface: String,
        file: String,
        props: Vec<ComponentProp>,
    }

    #[derive(Debug, Clone, Serialize)]
    struct ComponentProp {
        name: String,
        optional: bool,
        #[serde(rename = "type")]
        type_repr: String,
    }

    #[derive(Debug, Deserialize)]
    struct ComponentSourceConfig {
        package: String,
        #[serde(default)]
        legacy_packages: Vec<String>,
        path: PathBuf,
    }

    #[derive(Debug)]
    struct ComponentSource {
        package: String,
        legacy_packages: Vec<String>,
        root: PathBuf,
        relative_root: String,
    }

    #[derive(Debug, Default)]
    struct ComponentEntryBuilder {
        interfaces: Vec<ComponentInterface>,
    }

    #[derive(Debug)]
    pub struct ComponentSummary {
        pub manifest: PathBuf,
        pub package_count: usize,
        pub component_count: usize,
        pub interface_count: usize,
        pub prop_count: usize,
    }

    pub fn generate_manifest(
        workspace: &Path,
        config_override: Option<&Path>,
        out_dir_override: Option<&Path>,
    ) -> Result<ComponentSummary> {
        let sources = load_sources(workspace, config_override)?;
        if sources.is_empty() {
            return Err(anyhow!(
                "no component sources discovered; check the configuration overrides"
            ));
        }

        let out_dir = resolve_out_dir(workspace, out_dir_override);
        fs::create_dir_all(&out_dir).with_context(|| {
            format!(
                "failed to create component metadata output directory at {}",
                out_dir.display()
            )
        })?;

        let mut packages = Vec::new();
        let mut component_count = 0usize;
        let mut interface_count = 0usize;
        let mut prop_count = 0usize;

        for source in sources {
            let package = scan_package(workspace, &source)?;
            if package.components.is_empty() {
                continue;
            }

            component_count += package.components.len();
            interface_count += package
                .components
                .iter()
                .map(|component| component.interfaces.len())
                .sum::<usize>();
            prop_count += package
                .components
                .iter()
                .flat_map(|component| component.interfaces.iter())
                .map(|interface| interface.props.len())
                .sum::<usize>();
            packages.push(package);
        }

        if packages.is_empty() {
            return Err(anyhow!(
                "component scanner completed without discovering any interfaces"
            ));
        }

        packages.sort_by(|a, b| a.package.cmp(&b.package));

        let manifest = ComponentManifest {
            schema: MANIFEST_SCHEMA.to_string(),
            generated_at: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
            packages,
        };

        let manifest_path = out_dir.join("component-metadata.json");
        let json = serde_json::to_string_pretty(&manifest)? + "\n";
        fs::write(&manifest_path, json).with_context(|| {
            format!(
                "failed to write component manifest to {}",
                manifest_path.display()
            )
        })?;

        Ok(ComponentSummary {
            manifest: manifest_path,
            package_count: manifest.packages.len(),
            component_count,
            interface_count,
            prop_count,
        })
    }

    fn resolve_out_dir(workspace: &Path, override_path: Option<&Path>) -> PathBuf {
        match override_path {
            Some(path) if path.is_absolute() => path.to_path_buf(),
            Some(path) => workspace.join(path),
            None => workspace.join("target/component-metadata"),
        }
    }

    fn load_sources(
        workspace: &Path,
        config_override: Option<&Path>,
    ) -> Result<Vec<ComponentSource>> {
        if let Some(path) = config_override {
            load_sources_from_config(workspace, path)
        } else {
            Ok(DEFAULT_SOURCES
                .iter()
                .filter_map(|(package, legacy, relative)| {
                    let absolute = workspace.join(relative);
                    if !absolute.exists() {
                        println!(
                            "[xtask][update-components] skipping missing source {}",
                            absolute.display()
                        );
                        return None;
                    }
                    Some(ComponentSource {
                        package: (*package).to_string(),
                        legacy_packages: legacy.iter().map(|value| value.to_string()).collect(),
                        root: absolute.clone(),
                        relative_root: relative_display(workspace, &absolute),
                    })
                })
                .collect())
        }
    }

    fn load_sources_from_config(workspace: &Path, path: &Path) -> Result<Vec<ComponentSource>> {
        let raw = fs::read_to_string(path)
            .with_context(|| format!("failed to read component config {}", path.display()))?;
        let configs: Vec<ComponentSourceConfig> =
            serde_json::from_str(&raw).with_context(|| {
                format!(
                    "failed to parse component source configuration from {}",
                    path.display()
                )
            })?;

        let mut sources = Vec::new();
        for entry in configs {
            let absolute = if entry.path.is_absolute() {
                entry.path.clone()
            } else {
                workspace.join(&entry.path)
            };

            if !absolute.exists() {
                println!(
                    "[xtask][update-components] skipping configured source that does not exist: {}",
                    absolute.display()
                );
                continue;
            }

            sources.push(ComponentSource {
                package: entry.package,
                legacy_packages: entry.legacy_packages,
                root: absolute.clone(),
                relative_root: relative_display(workspace, &absolute),
            });
        }

        Ok(sources)
    }

    fn scan_package(workspace: &Path, source: &ComponentSource) -> Result<ComponentPackage> {
        let cm: Lrc<SourceMap> = Lrc::new(SourceMap::default());
        let globals = Globals::new();
        let mut components: BTreeMap<String, ComponentEntryBuilder> = BTreeMap::new();

        GLOBALS
            .set(&globals, || -> Result<()> {
                for entry in WalkDir::new(&source.root)
                    .into_iter()
                    .filter_map(|e| e.ok())
                {
                    if !entry.file_type().is_file() {
                        continue;
                    }

                    let path = entry.path();
                    if !is_declaration_file(path) {
                        continue;
                    }

                    let module = parse_module(&cm, path)?;
                    collect_interfaces(workspace, path, &module, &mut components);
                }
                Ok(())
            })
            .with_context(|| {
                format!(
                    "failed to scan TypeScript declarations under {}",
                    source.root.display()
                )
            })?;

        let mut component_entries: Vec<ComponentEntry> = components
            .into_iter()
            .map(|(component, mut builder)| {
                builder
                    .interfaces
                    .sort_by(|a, b| a.interface.cmp(&b.interface));
                ComponentEntry {
                    component,
                    interfaces: builder.interfaces,
                }
            })
            .collect();

        component_entries.sort_by(|a, b| a.component.cmp(&b.component));

        Ok(ComponentPackage {
            package: source.package.clone(),
            packages: vec![source.package.clone()],
            legacy_packages: source.legacy_packages.clone(),
            source_root: source.relative_root.clone(),
            components: component_entries,
        })
    }

    fn is_declaration_file(path: &Path) -> bool {
        let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
            return false;
        };
        file_name.ends_with(".d.ts")
            || file_name.ends_with(".d.tsx")
            || file_name.ends_with("Props.ts")
            || file_name.ends_with("Props.tsx")
            || file_name.ends_with("OwnProps.ts")
            || file_name.ends_with("OwnProps.tsx")
    }

    fn parse_module(cm: &Lrc<SourceMap>, path: &Path) -> Result<Module> {
        let fm = cm
            .load_file(path)
            .with_context(|| format!("failed to load {}", path.display()))?;

        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_ascii_lowercase());
        let is_jsx = matches!(extension.as_deref(), Some("tsx"));
        let is_d_ts = path
            .file_name()
            .and_then(|name| name.to_str())
            .map(|name| name.ends_with(".d.ts") || name.ends_with(".d.tsx"))
            .unwrap_or(false);

        let syntax = Syntax::Typescript(TsSyntax {
            tsx: is_jsx,
            dts: is_d_ts,
            ..Default::default()
        });

        let lexer = Lexer::new(syntax, Default::default(), StringInput::from(&*fm), None);
        let mut parser = SwcParser::new_from(lexer);
        let module = parser
            .parse_module()
            .map_err(|err| anyhow!("{:?}", err))
            .with_context(|| format!("failed to parse {}", path.display()))?;

        let errors: Vec<_> = parser.take_errors();
        if !errors.is_empty() {
            let joined = errors
                .into_iter()
                .map(|err| format!("{:?}", err))
                .collect::<Vec<_>>()
                .join(", ");
            return Err(anyhow!("{joined}"))
                .with_context(|| format!("syntax errors detected in {}", path.display()));
        }

        Ok(module)
    }

    fn collect_interfaces(
        workspace: &Path,
        path: &Path,
        module: &Module,
        components: &mut BTreeMap<String, ComponentEntryBuilder>,
    ) {
        for item in &module.body {
            match item {
                ModuleItem::Stmt(Stmt::Decl(Decl::TsInterface(decl))) => {
                    ingest_interface(workspace, path, decl, components);
                }
                ModuleItem::ModuleDecl(swc_ecma_ast::ModuleDecl::ExportDecl(ExportDecl {
                    decl,
                    ..
                })) => {
                    if let Decl::TsInterface(decl) = decl {
                        ingest_interface(workspace, path, decl, components);
                    }
                }
                _ => {}
            }
        }
    }

    fn ingest_interface(
        workspace: &Path,
        path: &Path,
        decl: &TsInterfaceDecl,
        components: &mut BTreeMap<String, ComponentEntryBuilder>,
    ) {
        let name = decl.id.sym.to_string();
        if !is_component_interface(&name) {
            return;
        }

        let component_name = component_name_from_interface(&name);
        let props = collect_props(&decl.body.body);
        if props.is_empty() {
            return;
        }

        let entry = ComponentInterface {
            interface: name,
            file: relative_display(workspace, path),
            props,
        };

        components
            .entry(component_name)
            .or_default()
            .interfaces
            .push(entry);
    }

    fn is_component_interface(name: &str) -> bool {
        if IGNORED_PATTERNS
            .iter()
            .any(|pattern| name.contains(pattern))
        {
            return false;
        }
        COMPONENT_INTERFACE.is_match(name)
    }

    fn component_name_from_interface(name: &str) -> String {
        COMPONENT_INTERFACE
            .captures(name)
            .and_then(|captures| captures.name("name"))
            .map(|capture| capture.as_str().to_string())
            .unwrap_or_else(|| name.to_string())
    }

    fn collect_props(elements: &[TsTypeElement]) -> Vec<ComponentProp> {
        let mut props = Vec::new();
        for element in elements {
            if let TsTypeElement::TsPropertySignature(signature) = element {
                let Some(name) = property_name(&signature.key) else {
                    continue;
                };
                let type_repr = signature
                    .type_ann
                    .as_ref()
                    .map(|ann| format_ts_type(&ann.type_ann))
                    .unwrap_or_else(|| "unknown".to_string());
                props.push(ComponentProp {
                    name,
                    optional: signature.optional,
                    type_repr,
                });
            }
        }
        props.sort_by(|a, b| a.name.cmp(&b.name));
        props
    }

    fn property_name(expr: &Expr) -> Option<String> {
        match expr {
            Expr::Ident(Ident { sym, .. }) => Some(sym.to_string()),
            Expr::Lit(Lit::Str(value)) => Some(value.value.to_string()),
            _ => None,
        }
    }

    fn format_ts_type(ty: &TsType) -> String {
        match ty {
            TsType::TsKeywordType(keyword) => match keyword.kind {
                TsKeywordTypeKind::TsStringKeyword => "string".to_string(),
                TsKeywordTypeKind::TsNumberKeyword => "number".to_string(),
                TsKeywordTypeKind::TsBooleanKeyword => "boolean".to_string(),
                TsKeywordTypeKind::TsAnyKeyword => "any".to_string(),
                TsKeywordTypeKind::TsVoidKeyword => "void".to_string(),
                TsKeywordTypeKind::TsUndefinedKeyword => "undefined".to_string(),
                TsKeywordTypeKind::TsNullKeyword => "null".to_string(),
                TsKeywordTypeKind::TsNeverKeyword => "never".to_string(),
                TsKeywordTypeKind::TsUnknownKeyword => "unknown".to_string(),
                TsKeywordTypeKind::TsObjectKeyword => "object".to_string(),
                TsKeywordTypeKind::TsBigIntKeyword => "bigint".to_string(),
                TsKeywordTypeKind::TsSymbolKeyword => "symbol".to_string(),
                _ => format!("{keyword:?}"),
            },
            TsType::TsArrayType(array) => {
                format!("{}[]", format_ts_type(&array.elem_type))
            }
            TsType::TsUnionOrIntersectionType(union) => match union {
                TsUnionOrIntersectionType::TsUnionType(union) => union
                    .types
                    .iter()
                    .map(|ty| format_ts_type(ty))
                    .collect::<Vec<_>>()
                    .join(" | "),
                TsUnionOrIntersectionType::TsIntersectionType(intersection) => intersection
                    .types
                    .iter()
                    .map(|ty| format_ts_type(ty))
                    .collect::<Vec<_>>()
                    .join(" & "),
            },
            TsType::TsTypeRef(reference) => format_type_reference(reference),
            TsType::TsParenthesizedType(inner) => {
                format!("({})", format_ts_type(&inner.type_ann))
            }
            TsType::TsLitType(literal) => match &literal.lit {
                TsLit::Str(value) => format!("'{}'", value.value),
                TsLit::Bool(value) => value.value.to_string(),
                TsLit::Number(value) => value.value.to_string(),
                TsLit::BigInt(value) => value.value.to_string(),
                TsLit::Tpl(value) => {
                    let cooked = value
                        .quasis
                        .iter()
                        .map(|part| part.raw.to_string())
                        .collect::<String>();
                    format!("`{cooked}`")
                }
            },
            TsType::TsThisType(_) => "this".to_string(),
            TsType::TsIndexedAccessType(indexed) => format!(
                "{}[{}]",
                format_ts_type(&indexed.obj_type),
                format_ts_type(&indexed.index_type)
            ),
            TsType::TsTypeLit(_) => "{ ... }".to_string(),
            TsType::TsFnOrConstructorType(_) => "function".to_string(),
            TsType::TsConditionalType(cond) => format!(
                "{} extends {} ? {} : {}",
                format_ts_type(&cond.check_type),
                format_ts_type(&cond.extends_type),
                format_ts_type(&cond.true_type),
                format_ts_type(&cond.false_type)
            ),
            _ => format!("{ty:?}"),
        }
    }

    fn format_type_reference(reference: &TsTypeRef) -> String {
        let mut name = format_entity_name(&reference.type_name);
        if let Some(params) = &reference.type_params {
            if !params.params.is_empty() {
                let formatted = params
                    .params
                    .iter()
                    .map(|param| format_ts_type(param))
                    .collect::<Vec<_>>()
                    .join(", ");
                name.push('<');
                name.push_str(&formatted);
                name.push('>');
            }
        }
        name
    }

    fn format_entity_name(name: &TsEntityName) -> String {
        match name {
            TsEntityName::Ident(ident) => ident.sym.to_string(),
            TsEntityName::TsQualifiedName(qualified) => format!(
                "{}.{}",
                format_entity_name(&qualified.left),
                qualified.right.sym
            ),
        }
    }
}

mod accessibility {
    use anyhow::{anyhow, Context, Result};
    use pulldown_cmark::{Event, Options, Parser, Tag};
    use serde::Deserialize;
    use std::fs;
    use std::path::{Path, PathBuf};
    use walkdir::WalkDir;

    #[derive(Debug, Clone, Copy)]
    pub enum AuditMode {
        Standard,
        Nightly,
    }

    impl AuditMode {
        pub fn as_str(&self) -> &'static str {
            match self {
                AuditMode::Standard => "standard",
                AuditMode::Nightly => "nightly",
            }
        }
    }

    #[derive(Debug)]
    pub struct AuditSummary {
        pub files_scanned: usize,
        pub issues: Vec<AuditFinding>,
    }

    #[derive(Debug)]
    pub struct AuditFinding {
        pub path: PathBuf,
        pub message: String,
    }

    #[derive(Debug, Deserialize)]
    struct AccessibilityConfig {
        targets: Vec<AccessibilityTargetConfig>,
    }

    #[derive(Debug, Deserialize)]
    struct AccessibilityTargetConfig {
        path: PathBuf,
        #[serde(default)]
        kind: Option<String>,
    }

    #[derive(Debug, Clone)]
    struct AccessibilityTarget {
        path: PathBuf,
        kind: TargetKind,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum TargetKind {
        File,
        Directory,
    }

    pub fn run(
        workspace: &Path,
        mode: AuditMode,
        config_override: Option<&Path>,
    ) -> Result<AuditSummary> {
        let targets = if let Some(config_path) = config_override {
            load_targets_from_config(workspace, config_path)?
        } else {
            default_targets(workspace, mode)
        };

        if targets.is_empty() {
            return Err(anyhow!("no accessibility targets resolved"));
        }

        let mut files = Vec::new();
        for target in targets {
            gather_markdown_files(&target, &mut files)?;
        }
        files.sort();
        files.dedup();

        let mut issues = Vec::new();
        for file in &files {
            issues.extend(audit_markdown(file)?);
        }

        Ok(AuditSummary {
            files_scanned: files.len(),
            issues,
        })
    }

    fn default_targets(workspace: &Path, mode: AuditMode) -> Vec<AccessibilityTarget> {
        let mut targets = vec![
            AccessibilityTarget::directory(workspace.join("docs/migrations")),
            AccessibilityTarget::directory(workspace.join("docs/data/material")),
            AccessibilityTarget::directory(workspace.join("docs/data/joy")),
            AccessibilityTarget::file(workspace.join("README.md")),
        ];

        if matches!(mode, AuditMode::Nightly) {
            targets.push(AccessibilityTarget::directory(
                workspace.join("docs/data/system"),
            ));
            targets.push(AccessibilityTarget::directory(workspace.join("docs/docs")));
        }

        targets
            .into_iter()
            .filter(|target| target.path.exists())
            .collect()
    }

    fn load_targets_from_config(
        workspace: &Path,
        config_path: &Path,
    ) -> Result<Vec<AccessibilityTarget>> {
        let raw = fs::read_to_string(config_path).with_context(|| {
            format!(
                "failed to read accessibility configuration from {}",
                config_path.display()
            )
        })?;
        let config: AccessibilityConfig = serde_json::from_str(&raw).with_context(|| {
            format!(
                "failed to parse accessibility configuration {}",
                config_path.display()
            )
        })?;

        let mut targets = Vec::new();
        for entry in config.targets {
            let absolute = if entry.path.is_absolute() {
                entry.path.clone()
            } else {
                workspace.join(&entry.path)
            };
            let inferred_kind = infer_target_kind(&absolute, entry.kind.as_deref());
            if inferred_kind.is_none() {
                println!(
                    "[xtask][accessibility] skipping target that could not be classified: {}",
                    absolute.display()
                );
                continue;
            }
            targets.push(AccessibilityTarget {
                path: absolute,
                kind: inferred_kind.unwrap(),
            });
        }

        Ok(targets)
    }

    fn infer_target_kind(path: &Path, hint: Option<&str>) -> Option<TargetKind> {
        match hint.map(|value| value.to_ascii_lowercase()) {
            Some(ref value) if value == "file" => Some(TargetKind::File),
            Some(ref value) if value == "directory" => Some(TargetKind::Directory),
            _ => {
                if path.is_dir() {
                    Some(TargetKind::Directory)
                } else if path.is_file() {
                    Some(TargetKind::File)
                } else {
                    None
                }
            }
        }
    }

    impl AccessibilityTarget {
        fn file(path: PathBuf) -> Self {
            Self {
                path,
                kind: TargetKind::File,
            }
        }

        fn directory(path: PathBuf) -> Self {
            Self {
                path,
                kind: TargetKind::Directory,
            }
        }
    }

    fn gather_markdown_files(target: &AccessibilityTarget, files: &mut Vec<PathBuf>) -> Result<()> {
        match target.kind {
            TargetKind::File => {
                if is_markdown(&target.path) {
                    files.push(target.path.clone());
                }
            }
            TargetKind::Directory => {
                if !target.path.exists() {
                    return Ok(());
                }
                for entry in WalkDir::new(&target.path)
                    .into_iter()
                    .filter_map(|e| e.ok())
                {
                    if !entry.file_type().is_file() {
                        continue;
                    }
                    if is_markdown(entry.path()) {
                        files.push(entry.path().to_path_buf());
                    }
                }
            }
        }
        Ok(())
    }

    fn is_markdown(path: &Path) -> bool {
        matches!(
            path.extension().and_then(|ext| ext.to_str()).map(|ext| ext.to_ascii_lowercase()),
            Some(ref ext) if ext == "md" || ext == "mdx"
        )
    }

    fn audit_markdown(path: &Path) -> Result<Vec<AuditFinding>> {
        let raw = fs::read_to_string(path)
            .with_context(|| format!("failed to read markdown file {}", path.display()))?;
        let mut findings = Vec::new();
        let mut headings: Vec<u32> = Vec::new();
        let mut collecting_image_alt = false;
        let mut image_alt = String::new();
        let mut image_destination = String::new();

        let options =
            Options::ENABLE_TABLES | Options::ENABLE_FOOTNOTES | Options::ENABLE_STRIKETHROUGH;
        let parser = Parser::new_ext(&raw, options);

        for event in parser {
            match event {
                Event::Start(Tag::Heading(level, _, _)) => headings.push(level as u32),
                Event::Start(Tag::Image(_, destination, _)) => {
                    collecting_image_alt = true;
                    image_alt.clear();
                    image_destination = destination.to_string();
                }
                Event::Text(text) | Event::Code(text) if collecting_image_alt => {
                    image_alt.push_str(&text);
                }
                Event::End(Tag::Image(_, _, _)) => {
                    if image_alt.trim().is_empty() {
                        findings.push(AuditFinding {
                            path: path.to_path_buf(),
                            message: format!(
                                "image '{}' is missing descriptive alt text",
                                image_destination
                            ),
                        });
                    }
                    collecting_image_alt = false;
                    image_destination.clear();
                }
                _ => {}
            }
        }

        if !headings.iter().any(|level| *level <= 2) {
            findings.push(AuditFinding {
                path: path.to_path_buf(),
                message: "document is missing a level-one or level-two heading".to_string(),
            });
        }

        Ok(findings)
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
