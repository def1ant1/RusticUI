//! Rust-first replacements for the legacy docs automation scripts.
//!
//! The commands centralise service worker builds and screenshot manifest
//! generation in the `xtask` binary so CI and local contributors share the
//! same tooling surface.

use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use chrono::Utc;
use clap::{Args, ValueEnum};
use serde::Serialize;
use walkdir::WalkDir;

/// Configuration options for the docs automation command.
#[derive(Args, Debug)]
pub struct DocsAssetsArgs {
    /// Task to execute. Defaults to running both service worker and screenshot generation.
    #[arg(long, value_enum, default_value_t = DocsAssetsMode::All)]
    pub mode: DocsAssetsMode,
    /// Limit screenshot manifest generation to a single project key.
    #[arg(long)]
    pub project: Option<String>,
    /// Emit log messages without writing files.
    #[arg(long)]
    pub dry_run: bool,
    /// Override the screenshot manifest output path.
    #[arg(long)]
    pub output: Option<PathBuf>,
}

/// Supported docs automation tasks.
#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum DocsAssetsMode {
    /// Generate both the service worker and screenshot manifest.
    All,
    /// Only rebuild the service worker bundle.
    ServiceWorker,
    /// Only regenerate the screenshot manifest.
    Screenshots,
}

/// Execute the requested docs automation flow.
pub fn docs_assets(args: DocsAssetsArgs) -> Result<()> {
    match args.mode {
        DocsAssetsMode::All => {
            build_service_worker(args.dry_run)?;
            generate_screenshot_manifest(
                args.project.as_deref(),
                args.output.as_deref(),
                args.dry_run,
            )?;
        }
        DocsAssetsMode::ServiceWorker => build_service_worker(args.dry_run)?,
        DocsAssetsMode::Screenshots => {
            generate_screenshot_manifest(
                args.project.as_deref(),
                args.output.as_deref(),
                args.dry_run,
            )?;
        }
    }
    Ok(())
}

fn build_service_worker(dry_run: bool) -> Result<()> {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..");
    let src = repo_root.join("docs/src/sw.js");
    let dest_root = repo_root.join("docs/export");
    let dest = dest_root.join("sw.js");

    println!(
        "[docs-assets] building service worker: {} -> {}",
        src.display(),
        dest.display()
    );
    if dry_run {
        return Ok(());
    }

    fs::create_dir_all(&dest_root).context("create docs/export directory")?;
    let mut output = fs::File::create(&dest).context("create service worker destination")?;
    let banner = format!("// uuid: {}\n", Utc::now().to_rfc3339());
    output
        .write_all(banner.as_bytes())
        .context("write service worker banner")?;
    let content = fs::read(&src).context("read source service worker")?;
    output
        .write_all(&content)
        .context("write service worker body")?;
    println!("[docs-assets] wrote {} bytes", content.len());
    Ok(())
}

#[derive(Debug, Serialize)]
struct ScreenshotManifest {
    generated_at: String,
    host: String,
    projects: Vec<ProjectManifest>,
}

#[derive(Debug, Serialize)]
struct ProjectManifest {
    name: String,
    viewport: [u32; 2],
    routes: Vec<RouteManifest>,
}

#[derive(Debug, Serialize)]
struct RouteManifest {
    source: String,
    url: String,
    output: String,
    modes: Vec<&'static str>,
}

fn generate_screenshot_manifest(
    project: Option<&str>,
    output_override: Option<&Path>,
    dry_run: bool,
) -> Result<()> {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..");
    let docs_root = repo_root.join("docs");

    let projects = [
        ProjectConfig {
            key: "material-ui",
            input: "pages/material-ui/getting-started/templates",
            viewport: (1626, 914),
        },
        ProjectConfig {
            key: "joy-ui",
            input: "pages/joy-ui/getting-started/templates",
            viewport: (1600, 800),
        },
    ];

    let selected: Vec<_> = projects
        .iter()
        .filter(|config| project.map_or(true, |p| p == config.key))
        .collect();

    let host = env::var("DEPLOY_PREVIEW").unwrap_or_else(|_| "http://localhost:3000".to_string());
    let mut project_manifests = Vec::new();

    for config in selected {
        let dir = docs_root.join(&config.input);
        let mut routes = Vec::new();
        for entry in WalkDir::new(&dir)
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_file())
        {
            let path = entry.path();
            let file_name = path
                .file_name()
                .and_then(|os| os.to_str())
                .unwrap_or_default();
            if file_name.starts_with("index") {
                continue;
            }
            let stem = path
                .file_stem()
                .and_then(|os| os.to_str())
                .unwrap_or(file_name);
            let route = format!("/{}/getting-started/templates/{}", config.key, stem);
            let trimmed = route.trim_end_matches('/');
            let output_path = format!("docs/public/static/screenshots{}.jpg", trimmed);
            routes.push(RouteManifest {
                source: path.strip_prefix(&docs_root).unwrap().display().to_string(),
                url: format!("{host}{route}"),
                output: output_path,
                modes: vec!["light", "dark", "default", "default-dark"],
            });
        }
        project_manifests.push(ProjectManifest {
            name: config.key.to_string(),
            viewport: [config.viewport.0, config.viewport.1],
            routes,
        });
    }

    let manifest = ScreenshotManifest {
        generated_at: Utc::now().to_rfc3339(),
        host,
        projects: project_manifests,
    };

    let manifest_json = serde_json::to_string_pretty(&manifest)?;

    if dry_run {
        println!("[docs-assets] dry run manifest:\n{manifest_json}");
        return Ok(());
    }

    let output_path = output_override
        .map(PathBuf::from)
        .unwrap_or_else(|| docs_root.join("public/static/screenshots/manifest.json"));
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).context("create screenshot manifest directory")?;
    }
    fs::write(&output_path, manifest_json)
        .with_context(|| format!("write screenshot manifest to {}", output_path.display()))?;
    println!(
        "[docs-assets] wrote screenshot manifest to {}",
        output_path.display()
    );
    Ok(())
}

struct ProjectConfig {
    key: &'static str,
    input: &'static str,
    viewport: (u32, u32),
}
