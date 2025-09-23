//! Maintenance utilities shared between the `update_icons` binary and its
//! companion tests.
//!
//! The module is intentionally verbose with documentation so future
//! contributors understand the data-flow at a glance. Automating the Material
//! icon ingestion pipeline is critical for keeping this repository
//! maintainable at scale, so every helper is carefully annotated with its
//! responsibilities and invariants.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};
use ureq::Response;
use zip::ZipArchive;

/// Upstream GitHub archive that houses the canonical Material Design icons.
///
/// The binary exposes a flag so alternate mirrors or internal artifact stores
/// can be supplied without having to modify the source code.
pub const DEFAULT_ZIP_URL: &str =
    "https://github.com/google/material-design-icons/archive/refs/heads/master.zip";

/// Canonical cache directory used by the updater to persist HTTP metadata.
const CACHE_FILE: &str = "metadata.json";

/// Wrapper struct describing how the icon updater should behave.
///
/// Keeping the configuration in a single struct keeps the binary ergonomic and
/// makes the logic easy to unit test – we can spin up temporary directories and
/// point the updater at them without touching real workspace files.
#[derive(Debug, Clone)]
pub struct UpdateOptions {
    /// Download endpoint for the ZIP archive.
    pub source_url: String,
    /// Skip conditional requests and force a full refresh if set.
    pub force_refresh: bool,
    /// Directory backing the metadata cache.
    pub cache_dir: PathBuf,
    /// Destination directory that stores materialized SVGs.
    pub icon_dir: PathBuf,
    /// Path to the `Cargo.toml` manifest that should receive regenerated
    /// feature listings.
    pub manifest_path: PathBuf,
}

impl Default for UpdateOptions {
    fn default() -> Self {
        // `CARGO_MANIFEST_DIR` resolves to `crates/mui-icons-material`. Walking up
        // two ancestors yields the workspace root, which hosts the shared target
        // directory. We intentionally panic if this expectation ever breaks so
        // contributors notice the misconfiguration immediately.
        let crate_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let workspace_root = crate_dir
            .ancestors()
            .nth(2)
            .expect("mui-icons-material lives two levels below the workspace root")
            .to_path_buf();
        Self {
            source_url: DEFAULT_ZIP_URL.to_string(),
            force_refresh: false,
            cache_dir: workspace_root.join("target/.icon-cache"),
            icon_dir: crate_dir.join("material-icons"),
            manifest_path: crate_dir.join("Cargo.toml"),
        }
    }
}

/// Signals whether the update process reused existing assets or performed a
/// full refresh.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateOutcome {
    /// Assets on disk were reused and no files were modified.
    Reused { reason: UpdateReuseReason },
    /// The archive introduced changes and the icons/manifest were rewritten.
    Updated { installed: usize },
}

/// Specific reuse reasons exported as a separate enum so automation can report
/// the underlying cause with fidelity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateReuseReason {
    /// The HTTP server returned `304 Not Modified` based on the cached ETag or
    /// Last-Modified header, so no archive download was required.
    HttpNotModified,
    /// A new archive was downloaded but its contents matched the existing icon
    /// set byte-for-byte, so there was nothing to write.
    ChecksumMatch,
}

/// Internal metadata persisted in the cache directory. Storing the `archive_hash`
/// allows us to debug unexpected server behaviour (e.g. mismatched responses) by
/// examining the last known digest.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
struct CacheMetadata {
    etag: Option<String>,
    last_modified: Option<String>,
    archive_hash: Option<String>,
}

impl CacheMetadata {
    fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let contents = fs::read_to_string(path)
            .with_context(|| format!("failed to read cache metadata from {}", path.display()))?;
        let meta = serde_json::from_str(&contents).with_context(|| {
            format!("failed to deserialize cache metadata at {}", path.display())
        })?;
        Ok(meta)
    }

    fn store(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!(
                    "failed to create cache directory {} when persisting icon metadata",
                    parent.display()
                )
            })?;
        }
        let payload = serde_json::to_string_pretty(self)
            .context("failed to serialize icon cache metadata to JSON")?;
        fs::write(path, payload)
            .with_context(|| format!("failed to write cache metadata to {}", path.display()))?;
        Ok(())
    }
}

/// Abstraction over the HTTP client so tests can inject deterministic
/// responses. The implementation used by the binary simply wraps `ureq`.
pub trait Fetcher {
    fn fetch(&self, url: &str, headers: &[(String, String)]) -> Result<FetchResponse>;
}

/// Lightweight response wrapper returned by [`Fetcher::fetch`].
#[derive(Debug, Clone)]
pub struct FetchResponse {
    pub status: u16,
    pub body: Vec<u8>,
    pub etag: Option<String>,
    pub last_modified: Option<String>,
}

/// Production HTTP client that speaks over TLS using `ureq`.
#[derive(Default)]
pub struct HttpFetcher;

impl Fetcher for HttpFetcher {
    fn fetch(&self, url: &str, headers: &[(String, String)]) -> Result<FetchResponse> {
        let mut request = ureq::get(url);
        for (name, value) in headers {
            request = request.set(name, value);
        }
        let response = request
            .call()
            .with_context(|| format!("failed to download icon archive from {url}"))?;
        parse_response(response, url)
    }
}

fn parse_response(response: Response, url: &str) -> Result<FetchResponse> {
    let status = response.status();
    let etag = response.header("ETag").map(|value| value.to_string());
    let last_modified = response
        .header("Last-Modified")
        .map(|value| value.to_string());

    if status == 304 {
        return Ok(FetchResponse {
            status,
            body: Vec::new(),
            etag,
            last_modified,
        });
    }

    if !(200..300).contains(&status) {
        return Err(anyhow!(
            "unexpected status code {status} when downloading Material icons from {url}"
        ));
    }

    let mut bytes = Vec::new();
    response
        .into_reader()
        .read_to_end(&mut bytes)
        .context("failed to read icon archive body into memory")?;

    Ok(FetchResponse {
        status,
        body: bytes,
        etag,
        last_modified,
    })
}

/// Entry point used by the binary and tests. Handles downloading, caching and
/// manifest regeneration. The function is intentionally side-effect free until
/// it knows the archive is newer than what is already on disk.
pub fn run_update<F: Fetcher>(fetcher: &F, options: &UpdateOptions) -> Result<UpdateOutcome> {
    let cache_file = options.cache_dir.join(CACHE_FILE);
    let mut metadata = CacheMetadata::load(&cache_file)?;

    let mut headers = Vec::new();
    if !options.force_refresh {
        if let Some(tag) = &metadata.etag {
            headers.push(("If-None-Match".to_string(), tag.clone()));
        }
        if let Some(modified) = &metadata.last_modified {
            headers.push(("If-Modified-Since".to_string(), modified.clone()));
        }
    }

    let response = fetcher.fetch(&options.source_url, &headers)?;
    if response.status == 304 {
        // Persist metadata so downstream tooling can inspect the cache even if no
        // download was required.
        metadata.store(&cache_file)?;
        return Ok(UpdateOutcome::Reused {
            reason: UpdateReuseReason::HttpNotModified,
        });
    }

    let archive_hash = compute_sha256(&response.body);
    metadata.etag = response.etag;
    metadata.last_modified = response.last_modified;
    metadata.archive_hash = Some(archive_hash);

    let extracted = extract_icons(&response.body)?;
    let existing = load_existing_icons(&options.icon_dir)?;

    let icons_identical = !options.force_refresh && extracted == existing;
    if icons_identical {
        metadata.store(&cache_file)?;
        return Ok(UpdateOutcome::Reused {
            reason: UpdateReuseReason::ChecksumMatch,
        });
    }

    write_icons(&options.icon_dir, &extracted)?;
    let icons: Vec<String> = extracted
        .keys()
        .map(|name| name.trim_end_matches(".svg").to_string())
        .collect();
    update_manifest_features(&options.manifest_path, &icons)?;
    metadata.store(&cache_file)?;

    Ok(UpdateOutcome::Updated {
        installed: icons.len(),
    })
}

fn compute_sha256(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    format!("{:x}", digest)
}

fn extract_icons(bytes: &[u8]) -> Result<BTreeMap<String, Vec<u8>>> {
    let reader = Cursor::new(bytes);
    let mut archive =
        ZipArchive::new(reader).context("failed to parse icon archive as a ZIP file")?;
    let mut icons = BTreeMap::new();

    for index in 0..archive.len() {
        let mut file = archive
            .by_index(index)
            .with_context(|| format!("failed to read entry {index} from icon archive"))?;
        let name = file.name().to_string();
        if !name.contains("materialicons/")
            || !name.contains("/svg/")
            || !name.ends_with("24px.svg")
        {
            continue;
        }
        let base = name
            .rsplit('/')
            .next()
            .expect("zip entries always include a filename")
            .to_string();
        let mut data = Vec::new();
        file.read_to_end(&mut data)
            .with_context(|| format!("failed to read {name} from the icon archive"))?;
        icons.insert(base, data);
    }

    Ok(icons)
}

fn load_existing_icons(dir: &Path) -> Result<BTreeMap<String, Vec<u8>>> {
    let mut icons = BTreeMap::new();
    if !dir.exists() {
        return Ok(icons);
    }

    for entry in fs::read_dir(dir)
        .with_context(|| format!("failed to iterate icon directory at {}", dir.display()))?
    {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }
        let name = entry
            .file_name()
            .into_string()
            .map_err(|_| anyhow!("icon filename contained invalid UTF-8"))?;
        if !name.ends_with(".svg") {
            continue;
        }
        let bytes = fs::read(entry.path())
            .with_context(|| format!("failed to read existing icon {}", entry.path().display()))?;
        icons.insert(name, bytes);
    }

    Ok(icons)
}

fn write_icons(dir: &Path, icons: &BTreeMap<String, Vec<u8>>) -> Result<()> {
    if dir.exists() {
        fs::remove_dir_all(dir).with_context(|| {
            format!(
                "failed to clear out existing icons directory {}",
                dir.display()
            )
        })?;
    }
    fs::create_dir_all(dir)
        .with_context(|| format!("failed to create icons directory {}", dir.display()))?;

    for (name, data) in icons {
        let path = dir.join(name);
        fs::write(&path, data)
            .with_context(|| format!("failed to write extracted icon {}", path.display()))?;
    }

    Ok(())
}

fn update_manifest_features(manifest_path: &Path, icons: &[String]) -> Result<()> {
    const START: &str = "# BEGIN ICON FEATURES -- auto-generated, do not edit by hand.";
    const END: &str = "# END ICON FEATURES";

    let manifest = fs::read_to_string(manifest_path).with_context(|| {
        format!(
            "failed to read manifest at {} when regenerating icon features",
            manifest_path.display()
        )
    })?;

    let start = manifest
        .find(START)
        .ok_or_else(|| anyhow!("start marker not found in {}", manifest_path.display()))?;
    let end = manifest
        .find(END)
        .ok_or_else(|| anyhow!("end marker not found in {}", manifest_path.display()))?;

    if start >= end {
        return Err(anyhow!(
            "icon feature markers are in an unexpected order within {}",
            manifest_path.display()
        ));
    }

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

    fs::write(manifest_path, new_manifest).with_context(|| {
        format!(
            "failed to rewrite manifest at {} with refreshed icon features",
            manifest_path.display()
        )
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::io::Write;
    use tempfile::TempDir;

    struct MockFetcher {
        responses: RefCell<Vec<FetchResponse>>,
        recorded_headers: RefCell<Vec<Vec<(String, String)>>>,
    }

    impl MockFetcher {
        fn new(responses: Vec<FetchResponse>) -> Self {
            Self {
                responses: RefCell::new(responses),
                recorded_headers: RefCell::new(Vec::new()),
            }
        }

        fn last_headers(&self) -> Vec<(String, String)> {
            self.recorded_headers
                .borrow()
                .last()
                .cloned()
                .unwrap_or_default()
        }
    }

    impl Fetcher for MockFetcher {
        fn fetch(&self, _url: &str, headers: &[(String, String)]) -> Result<FetchResponse> {
            self.recorded_headers.borrow_mut().push(headers.to_vec());
            self.responses
                .borrow_mut()
                .pop()
                .ok_or_else(|| anyhow!("mock fetcher exhausted"))
        }
    }

    fn base_manifest() -> String {
        r#"[features]
# BEGIN ICON FEATURES -- auto-generated, do not edit by hand.
placeholder
# END ICON FEATURES
"#
        .to_string()
    }

    fn write_manifest(dir: &TempDir) -> PathBuf {
        let path = dir.path().join("Cargo.toml");
        fs::write(&path, base_manifest()).expect("manifest write succeeds");
        path
    }

    fn write_icons_fixture(dir: &TempDir, icons: &[(&str, &str)]) -> PathBuf {
        let icon_dir = dir.path().join("icons");
        fs::create_dir_all(&icon_dir).unwrap();
        for (name, data) in icons {
            fs::write(icon_dir.join(name), data).unwrap();
        }
        icon_dir
    }

    fn zip_bytes(entries: &[(&str, &str)]) -> Vec<u8> {
        let cursor = Cursor::new(Vec::new());
        let mut zip = zip::ZipWriter::new(cursor);
        let options = zip::write::FileOptions::default();
        for &(name, data) in entries {
            zip.start_file(name, options).unwrap();
            zip.write_all(data.as_bytes()).unwrap();
        }
        zip.finish().unwrap().into_inner()
    }

    #[test]
    fn cache_hit_skips_download() {
        let temp = TempDir::new().unwrap();
        let manifest = write_manifest(&temp);
        let icons_dir = write_icons_fixture(&temp, &[("10k_24px.svg", "<svg />")]);
        let cache_dir = temp.path().join("cache");
        fs::create_dir_all(&cache_dir).unwrap();
        CacheMetadata {
            etag: Some("abc".to_string()),
            last_modified: Some("Mon, 01 Jan 2024 00:00:00 GMT".to_string()),
            archive_hash: Some("deadbeef".to_string()),
        }
        .store(&cache_dir.join(CACHE_FILE))
        .unwrap();

        let fetcher = MockFetcher::new(vec![FetchResponse {
            status: 304,
            body: Vec::new(),
            etag: None,
            last_modified: None,
        }]);

        let mut options = UpdateOptions::default();
        options.cache_dir = cache_dir;
        options.icon_dir = icons_dir;
        options.manifest_path = manifest;
        options.source_url = "https://example.test/icons.zip".to_string();

        let outcome = run_update(&fetcher, &options).unwrap();
        assert_eq!(
            outcome,
            UpdateOutcome::Reused {
                reason: UpdateReuseReason::HttpNotModified
            }
        );
        let headers = fetcher.last_headers();
        assert!(headers.iter().any(|(name, _)| name == "If-None-Match"));
        assert!(headers.iter().any(|(name, _)| name == "If-Modified-Since"));

        let manifest_contents = fs::read_to_string(&options.manifest_path).unwrap();
        assert!(manifest_contents.contains("placeholder"));
    }

    #[test]
    fn identical_archive_reuses_existing_assets() {
        let temp = TempDir::new().unwrap();
        let manifest = write_manifest(&temp);
        let icons_dir = write_icons_fixture(&temp, &[("10k_24px.svg", "<svg />")]);
        let cache_dir = temp.path().join("cache");

        let archive = zip_bytes(&[(
            "material-design-icons/src/materialicons/svg/production/10k_24px.svg",
            "<svg />",
        )]);
        let fetcher = MockFetcher::new(vec![FetchResponse {
            status: 200,
            body: archive,
            etag: Some("etag".to_string()),
            last_modified: Some("Tue, 02 Jan 2024 00:00:00 GMT".to_string()),
        }]);

        let mut options = UpdateOptions::default();
        options.cache_dir = cache_dir.clone();
        options.icon_dir = icons_dir.clone();
        options.manifest_path = manifest.clone();
        options.source_url = "https://example.test/icons.zip".to_string();

        let outcome = run_update(&fetcher, &options).unwrap();
        assert_eq!(
            outcome,
            UpdateOutcome::Reused {
                reason: UpdateReuseReason::ChecksumMatch
            }
        );

        let manifest_contents = fs::read_to_string(&options.manifest_path).unwrap();
        assert!(manifest_contents.contains("placeholder"));

        let metadata = CacheMetadata::load(&cache_dir.join(CACHE_FILE)).unwrap();
        assert_eq!(metadata.etag.as_deref(), Some("etag"));
    }

    #[test]
    fn archive_changes_trigger_full_refresh() {
        let temp = TempDir::new().unwrap();
        let manifest = write_manifest(&temp);
        let icons_dir = write_icons_fixture(&temp, &[("10k_24px.svg", "<svg />")]);
        let cache_dir = temp.path().join("cache");

        let archive = zip_bytes(&[
            (
                "material-design-icons/src/materialicons/svg/production/10k_24px.svg",
                "<svg>updated</svg>",
            ),
            (
                "material-design-icons/src/materialicons/svg/production/20mp_24px.svg",
                "<svg>new</svg>",
            ),
        ]);
        let fetcher = MockFetcher::new(vec![FetchResponse {
            status: 200,
            body: archive,
            etag: Some("fresh".to_string()),
            last_modified: Some("Wed, 03 Jan 2024 00:00:00 GMT".to_string()),
        }]);

        let mut options = UpdateOptions::default();
        options.cache_dir = cache_dir.clone();
        options.icon_dir = icons_dir.clone();
        options.manifest_path = manifest.clone();
        options.source_url = "https://example.test/icons.zip".to_string();

        let outcome = run_update(&fetcher, &options).unwrap();
        assert_eq!(outcome, UpdateOutcome::Updated { installed: 2 });

        let refreshed = fs::read_dir(&options.icon_dir)
            .unwrap()
            .map(|entry| entry.unwrap().file_name().into_string().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(refreshed.len(), 2);
        assert!(refreshed.contains(&"10k_24px.svg".to_string()));
        assert!(refreshed.contains(&"20mp_24px.svg".to_string()));

        let manifest_contents = fs::read_to_string(&options.manifest_path).unwrap();
        assert!(manifest_contents.contains("icon-10k_24px"));
        assert!(manifest_contents.contains("icon-20mp_24px"));

        let metadata = CacheMetadata::load(&cache_dir.join(CACHE_FILE)).unwrap();
        assert_eq!(metadata.etag.as_deref(), Some("fresh"));
        assert!(metadata
            .archive_hash
            .as_ref()
            .map(|hash| !hash.is_empty())
            .unwrap_or(false));
    }
}
