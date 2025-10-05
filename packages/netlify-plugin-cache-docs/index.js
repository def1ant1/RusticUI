/* eslint-disable no-console */
const path = require('path');
const fs = require('node:fs');

const CACHE_OUTPUT_FILE = 'cache-output.json';

function generateAbsolutePaths(context) {
  const { constants } = context;

  const workspaceRoot = path.dirname(constants.CONFIG_PATH);
  const targetRoot = path.join(workspaceRoot, 'target');
  const wasmArtifactsDir = path.join(targetRoot, 'wasm32-unknown-unknown');
  const apiDocsDir = path.join(targetRoot, 'doc');
  const deployOutputDir = path.join(targetRoot, 'deploy', 'docs');

  return { workspaceRoot, wasmArtifactsDir, apiDocsDir, deployOutputDir };
}

async function ensureDirectoryExists(directory) {
  await fs.promises.mkdir(directory, { recursive: true });
}

async function restoreCacheDirectory(utils, label, directory) {
  await ensureDirectoryExists(path.dirname(directory));
  console.log("[cache] restoring %s artifacts from '%s'", label, directory);
  const restored = await utils.cache.restore(directory);
  console.log("[cache] restore status for %s: %s", label, String(restored));
}

async function saveCacheDirectory(utils, label, directory) {
  if (!fs.existsSync(directory)) {
    console.log("[cache] skipping %s cache save because '%s' does not exist", label, directory);
    return;
  }

  console.log("[cache] persisting %s artifacts from '%s'", label, directory);
  const saved = await utils.cache.save(directory);
  console.log("[cache] save status for %s: %s", label, String(saved));
}

module.exports = {
  // Restore the `.next/cache` folder
  // based on: https://github.com/netlify/next-runtime/blob/733a0219e5413aa1eea790af48c745322dbce917/src/index.ts
  async onPreBuild(context) {
    const { constants, utils } = context;
    const { wasmArtifactsDir, apiDocsDir } = generateAbsolutePaths({ constants });

    await restoreCacheDirectory(utils, 'wasm32-unknown-unknown', wasmArtifactsDir);
    await restoreCacheDirectory(utils, 'cargo doc', apiDocsDir);
  },
  // On build, cache the `.next/cache` folder
  // based on: https://github.com/netlify/next-runtime/blob/733a0219e5413aa1eea790af48c745322dbce917/src/index.ts
  // This hook is called immediately after the build command is executed.
  async onBuild(context) {
    const { constants, utils } = context;
    const { wasmArtifactsDir, apiDocsDir } = generateAbsolutePaths({ constants });

    await saveCacheDirectory(utils, 'wasm32-unknown-unknown', wasmArtifactsDir);
    await saveCacheDirectory(utils, 'cargo doc', apiDocsDir);
  },
  // debug
  // based on: https://github.com/netlify-labs/netlify-plugin-debug-cache/blob/v1.0.3/index.js
  async onEnd({ constants, utils }) {
    const { PUBLISH_DIR } = constants;
    const cacheManifestFileName = CACHE_OUTPUT_FILE;
    const cacheManifestPath = path.join(PUBLISH_DIR, cacheManifestFileName);
    console.log('Saving cache file manifest for debugging...');
    const files = await utils.cache.list();
    await fs.promises.mkdir(PUBLISH_DIR, { recursive: true });
    await fs.promises.writeFile(cacheManifestPath, JSON.stringify(files, null, 2));
    console.log(`Cache file count: ${files.length}`);
    console.log(`Cache manifest saved to ${cacheManifestPath}`);
    console.log(`Please download the build files to inspect ${cacheManifestFileName}.`);
    console.log('Instructions => http://bit.ly/netlify-dl-cache');
  },
};
