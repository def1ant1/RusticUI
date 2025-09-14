// update_icons.rs - maintenance utility for downloading and refreshing the
// Material Design SVG icon set. The goal is to keep contributors from having
// to manually track and copy thousands of files. Instead, a single command
// fetches the upstream repository, extracts the production-ready `24px` SVGs,
// and writes them into the crate's `material-icons/` directory.
//
// Beyond copying files, the tool also regenerates the `[features]` section in
// the crate's `Cargo.toml`. Each icon becomes an optional feature so end users
// can selectively include only the symbols they need, keeping compile times
// and binary sizes manageable for large applications.
//
// This binary is gated behind the `update-icons` feature to avoid pulling the
// networking and ZIP dependencies into downstream builds. CI and maintainers
// can invoke it via `make icons`.

use std::{
    fs,
    io::{self, Cursor, Read},
    path::Path,
};

use ureq;
use zip::ZipArchive;

/// URL of the upstream Material Design icons repository as a zip archive.
const ZIP_URL: &str =
    "https://github.com/google/material-design-icons/archive/refs/heads/master.zip";
/// Location where the extracted SVGs will be written.
const ICON_DIR: &str = "crates/mui-icons-material/material-icons";
/// Path to the crate's manifest for feature regeneration.
const MANIFEST_PATH: &str = "crates/mui-icons-material/Cargo.toml";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Fetching icons from {ZIP_URL} ...");
    let response = ureq::get(ZIP_URL).call()?;
    let mut bytes = Vec::new();
    response.into_reader().read_to_end(&mut bytes)?;

    // Extract the downloaded archive directly from memory. Using an in-memory
    // cursor keeps the implementation straightforward while avoiding temporary
    // files on disk.
    let reader = Cursor::new(bytes);
    let mut archive = ZipArchive::new(reader)?;

    let dest = Path::new(ICON_DIR);
    if dest.exists() {
        // Remove any pre-existing icons to avoid stale files lingering after
        // upstream deletions or renames.
        fs::remove_dir_all(dest)?;
    }
    fs::create_dir_all(dest)?;

    // Collect the base names of all extracted icons. This drives the feature
    // regeneration step later on.
    let mut icons: Vec<String> = Vec::new();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let name = file.name().to_string();
        // The Google repo contains multiple formats and sizes. We only grab the
        // production ready 24px Material icons.
        if !name.contains("materialicons/")
            || !name.contains("/svg/")
            || !name.ends_with("24px.svg")
        {
            continue;
        }
        let base = name.rsplit('/').next().unwrap().to_string();
        let out_path = dest.join(&base);
        let mut out_file = fs::File::create(&out_path)?;
        io::copy(&mut file, &mut out_file)?;
        icons.push(base.trim_end_matches(".svg").to_string());
    }

    icons.sort();
    update_features(&icons)?;

    println!("Installed {} icons into {}", icons.len(), ICON_DIR);
    Ok(())
}

/// Rewrites the `[features]` section of `Cargo.toml` so each icon can be opted
/// into individually. A helper feature `all-icons` re-enables the full set for
/// consumers that prefer convenience over binary size.
fn update_features(icons: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    const START: &str = "# BEGIN ICON FEATURES -- auto-generated, do not edit by hand.";
    const END: &str = "# END ICON FEATURES";

    let manifest = fs::read_to_string(MANIFEST_PATH)?;
    let start = manifest
        .find(START)
        .ok_or("start marker not found in Cargo.toml")?;
    let end = manifest
        .find(END)
        .ok_or("end marker not found in Cargo.toml")?;

    // Compose the replacement feature block.
    let mut block = String::new();
    block.push_str("all-icons = [\n");
    for icon in icons {
        block.push_str(&format!("    \"icon-{icon}\",\n"));
    }
    block.push_str("]\n");
    for icon in icons {
        block.push_str(&format!("icon-{icon} = []\n"));
    }

    let new_manifest = format!(
        "{}\n{}{}",
        &manifest[..start + START.len()],
        block,
        &manifest[end..]
    );
    fs::write(MANIFEST_PATH, new_manifest)?;
    Ok(())
}
