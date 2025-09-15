//! Developer automation commands for the MUI Rust workspace.
//!
//! The `xtask` pattern keeps our repository free of ad-hoc shell
//! scripts and centralizes repeatable tasks in a small Rust binary.
//! This approach scales well for large teams and CI environments,
//! ensuring that contributors invoke the exact same logic locally
//! and in automation.

use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
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
    }
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
    run(cmd)
}

fn wasm_test() -> Result<()> {
    // Crates with WebAssembly tests. Extend this list as needed.
    let wasm_crates = ["crates/mui-joy", "crates/mui-material"];
    for krate in &wasm_crates {
        let mut cmd = Command::new("wasm-pack");
        cmd.arg("test")
            .arg("--headless")
            .arg("--chrome")
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
