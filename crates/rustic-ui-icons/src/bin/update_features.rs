//! update_features.rs -- regenerates the `[features]` manifest for the
//! `mui-icons` crate based on the SVG assets currently present on disk.
//!
//! The binary intentionally lives alongside the crate so maintainers can run it
//! without pulling in heavyweight external tooling. It is invoked automatically
//! from `cargo xtask icon-update` and may also be executed manually when new
//! icon sets are added or renamed.

use std::{
    collections::BTreeMap,
    env,
    error::Error,
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
};

/// Marker inserted at the beginning of the auto-generated feature block.
const START_MARKER: &str = "# BEGIN AUTO-GENERATED ICON FEATURES -- DO NOT EDIT.";
/// Marker terminating the generated block. Everything in-between is replaced on
/// each invocation of the tool to keep the manifest deterministic.
const END_MARKER: &str = "# END AUTO-GENERATED ICON FEATURES";

/// Small configuration object derived from CLI arguments. Keeping it explicit
/// allows tests to inject temporary directories without mutating global state.
#[derive(Debug)]
struct Config {
    /// Location of the icons directory containing per-set subfolders.
    icons_dir: PathBuf,
    /// Path to the `Cargo.toml` manifest whose feature block will be rewritten.
    manifest_path: PathBuf,
}

impl Config {
    /// Derives configuration from CLI arguments. The binary accepts two optional
    /// flags so tests and power users can override the defaults:
    /// `--icons-dir <path>` and `--manifest-path <path>`.
    fn from_env() -> Result<Self, Box<dyn Error>> {
        let args = env::args_os().skip(1).collect::<Vec<OsString>>();
        let mut icons_dir: Option<PathBuf> = None;
        let mut manifest_path: Option<PathBuf> = None;

        let mut i = 0;
        while i < args.len() {
            let flag = &args[i];
            if flag == "--icons-dir" {
                let value = args
                    .get(i + 1)
                    .ok_or("missing value for --icons-dir")?
                    .clone();
                icons_dir = Some(PathBuf::from(value));
                i += 2;
            } else if flag == "--manifest-path" {
                let value = args
                    .get(i + 1)
                    .ok_or("missing value for --manifest-path")?
                    .clone();
                manifest_path = Some(PathBuf::from(value));
                i += 2;
            } else {
                return Err(format!("unrecognized argument: {:?}", flag).into());
            }
        }

        // Default to the canonical crate locations when overrides are absent.
        let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let default_icons = crate_root.join("icons");
        let default_manifest = crate_root.join("Cargo.toml");

        Ok(Self {
            icons_dir: icons_dir.unwrap_or(default_icons),
            manifest_path: manifest_path.unwrap_or(default_manifest),
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_env()?;
    println!(
        "[mui-icons] regenerating feature manifest using icons from {}",
        config.icons_dir.display()
    );

    let sets = discover_icon_sets(&config.icons_dir)?;
    println!(
        "[mui-icons] discovered {} icon set(s): {}",
        sets.len(),
        sets.keys().cloned().collect::<Vec<_>>().join(", ")
    );

    let block = render_feature_block(&sets);
    rewrite_manifest(&config.manifest_path, &block)?;

    Ok(())
}

/// Walks the icons directory and builds a sorted mapping of `set -> [icons]`.
///
/// * Sets are alphabetically sorted to keep diffs stable when new folders are
///   introduced.
/// * Icon names are likewise sorted to avoid churn when icons are renamed or
///   removed.
fn discover_icon_sets(path: &Path) -> Result<BTreeMap<String, Vec<String>>, Box<dyn Error>> {
    if !path.exists() {
        return Err(format!("icons directory does not exist: {}", path.display()).into());
    }

    let mut sets: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        if !metadata.is_dir() {
            continue;
        }
        let set_name = entry.file_name().to_string_lossy().into_owned();
        let mut icons = Vec::new();
        for icon in fs::read_dir(entry.path())? {
            let icon = icon?;
            let icon_path = icon.path();
            if icon_path.extension().and_then(|ext| ext.to_str()) != Some("svg") {
                continue;
            }
            let name = icon_path
                .file_stem()
                .ok_or("icon missing file name")?
                .to_string_lossy()
                .into_owned();
            icons.push(name);
        }
        icons.sort();
        if icons.is_empty() {
            // Skip empty directories so partially migrated sets do not produce
            // dangling features. This keeps the manifest stable for in-flight
            // refactors where assets are staged incrementally.
            continue;
        }
        sets.insert(set_name, icons);
    }

    Ok(sets)
}

/// Renders the TOML snippet containing the feature declarations.
fn render_feature_block(sets: &BTreeMap<String, Vec<String>>) -> String {
    let mut block = String::new();
    block.push_str(START_MARKER);
    block.push('\n');
    block.push_str("# Automatically generated by `cargo xtask icon-update`.\n");
    block.push_str("# The automation keeps feature wiring in sync with the icons on disk.\n\n");

    block.push_str("all-icons = [\n");
    for set in sets.keys() {
        block.push_str(&format!("    \"set-{set}\",\n"));
    }
    block.push_str("]\n\n");

    for (set, icons) in sets {
        block.push_str(&format!("set-{set} = [\n"));
        for icon in icons {
            block.push_str(&format!("    \"icon-{set}-{icon}\",\n"));
        }
        block.push_str("]\n\n");
    }

    for (set, icons) in sets {
        for icon in icons {
            block.push_str(&format!("icon-{set}-{icon} = []\n"));
        }
        block.push('\n');
    }

    block.push_str(END_MARKER);
    block.push('\n');
    block
}

/// Rewrites the manifest with the freshly generated block, keeping surrounding
/// configuration intact.
fn rewrite_manifest(path: &Path, block: &str) -> Result<(), Box<dyn Error>> {
    println!("[mui-icons] updating manifest at {}", path.display());
    let manifest = fs::read_to_string(path)?;
    let start = manifest
        .find(START_MARKER)
        .ok_or("start marker missing from manifest")?;
    let end_rel = manifest[start..]
        .find(END_MARKER)
        .ok_or("end marker missing from manifest")?;
    let mut end = start + end_rel + END_MARKER.len();
    if manifest[end..].starts_with('\n') {
        end += 1;
    }

    let new_manifest = format!("{}{}{}", &manifest[..start], block, &manifest[end..]);

    if new_manifest == manifest {
        println!("[mui-icons] manifest already up to date; no changes written");
        return Ok(());
    }

    fs::write(path, new_manifest)?;
    println!("[mui-icons] manifest features refreshed successfully");
    Ok(())
}
