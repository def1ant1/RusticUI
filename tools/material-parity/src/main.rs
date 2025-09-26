use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Utc};
use clap::Parser;
use heck::ToSnakeCase;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use swc_common::{sync::Lrc, Globals, SourceMap, GLOBALS};
use swc_ecma_ast::{ExportSpecifier, Ident, Module, ModuleDecl, ModuleExportName, ModuleItem};
use swc_ecma_parser::lexer::Lexer;
use swc_ecma_parser::{EsSyntax, Parser as SwcParser, StringInput, Syntax, TsSyntax};
use walkdir::WalkDir;

/// Command line arguments for the component parity scanner.
#[derive(Parser, Debug)]
#[command(
    name = "material-parity",
    about = "Scans the React source tree to measure Rust component coverage.",
    version
)]
struct Cli {
    /// Root of the JavaScript implementation (the canonical source of truth).
    #[arg(long, default_value = "packages/mui-material/src")]
    material_src: PathBuf,

    /// Location of the Rust Material crate that should mirror the JS API surface.
    #[arg(long, default_value = "crates/rustic-ui-material/src")]
    rust_material: PathBuf,

    /// Location of the headless primitives crate for comparison.
    #[arg(long, default_value = "crates/rustic-ui-headless/src")]
    rust_headless: PathBuf,

    /// Destination markdown file that will host the parity report.
    #[arg(long, default_value = "docs/material-component-parity.md")]
    report: PathBuf,

    /// Optional standalone JSON export path for downstream automation.
    #[arg(long)]
    json: Option<PathBuf>,

    /// Number of missing components to highlight as the most urgent backlog.
    #[arg(long, default_value_t = 10)]
    top_n: usize,
}

/// Canonical representation of a component export from the React source tree.
#[derive(Debug, Clone, Serialize)]
struct ComponentEntry {
    /// Display name that React exposes (e.g. `Button`).
    name: String,
    /// Normalized identifier derived from the name (e.g. `button`).
    normalized: String,
    /// Relative module specifier found in the source re-export (e.g. `./Button`).
    source: String,
    /// Path to the index file that declared the export. Useful for debugging duplicates.
    declared_in: String,
}

/// Snapshot of Rust coverage alongside the full component inventory.
#[derive(Debug, Serialize)]
struct CoverageReport {
    generated_at: DateTime<Utc>,
    total_components: usize,
    supported_in_material: usize,
    supported_in_headless: usize,
    material_coverage: f32,
    headless_coverage: f32,
    components: Vec<ComponentEntry>,
    missing_from_material: Vec<ComponentEntry>,
    missing_from_headless: Vec<ComponentEntry>,
    extra_in_material: Vec<String>,
    extra_in_headless: Vec<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let report = build_report(&cli)?;
    write_markdown_report(&report, &cli.report, cli.top_n)?;

    if let Some(json_path) = &cli.json {
        let json = serde_json::to_string_pretty(&report)?;
        fs::write(json_path, format!("{json}\n"))
            .with_context(|| format!("failed to write JSON report to {}", json_path.display()))?;
    }

    println!(
        "[material-parity] wrote {} (total components: {}, Rust coverage: {:.1}% / {:.1}%)",
        cli.report.display(),
        report.total_components,
        report.material_coverage * 100.0,
        report.headless_coverage * 100.0,
    );

    Ok(())
}

/// Entrypoint for orchestrating the full parity analysis flow.
fn build_report(cli: &Cli) -> Result<CoverageReport> {
    let js_components = scan_material_components(&cli.material_src)?;
    let js_component_list: Vec<ComponentEntry> = js_components.values().cloned().collect();
    let js_keys: BTreeSet<String> = js_components.keys().cloned().collect();
    let material_modules = discover_rust_modules(&cli.rust_material)?;
    let headless_modules = discover_rust_modules(&cli.rust_headless)?;

    let mut supported_in_material = 0usize;
    let mut supported_in_headless = 0usize;
    let mut missing_from_material = Vec::new();
    let mut missing_from_headless = Vec::new();

    for entry in &js_component_list {
        if material_modules.contains(&entry.normalized) {
            supported_in_material += 1;
        } else {
            missing_from_material.push(entry.clone());
        }

        if headless_modules.contains(&entry.normalized) {
            supported_in_headless += 1;
        } else {
            missing_from_headless.push(entry.clone());
        }
    }

    let extra_in_material: Vec<String> = material_modules.difference(&js_keys).cloned().collect();
    let extra_in_headless: Vec<String> = headless_modules.difference(&js_keys).cloned().collect();

    let total = js_component_list.len();
    let material_coverage = if total == 0 {
        0.0
    } else {
        supported_in_material as f32 / total as f32
    };
    let headless_coverage = if total == 0 {
        0.0
    } else {
        supported_in_headless as f32 / total as f32
    };

    Ok(CoverageReport {
        generated_at: Utc::now(),
        total_components: total,
        supported_in_material,
        supported_in_headless,
        material_coverage,
        headless_coverage,
        components: js_component_list,
        missing_from_material,
        missing_from_headless,
        extra_in_material,
        extra_in_headless,
    })
}

/// Parse the React source tree and extract component exports declared in `index` files.
fn scan_material_components(material_src: &Path) -> Result<BTreeMap<String, ComponentEntry>> {
    let cm: Lrc<SourceMap> = Lrc::new(SourceMap::default());
    let globals = Globals::new();

    GLOBALS.set(&globals, || -> Result<BTreeMap<String, ComponentEntry>> {
        let mut entries: BTreeMap<String, ComponentEntry> = BTreeMap::new();

        for entry in WalkDir::new(material_src).sort_by_file_name() {
            let entry = entry?;
            if !entry.file_type().is_file() {
                continue;
            }

            let path = entry.path();
            if !is_index_file(path) {
                continue;
            }

            let module = parse_module(&cm, path)?;
            for export in extract_component_exports(&module, path, material_src) {
                entries.entry(export.normalized.clone()).or_insert(export);
            }
        }

        Ok(entries)
    })
}

/// Determine if a filesystem path corresponds to an `index` module that may contain re-exports.
fn is_index_file(path: &Path) -> bool {
    let Some(file_name) = path.file_name().and_then(|s| s.to_str()) else {
        return false;
    };
    matches!(
        file_name,
        "index.ts" | "index.tsx" | "index.js" | "index.mjs" | "index.cjs"
    )
}

/// Parse a source file into an ECMAScript module using SWC.
fn parse_module(cm: &Lrc<SourceMap>, path: &Path) -> Result<Module> {
    let fm = cm
        .load_file(path)
        .with_context(|| format!("failed to load {}", path.display()))?;

    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_ascii_lowercase());
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("");
    let is_jsx = matches!(
        extension.as_deref(),
        Some("tsx") | Some("jsx") | Some("mjsx") | Some("cjsx")
    );
    let is_ts = matches!(
        extension.as_deref(),
        Some("ts") | Some("tsx") | Some("mts") | Some("cts")
    );
    let is_d_ts = file_name.ends_with(".d.ts") || file_name.ends_with(".d.tsx");

    let syntax = if is_ts {
        Syntax::Typescript(TsSyntax {
            tsx: is_jsx,
            dts: is_d_ts,
            ..Default::default()
        })
    } else {
        Syntax::Es(EsSyntax {
            jsx: is_jsx,
            ..Default::default()
        })
    };

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
        return Err(anyhow!("{}", joined))
            .with_context(|| format!("syntax errors in {}", path.display()));
    }

    Ok(module)
}

/// Extract component re-exports from a parsed module.
fn extract_component_exports(
    module: &Module,
    module_path: &Path,
    material_root: &Path,
) -> Vec<ComponentEntry> {
    let mut components = Vec::new();

    for item in &module.body {
        let ModuleItem::ModuleDecl(ModuleDecl::ExportNamed(named)) = item else {
            continue;
        };

        let Some(src) = &named.src else {
            // Skip `export { colors };` style statements which are not component entry points.
            continue;
        };

        let specifier_path = src.value.to_string();
        if !specifier_path.starts_with('.') {
            // Ignore packages re-exported from npm modules.
            continue;
        }

        for specifier in &named.specifiers {
            if let Some(entry) =
                build_component_entry(specifier, &specifier_path, module_path, material_root)
            {
                components.push(entry);
            }
        }
    }

    components
}

/// Convert an SWC export specifier into a structured [`ComponentEntry`] if it represents a widget.
fn build_component_entry(
    specifier: &ExportSpecifier,
    specifier_path: &str,
    module_path: &Path,
    material_root: &Path,
) -> Option<ComponentEntry> {
    let exported_name = match specifier {
        ExportSpecifier::Named(named) => {
            let orig = export_name_to_string(&named.orig)?;
            if orig != "default" {
                // Only consider default exports; other named exports typically expose hooks or constants.
                return None;
            }
            named
                .exported
                .as_ref()
                .and_then(export_name_to_string)
                .unwrap_or_else(|| derive_name_from_specifier(specifier_path))
        }
        ExportSpecifier::Default(default_spec) => ident_to_string(&default_spec.exported),
        ExportSpecifier::Namespace(_) => return None,
    };

    if !exported_name
        .chars()
        .next()
        .map(|c| c.is_uppercase())
        .unwrap_or(false)
    {
        return None;
    }

    let normalized = exported_name.to_snake_case();
    let source = material_root
        .join(specifier_path.trim_start_matches("./"))
        .display()
        .to_string()
        .replace('\\', "/");

    Some(ComponentEntry {
        name: exported_name,
        normalized,
        source: source.replace('\r', ""),
        declared_in: module_path.display().to_string(),
    })
}

/// Convert a [`ModuleExportName`] into a string if possible.
fn export_name_to_string(name: &ModuleExportName) -> Option<String> {
    match name {
        ModuleExportName::Ident(ident) => Some(ident.sym.to_string()),
        ModuleExportName::Str(str_lit) => Some(str_lit.value.to_string()),
    }
}

fn ident_to_string(ident: &Ident) -> String {
    ident.sym.to_string()
}

/// Derive a PascalCase component name from a relative module specifier.
fn derive_name_from_specifier(specifier: &str) -> String {
    let without_dots = specifier.trim_start_matches("./");
    let candidate = without_dots.rsplit('/').next().unwrap_or(without_dots);

    // The JavaScript source already uses PascalCase folder names so we simply capitalize the
    // first character to avoid pulling in a heavier inflector dependency.
    let mut chars = candidate.chars();
    match chars.next() {
        Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
        None => candidate.to_string(),
    }
}

/// Inspect a Rust crate directory and gather the module names that map to components.
fn discover_rust_modules(root: &Path) -> Result<BTreeSet<String>> {
    let mut modules = BTreeSet::new();

    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            continue;
        }

        let Some(ext) = path.extension().and_then(|ext| ext.to_str()) else {
            continue;
        };
        if ext != "rs" {
            continue;
        }

        let file_stem = match path.file_stem().and_then(|s| s.to_str()) {
            Some(stem)
                if stem != "lib"
                    && stem != "mod"
                    && stem != "macros"
                    && stem != "style_helpers" =>
            {
                stem.to_string()
            }
            _ => continue,
        };

        modules.insert(file_stem);
    }

    Ok(modules)
}

/// Render the markdown artifact that blends narrative and machine-readable data.
fn write_markdown_report(report: &CoverageReport, path: &Path, top_n: usize) -> Result<()> {
    let json_blob = serde_json::to_string_pretty(report)?;
    let mut markdown = String::new();

    markdown.push_str("# Material Component Parity\n\n");
    markdown.push_str(&format!(
        "_Last updated {} via `cargo xtask material-parity`._\n\n",
        report.generated_at.to_rfc3339()
    ));

    markdown.push_str("## Coverage snapshot\n\n");
    markdown.push_str(&format!(
        "- React exports analyzed: {}\\n",
        report.total_components
    ));
    markdown.push_str(&format!(
        "- `mui-material` coverage: {} ({:.1}%)\\n",
        report.supported_in_material,
        report.material_coverage * 100.0
    ));
    markdown.push_str(&format!(
        "- `mui-headless` coverage: {} ({:.1}%)\\n\n",
        report.supported_in_headless,
        report.headless_coverage * 100.0
    ));

    markdown.push_str("## Highest priority gaps\n\n");
    markdown.push_str("| Rank | Component | Source |\n");
    markdown.push_str("| --- | --- | --- |\n");

    for (idx, component) in report.missing_from_material.iter().take(top_n).enumerate() {
        markdown.push_str(&format!(
            "| {} | {} | `{}` |\n",
            idx + 1,
            component.name,
            component.source
        ));
    }

    if report.missing_from_material.is_empty() {
        markdown.push_str("| – | All caught up! | – |\n");
    }

    markdown.push_str("\n## Machine-readable snapshot\n\n");
    markdown.push_str("```json\n");
    markdown.push_str(&json_blob);
    markdown.push_str("\n```\n");

    fs::create_dir_all(
        path.parent()
            .ok_or_else(|| anyhow::anyhow!("report path has no parent"))?,
    )?;
    fs::write(path, markdown)
        .with_context(|| format!("failed to write report to {}", path.display()))?;

    Ok(())
}
