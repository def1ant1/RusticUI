/**
 * Adapter Storybook discovery utilities.
 *
 * Our adapter crates ship Storybook builds across multiple frameworks (React,
 * Yew, Leptos, Sycamore, …). Maintaining a hard-coded list of every Storybook
 * would rot quickly, so this module provides a deterministic discovery
 * mechanism. The Playwright suite imports `loadAdapterStorybooks()` to find
 * built Storybook directories and the stories they expose.
 *
 * The discovery algorithm is intentionally conservative:
 *
 * 1. If the CI job or a developer provides `ADAPTER_STORYBOOK_MANIFEST`, the
 *    JSON file at that path wins. We lean on this hook when teams generate
 *    Storybook builds in a separate pipeline and upload them as artifacts.
 * 2. Otherwise we scan `examples/<adapter>/storybook-static`. The adapter examples ship
 *    their Storybook exports under that folder name to match Storybook's default
 *    `build-storybook` output. Only folders that contain an `index.html` are
 *    considered valid Storybooks.
 *
 * The output is a list of `AdapterStorybookTarget` entries. Each entry bundles
 * Story metadata so callers do not need to understand Storybook's `index.json`
 * or `stories.json` formats.
 */

import * as fs from 'node:fs/promises';
import * as path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

/** Shape written by Storybook 7's `index.json`. */
interface StorybookIndexEntry {
  id: string;
  name: string;
  title?: string;
  type?: string;
}

/** Normalised representation used by the Playwright harness. */
export interface AdapterStory {
  /** Story identifier consumed by Storybook's iframe router. */
  readonly id: string;
  /** Human-readable name surfaced in logs and failure output. */
  readonly name: string;
}

/** Raw configuration prior to expanding the story list. */
interface AdapterStorybookConfig {
  /** Stable identifier used for logging, caching, and screenshot folders. */
  readonly id: string;
  /** Absolute path to the Storybook `storybook-static` directory. */
  readonly staticDir: string;
  /** Optional subset of story IDs to include. */
  readonly includeIds?: readonly string[];
  /** Optional story IDs that should never be exercised. */
  readonly excludeIds?: readonly string[];
  /** Milliseconds to wait after navigation before taking a screenshot. */
  readonly settleTimeoutMs?: number;
}

/** Final resolved target with stories ready to be rendered. */
export interface AdapterStorybookTarget extends AdapterStorybookConfig {
  /** Ordered stories pulled from `index.json`/`stories.json`. */
  readonly stories: readonly AdapterStory[];
}

/** Convenience helper to check if a path exists. */
async function fileExists(candidate: string): Promise<boolean> {
  try {
    await fs.access(candidate);
    return true;
  } catch (error: unknown) {
    if ((error as NodeJS.ErrnoException).code === 'ENOENT') {
      return false;
    }
    throw error;
  }
}

/**
 * Load the manifest provided via `ADAPTER_STORYBOOK_MANIFEST`.
 *
 * The JSON format is `{ storybooks: AdapterStorybookConfig[] }`. Using a JSON
 * wrapper keeps the file extensible without breaking older pipelines.
 */
async function loadExplicitManifest(manifestPath: string): Promise<AdapterStorybookConfig[]> {
  const raw = await fs.readFile(manifestPath, 'utf8');
  const parsed = JSON.parse(raw) as { storybooks?: AdapterStorybookConfig[] };
  if (!parsed.storybooks?.length) {
    return [];
  }

  // Normalise any relative paths so downstream logging uses absolute paths.
  return parsed.storybooks.map((entry) => ({
    ...entry,
    staticDir: path.resolve(path.dirname(manifestPath), entry.staticDir),
  }));
}

/**
 * Scan `examples/<adapter>/storybook-static` for Storybook builds.
 *
 * Every adapter example drops its Storybook export under `storybook-static/` to
 * mirror Storybook's default CLI output. This keeps the discovery logic simple
 * and encourages teams to produce predictable build artefacts.
 */
async function autoDiscoverStorybooks(workspaceRoot: string): Promise<AdapterStorybookConfig[]> {
  const examplesDir = path.join(workspaceRoot, 'examples');
  const entries = await fs
    .readdir(examplesDir, { withFileTypes: true })
    .catch(() => []);

  const discovered: AdapterStorybookConfig[] = [];
  await Promise.all(
    entries.map(async (entry) => {
      if (!entry.isDirectory()) {
        return;
      }

      const staticDir = path.join(examplesDir, entry.name, 'storybook-static');
      const indexPath = path.join(staticDir, 'index.html');
      if (!(await fileExists(indexPath))) {
        return;
      }

      discovered.push({
        id: entry.name,
        staticDir,
        settleTimeoutMs: 100,
      });
    }),
  );

  return discovered.sort((a, b) => a.id.localeCompare(b.id));
}

/**
 * Parse a Storybook `index.json`/`stories.json` export and normalise the story
 * list.
 */
async function readStoryList(staticDir: string): Promise<AdapterStory[]> {
  const indexCandidates = ['index.json', 'stories.json'];
  for (const candidate of indexCandidates) {
    const filePath = path.join(staticDir, candidate);
    if (!(await fileExists(filePath))) {
      continue;
    }

    const raw = await fs.readFile(filePath, 'utf8');
    const parsed = JSON.parse(raw) as {
      entries?: Record<string, StorybookIndexEntry>;
      stories?: Record<string, StorybookIndexEntry>;
    };

    const stories = parsed.entries ?? parsed.stories;
    if (!stories) {
      continue;
    }

    return Object.values(stories)
      .filter((entry) => entry.type !== 'docs')
      .map((entry) => ({ id: entry.id, name: entry.name ?? entry.id }))
      .sort((a, b) => a.id.localeCompare(b.id));
  }

  return [];
}

/** Apply include/exclude filters so teams can focus on critical paths. */
function filterStories(config: AdapterStorybookConfig, stories: readonly AdapterStory[]): AdapterStory[] {
  const include = new Set(config.includeIds ?? stories.map((story) => story.id));
  const exclude = new Set(config.excludeIds ?? []);

  return stories.filter((story) => include.has(story.id) && !exclude.has(story.id));
}

/**
 * Primary entry point used by Playwright.
 */
export async function loadAdapterStorybooks(): Promise<AdapterStorybookTarget[]> {
  const currentDirectory = path.dirname(fileURLToPath(import.meta.url));
  const workspaceRoot = path.resolve(path.join(currentDirectory, '../../..'));
  const manifestPath = process.env.ADAPTER_STORYBOOK_MANIFEST;

  const baseConfigs = manifestPath
    ? await loadExplicitManifest(manifestPath)
    : await autoDiscoverStorybooks(workspaceRoot);

  const enriched: AdapterStorybookTarget[] = [];
  for (const config of baseConfigs) {
    const stories = filterStories(config, await readStoryList(config.staticDir));

    if (!stories.length) {
      console.warn(
        `Adapter storybook at ${config.staticDir} did not expose stories via index.json; skipping.`,
      );
      continue;
    }

    enriched.push({
      ...config,
      stories,
    });
  }

  return enriched;
}

/**
 * Helper exported for logging/debugging so local runs can inspect the resolved
 * manifest without stepping through the Playwright harness.
 */
export async function debugPrintManifest(): Promise<void> {
  const storybooks = await loadAdapterStorybooks();
  if (!storybooks.length) {
    console.warn('No adapter Storybooks discovered. Ensure `build-storybook` ran for the adapter examples.');
    return;
  }

  const manifest = storybooks.map((storybook) => ({
    id: storybook.id,
    staticDir: pathToFileURL(storybook.staticDir).toString(),
    stories: storybook.stories.map((story) => story.id),
  }));

  console.log(JSON.stringify({ storybooks: manifest }, null, 2));
}

if (process.env.DEBUG_ADAPTER_STORYBOOK_MANIFEST === '1') {
  debugPrintManifest().catch((error) => {
    console.error('Failed to resolve adapter Storybooks', error);
    process.exitCode = 1;
  });
}
