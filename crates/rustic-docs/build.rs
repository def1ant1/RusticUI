use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use heck::ToPascalCase;
use walkdir::WalkDir;

// build.rs intentionally documents how SSR/static export orchestration is wired.
//
// The script does not perform heavy work; instead it registers the knobs that
// CI/CD platforms and release automation can toggle. By emitting `rerun` hints
// we ensure that when infrastructure teams change the export directory or
// observability wiring, Cargo recompiles the crate with the updated metadata.
//
// The inline comments outline how enterprise adopters can attach scaling hooks
// (for example rotating CDN buckets or multi-region telemetry endpoints) while
// keeping the build pipeline reproducible.

fn main() {
    // Allow operators to override where static snapshots are exported. This is
    // particularly useful when plugging the crate into managed artifact stores
    // such as AWS S3 or GCS. The environment variable is intentionally named to
    // be self-explanatory for platform engineers.
    println!("cargo:rerun-if-env-changed=RUSTIC_DOCS_EXPORT_DIR");

    // Document where distributed tracing collectors may be defined. Runtime
    // initialisation reads the same variable, giving enterprises a single place
    // to set observability targets without duplicating configuration across
    // build scripts and server binaries.
    println!("cargo:rerun-if-env-changed=RUSTIC_DOCS_TRACING_ENDPOINT");

    // Rebuild whenever the legacy docs tree changes so the generated inventory
    // stays aligned with the source material.
    println!("cargo:rerun-if-changed=../../docs");

    generate_inventory().expect("failed to generate docs inventory");
}

fn generate_inventory() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let repo_root = manifest_dir.join("..").join("..");
    let docs_root = repo_root.join("docs");
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    let mut entries = Vec::new();
    let mut markdowns = Vec::new();

    let targets = [
        (
            "src/modules/components",
            "InventoryCategory::Component",
            "component",
        ),
        ("pages", "InventoryCategory::Page", "page"),
        ("data", "InventoryCategory::Data", "data"),
    ];

    for (relative, category, kind) in targets.iter() {
        let dir = docs_root.join(relative);
        if !dir.exists() {
            continue;
        }

        for entry in WalkDir::new(&dir).into_iter().filter_map(Result::ok) {
            if !entry.file_type().is_file() {
                continue;
            }
            let path = entry.path();
            let rel = path.strip_prefix(&docs_root)?.to_path_buf();
            entries.push((rel.clone(), *category, *kind));

            let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
            if matches!(extension, "md" | "mdx") {
                markdowns.push(rel);
            }
        }
    }

    entries.sort_by(|a, b| a.0.cmp(&b.0));
    markdowns.sort();

    let mut inventory_file = File::create(out_dir.join("docs_inventory.rs"))?;
    writeln!(
        inventory_file,
        "pub(crate) static DOCS_INVENTORY: &[InventoryEntry] = &["
    )?;

    for (rel, category, kind) in entries {
        let rel_str = rel.to_string_lossy().replace('\\', "/");
        let route = derive_route(kind, &rel_str);
        let component = component_hint(&rel_str);
        let primitives = primitives_literal(kind);
        let notes = escape_string(&plan_notes(kind, &rel_str));
        writeln!(
            inventory_file,
            "    InventoryEntry {{\n        source_path: \"{rel}\",\n        category: {category},\n        route_hint: \"{route}\",\n        frameworks: &[\n            FrameworkPlan {{ framework: \"leptos\", component: \"{component}Leptos\", module_path: \"crate::content::leptos_components\", notes: \"Use InventoryBoard to render this surface via RusticUI primitives.\" }},\n            FrameworkPlan {{ framework: \"yew\", component: \"{component}Yew\", module_path: \"crate::content::yew_components\", notes: \"Reuse the Yew InventoryBoard component to surface the same plan.\" }},\n        ],\n        locales: &[LocalizedRoute {{ locale: \"en\", path: \"{route}\" }}],\n        recommended_primitives: &{primitives},\n        notes: \"{notes}\",\n    }},",
            rel = rel_str,
            category = category,
            route = route,
            component = component,
            primitives = primitives,
            notes = notes,
        )?;
    }

    writeln!(inventory_file, "];")?;

    let mut markdown_file = File::create(out_dir.join("docs_markdown.rs"))?;
    writeln!(
        markdown_file,
        "pub(crate) static MARKDOWN_DOCUMENTS: &[MarkdownDocument] = &["
    )?;

    for rel in markdowns {
        let rel_str = rel.to_string_lossy().replace('\\', "/");
        let route = derive_route("page", &rel_str);
        let title = component_hint(&rel_str);
        writeln!(
            markdown_file,
            "    MarkdownDocument {{\n        source_path: \"{rel}\",\n        route_hint: \"{route}\",\n        title: \"{title}\",\n        body: include_str!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/../../docs/{rel}\")),\n    }},",
            rel = rel_str,
            route = route,
            title = title,
        )?;
    }

    writeln!(markdown_file, "];")?;

    Ok(())
}

fn derive_route(kind: &str, rel: &str) -> String {
    let mut cleaned = strip_extension(rel);
    if cleaned.ends_with("/index") {
        cleaned.truncate(cleaned.len() - "/index".len());
    }
    let mut route = cleaned.trim_matches('/').replace("index", "");
    route = route.trim_matches('/').to_string();
    match kind {
        "component" => format!("/components/{}", route)
            .trim_end_matches('/')
            .to_string(),
        "data" => format!("/data/{}", route).trim_end_matches('/').to_string(),
        _ => {
            if route.is_empty() {
                "/".to_string()
            } else {
                format!("/{}", route)
            }
        }
    }
    .trim_end_matches('/')
    .to_string()
}

fn strip_extension(rel: &str) -> String {
    const EXTENSIONS: [&str; 9] = [
        ".tsx", ".ts", ".js", ".jsx", ".md", ".mdx", ".json", ".yaml", ".yml",
    ];
    for ext in EXTENSIONS {
        if rel.ends_with(ext) {
            let mut owned = rel.to_string();
            owned.truncate(owned.len() - ext.len());
            return owned;
        }
    }
    rel.to_string()
}

fn component_hint(rel: &str) -> String {
    let cleaned = strip_extension(rel);
    let normalised = cleaned
        .split('/')
        .filter(|segment| !segment.is_empty() && *segment != "index")
        .collect::<Vec<_>>()
        .join(" ");
    let fallback = if normalised.is_empty() {
        "home".to_string()
    } else {
        normalised
    };
    fallback.replace(['-', '_'], " ").to_pascal_case()
}

fn primitives_literal(kind: &str) -> String {
    match kind {
        "component" => format!("{:?}", COMPONENT_PRIMITIVES),
        "data" => format!("{:?}", DATA_PRIMITIVES),
        _ => format!("{:?}", PAGE_PRIMITIVES),
    }
}

fn plan_notes(kind: &str, rel: &str) -> String {
    match kind {
        "component" => format!(
            "Promote {rel} into a Leptos/Yew demo using RusticUI Card + ThemeProvider scaffolding."
        ),
        "data" => format!(
            "Expose {rel} via strongly typed loaders and surface it through RusticUI tables."
        ),
        _ => format!("Rebuild {rel} as a routed page leveraging the shared DocsThemeShell."),
    }
}

fn escape_string(input: &str) -> String {
    input.replace('\\', "\\").replace('"', "\\\"")
}

const COMPONENT_PRIMITIVES: [&str; 3] = ["Card", "AppBar", "ThemeProvider"];
const PAGE_PRIMITIVES: [&str; 3] = ["AppBar", "Router", "Card"];
const DATA_PRIMITIVES: [&str; 3] = ["Table", "List", "Card"];
