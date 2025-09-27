import path from 'path';
import fs from 'node:fs';

/**
 * Centralizes how the docs pipeline resolves source roots now that
 * the Rust toolchain is the canonical source of truth. Automation can
 * flip feature flags (via env vars) to prefer or require Rust outputs
 * without forcing local contributors to understand the entire matrix.
 */
const LEGACY_ARCHIVE_ROOT = path.join(process.cwd(), 'archives', 'mui-packages');

const rustMetadataRootEnv = process.env.RUSTIC_UI_RUST_METADATA_ROOT;
const rustMetadataRoot = rustMetadataRootEnv
  ? path.resolve(process.cwd(), rustMetadataRootEnv)
  : null;

const rustAuthoritativeFlag = process.env.RUSTIC_UI_DOCS_RUST_AUTHORITATIVE;
const preferRustFlag = process.env.RUSTIC_UI_DOCS_PREFER_RUST;

const rustAuthoritative = rustAuthoritativeFlag === 'true' || rustAuthoritativeFlag === '1';
const preferRust = rustAuthoritative || preferRustFlag === 'true' || preferRustFlag === '1';

function resolveRustProjectRoot(packageSlug) {
  if (!rustMetadataRoot) {
    return null;
  }

  const candidate = path.join(rustMetadataRoot, packageSlug);
  try {
    const stats = fs.statSync(candidate);
    if (stats.isDirectory()) {
      return candidate;
    }
  } catch (error) {
    // Silently ignore missing folders – the caller decides how to react.
  }

  return null;
}

export function resolvePackageSourceRoot(packageSlug) {
  const rustCandidate = resolveRustProjectRoot(packageSlug);

  if (rustCandidate && preferRust) {
    return rustCandidate;
  }

  if (rustAuthoritative && !rustCandidate) {
    console.warn(
      `⚠️  RUSTIC_UI_DOCS_RUST_AUTHORITATIVE requested but no Rust metadata for "${packageSlug}" was found under ${rustMetadataRoot}. Falling back to archived JS sources to avoid breaking local flows.`,
    );
  }

  return path.join(LEGACY_ARCHIVE_ROOT, packageSlug);
}

export const rustDocFlags = {
  legacyArchiveRoot: LEGACY_ARCHIVE_ROOT,
  rustMetadataRoot,
  preferRust,
  rustAuthoritative,
  /**
   * When true, automation is expected to source data from Rust-generated
   * JSON payloads and should skip touching the archived JavaScript packages.
   */
  shouldSkipArchives: rustAuthoritative && !!rustMetadataRoot,
};

export function describeSourceFor(packageSlug) {
  const archivePath = path.join(LEGACY_ARCHIVE_ROOT, packageSlug);
  const rustPath = resolveRustProjectRoot(packageSlug);
  return {
    archivePath,
    rustPath,
    activePath: resolvePackageSourceRoot(packageSlug),
  };
}
