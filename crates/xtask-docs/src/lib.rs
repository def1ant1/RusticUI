#![doc = r#"
Enterprise-grade automation primitives powering the RusticUI documentation
pipeline.

This crate exists so `cargo xtask` (and CI workflows) can drive the Leptos
showcase without bespoke shell scripts. The helpers exposed here intentionally
speak in terms of reusable building blocks—build, test, and package—so future
workflows (nightly smoke tests, multi-tenant doc deployments, etc.) can compose
these primitives instead of re-implementing orchestration logic.

# Web testing prerequisites

`docs_test()` shells out to `wasm-pack test --headless --chrome`, which requires
Playwright's Chromium bundle. Ensure `wasm-pack` is installed and `npx
playwright install --with-deps chromium` (or an equivalent offline mirror) has
been executed on the host running these tasks. Without the Playwright binaries
the test harness will fail before the compiled WASM bundle is even exercised.

The command also runs the docs `quick-start-gallery` Playwright spec using
`pnpm --dir docs exec -- playwright test`. Provide a running docs host (for
example via `pnpm --dir docs dev`) and set `PLAYWRIGHT_TEST_BASE_URL` to the
corresponding URL so the iframe-based Sandpack preview can hydrate before the
assertions execute.
"#]

use camino::{Utf8Path, Utf8PathBuf};
use cargo_metadata::MetadataCommand;
use hex::ToHex;
use rustic_ui_error::{ResultContextExt, RusticUiError, RusticUiResult};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::env;
use std::fmt::Write as _;
use std::process::Stdio;
use tokio::fs;
use tokio::process::Command;
use walkdir::WalkDir;

/// Build the RusticUI docs host binary and its WASM bundle.
///
/// The helper reuses `CARGO_TARGET_DIR` when provided to ensure CI caches are
/// respected and executes the host+wasm compilation in parallel via
/// `tokio::try_join!`.
///
/// # Examples
///
/// ```no_run
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// # use xtask_docs::docs_build;
/// docs_build().await?;
/// # Ok::<(), rustic_ui_error::RusticUiError>(())
/// # }).unwrap();
/// ```
pub async fn docs_build() -> RusticUiResult<()> {
    let ctx = WorkspaceContext::detect()?;
    validate_mermaid_diagrams(&ctx).await?;
    build_artifacts(&ctx, BuildProfile::Debug).await.map(|_| ())
}

/// Execute the docs smoke tests.
///
/// The workflow now covers two surfaces: the legacy wasm-pack smoke test that
/// validates the Leptos bundle and a Playwright scenario that asserts the docs
/// quick-start gallery renders the shared CTA with the correct automation
/// hooks. Failures are captured and recorded in `target/logs/docs-test.log` and
/// `target/logs/docs-playwright.log` respectively with the invoked command line
/// so CI operators can diagnose why Chrome/Playwright exited with a specific
/// status.
///
/// # Examples
///
/// ```no_run
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// # use xtask_docs::docs_test;
/// docs_test().await?;
/// # Ok::<(), rustic_ui_error::RusticUiError>(())
/// # }).unwrap();
/// ```
pub async fn docs_test() -> RusticUiResult<()> {
    let ctx = WorkspaceContext::detect()?;
    run_wasm_tests(&ctx).await?;
    run_playwright_quick_start_smoke(&ctx).await
}

/// Produce a deploy-ready documentation payload containing SSR + WASM assets.
///
/// The export directory defaults to `target/deploy/docs` but can be overridden
/// through `RUSTIC_DOCS_EXPORT_DIR` for environments that persist artifacts
/// elsewhere (e.g., Bazel or containerized CI runners).
#[derive(Clone, Debug)]
pub struct DocsPackageOutcome {
    pub export_dir: Utf8PathBuf,
    pub manifest_path: Utf8PathBuf,
}

///
/// # Examples
///
/// ```no_run
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// # use xtask_docs::docs_package;
/// let outcome = docs_package().await?;
/// println!("exported docs to {}", outcome.export_dir);
/// # Ok::<(), rustic_ui_error::RusticUiError>(())
/// # }).unwrap();
/// ```
pub async fn docs_package() -> RusticUiResult<DocsPackageOutcome> {
    let ctx = WorkspaceContext::detect()?;
    let artifacts = build_artifacts(&ctx, BuildProfile::Release).await?;
    package_release(&ctx, &artifacts).await
}

struct WorkspaceContext {
    workspace_root: Utf8PathBuf,
    rustic_docs_crate: Utf8PathBuf,
    target_dir: Utf8PathBuf,
    wasm_staging: Utf8PathBuf,
}

impl WorkspaceContext {
    fn detect() -> RusticUiResult<Self> {
        let metadata = MetadataCommand::new()
            .no_deps()
            .exec()
            .context("failed to query workspace metadata")?;
        let workspace_root =
            Utf8PathBuf::from_path_buf(metadata.workspace_root.into_std_path_buf())
                .map_err(|_| RusticUiError::message("workspace path was not valid UTF-8"))?;
        let rustic_docs_crate = workspace_root.join("crates").join("rustic-docs");
        let target_dir = env::var("CARGO_TARGET_DIR")
            .map(Utf8PathBuf::from)
            .unwrap_or_else(|_| workspace_root.join("target"));
        let wasm_staging = target_dir.join("rustic-docs-wasm");
        Ok(Self {
            workspace_root,
            rustic_docs_crate,
            target_dir,
            wasm_staging,
        })
    }
}

#[derive(Copy, Clone, Debug)]
enum BuildProfile {
    Debug,
    Release,
}

impl BuildProfile {
    fn cargo_flag(self) -> Option<&'static str> {
        match self {
            BuildProfile::Debug => None,
            BuildProfile::Release => Some("--release"),
        }
    }

    fn artifact_dir(self) -> &'static str {
        match self {
            BuildProfile::Debug => "debug",
            BuildProfile::Release => "release",
        }
    }
}

#[derive(Clone, Debug)]
struct DocsBuildArtifacts {
    wasm_js: Utf8PathBuf,
    wasm_bg: Utf8PathBuf,
    server_binary: Utf8PathBuf,
}

#[derive(Clone, Debug)]
struct MermaidFailure {
    path: Utf8PathBuf,
    line: usize,
    message: String,
}

async fn build_artifacts(
    ctx: &WorkspaceContext,
    profile: BuildProfile,
) -> RusticUiResult<DocsBuildArtifacts> {
    let host_build = build_host(ctx, profile);
    let wasm_build = build_wasm(ctx, profile);

    // Building the host binary and wasm bundle concurrently keeps the overall
    // wall-clock time low while still letting both targets benefit from the
    // shared Cargo target directory cache.
    tokio::try_join!(host_build, wasm_build)?;

    let wasm_artifacts = run_wasm_bindgen(ctx, profile).await?;
    let server_binary = ctx
        .target_dir
        .join(profile.artifact_dir())
        .join(format!("rustic-docs-server{}", env::consts::EXE_SUFFIX));

    Ok(DocsBuildArtifacts {
        wasm_js: wasm_artifacts.js,
        wasm_bg: wasm_artifacts.wasm,
        server_binary,
    })
}

async fn validate_mermaid_diagrams(ctx: &WorkspaceContext) -> RusticUiResult<()> {
    let docs_root = ctx.workspace_root.join("docs");
    let mut failures = Vec::new();

    for entry in WalkDir::new(&docs_root) {
        let entry = entry.context("failed to walk docs directory")?;
        if !entry.file_type().is_file() {
            continue;
        }
        if entry.path().extension().and_then(|ext| ext.to_str()) != Some("md") {
            continue;
        }

        let path = Utf8PathBuf::from_path_buf(entry.into_path()).map_err(|_| {
            RusticUiError::message("encountered non UTF-8 path while scanning docs")
        })?;
        let contents = fs::read_to_string(&path)
            .await
            .with_context(|| format!("failed to read {path}"))?;

        let mut lines = contents.lines().enumerate();
        while let Some((start_idx, line)) = lines.next() {
            if !line.trim_start().starts_with("```mermaid") {
                continue;
            }

            let mut body = Vec::new();
            let mut closed = false;
            while let Some((_, candidate)) = lines.next() {
                if candidate.trim_start().starts_with("```") {
                    closed = true;
                    break;
                }
                body.push(candidate);
            }

            if !closed {
                failures.push(MermaidFailure {
                    path: path.clone(),
                    line: start_idx + 1,
                    message: "Mermaid fence was not terminated with ```".to_owned(),
                });
                break;
            }

            let code = body.join("\n");
            let trimmed = code.trim();
            if trimmed.is_empty() {
                failures.push(MermaidFailure {
                    path: path.clone(),
                    line: start_idx + 2,
                    message: "Mermaid diagram body was empty".to_owned(),
                });
                continue;
            }

            if let Err(message) = lint_mermaid_block(trimmed) {
                failures.push(MermaidFailure {
                    path: path.clone(),
                    line: start_idx + 2,
                    message,
                });
            }
        }
    }

    if failures.is_empty() {
        return Ok(());
    }

    let logs_dir = ctx.target_dir.join("logs");
    fs::create_dir_all(&logs_dir)
        .await
        .context("failed to create target/logs for mermaid lint output")?;
    let log_path = logs_dir.join("docs-mermaid-lint.log");
    let mut log = String::new();
    writeln!(&mut log, "detected {} Mermaid issues", failures.len())?;
    for failure in &failures {
        writeln!(
            &mut log,
            "{}:{} - {}",
            failure.path, failure.line, failure.message
        )?;
    }
    fs::write(&log_path, log)
        .await
        .with_context(|| format!("failed to persist mermaid lint output to {log_path}"))?;

    Err(RusticUiError::message(format!(
        "Mermaid diagram validation failed. Inspect {:?} for the detailed log.",
        log_path
    )))
}

async fn build_host(ctx: &WorkspaceContext, profile: BuildProfile) -> RusticUiResult<()> {
    let mut display = CommandDisplay::new("cargo");
    display.push("build");
    display.push("-p");
    display.push("rustic-docs");
    display.push("--bin");
    display.push("rustic-docs-server");
    display.push("--features");
    display.push("ssr");
    let mut cmd = Command::new("cargo");
    cmd.current_dir(&ctx.workspace_root)
        .arg("build")
        .arg("-p")
        .arg("rustic-docs")
        .arg("--bin")
        .arg("rustic-docs-server")
        .arg("--features")
        .arg("ssr");
    if let Some(flag) = profile.cargo_flag() {
        display.push(flag);
        cmd.arg(flag);
    }
    cmd.env("CARGO_TARGET_DIR", &ctx.target_dir);
    let status = cmd
        .status()
        .await
        .with_context(|| format!("failed to execute {}", display.render()))?;
    if !status.success() {
        return Err(RusticUiError::message(format!(
            "cargo build for rustic-docs-server failed with status {status}"
        )));
    }
    Ok(())
}

async fn build_wasm(ctx: &WorkspaceContext, profile: BuildProfile) -> RusticUiResult<()> {
    let mut display = CommandDisplay::new("cargo");
    display.push("build");
    display.push("-p");
    display.push("rustic-docs");
    display.push("--bin");
    display.push("rustic-docs");
    display.push("--target");
    display.push("wasm32-unknown-unknown");
    display.push("--no-default-features");
    display.push("--features");
    display.push("hydrate");
    let mut cmd = Command::new("cargo");
    cmd.current_dir(&ctx.workspace_root)
        .arg("build")
        .arg("-p")
        .arg("rustic-docs")
        .arg("--bin")
        .arg("rustic-docs")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .arg("--no-default-features")
        .arg("--features")
        .arg("hydrate");
    if let Some(flag) = profile.cargo_flag() {
        display.push(flag);
        cmd.arg(flag);
    }
    cmd.env("CARGO_TARGET_DIR", &ctx.target_dir);
    let status = cmd
        .status()
        .await
        .with_context(|| format!("failed to execute {}", display.render()))?;
    if !status.success() {
        return Err(RusticUiError::message(
            "cargo build for rustic-docs wasm bundle failed",
        ));
    }
    Ok(())
}

struct WasmBindgenArtifacts {
    js: Utf8PathBuf,
    wasm: Utf8PathBuf,
}

async fn run_wasm_bindgen(
    ctx: &WorkspaceContext,
    profile: BuildProfile,
) -> RusticUiResult<WasmBindgenArtifacts> {
    let input_wasm = ctx
        .target_dir
        .join("wasm32-unknown-unknown")
        .join(profile.artifact_dir())
        .join("rustic_docs.wasm");
    let output_dir = ctx.wasm_staging.join(profile.artifact_dir());
    fs::create_dir_all(&output_dir)
        .await
        .with_context(|| format!("failed to create wasm staging dir {output_dir}"))?;

    let mut display = CommandDisplay::new("wasm-bindgen");
    display.push(input_wasm.as_str());
    display.push("--out-dir");
    display.push(output_dir.as_str());
    display.push("--target");
    display.push("web");
    display.push("--no-typescript");
    display.push("--omit-default-module-path");
    let mut cmd = Command::new("wasm-bindgen");
    cmd.arg(&input_wasm)
        .arg("--out-dir")
        .arg(&output_dir)
        .arg("--target")
        .arg("web")
        .arg("--no-typescript")
        .arg("--omit-default-module-path")
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    let status = cmd
        .status()
        .await
        .with_context(|| format!("failed to execute {}", display.render()))?;
    if !status.success() {
        return Err(RusticUiError::message(
            "wasm-bindgen returned non-zero exit status",
        ));
    }

    let js = output_dir.join("rustic_docs.js");
    let wasm = output_dir.join("rustic_docs_bg.wasm");
    Ok(WasmBindgenArtifacts { js, wasm })
}

async fn run_wasm_tests(ctx: &WorkspaceContext) -> RusticUiResult<()> {
    let mut display = CommandDisplay::new("wasm-pack");
    display.push("test");
    display.push("--headless");
    display.push("--chrome");
    display.push("--release");
    display.push("--");
    display.push("--features");
    display.push("hydrate");
    let mut cmd = Command::new("wasm-pack");
    cmd.current_dir(&ctx.rustic_docs_crate)
        .arg("test")
        .arg("--headless")
        .arg("--chrome")
        .arg("--release")
        .arg("--")
        .arg("--features")
        .arg("hydrate")
        .env("CARGO_TARGET_DIR", &ctx.target_dir);

    let output = cmd
        .output()
        .await
        .with_context(|| format!("failed to execute {}", display.render()))?;
    if output.status.success() {
        return Ok(());
    }

    let logs_dir = ctx.target_dir.join("logs");
    fs::create_dir_all(&logs_dir)
        .await
        .context("failed to create target/logs for wasm-pack output")?;
    let log_path = logs_dir.join("docs-test.log");
    let mut log = String::new();
    writeln!(&mut log, "command: {}", display.render())?;
    writeln!(&mut log, "status: {}", output.status)?;
    writeln!(
        &mut log,
        "\nstdout:\n{}",
        String::from_utf8_lossy(&output.stdout)
    )?;
    writeln!(
        &mut log,
        "\nstderr:\n{}",
        String::from_utf8_lossy(&output.stderr)
    )?;
    fs::write(&log_path, log)
        .await
        .with_context(|| format!("failed to persist wasm-pack output to {log_path}"))?;

    Err(RusticUiError::message(format!(
        "wasm-pack smoke tests failed. Inspect {:?} for the detailed log.",
        log_path
    )))
}

async fn run_playwright_quick_start_smoke(ctx: &WorkspaceContext) -> RusticUiResult<()> {
    let mut display = CommandDisplay::new("pnpm");
    display.push("--dir");
    display.push("docs");
    display.push("exec");
    display.push("--");
    display.push("playwright");
    display.push("test");
    display.push("--config");
    display.push("test/e2e-website/playwright.config.ts");
    display.push("test/e2e-website/quick-start-gallery.spec.ts");

    let mut cmd = Command::new("pnpm");
    cmd.current_dir(&ctx.workspace_root)
        .arg("--dir")
        .arg("docs")
        .arg("exec")
        .arg("--")
        .arg("playwright")
        .arg("test")
        .arg("--config")
        .arg("test/e2e-website/playwright.config.ts")
        .arg("test/e2e-website/quick-start-gallery.spec.ts")
        .env("CARGO_TARGET_DIR", &ctx.target_dir);

    let output = cmd
        .output()
        .await
        .with_context(|| format!("failed to execute {}", display.render()))?;
    if output.status.success() {
        return Ok(());
    }

    let logs_dir = ctx.target_dir.join("logs");
    fs::create_dir_all(&logs_dir)
        .await
        .context("failed to create target/logs for Playwright output")?;
    let log_path = logs_dir.join("docs-playwright.log");
    let mut log = String::new();
    writeln!(&mut log, "command: {}", display.render())?;
    writeln!(&mut log, "status: {}", output.status)?;
    writeln!(
        &mut log,
        "\nstdout:\n{}",
        String::from_utf8_lossy(&output.stdout)
    )?;
    writeln!(
        &mut log,
        "\nstderr:\n{}",
        String::from_utf8_lossy(&output.stderr)
    )?;
    fs::write(&log_path, log)
        .await
        .with_context(|| format!("failed to persist Playwright output to {log_path}"))?;

    Err(RusticUiError::message(format!(
        "Playwright docs smoke tests failed. Inspect {:?} for the detailed log.",
        log_path
    )))
}

async fn package_release(
    ctx: &WorkspaceContext,
    artifacts: &DocsBuildArtifacts,
) -> RusticUiResult<DocsPackageOutcome> {
    let export_dir = env::var("RUSTIC_DOCS_EXPORT_DIR")
        .map(Utf8PathBuf::from)
        .unwrap_or_else(|_| ctx.target_dir.join("deploy").join("docs"));
    fs::create_dir_all(&export_dir)
        .await
        .with_context(|| format!("failed to create export directory {export_dir}"))?;

    let wasm_out = export_dir.join("wasm");
    fs::create_dir_all(&wasm_out)
        .await
        .context("failed to create wasm export directory")?;
    let wasm_js_dest = wasm_out.join(
        artifacts
            .wasm_js
            .file_name()
            .ok_or_else(|| RusticUiError::message("missing wasm-bindgen js artifact name"))?,
    );
    let wasm_bg_dest = wasm_out.join(
        artifacts
            .wasm_bg
            .file_name()
            .ok_or_else(|| RusticUiError::message("missing wasm-bindgen wasm artifact name"))?,
    );

    fs::copy(&artifacts.wasm_js, &wasm_js_dest)
        .await
        .context("failed to copy wasm-bindgen js glue into export dir")?;
    fs::copy(&artifacts.wasm_bg, &wasm_bg_dest)
        .await
        .context("failed to copy wasm-bindgen wasm into export dir")?;

    let bin_dir = export_dir.join("bin");
    fs::create_dir_all(&bin_dir)
        .await
        .context("failed to create bin export directory")?;
    let server_dest = bin_dir.join(format!("rustic-docs-server{}", env::consts::EXE_SUFFIX));
    fs::copy(&artifacts.server_binary, &server_dest)
        .await
        .context("failed to copy server binary into export dir")?;

    let snapshot_path = export_dir.join("index.html");
    let snapshot = rustic_docs::render_static_snapshot();
    fs::write(&snapshot_path, snapshot)
        .await
        .context("failed to write static snapshot")?;

    let contract_path = export_dir.join("static-manifest.json");
    let contract = rustic_docs::static_manifest_contract();
    fs::write(
        &contract_path,
        serde_json::to_vec_pretty(&contract).context("failed to serialize static manifest")?,
    )
    .await
    .context("failed to write static manifest contract")?;

    let manifest = BuildManifest::from_paths(
        ctx,
        &export_dir,
        &wasm_js_dest,
        &wasm_bg_dest,
        &server_dest,
        &snapshot_path,
        &contract_path,
    )
    .await?;
    let manifest_path = export_dir.join("docs-bundle-manifest.json");
    let manifest_bytes =
        serde_json::to_vec_pretty(&manifest).context("failed to serialize docs bundle manifest")?;
    fs::write(&manifest_path, manifest_bytes)
        .await
        .context("failed to write docs bundle manifest")?;

    Ok(DocsPackageOutcome {
        export_dir,
        manifest_path,
    })
}

#[derive(Serialize)]
struct BuildManifestEntry {
    path: String,
    sha256: String,
}

#[derive(Serialize)]
struct BuildManifest {
    wasm_js: BuildManifestEntry,
    wasm_bg: BuildManifestEntry,
    server_binary: BuildManifestEntry,
    static_snapshot: BuildManifestEntry,
    static_manifest: BuildManifestEntry,
}

impl BuildManifest {
    async fn from_paths(
        ctx: &WorkspaceContext,
        export_dir: &Utf8Path,
        wasm_js: &Utf8Path,
        wasm_bg: &Utf8Path,
        server: &Utf8Path,
        snapshot: &Utf8Path,
        contract: &Utf8Path,
    ) -> RusticUiResult<Self> {
        Ok(Self {
            wasm_js: manifest_entry(ctx, export_dir, wasm_js).await?,
            wasm_bg: manifest_entry(ctx, export_dir, wasm_bg).await?,
            server_binary: manifest_entry(ctx, export_dir, server).await?,
            static_snapshot: manifest_entry(ctx, export_dir, snapshot).await?,
            static_manifest: manifest_entry(ctx, export_dir, contract).await?,
        })
    }
}

async fn manifest_entry(
    ctx: &WorkspaceContext,
    export_dir: &Utf8Path,
    path: &Utf8Path,
) -> RusticUiResult<BuildManifestEntry> {
    let bytes = fs::read(path)
        .await
        .with_context(|| format!("failed to read {path} for hashing"))?;
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    let sha256 = hasher.finalize().encode_hex::<String>();
    let relative = path
        .strip_prefix(&ctx.workspace_root)
        .or_else(|_| path.strip_prefix(export_dir))
        .unwrap_or(path)
        .to_string();
    Ok(BuildManifestEntry {
        path: relative,
        sha256,
    })
}

fn lint_mermaid_block(code: &str) -> Result<(), String> {
    let lines: Vec<&str> = code.lines().collect();
    let first_idx = lines
        .iter()
        .position(|line| !line.trim().is_empty())
        .ok_or_else(|| "Mermaid diagram has no content".to_owned())?;
    let header = lines[first_idx].trim();

    if header.starts_with("stateDiagram") {
        return lint_state_diagram(&lines[first_idx..]);
    }
    if header.starts_with("sequenceDiagram") {
        return lint_sequence_diagram(&lines[first_idx..]);
    }
    if header.starts_with("graph ") {
        return lint_graph_diagram(&lines[first_idx..]);
    }

    Err(format!("unsupported Mermaid root '{header}'"))
}

fn lint_state_diagram(lines: &[&str]) -> Result<(), String> {
    let mut block_depth = 0usize;
    let mut saw_transition = false;

    for line in lines.iter().skip(1) {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("%%") {
            continue;
        }
        if trimmed.starts_with("direction ") {
            continue;
        }
        if trimmed.starts_with("state ") {
            if trimmed.ends_with('{') {
                block_depth += 1;
            } else if trimmed.contains('{') {
                return Err(format!("state declaration must end with '{{': {trimmed}"));
            }
            continue;
        }
        if trimmed == "}" {
            if block_depth == 0 {
                return Err("state diagram closed more blocks than opened".into());
            }
            block_depth -= 1;
            continue;
        }
        if trimmed.contains("-->") {
            saw_transition = true;
            continue;
        }

        return Err(format!("unrecognized state diagram expression '{trimmed}'"));
    }

    if block_depth != 0 {
        return Err("state diagram ended with unclosed state blocks".into());
    }
    if !saw_transition {
        return Err("state diagram did not declare any transitions".into());
    }

    Ok(())
}

fn lint_sequence_diagram(lines: &[&str]) -> Result<(), String> {
    let mut block_stack: Vec<&'static str> = Vec::new();
    let mut saw_message = false;

    for line in lines.iter().skip(1) {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("%%") {
            continue;
        }
        if trimmed.eq_ignore_ascii_case("autonumber") {
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("participant ") {
            if rest.trim().is_empty() {
                return Err("participant declaration is missing an identifier".into());
            }
            continue;
        }
        if trimmed.starts_with("opt ") {
            block_stack.push("opt");
            continue;
        }
        if trimmed == "end" {
            block_stack
                .pop()
                .ok_or_else(|| "`end` without a matching block opener".to_owned())?;
            continue;
        }

        if let Some((source, target)) = split_sequence_arrow(trimmed) {
            if source.trim().is_empty() {
                return Err("message source may not be empty".into());
            }
            if target.trim().is_empty() {
                return Err("message target may not be empty".into());
            }
            saw_message = true;
            continue;
        }

        return Err(format!(
            "unrecognized sequence diagram expression '{trimmed}'"
        ));
    }

    if !block_stack.is_empty() {
        return Err("sequence diagram ended with unterminated block".into());
    }
    if !saw_message {
        return Err("sequence diagram did not declare any messages".into());
    }

    Ok(())
}

fn split_sequence_arrow(input: &str) -> Option<(&str, &str)> {
    const ARROWS: [&str; 4] = ["-->>", "->>", "-->", "->"];
    for arrow in ARROWS {
        if let Some(idx) = input.find(arrow) {
            let (left, right) = input.split_at(idx);
            let target = &right[arrow.len()..];
            let target = target.split_once(':').map_or(target, |(head, _)| head);
            return Some((left, target));
        }
    }
    None
}

fn lint_graph_diagram(lines: &[&str]) -> Result<(), String> {
    let mut saw_edge = false;

    for line in lines.iter().skip(1) {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("%%") {
            continue;
        }
        if trimmed.starts_with("classDef ") {
            continue;
        }

        if let Some(idx) = trimmed.rfind("-->") {
            let before = &trimmed[..idx];
            let target = trimmed[idx + 3..].trim();
            if target.is_empty() {
                return Err("graph edge is missing a target node".into());
            }
            let source = before
                .split_once("--")
                .map(|(left, _)| left.trim())
                .unwrap_or_else(|| before.trim());
            if source.is_empty() {
                return Err("graph edge is missing a source node".into());
            }
            saw_edge = true;
            continue;
        }

        if let Some(idx) = trimmed.find('[') {
            if trimmed.ends_with(']') && !trimmed[..idx].trim().is_empty() {
                continue;
            }
        }
        if let Some(idx) = trimmed.find('(') {
            if trimmed.ends_with(')') && !trimmed[..idx].trim().is_empty() {
                continue;
            }
        }

        return Err(format!("unrecognized graph expression '{trimmed}'"));
    }

    if !saw_edge {
        return Err("graph diagram did not declare any edges".into());
    }

    Ok(())
}

struct CommandDisplay {
    program: String,
    args: Vec<String>,
}

impl CommandDisplay {
    fn new(program: &str) -> Self {
        Self {
            program: program.to_string(),
            args: Vec::new(),
        }
    }

    fn push<S>(&mut self, arg: S)
    where
        S: AsRef<str>,
    {
        self.args.push(arg.as_ref().to_string());
    }

    fn render(&self) -> String {
        let mut repr = quote_segment(&self.program);
        for arg in &self.args {
            repr.push(' ');
            repr.push_str(&quote_segment(arg));
        }
        repr
    }
}

fn quote_segment(segment: &str) -> String {
    if segment.is_empty()
        || segment.contains(|c: char| c.is_whitespace() || matches!(c, '"' | '\''))
    {
        format!("\"{}\"", segment.replace('"', "\\\""))
    } else {
        segment.to_string()
    }
}
