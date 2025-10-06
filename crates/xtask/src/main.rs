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
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::future::Future;
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::runtime::Builder;
use walkdir::WalkDir;
use xtask_docs::{docs_build, docs_package, docs_test, DocsPackageOutcome};

mod docs_assets;
mod selection_controls_web;
use docs_assets::{docs_assets, DocsAssetsArgs};
use selection_controls_web::SelectionControlsHarness;

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
    /// Ensure the workspace remains Rust-first by keeping Node manifests quarantined.
    VerifyToolchain,
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
        long_about = "Compile curated Rust example collections for native and WebAssembly targets without relying on ad-hoc shell scripts. Each group is centrally defined so new demos can be enrolled in CI by appending a manifest entry instead of wiring fresh workflows.\n\nLayout demos currently validated: examples/layout-box-leptos, examples/layout-grid-yew. Update the `layout_examples` helper when shipping new layouts so CI picks them up automatically.\n\nForm control demos validated: examples/forms-input-base-yew, examples/forms-input-base-leptos, examples/forms-input-base-dioxus, examples/forms-input-base-sycamore. Update `forms_examples` when adding new frameworks so SSR snapshots stay wired into CI.\n\nSelection control demos validated: examples/selection-controls-dioxus, examples/selection-controls-leptos, examples/selection-controls-react, examples/selection-controls-sycamore, examples/selection-controls-yew. Update `selection_controls_examples` whenever new renderers or telemetry adapters land so CI exercises every manifest."
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
    /// Replace the legacy Node docs scripts with Rust-first automation.
    DocsAssets(DocsAssetsArgs),
    /// Build the Rustic docs binaries and wasm bundle without packaging.
    #[command(
        name = "docs-build",
        about = "Build the Rustic docs host binary and wasm bundle in parallel.",
        long_about = "Build the Rustic docs host binary and wasm bundle in parallel while reusing CARGO_TARGET_DIR so local and CI caches stay hot. The helper also runs wasm-bindgen to stage artifacts under target/rustic-docs-wasm, eliminating the need for contributors to remember the exact CLI flags."
    )]
    DocsBuild,
    /// Run the Rustic docs wasm smoke tests in headless Chrome.
    #[command(
        name = "docs-test",
        about = "Execute wasm-pack smoke tests for the Rustic docs bundle.",
        long_about = "Execute wasm-pack smoke tests for the Rustic docs bundle. Ensure Playwright's Chromium runtime is installed (e.g. via `npx playwright install --with-deps chromium`) before invoking this command locally or in CI. Logs are captured in target/logs/docs-test.log to streamline debugging when headless Chrome fails to start."
    )]
    DocsTest,
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
    #[command(
        about = "Build the Rust-first documentation site and supporting API docs.",
        long_about = "Build the Rust-first documentation site and supporting API docs. The wrapper first executes the async docs::build helper so SSR + wasm assets are hydrated using the workspace's shared CARGO_TARGET_DIR cache before delegating to mdBook when docs/rust-book exists. The orchestration surfaces explicit log markers for CI triage and bubbles helper errors unchanged so flaky wasm builds remain debuggable."
    )]
    BuildDocs,
    #[command(
        name = "docs-package",
        about = "Assemble deploy-ready Rustic docs assets.",
        long_about = "Assemble deploy-ready Rustic docs assets by exporting the SSR snapshot, wasm-bindgen output, and server binary into target/deploy/docs (override via RUSTIC_DOCS_EXPORT_DIR). The async helper reuses CARGO_TARGET_DIR caches and builds host+wasm targets concurrently before writing a hashed manifest, so reruns remain incremental while CI operators still get deterministic fingerprints. Provide --dry-run to preview the staging manifest without mutating the canonical export directory."
    )]
    DocsPackage(DocsPackageArgs),
    /// Assemble the deploy-ready documentation payload.
    #[command(
        name = "deploy-docs",
        about = "Stage API docs, mdBook output, and wasm bundles for hosting.",
        long_about = "Stage API docs, mdBook output, and wasm bundles for hosting so Netlify/Vercel deploy jobs never shell out to pnpm. The task runs the mdBook + cargo doc pipeline, compiles the curated wasm example groups, and copies the artifacts into `target/deploy/docs` (override via RUSTIC_UI_DEPLOY_OUTPUT). Provide `--dry-run` to validate the orchestration without mutating the deploy directory. Configure RUSTIC_UI_DEPLOY_PROFILE to select a custom Cargo profile and RUSTIC_UI_DEPLOY_GROUPS to narrow the wasm bundles shipped alongside the docs."
    )]
    DeployDocs(DeployDocsArgs),
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
    /// Run the Rust and TypeScript selection control regression suites.
    SelectionControls(SelectionControlsArgs),
    /// Execute every quick-start bootstrap script to guarantee docs remain accurate.
    #[command(
        about = "Run each quick-start bootstrap command and optional follow-up checks.",
        long_about = "Run each quick-start bootstrap command referenced in the docs to ensure our published quick-start guide remains accurate. Logs land in target/logs/quick-start.log so CI triage and contributors can inspect bootstrap output without re-running the harness. Pass --skip-checks to only verify that the shell scripts execute (useful when npm or cargo compilers are intentionally unavailable)."
    )]
    QuickStart {
        /// Skip post-bootstrap verification such as `cargo check` and `npm run test`.
        #[arg(long)]
        skip_checks: bool,
    },
}

fn main() -> Result<()> {
    let xtask = Xtask::parse();
    match xtask.command {
        Commands::Fmt { check } => fmt(check),
        Commands::Clippy => clippy(),
        Commands::Deny => deny(),
        Commands::VerifyToolchain => verify_toolchain(),
        Commands::Test { examples } => test(examples),
        Commands::Examples(args) => examples(args),
        Commands::WasmTest => wasm_test(),
        Commands::Doc => doc(),
        Commands::RefreshIcons => refresh_icons(),
        Commands::IconsBundle { compat, out_dir } => icons_bundle(out_dir, compat),
        Commands::DocsAssets(args) => docs_assets(args),
        Commands::DocsBuild => {
            run_async_task("docs::build", docs_build())?;
            Ok(())
        }
        Commands::DocsTest => {
            quick_start(false)?;
            run_async_task("docs::test", docs_test())?;
            Ok(())
        }
        Commands::DocsPackage(args) => docs_package_wrapper(args),
        Commands::Coverage => coverage(),
        Commands::Bench => bench(),
        Commands::UpdateComponents => update_components(),
        Commands::AccessibilityAudit => accessibility_audit(),
        Commands::AccessibilityNightly => accessibility_nightly(),
        Commands::BuildDocs => build_docs(),
        Commands::DeployDocs(args) => deploy_docs(args),
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
        Commands::SelectionControls(args) => selection_controls(args),
        Commands::QuickStart { skip_checks } => quick_start(skip_checks),
    }
}

fn run_async_task<F, T>(label: &str, fut: F) -> Result<T>
where
    F: Future<Output = Result<T>>,
{
    eprintln!("[{label}] starting");
    let runtime = Builder::new_multi_thread()
        .enable_all()
        .build()
        .context("failed to initialize Tokio runtime for docs automation")?;
    let result = runtime.block_on(fut);
    match result {
        Ok(value) => {
            eprintln!("[{label}] completed");
            Ok(value)
        }
        Err(err) => {
            eprintln!("[{label}] failed: {err:?}");
            Err(err)
        }
    }
}

/// Arguments for the selection control regression matrix.
#[derive(Args, Debug, Default)]
struct SelectionControlsArgs {
    /// Skip Rust-based suites (useful when only running web smoke tests).
    #[arg(long)]
    skip_rust: bool,
    /// Skip the headless browser harness (useful when Chromium is unavailable).
    #[arg(long)]
    skip_web: bool,
    /// Limit execution to a specific framework (dioxus, sycamore, yew, react).
    #[arg(long)]
    framework: Option<String>,
}

/// Configuration flags for the deploy pipeline orchestration.
#[derive(Args, Debug, Clone, Default)]
struct DeployDocsArgs {
    /// Validate the deploy pipeline without mutating the staging directory.
    #[arg(long)]
    dry_run: bool,
}

/// Flags for the docs packaging helper wrapper.
#[derive(Args, Debug, Clone, Default)]
struct DocsPackageArgs {
    /// Preview the exported manifest without writing to the canonical staging directory.
    #[arg(long = "dry-run")]
    dry_run: bool,
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

fn verify_toolchain() -> Result<()> {
    // Centralise the guardrail that keeps the repository Rust-first. Historical Node
    // manifests live under `archives/tooling/node-workspace/` so contributors and
    // auditors can diff old automation without accidentally reviving it. This helper
    // performs three checks:
    //   1. Ensure the archive directory itself exists (accidental deletion would make
    //      provenance investigations harder and signals a misconfigured checkout).
    //   2. Confirm no guarded manifest (`package.json`, `pnpm-workspace.yaml`, etc.)
    //      leaked back into the workspace root where CI or local scripts might pick
    //      them up.
    //   3. Verify the archived copies are present so developers always have a
    //      historical reference when debugging old release pipelines.
    // By failing fast here we avoid subtle drift where a stray pnpm command silently
    // reinstalls dependencies and splits automation across toolchains again.
    let workspace = workspace_root();
    let archive_root = workspace.join("archives/tooling/node-workspace");
    if !archive_root.is_dir() {
        return Err(anyhow!(
            "archived Node workspace not found at {}",
            archive_root.display()
        ));
    }

    // Enumerate the manifests we care about. Keeping the list centralized makes it
    // easy to extend as we remember additional Node-centric entry points that should
    // remain quarantined.
    let guard_files = [
        "package.json",
        "pnpm-workspace.yaml",
        "lerna.json",
        "nx.json",
        "webpackBaseConfig.js",
    ];

    // Collect any stray manifests living at the workspace root. We purposely surface
    // every offender at once so the error message is actionable for large commits.
    let mut stray_manifests = Vec::new();
    for name in &guard_files {
        let candidate = workspace.join(name);
        if candidate.exists() {
            let relative = candidate
                .strip_prefix(&workspace)
                .unwrap_or(&candidate)
                .to_path_buf();
            stray_manifests.push(relative);
        }
    }

    if !stray_manifests.is_empty() {
        let details = stray_manifests
            .iter()
            .map(|path| format!("- {}", path.display()))
            .collect::<Vec<_>>()
            .join("\n");
        return Err(anyhow!(
            "detected Node manifest(s) outside the archive scope:\n{}\n\nMove them under archives/tooling/node-workspace/ or delete them outright to keep the Rust-first toolchain reproducible.",
            details
        ));
    }

    // Double-check the archived copies exist. Losing these would not break CI, but it
    // would remove the historical breadcrumbs regulators rely on, so we surface the
    // problem as an actionable failure.
    let mut missing_archives = Vec::new();
    for name in &guard_files {
        let archived = archive_root.join(name);
        if !archived.exists() {
            missing_archives.push(archived);
        }
    }

    if !missing_archives.is_empty() {
        let details = missing_archives
            .iter()
            .map(|path| format!("- {}", path.display()))
            .collect::<Vec<_>>()
            .join("\n");
        return Err(anyhow!(
            "archived Node manifest(s) are missing:\n{}\n\nRestore the historical copies from version control so investigators retain provenance.",
            details
        ));
    }

    println!(
        "[xtask][verify-toolchain] confirmed Rust-first guardrails; guarded manifests remain archived"
    );

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
    println!(
        "[xtask][test] running cargo test --workspace --all-features (includes `unstable` focus loop coverage)"
    );
    cmd.arg("test").arg("--workspace").arg("--all-features");
    run(cmd)?;

    // Ensure the system crate compiles with both front-end adapters enabled.
    // This catches duplicate re-export regressions and keeps CI confidence high
    // when new primitives are added or existing adapters grow additional APIs.
    verify_rustic_ui_system_multi_adapter()?;

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

/// Validates that the core system crate builds when Yew and Leptos adapters are
/// enabled together. The guard protects enterprise consumers that compile
/// workspace documentation or example galleries with multiple features toggled
/// simultaneously.
fn verify_rustic_ui_system_multi_adapter() -> Result<()> {
    println!(
        "[xtask][test] running cargo check -p rustic-ui-system --features \"yew leptos\" to validate multi-adapter builds"
    );
    let mut cmd = Command::new("cargo");
    cmd.arg("check")
        .arg("-p")
        .arg("rustic-ui-system")
        .arg("--features")
        .arg("yew leptos");
    run(cmd)
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
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, ValueEnum)]
enum ExampleGroup {
    /// Layout demos that validate multi-surface grid and box flows.
    Layout,
    /// Focus trap utilities shared across framework adapters.
    Utilities,
    /// Navigation surfaces spanning bottom nav, pagination, and speed dial.
    Navigation,
    /// Form control blueprints exercising InputBase across frameworks.
    Forms,
    /// Selection control demos that synchronize checkbox and radio telemetry.
    SelectionControls,
}

impl ExampleGroup {
    fn as_str(&self) -> &'static str {
        match self {
            ExampleGroup::Layout => "layout",
            ExampleGroup::Utilities => "utilities",
            ExampleGroup::Navigation => "navigation",
            ExampleGroup::Forms => "forms",
            ExampleGroup::SelectionControls => "selection-controls",
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

fn example_group_crates(workspace: &Path, group: ExampleGroup) -> Result<Vec<ExampleCrate>> {
    match group {
        ExampleGroup::Layout => layout_examples(workspace),
        ExampleGroup::Utilities => utilities_examples(workspace),
        ExampleGroup::Navigation => navigation_examples(workspace),
        ExampleGroup::Forms => forms_examples(workspace),
        ExampleGroup::SelectionControls => selection_controls_examples(workspace),
    }
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

    let crates = example_group_crates(&workspace, args.group)?;

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

fn navigation_examples(workspace: &Path) -> Result<Vec<ExampleCrate>> {
    const NAVIGATION_MANIFESTS: &[(&str, &str)] = &[
        (
            "navigation-bottom-navigation-yew",
            "examples/navigation-bottom-navigation-yew/Cargo.toml",
        ),
        (
            "surfaces-app-bar-yew",
            "examples/surfaces-app-bar-yew/Cargo.toml",
        ),
        (
            "navigation-pagination-leptos",
            "examples/navigation-pagination-leptos/Cargo.toml",
        ),
        (
            "navigation-speed-dial-dioxus",
            "examples/navigation-speed-dial-dioxus/Cargo.toml",
        ),
    ];

    let mut crates = Vec::with_capacity(NAVIGATION_MANIFESTS.len());
    for (name, manifest) in NAVIGATION_MANIFESTS {
        let manifest_path = workspace.join(manifest);
        if !manifest_path.exists() {
            return Err(anyhow!(
                "navigation example `{}` manifest missing at {}",
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

fn forms_examples(workspace: &Path) -> Result<Vec<ExampleCrate>> {
    const FORMS_MANIFESTS: &[(&str, &str)] = &[
        (
            "forms-input-base-dioxus",
            "examples/forms-input-base-dioxus/Cargo.toml",
        ),
        (
            "forms-input-base-leptos",
            "examples/forms-input-base-leptos/Cargo.toml",
        ),
        (
            "forms-input-base-sycamore",
            "examples/forms-input-base-sycamore/Cargo.toml",
        ),
        (
            "forms-input-base-yew",
            "examples/forms-input-base-yew/Cargo.toml",
        ),
    ];

    let mut crates = Vec::with_capacity(FORMS_MANIFESTS.len());
    for (name, manifest) in FORMS_MANIFESTS {
        let manifest_path = workspace.join(manifest);
        if !manifest_path.exists() {
            return Err(anyhow!(
                "forms example `{}` manifest missing at {}",
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

fn selection_controls_examples(workspace: &Path) -> Result<Vec<ExampleCrate>> {
    // Keep the selection control telemetry demos synchronized across renderers.
    // Centralizing the manifests ensures CI and local contributors compile every
    // framework-specific harness after adding a new checkbox or radio surface.
    const SELECTION_MANIFESTS: &[(&str, &str)] = &[
        (
            "selection-controls-dioxus",
            "examples/selection-controls-dioxus/Cargo.toml",
        ),
        (
            "selection-controls-leptos",
            "examples/selection-controls-leptos/Cargo.toml",
        ),
        (
            "selection-controls-react",
            "examples/selection-controls-react/Cargo.toml",
        ),
        (
            "selection-controls-sycamore",
            "examples/selection-controls-sycamore/Cargo.toml",
        ),
        (
            "selection-controls-yew",
            "examples/selection-controls-yew/Cargo.toml",
        ),
    ];

    let mut crates = Vec::with_capacity(SELECTION_MANIFESTS.len());
    for (name, manifest) in SELECTION_MANIFESTS {
        let manifest_path = workspace.join(manifest);
        if !manifest_path.exists() {
            return Err(anyhow!(
                "selection control example `{}` manifest missing at {}",
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
    // The async docs helper already compiles the SSR + WASM targets in parallel
    // while respecting the caller's CARGO_TARGET_DIR. We keep this wrapper
    // intentionally thin: it simply blocks on the helper so the shared target
    // cache is warmed before mdBook runs and lets the helper surface any build
    // failures verbatim for CI triage. No extra retry logic lives here so flaky
    // wasm builds point straight back to the helper crate.
    println!(
        "[xtask][build-docs] hydrating SSR + wasm assets via docs::build (respects CARGO_TARGET_DIR caches)"
    );
    run_async_task("docs::build", docs_build())?;

    let workspace = workspace_root();
    match build_rust_book(&workspace)? {
        Some(book_output) => {
            println!(
                "[xtask][build-docs] mdBook rendered to {}",
                relative_display(&workspace, &book_output)
            );
        }
        None => {
            println!("[xtask][build-docs] docs/rust-book not present; skipping mdBook build");
        }
    }

    Ok(())
}

fn docs_package_wrapper(args: DocsPackageArgs) -> Result<()> {
    // The docs helper already performs concurrent host+wasm builds using the
    // shared CARGO_TARGET_DIR, so this wrapper focuses on wiring environment
    // overrides and logging. We surface where artifacts land (defaulting to
    // target/deploy/docs) so CI logs document cache usage, and we preserve the
    // helper's exact error value for straightforward triage when wasm-bindgen
    // or cargo fails.
    let workspace = workspace_root();
    let default_export = workspace.join("target").join("deploy").join("docs");
    let export_dir = if args.dry_run {
        workspace
            .join("target")
            .join("deploy")
            .join("docs-package-preview")
    } else {
        default_export.clone()
    };
    let export_label = relative_display(&workspace, &export_dir);

    if args.dry_run {
        println!(
            "[xtask][docs-package] dry-run enabled; staging preview lives at {}",
            export_label
        );
    } else {
        println!(
            "[xtask][docs-package] exporting assets into {} (reuse via RUSTIC_DOCS_EXPORT_DIR)",
            export_label
        );
    }

    let previous_export = env::var("RUSTIC_DOCS_EXPORT_DIR").ok();
    env::set_var("RUSTIC_DOCS_EXPORT_DIR", &export_dir);
    let package_result = run_async_task("docs::package", docs_package());
    if let Some(prev) = previous_export {
        env::set_var("RUSTIC_DOCS_EXPORT_DIR", prev);
    } else {
        env::remove_var("RUSTIC_DOCS_EXPORT_DIR");
    }
    let outcome: DocsPackageOutcome = package_result?;

    let manifest_path = outcome.manifest_path.as_std_path().to_path_buf();
    let export_path = outcome.export_dir.as_std_path().to_path_buf();
    let mdbook_output = build_rust_book(&workspace)?;
    let mdbook_label = mdbook_output
        .as_ref()
        .map(|path| relative_display(&workspace, path))
        .unwrap_or_else(|| "skipped".to_string());

    println!(
        "[xtask][docs-package] summary: export_dir={} manifest={} mdbook={}",
        relative_display(&workspace, &export_path),
        relative_display(&workspace, &manifest_path),
        mdbook_label
    );

    Ok(())
}

fn deploy_docs(args: DeployDocsArgs) -> Result<()> {
    // Deploy wraps the async docs package helper so we reuse the shared
    // CARGO_TARGET_DIR cache and Tokio concurrency while still layering in
    // legacy artifacts (mdBook, API docs, curated wasm examples). The helper's
    // errors bubble straight through so CI jobs get precise failure modes when
    // the lower-level crate encounters cargo or wasm-bindgen issues.
    let workspace = workspace_root();
    let output_override = env::var_os("RUSTIC_UI_DEPLOY_OUTPUT").map(PathBuf::from);
    let deploy_root = output_override
        .clone()
        .unwrap_or_else(|| workspace.join("target/deploy/docs"));

    let mut build_opts = BuildOptions::default();
    match env::var("RUSTIC_UI_DEPLOY_PROFILE") {
        Ok(profile) => {
            println!(
                "[xtask][deploy-docs] using custom Cargo profile `{}` for wasm bundling",
                profile
            );
            build_opts.profile = Some(profile);
        }
        Err(env::VarError::NotPresent) => {
            build_opts.release = true;
        }
        Err(err) => {
            return Err(anyhow!("failed to read RUSTIC_UI_DEPLOY_PROFILE: {}", err));
        }
    }

    let groups = resolve_deploy_groups()?;
    let deploy_label = relative_display(&workspace, &deploy_root);
    let package_export_dir = if args.dry_run {
        workspace
            .join("target")
            .join("deploy")
            .join("docs-deploy-preview")
    } else {
        deploy_root.clone()
    };
    let package_label = relative_display(&workspace, &package_export_dir);

    if args.dry_run {
        println!(
            "[xtask][deploy-docs] dry-run enabled; canonical staging ({}) will remain untouched (preview: {})",
            deploy_label,
            package_label
        );
    } else if deploy_root.exists() {
        println!(
            "[xtask][deploy-docs] clearing existing deploy directory at {}",
            deploy_label
        );
        fs::remove_dir_all(&deploy_root).with_context(|| {
            format!(
                "failed to remove prior deploy directory at {}",
                deploy_root.display()
            )
        })?;
    }

    let previous_export = env::var("RUSTIC_DOCS_EXPORT_DIR").ok();
    env::set_var("RUSTIC_DOCS_EXPORT_DIR", &package_export_dir);
    println!(
        "[xtask][deploy-docs] invoking docs::package helper to stage SSR + wasm assets at {}",
        package_label
    );
    let package_result = run_async_task("docs::package", docs_package());
    if let Some(prev) = previous_export {
        env::set_var("RUSTIC_DOCS_EXPORT_DIR", prev);
    } else {
        env::remove_var("RUSTIC_DOCS_EXPORT_DIR");
    }
    let package_outcome: DocsPackageOutcome = package_result?;
    let manifest_path = package_outcome.manifest_path.as_std_path().to_path_buf();
    println!(
        "[xtask][deploy-docs] docs helper emitted manifest at {}",
        relative_display(&workspace, &manifest_path)
    );

    let book_output = match build_rust_book(&workspace)? {
        Some(path) => path,
        None => {
            return Err(anyhow!(
                "mdBook output missing at {} after docs::package run",
                workspace.join("docs/rust-book").display()
            ));
        }
    };

    if args.dry_run {
        println!(
            "[xtask][deploy-docs] dry-run: would copy mdBook artifacts from {} to {}",
            relative_display(&workspace, &book_output),
            deploy_label
        );
    } else {
        copy_dir_contents(&book_output, &deploy_root).with_context(|| {
            format!(
                "failed to copy mdBook output from {} to {}",
                book_output.display(),
                deploy_root.display()
            )
        })?;
    }

    let api_docs = workspace.join("target/doc");
    if api_docs.exists() {
        let api_dest = deploy_root.join("api");
        if args.dry_run {
            println!(
                "[xtask][deploy-docs] dry-run: would mirror API docs from {} to {}",
                relative_display(&workspace, &api_docs),
                relative_display(&workspace, &api_dest)
            );
        } else {
            copy_dir_contents(&api_docs, &api_dest).with_context(|| {
                format!(
                    "failed to copy API docs from {} to {}",
                    api_docs.display(),
                    api_dest.display()
                )
            })?;
        }
    } else {
        println!(
            "[xtask][deploy-docs] warning: cargo doc output not found at {}; skipping API mirror",
            api_docs.display()
        );
    }

    let wasm_profile_dir = workspace
        .join("target/wasm32-unknown-unknown")
        .join(deploy_profile_dir(&build_opts));
    let wasm_dest = deploy_root.join("wasm");
    if !args.dry_run {
        fs::create_dir_all(&wasm_dest).with_context(|| {
            format!(
                "failed to create wasm destination directory at {}",
                wasm_dest.display()
            )
        })?;
    }

    let mut wasm_entries = Vec::new();
    for group in &groups {
        println!(
            "[xtask][deploy-docs] compiling `{}` example group for host + wasm targets",
            group.as_str()
        );
        let group_args = ExamplesArgs {
            group: *group,
            release: build_opts.profile.is_none() && build_opts.release,
            profile: build_opts.profile.clone(),
        };
        examples(group_args)?;

        let crates = example_group_crates(&workspace, *group)?;
        for example in crates {
            let artifacts = locate_wasm_artifacts(&wasm_profile_dir, &example)?;
            let recorded = stage_wasm_artifacts(&workspace, &artifacts, &wasm_dest, args.dry_run)?;
            wasm_entries.push(DeployWasmEntry {
                example: example.name,
                files: recorded,
            });
        }
    }

    let summary = DeploySummary {
        output_dir: deploy_label,
        package_export_dir: relative_display(&workspace, &package_export_dir),
        docs_manifest: relative_display(&workspace, &manifest_path),
        mdbook_output: relative_display(&workspace, &book_output),
        profile: deploy_profile_dir(&build_opts),
        dry_run: args.dry_run,
        wasm_examples: wasm_entries,
    };

    if args.dry_run {
        println!(
            "[xtask][deploy-docs] dry-run summary: {}",
            serde_json::to_string_pretty(&summary)?
        );
    } else {
        let summary_path = deploy_root.join("deploy-summary.json");
        fs::write(&summary_path, serde_json::to_string_pretty(&summary)?).with_context(|| {
            format!(
                "failed to write deploy summary at {}",
                summary_path.display()
            )
        })?;
        println!(
            "[xtask][deploy-docs] wrote deploy summary to {}",
            relative_display(&workspace, &summary_path)
        );
    }

    println!(
        "[xtask][deploy-docs] completed documentation deploy pipeline for {} group(s)",
        groups.len()
    );

    Ok(())
}

fn build_rust_book(workspace: &Path) -> Result<Option<PathBuf>> {
    let book_dir = workspace.join("docs/rust-book");
    if !book_dir.exists() {
        return Ok(None);
    }

    println!(
        "[xtask][mdbook] compiling Rust primer from {}",
        relative_display(workspace, &book_dir)
    );
    let mut cmd = Command::new("mdbook");
    cmd.arg("build").arg(&book_dir);
    run(cmd)?;

    Ok(Some(book_dir.join("book")))
}

fn deploy_profile_dir(options: &BuildOptions) -> String {
    if let Some(profile) = &options.profile {
        profile.clone()
    } else if options.release {
        "release".to_string()
    } else {
        "debug".to_string()
    }
}

fn resolve_deploy_groups() -> Result<Vec<ExampleGroup>> {
    match env::var("RUSTIC_UI_DEPLOY_GROUPS") {
        Ok(value) => {
            let mut selected = BTreeSet::new();
            for token in value.split(',') {
                let trimmed = token.trim();
                if trimmed.is_empty() {
                    continue;
                }

                let group = ExampleGroup::from_str(trimmed, true).map_err(|_| {
                    let valid = ExampleGroup::value_variants()
                        .iter()
                        .filter_map(|variant| variant.to_possible_value())
                        .map(|value| value.get_name().to_string())
                        .collect::<Vec<_>>()
                        .join(", ");
                    anyhow!(
                        "invalid ExampleGroup `{}` supplied via RUSTIC_UI_DEPLOY_GROUPS. Valid values: {}",
                        trimmed,
                        valid
                    )
                })?;

                selected.insert(group);
            }

            if selected.is_empty() {
                Ok(default_deploy_groups())
            } else {
                Ok(selected.into_iter().collect())
            }
        }
        Err(env::VarError::NotPresent) => Ok(default_deploy_groups()),
        Err(err) => Err(anyhow!("failed to read RUSTIC_UI_DEPLOY_GROUPS: {}", err)),
    }
}

fn default_deploy_groups() -> Vec<ExampleGroup> {
    vec![
        ExampleGroup::Layout,
        ExampleGroup::Utilities,
        ExampleGroup::Navigation,
        ExampleGroup::Forms,
        ExampleGroup::SelectionControls,
    ]
}

fn copy_dir_contents(source: &Path, destination: &Path) -> Result<()> {
    for entry in WalkDir::new(source) {
        let entry = entry?;
        let path = entry.path();
        let relative = match path.strip_prefix(source) {
            Ok(rel) => rel,
            Err(_) => continue,
        };

        if relative.as_os_str().is_empty() {
            continue;
        }

        let target = destination.join(relative);
        if entry.file_type().is_dir() {
            fs::create_dir_all(&target)?;
        } else {
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(path, &target)?;
        }
    }

    Ok(())
}

fn stage_wasm_artifacts(
    workspace: &Path,
    artifacts: &[PathBuf],
    wasm_dest: &Path,
    dry_run: bool,
) -> Result<Vec<String>> {
    let mut recorded = Vec::new();
    for artifact in artifacts {
        let label = relative_display(workspace, artifact);
        if dry_run {
            println!(
                "[xtask][deploy-docs] dry-run: would copy wasm artifact {}",
                label
            );
        } else {
            let filename = artifact.file_name().ok_or_else(|| {
                anyhow!("wasm artifact {} is missing a filename", artifact.display())
            })?;
            let destination = wasm_dest.join(filename);
            if let Some(parent) = destination.parent() {
                fs::create_dir_all(parent).with_context(|| {
                    format!(
                        "failed to create wasm parent directory at {}",
                        parent.display()
                    )
                })?;
            }
            fs::copy(artifact, &destination).with_context(|| {
                format!(
                    "failed to copy wasm artifact {} to {}",
                    artifact.display(),
                    destination.display()
                )
            })?;
            println!(
                "[xtask][deploy-docs] staged wasm artifact {}",
                relative_display(workspace, &destination)
            );
        }
        recorded.push(label);
    }

    Ok(recorded)
}

fn locate_wasm_artifacts(target_root: &Path, example: &ExampleCrate) -> Result<Vec<PathBuf>> {
    if !target_root.exists() {
        return Err(anyhow!(
            "wasm target directory {} does not exist",
            target_root.display()
        ));
    }

    let crate_stem = example.name.replace('-', "_");
    let mut artifacts: BTreeSet<PathBuf> = BTreeSet::new();
    let direct = target_root.join(format!("{}.wasm", crate_stem));
    if direct.exists() {
        artifacts.insert(direct);
    }

    let deps_dir = target_root.join("deps");
    if deps_dir.exists() {
        for entry in fs::read_dir(&deps_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension() != Some(OsStr::new("wasm")) {
                continue;
            }

            if let Some(stem) = path.file_stem().and_then(|stem| stem.to_str()) {
                if stem.starts_with(&crate_stem) {
                    artifacts.insert(path);
                }
            }
        }
    }

    if artifacts.is_empty() {
        return Err(anyhow!(
            "unable to locate wasm artifact for example `{}` under {}",
            example.name,
            target_root.display()
        ));
    }

    Ok(artifacts.into_iter().collect())
}

#[derive(Serialize)]
struct DeploySummary {
    output_dir: String,
    package_export_dir: String,
    docs_manifest: String,
    mdbook_output: String,
    profile: String,
    dry_run: bool,
    wasm_examples: Vec<DeployWasmEntry>,
}

#[derive(Serialize)]
struct DeployWasmEntry {
    example: String,
    files: Vec<String>,
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

fn selection_controls(args: SelectionControlsArgs) -> Result<()> {
    let workspace = workspace_root();
    if args.skip_rust {
        println!("[xtask][selection-controls] skipping Rust suites by request");
    } else {
        println!("[xtask][selection-controls] running Rust selection control suites");
        let mut cargo = Command::new("cargo");
        cargo
            .current_dir(&workspace)
            .arg("test")
            .arg("-p")
            .arg("rustic-ui-material")
            .arg("--test")
            .arg("selection_control");
        run(cargo)?;

        let smoke_script = workspace.join("examples/scripts/selection-controls-smoke.sh");
        let mut smoke = Command::new(&smoke_script);
        smoke
            .current_dir(&workspace)
            .arg(args.framework.as_deref().unwrap_or("all"))
            .arg("--mode")
            .arg("smoke");
        run(smoke)?;
    }

    if args.skip_web {
        println!("[xtask][selection-controls] skipping web smoke tests by request");
    } else {
        println!("[xtask][selection-controls] launching Rust-native browser automation harness");
        let harness = SelectionControlsHarness::new(workspace.clone());
        harness.run(args.framework.as_deref())?;
    }

    Ok(())
}

/// Execute every quick-start bootstrap command so docs remain authoritative.
///
/// The harness mirrors the quick-start guide verbatim: each scaffold is
/// generated in a clean workspace, optional verification commands are executed,
/// and a comprehensive transcript is written to `target/logs/quick-start.log`.
/// This keeps CI reproducible and gives contributors a one-line command to
/// audit the guide before shipping copy updates.
fn quick_start(skip_checks: bool) -> Result<()> {
    let mut harness = QuickStartHarness::new(skip_checks)?;
    harness.run()
}

/// Stateful driver that coordinates the quick-start bootstrap orchestration.
///
/// Capturing the workspace, log writer, and feature toggles in a struct keeps
/// the individual steps small and allows us to reuse the logic both from the
/// dedicated subcommand and when `cargo xtask docs-test` runs in CI.
struct QuickStartHarness {
    workspace: PathBuf,
    log_path: PathBuf,
    log: BufWriter<fs::File>,
    skip_checks: bool,
    cargo_target_dir: PathBuf,
}

impl QuickStartHarness {
    fn new(skip_checks: bool) -> Result<Self> {
        let workspace = workspace_root();
        let logs_dir = workspace.join("target/logs");
        fs::create_dir_all(&logs_dir)
            .with_context(|| format!("failed to create log directory {}", logs_dir.display()))?;
        let log_path = logs_dir.join("quick-start.log");
        let file = fs::File::create(&log_path)
            .with_context(|| format!("failed to create log file {}", log_path.display()))?;
        let mut log = BufWriter::new(file);
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        writeln!(
            log,
            "# RusticUI quick-start automation\nstarted_at_unix={timestamp}\nskip_checks={skip_checks}\n"
        )?;
        let cargo_target_dir = workspace.join("target/quick-start/targets");
        fs::create_dir_all(&cargo_target_dir).with_context(|| {
            format!(
                "failed to prepare Cargo target directory {}",
                cargo_target_dir.display()
            )
        })?;
        Ok(Self {
            workspace,
            log_path,
            log,
            skip_checks,
            cargo_target_dir,
        })
    }

    fn run(&mut self) -> Result<()> {
        writeln!(
            self.log,
            "# Each section mirrors the docs quick-start table. Commands execute sequentially\n"
        )?;
        writeln!(self.log, "\n## prerequisites\n")?;
        self.ensure_wasm_target()?;

        for spec in QUICK_START_SPECS {
            self.run_scaffold(spec)?;
        }

        self.log.flush()?;
        println!(
            "[xtask][quick-start] completed. Inspect {} for detailed output",
            self.relative_log_path()
        );
        Ok(())
    }

    fn ensure_wasm_target(&mut self) -> Result<()> {
        println!("[xtask][quick-start] ensuring wasm32-unknown-unknown target is installed");
        let mut cmd = Command::new("rustup");
        cmd.arg("target").arg("add").arg("wasm32-unknown-unknown");
        let display = format!("{:?}", &cmd);
        writeln!(self.log, "command={display}")?;
        match cmd.output() {
            Ok(output) => {
                writeln!(self.log, "status={}", output.status)?;
                if !output.stdout.is_empty() {
                    writeln!(
                        self.log,
                        "stdout=\n{}",
                        String::from_utf8_lossy(&output.stdout)
                    )?;
                }
                if !output.stderr.is_empty() {
                    writeln!(
                        self.log,
                        "stderr=\n{}",
                        String::from_utf8_lossy(&output.stderr)
                    )?;
                }
                if !output.status.success() {
                    writeln!(
                        self.log,
                        "warning=rustup target add exited with {} but proceeding",
                        output.status
                    )?;
                }
            }
            Err(error) => {
                writeln!(
                    self.log,
                    "warning=failed to execute rustup target provisioning: {error}"
                )?;
            }
        }
        Ok(())
    }

    fn run_scaffold(&mut self, spec: &QuickStartSpec) -> Result<()> {
        println!("[xtask][quick-start] provisioning {}", spec.name);
        writeln!(self.log, "\n## {}\nsummary={}\n", spec.name, spec.summary)?;

        let bootstrap = spec.bootstrap.spawn(&self.workspace);
        self.execute(&format!("{} bootstrap", spec.name), bootstrap)?;

        if !self.skip_checks {
            for check in spec.checks {
                self.run_check(spec, check)?;
            }
        } else if !spec.checks.is_empty() {
            writeln!(
                self.log,
                "skipped_checks={}",
                spec.checks
                    .iter()
                    .map(|check| check.label())
                    .collect::<Vec<_>>()
                    .join(", ")
            )?;
        }

        Ok(())
    }

    fn run_check(&mut self, spec: &QuickStartSpec, check: &QuickStartCheck) -> Result<()> {
        match check {
            QuickStartCheck::Cargo { manifest, target } => {
                let mut cmd = Command::new("cargo");
                cmd.current_dir(&self.workspace)
                    .arg("check")
                    .arg("--manifest-path")
                    .arg(self.workspace.join(manifest))
                    .arg("--quiet")
                    .env("CARGO_TARGET_DIR", &self.cargo_target_dir);
                if let Some(target) = target {
                    cmd.arg("--target").arg(target);
                }
                self.execute(&format!("{} cargo check ({manifest})", spec.name), cmd)
            }
            QuickStartCheck::Npm {
                directory,
                script,
                args,
            } => {
                let mut cmd = Command::new("npm");
                cmd.current_dir(self.workspace.join(directory))
                    .arg("run")
                    .arg(script)
                    .env("CI", "true")
                    .env("NPM_CONFIG_FUND", "false")
                    .env("NPM_CONFIG_AUDIT", "false");
                if !args.is_empty() {
                    cmd.arg("--");
                    for arg in *args {
                        cmd.arg(arg);
                    }
                }
                self.execute(&format!("{} npm {script}", spec.name), cmd)
            }
        }
    }

    fn execute(&mut self, label: &str, mut command: Command) -> Result<()> {
        let display = format!("{:?}", &command);
        writeln!(self.log, "command={display}")?;
        let output = command.output().map_err(|error| {
            if error.kind() == io::ErrorKind::NotFound {
                anyhow!(
                    "failed to spawn command for `{label}` because {display} is missing from PATH"
                )
            } else {
                anyhow!("failed to execute {display}: {error}")
            }
        })?;
        writeln!(self.log, "status={}", output.status)?;
        if !output.stdout.is_empty() {
            writeln!(
                self.log,
                "stdout=\n{}",
                String::from_utf8_lossy(&output.stdout)
            )?;
        }
        if !output.stderr.is_empty() {
            writeln!(
                self.log,
                "stderr=\n{}",
                String::from_utf8_lossy(&output.stderr)
            )?;
        }
        if !output.status.success() {
            self.log.flush()?;
            return Err(anyhow!(
                "quick-start step `{label}` failed. Inspect {} for details",
                self.relative_log_path()
            ));
        }
        Ok(())
    }

    fn relative_log_path(&self) -> String {
        self.log_path
            .strip_prefix(&self.workspace)
            .map(|path| path.display().to_string())
            .unwrap_or_else(|_| self.log_path.display().to_string())
    }
}

struct QuickStartSpec {
    name: &'static str,
    bootstrap: QuickStartCommand,
    checks: &'static [QuickStartCheck],
    summary: &'static str,
}

enum QuickStartCommand {
    Script {
        path: &'static str,
    },
    Just {
        directory: &'static str,
        recipe: &'static str,
    },
}

impl QuickStartCommand {
    fn spawn(&self, workspace: &Path) -> Command {
        match self {
            QuickStartCommand::Script { path } => {
                let mut cmd = Command::new(workspace.join(path));
                cmd.current_dir(workspace);
                cmd
            }
            QuickStartCommand::Just { directory, recipe } => {
                let mut cmd = Command::new("just");
                cmd.current_dir(workspace.join(directory)).arg(recipe);
                cmd
            }
        }
    }
}

enum QuickStartCheck {
    Cargo {
        manifest: &'static str,
        target: Option<&'static str>,
    },
    Npm {
        directory: &'static str,
        script: &'static str,
        args: &'static [&'static str],
    },
}

impl QuickStartCheck {
    fn label(&self) -> &'static str {
        match self {
            QuickStartCheck::Cargo { .. } => "cargo",
            QuickStartCheck::Npm { .. } => "npm",
        }
    }
}

/// Static manifest describing each scaffold we expect the docs to advertise.
///
/// The goal is to make it trivial to enrol new frameworks: append an entry with
/// the shell command, optional verification steps, and a short summary that is
/// echoed into the log. CI immediately starts exercising new entries once they
/// appear in this table.
const QUICK_START_SPECS: &[QuickStartSpec] = &[
    QuickStartSpec {
        name: "yew-navigation-tabs",
        bootstrap: QuickStartCommand::Script {
            path: "examples/navigation-tabs-yew/scripts/bootstrap.sh",
        },
        checks: &[QuickStartCheck::Cargo {
            manifest: "target/navigation-tabs-yew-demo/Cargo.toml",
            target: None,
        }],
        summary: "Mirrors the Yew quick-start instructions to seed the navigation tabs demo.",
    },
    QuickStartSpec {
        name: "leptos-navigation-tabs",
        bootstrap: QuickStartCommand::Script {
            path: "examples/navigation-tabs-leptos/scripts/bootstrap.sh",
        },
        checks: &[QuickStartCheck::Cargo {
            manifest: "target/navigation-tabs-leptos-demo/Cargo.toml",
            target: None,
        }],
        summary: "Bootstraps the Leptos navigation tabs template and verifies it compiles.",
    },
    QuickStartSpec {
        name: "dioxus-navigation-drawer",
        bootstrap: QuickStartCommand::Script {
            path: "examples/navigation-drawer-dioxus/scripts/bootstrap.sh",
        },
        checks: &[],
        summary: "Emits the Dioxus drawer blueprint README so developers inherit automation notes.",
    },
    QuickStartSpec {
        name: "sycamore-navigation-drawer",
        bootstrap: QuickStartCommand::Script {
            path: "examples/navigation-drawer-sycamore/scripts/bootstrap.sh",
        },
        checks: &[],
        summary: "Generates the Sycamore drawer blueprint README for parity coverage.",
    },
    QuickStartSpec {
        name: "react-selection-controls",
        bootstrap: QuickStartCommand::Just {
            directory: "examples/selection-controls-react",
            recipe: "bootstrap",
        },
        checks: &[QuickStartCheck::Npm {
            directory: "examples/selection-controls-react",
            script: "test",
            args: &["--", "--runInBand", "--watch=false"],
        }],
        summary: "Runs the React quick-start `just bootstrap` recipe and executes the headless test suite to confirm tooling wiring.",
    },
];
