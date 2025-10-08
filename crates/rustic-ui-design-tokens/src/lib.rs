#![deny(missing_docs)]
#![doc = r"RusticUI design token helpers
=================================

This crate centralizes the serialization and packaging logic used by the
`cargo xtask` automation. Historically these responsibilities lived in
multiple npm packages—`@mui/material`, `@mui/system`, and
`@mui/icons-material`—that exposed pre-baked JSON, CSS, and SVG bundles.
Enterprise users relied on those bundles to hydrate design systems inside
monorepos or CI pipelines. As the Rust-first toolchain matured we ported the
artifact generation to strongly typed binaries. The helper APIs exposed here
make that migration explicit by documenting how each output maps back to the
legacy npm deliverables.

The modules intentionally provide verbose documentation so integrators can
trace which function to call when they need to reproduce a former npm bundle.
They are heavily annotated because most consumers interact with them through
automation in CI/CD environments."]

use anyhow::{anyhow, Context, Result};
use chrono::Utc;
use flate2::{write::GzEncoder, Compression};
use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fmt;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use tar::Builder as TarBuilder;
use walkdir::WalkDir;
use zip::write::FileOptions;

/// Builder that stages individual design token files before writing archive bundles.
///
/// The builder owns a dedicated payload directory where raw assets are copied so we can
/// calculate deterministic checksums and compose ZIP/TAR archives without mutating the
/// source tree. Each ingested file records metadata that mirrors the manifests published
/// by the former npm packages.
#[derive(Debug)]
pub struct ArtifactBundleBuilder {
    root: PathBuf,
    payload_dir: PathBuf,
    archive_stem: String,
    entries: Vec<ManifestEntry>,
}

/// Summary describing the bundle that was written to disk.
#[derive(Debug, Clone, Serialize)]
pub struct BundleSummary {
    /// Directory containing the raw payload alongside the manifest and archives.
    pub bundle_root: PathBuf,
    /// Directory containing just the raw payload files.
    pub payload_dir: PathBuf,
    /// Manifest describing every emitted artifact.
    pub manifest: PathBuf,
    /// Archives produced for distribution. Typically includes both ZIP and TAR.GZ variants.
    pub archives: Vec<PathBuf>,
    /// Manifest entries so downstream automation can inspect them without reading the JSON again.
    pub entries: Vec<ManifestEntry>,
    /// High level metadata stored alongside the manifest.
    pub metadata: Value,
    archive_stem: String,
}

impl BundleSummary {
    /// Recursively synchronises the bundle into a compatibility directory.
    ///
    /// Legacy automation expects the archived assets to mirror the npm bundle layout. We copy the
    /// entire bundle directory (manifest, payload, and archives) so the contract remains intact.
    pub fn sync_to<P: AsRef<Path>>(&self, destination_root: P) -> Result<PathBuf> {
        let destination_root = destination_root.as_ref();
        fs::create_dir_all(destination_root).with_context(|| {
            format!(
                "failed to create compatibility root at {}",
                destination_root.display()
            )
        })?;
        let bundle_destination = destination_root.join(&self.archive_stem);
        if bundle_destination.exists() {
            fs::remove_dir_all(&bundle_destination).with_context(|| {
                format!(
                    "failed to remove stale bundle at {} before sync",
                    bundle_destination.display()
                )
            })?;
        }
        copy_directory(&self.bundle_root, &bundle_destination)?;
        Ok(bundle_destination)
    }
}

impl ArtifactBundleBuilder {
    /// Creates a new bundle builder rooted at `bundle_root` and names archives using `archive_stem`.
    ///
    /// The constructor clears any previous payload so repeated runs (for example in CI) always emit a
    /// clean bundle. A `payload/` directory is created inside the bundle root where raw files are
    /// staged before manifests and archives are generated.
    pub fn new<P: AsRef<Path>>(bundle_root: P, archive_stem: impl Into<String>) -> Result<Self> {
        let bundle_root = bundle_root.as_ref().to_path_buf();
        if bundle_root.exists() {
            fs::remove_dir_all(&bundle_root).with_context(|| {
                format!(
                    "failed to clear existing bundle directory at {}",
                    bundle_root.display()
                )
            })?;
        }
        fs::create_dir_all(&bundle_root)?;
        let payload_dir = bundle_root.join("payload");
        fs::create_dir_all(&payload_dir)?;
        Ok(Self {
            root: bundle_root,
            payload_dir,
            archive_stem: archive_stem.into(),
            entries: Vec::new(),
        })
    }

    /// Adds a single file to the bundle and records manifest metadata.
    ///
    /// * `source` – path to the file that should be copied into the payload directory.
    /// * `relative_path` – path relative to the payload root where the file should be staged.
    /// * `kind` – human-readable classification (for example `material-theme-json`).
    /// * `media_type` – MIME-like descriptor so downstream tooling can route the asset appropriately.
    /// * `metadata` – any additional JSON blob that should accompany the manifest entry. This keeps the
    ///   manifest extensible without changing the schema.
    pub fn ingest_file<S: AsRef<Path>, R: AsRef<Path>, K: Into<String>, M: Into<String>>(
        &mut self,
        source: S,
        relative_path: R,
        kind: K,
        media_type: M,
        metadata: Value,
    ) -> Result<PathBuf> {
        let source = source.as_ref();
        let relative_path = relative_path.as_ref();
        let destination = self.payload_dir.join(relative_path);
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }
        let bytes = fs::read(source).with_context(|| {
            format!(
                "failed to read source file {} while preparing bundle",
                source.display()
            )
        })?;
        fs::write(&destination, &bytes).with_context(|| {
            format!(
                "failed to copy payload file into bundle at {}",
                destination.display()
            )
        })?;
        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        let checksum = format!("{:x}", hasher.finalize());
        let entry = ManifestEntry {
            relative_path: unix_string(relative_path),
            bytes: bytes.len() as u64,
            sha256: checksum,
            kind: kind.into(),
            media_type: media_type.into(),
            source: unix_string(source),
            metadata,
        };
        self.entries.push(entry);
        Ok(destination)
    }

    /// Recursively ingests a directory, mirroring its structure under the payload root.
    ///
    /// The entire directory tree is copied so the resulting archives remain deterministic even if the
    /// source location contains nested folders. Each file inherits the provided `kind` and
    /// `media_type`; callers can attach additional metadata via the closure.
    pub fn ingest_directory<S: AsRef<Path>, R: AsRef<Path>, F: Fn(&Path) -> Value>(
        &mut self,
        source_dir: S,
        relative_root: R,
        kind: &str,
        media_type: &str,
        metadata: F,
    ) -> Result<()> {
        let source_dir = source_dir.as_ref();
        let relative_root = relative_root.as_ref();
        for entry in WalkDir::new(source_dir).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let relative_path = entry
                    .path()
                    .strip_prefix(source_dir)
                    .map_err(|error| anyhow!(error))?;
                let destination_relative = relative_root.join(relative_path);
                let metadata_blob = metadata(entry.path());
                self.ingest_file(
                    entry.path(),
                    destination_relative,
                    kind.to_string(),
                    media_type.to_string(),
                    metadata_blob,
                )?;
            }
        }
        Ok(())
    }

    /// Finalises the bundle by writing the manifest and producing ZIP + TAR.GZ archives.
    ///
    /// The returned [`BundleSummary`] includes every emitted artifact so callers can surface
    /// machine-readable summaries to CI systems or copy the outputs elsewhere.
    pub fn finalize(self, metadata: Value) -> Result<BundleSummary> {
        let manifest_path = self
            .root
            .join(format!("{}.manifest.json", self.archive_stem));
        let manifest = BundleManifest {
            schema_version: "1".to_string(),
            generated_at: Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
            bundle: self.archive_stem.clone(),
            metadata: metadata.clone(),
            entries: self.entries.clone(),
        };
        let manifest_json = serde_json::to_string_pretty(&manifest)? + "\n";
        fs::write(&manifest_path, manifest_json).with_context(|| {
            format!(
                "failed to write bundle manifest at {}",
                manifest_path.display()
            )
        })?;

        let mut archives = Vec::new();
        let zip_path = self.root.join(format!("{}.zip", self.archive_stem));
        write_zip(&self.payload_dir, &zip_path)?;
        archives.push(zip_path);

        let tar_path = self.root.join(format!("{}.tar.gz", self.archive_stem));
        write_tar_gz(&self.payload_dir, &tar_path)?;
        archives.push(tar_path);

        Ok(BundleSummary {
            bundle_root: self.root,
            payload_dir: self.payload_dir,
            manifest: manifest_path,
            archives,
            entries: self.entries,
            metadata,
            archive_stem: self.archive_stem,
        })
    }
}

/// Internal manifest structure written alongside each bundle.
#[derive(Debug, Clone, Serialize)]
struct BundleManifest {
    schema_version: String,
    generated_at: String,
    bundle: String,
    metadata: Value,
    entries: Vec<ManifestEntry>,
}

/// Record describing a single asset inside a bundle.
#[derive(Debug, Clone, Serialize)]
pub struct ManifestEntry {
    /// Path relative to the payload root (`payload/`).
    pub relative_path: String,
    /// Size of the file in bytes.
    pub bytes: u64,
    /// SHA-256 checksum for integrity verification.
    pub sha256: String,
    /// High level classification for the asset (for example `icon-svg`).
    pub kind: String,
    /// Media type / MIME hint used by downstream automation.
    pub media_type: String,
    /// Original source path prior to staging.
    pub source: String,
    /// Additional metadata, typically referencing upstream npm package names or framework hints.
    pub metadata: Value,
}

impl fmt::Display for ManifestEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({} bytes, {})",
            self.relative_path, self.bytes, self.kind
        )
    }
}

fn write_zip(payload_dir: &Path, destination: &Path) -> Result<()> {
    let file = fs::File::create(destination)
        .with_context(|| format!("failed to create ZIP archive at {}", destination.display()))?;
    let mut zip = zip::ZipWriter::new(file);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    for entry in WalkDir::new(payload_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let relative = path
            .strip_prefix(payload_dir)
            .map_err(|error| anyhow!(error))?;
        if entry.file_type().is_dir() {
            if !relative.as_os_str().is_empty() {
                zip.add_directory(unix_string(relative), options)?;
            }
            continue;
        }
        zip.start_file(unix_string(relative), options)?;
        let bytes = fs::read(path)?;
        zip.write_all(&bytes)?;
    }
    zip.finish()?;
    Ok(())
}

fn write_tar_gz(payload_dir: &Path, destination: &Path) -> Result<()> {
    let file = fs::File::create(destination).with_context(|| {
        format!(
            "failed to create TAR.GZ archive at {}",
            destination.display()
        )
    })?;
    let encoder = GzEncoder::new(file, Compression::default());
    let mut tar = TarBuilder::new(encoder);
    tar.append_dir_all(".", payload_dir)?;
    tar.finish()?;
    Ok(())
}

fn unix_string(path: &Path) -> String {
    path.components()
        .map(|comp| comp.as_os_str().to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join("/")
}

fn copy_directory(source: &Path, destination: &Path) -> Result<()> {
    for entry in WalkDir::new(source).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let relative = path.strip_prefix(source).map_err(|error| anyhow!(error))?;
        let dest_path = destination.join(relative);
        if entry.file_type().is_dir() {
            fs::create_dir_all(&dest_path)?;
        } else {
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(path, &dest_path)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn builder_writes_manifest_and_archives() -> Result<()> {
        let temp = tempdir()?;
        let bundle_root = temp.path().join("bundle-test");
        let mut builder = ArtifactBundleBuilder::new(&bundle_root, "test")?;
        let input_dir = temp.path().join("inputs");
        fs::create_dir_all(&input_dir)?;
        let json_path = input_dir.join("theme.json");
        fs::write(&json_path, b"{}")?;
        builder.ingest_file(
            &json_path,
            "material/theme.json",
            "material-theme-json",
            "application/json",
            serde_json::json!({ "legacy": "@mui/material" }),
        )?;
        let summary = builder.finalize(serde_json::json!({ "bundle": "unit-test" }))?;
        assert!(summary.manifest.exists());
        assert_eq!(summary.entries.len(), 1);
        assert!(summary.archives.iter().all(|path| path.exists()));
        Ok(())
    }
}
