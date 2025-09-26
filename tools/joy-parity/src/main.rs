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

/// Command line contract for the Joy component inventory scanner.
///
/// The CLI mirrors `tools/material-parity` so that automation can treat
/// both reports uniformly.  Defaults intentionally point at the
/// canonical upstream sources so local developers and CI run the exact
/// same code paths without remembering flags.
#[derive(Parser, Debug)]
#[command(
    name = "joy-parity",
    about = "Scans Joy UI exports and compares them with the Rust crates.",
    version
)]
struct Cli {
    /// Root of the Joy UI TypeScript sources (authoritative implementation).
    #[arg(long, default_value = "packages/mui-joy/src")]
    joy_src: PathBuf,

    /// Location of the Rust Joy crate that should mirror the React API surface.
    #[arg(long, default_value = "crates/rustic-ui-joy/src")]
    rust_joy: PathBuf,

    /// Location of the shared headless primitives for parity comparisons.
    #[arg(long, default_value = "crates/rustic-ui-headless/src")]
    rust_headless: PathBuf,

    /// Destination markdown report path.  The artifact blends narrative and
    /// machine-readable JSON so diffs surface regressions immediately.
    #[arg(long, default_value = "docs/joy-component-parity.md")]
    report: PathBuf,

    /// Optional JSON-only export that mirrors the embedded report payload.
    #[arg(long)]
    json: Option<PathBuf>,

    /// Number of backlog items to highlight in the markdown dashboard.
    #[arg(long, default_value_t = 10)]
    top_n: usize,
}

/// Canonical representation of a component export discovered in the Joy UI sources.
#[derive(Debug, Clone, Serialize)]
struct ComponentEntry {
    /// Display name re-exported from the TypeScript index file.
    name: String,
    /// Normalized identifier (snake_case) for comparisons against Rust modules.
    normalized: String,
    /// Relative module specifier captured in the export statement.
    source: String,
    /// The concrete `index.ts` file that declared the export. Useful for troubleshooting duplicates.
    declared_in: String,
}

/// Snapshot capturing end-to-end coverage statistics for Joy UI.
#[derive(Debug, Serialize)]
struct CoverageReport {
    generated_at: DateTime<Utc>,
    total_components: usize,
    supported_in_joy: usize,
    supported_in_headless: usize,
    joy_coverage: f32,
    headless_coverage: f32,
    components: Vec<ComponentEntry>,
    missing_from_joy: Vec<ComponentEntry>,
    missing_from_headless: Vec<ComponentEntry>,
    extra_in_joy: Vec<String>,
    extra_in_headless: Vec<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Perform the heavy lifting up-front so command invocations remain single-purpose.
    let report = build_report(&cli)?;
    write_markdown_report(&report, &cli.report, cli.top_n)?;

    if let Some(json_path) = &cli.json {
        let json_blob = serde_json::to_string_pretty(&report)?;
        fs::write(json_path, format!("{json_blob}\n"))
            .with_context(|| format!("failed to write JSON report to {}", json_path.display()))?;
    }

    println!(
        "[joy-parity] wrote {} (React exports: {}, Rust coverage: Joy {:.1}% / Headless {:.1}%)",
        cli.report.display(),
        report.total_components,
        report.joy_coverage * 100.0,
        report.headless_coverage * 100.0,
    );

    Ok(())
}

/// Orchestrates the scanning flow: enumerate React exports, normalize names,
/// and compare them against Rust module inventories.
fn build_report(cli: &Cli) -> Result<CoverageReport> {
    let js_components = scan_joy_components(&cli.joy_src)?;
    let js_component_list: Vec<ComponentEntry> = js_components.values().cloned().collect();
    let js_keys: BTreeSet<String> = js_components.keys().cloned().collect();

    let joy_modules = discover_rust_modules(&cli.rust_joy)?;
    let headless_modules = discover_rust_modules(&cli.rust_headless)?;

    let mut supported_in_joy = 0usize;
    let mut supported_in_headless = 0usize;
    let mut missing_from_joy = Vec::new();
    let mut missing_from_headless = Vec::new();

    for entry in &js_component_list {
        if joy_modules.contains(&entry.normalized) {
            supported_in_joy += 1;
        } else {
            missing_from_joy.push(entry.clone());
        }

        if headless_modules.contains(&entry.normalized) {
            supported_in_headless += 1;
        } else {
            missing_from_headless.push(entry.clone());
        }
    }

    let extra_in_joy: Vec<String> = joy_modules.difference(&js_keys).cloned().collect();
    let extra_in_headless: Vec<String> = headless_modules.difference(&js_keys).cloned().collect();

    let total = js_component_list.len();
    let joy_coverage = if total == 0 {
        0.0
    } else {
        supported_in_joy as f32 / total as f32
    };
    let headless_coverage = if total == 0 {
        0.0
    } else {
        supported_in_headless as f32 / total as f32
    };

    Ok(CoverageReport {
        generated_at: Utc::now(),
        total_components: total,
        supported_in_joy,
        supported_in_headless,
        joy_coverage,
        headless_coverage,
        components: js_component_list,
        missing_from_joy,
        missing_from_headless,
        extra_in_joy,
        extra_in_headless,
    })
}

/// Parse the Joy UI source tree and build a deduplicated component export inventory.
fn scan_joy_components(joy_src: &Path) -> Result<BTreeMap<String, ComponentEntry>> {
    let cm: Lrc<SourceMap> = Lrc::new(SourceMap::default());
    let globals = Globals::new();

    GLOBALS.set(&globals, || -> Result<BTreeMap<String, ComponentEntry>> {
        let mut entries: BTreeMap<String, ComponentEntry> = BTreeMap::new();

        for entry in WalkDir::new(joy_src).sort_by_file_name() {
            let entry = entry?;
            if !entry.file_type().is_file() {
                continue;
            }

            let path = entry.path();
            if !is_index_file(path) {
                continue;
            }

            let module = parse_module(&cm, path)?;
            for export in extract_component_exports(&module, path, joy_src) {
                // Preserve the first occurrence. Duplicate exports can occur if packages expose
                // both default and themed variants; we only need one canonical record for parity.
                entries.entry(export.normalized.clone()).or_insert(export);
            }
        }

        Ok(entries)
    })
}

/// Determines whether a file path is an `index` module worth parsing for exports.
fn is_index_file(path: &Path) -> bool {
    let Some(file_name) = path.file_name().and_then(|s| s.to_str()) else {
        return false;
    };

    matches!(
        file_name,
        "index.ts" | "index.tsx" | "index.js" | "index.mjs" | "index.cjs"
    )
}

/// Parse a module using SWC so we can reason about export statements structurally.
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

/// Extract re-exported component entry points from a parsed module.
fn extract_component_exports(
    module: &Module,
    module_path: &Path,
    joy_root: &Path,
) -> Vec<ComponentEntry> {
    let mut components = Vec::new();

    for item in &module.body {
        let ModuleItem::ModuleDecl(ModuleDecl::ExportNamed(named)) = item else {
            continue;
        };

        let Some(src) = &named.src else {
            // Skip exports that simply re-surface identifiers from the same file.
            continue;
        };

        let specifier_path = src.value.to_string();
        if !specifier_path.starts_with('.') {
            // Ignore third-party package re-exports; they are not Joy components.
            continue;
        }

        for specifier in &named.specifiers {
            if let Some(entry) =
                build_component_entry(specifier, &specifier_path, module_path, joy_root)
            {
                components.push(entry);
            }
        }
    }

    components
}

/// Convert an export specifier into a [`ComponentEntry`] when it represents a component default export.
fn build_component_entry(
    specifier: &ExportSpecifier,
    specifier_path: &str,
    module_path: &Path,
    joy_root: &Path,
) -> Option<ComponentEntry> {
    let exported_name = match specifier {
        ExportSpecifier::Named(named) => {
            let orig = export_name_to_string(&named.orig)?;
            if orig != "default" {
                // Only treat default exports as components; named exports generally expose hooks or helpers.
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

    let normalized = normalize_component_name(&exported_name);
    let source = joy_root
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

/// Convert a [`ModuleExportName`] into a human-readable string.
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

    let mut chars = candidate.chars();
    match chars.next() {
        Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
        None => candidate.to_string(),
    }
}

/// Normalize component identifiers so `JoyAccordion` and `Accordion` resolve to the same module slot.
fn normalize_component_name(name: &str) -> String {
    let mut snake = name.to_snake_case();
    // Strip well-known prefixes that appear in alias exports. Apply repeatedly in case aliases combine them.
    let prefixes = ["joy_", "mui_", "rustic_ui_joy_", "unstable_", "experimental_"];
    let mut changed = true;
    while changed {
        changed = false;
        for prefix in prefixes {
            if let Some(stripped) = snake.strip_prefix(prefix) {
                snake = stripped.to_string();
                changed = true;
                break;
            }
        }
    }
    snake
}

/// Scan a Rust crate directory for module files that map to components.
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

/// Render the combined markdown and JSON artifact to disk.
fn write_markdown_report(report: &CoverageReport, path: &Path, top_n: usize) -> Result<()> {
    let json_blob = serde_json::to_string_pretty(report)?;
    let mut markdown = String::new();

    markdown.push_str("# Joy Component Parity\n\n");
    markdown.push_str(&format!(
        "_Last updated {} via `cargo xtask joy-inventory`._\n\n",
        report.generated_at.to_rfc3339()
    ));

    markdown.push_str("## Coverage snapshot\n\n");
    markdown.push_str(&format!(
        "- React exports analyzed: {}\\n",
        report.total_components
    ));
    markdown.push_str(&format!(
        "- `mui-joy` coverage: {} ({:.1}%)\\n",
        report.supported_in_joy,
        report.joy_coverage * 100.0
    ));
    markdown.push_str(&format!(
        "- `mui-headless` coverage: {} ({:.1}%)\\n\n",
        report.supported_in_headless,
        report.headless_coverage * 100.0
    ));

    markdown.push_str("## Highest priority gaps\n\n");
    markdown.push_str("| Rank | Component | Source |\\n");
    markdown.push_str("| --- | --- | --- |\\n");

    for (idx, component) in report.missing_from_joy.iter().take(top_n).enumerate() {
        markdown.push_str(&format!(
            "| {} | {} | `{}` |\\n",
            idx + 1,
            component.name,
            component.source
        ));
    }

    if report.missing_from_joy.is_empty() {
        markdown.push_str("| – | All caught up! | – |\\n");
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
