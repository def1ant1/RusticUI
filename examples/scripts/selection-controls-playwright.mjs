#!/usr/bin/env node
/**
 * Central Playwright harness for the selection control demos.
 *
 * The script orchestrates the lifecycle for every framework participating in the
 * selection control showcase:
 *   1. Spawn the shared shell helper in `serve` mode to provision toolchains and
 *      launch the appropriate dev server.
 *   2. Wait for the HTTP endpoint to respond before handing control to
 *      Playwright so automation never races the startup sequence.
 *   3. Assert that the framework exposes the canonical `data-automation-id`
 *      hooks and emit readable diagnostics if any selector is missing.
 *
 * Running Playwright programmatically keeps CI pipelines, `npm` scripts, and the
 * Rust `xtask` binary perfectly aligned.  The Node process streams server logs to
 * STDOUT so analytics breadcrumbs remain visible during debugging, mirroring the
 * behaviour enterprise operators expect from long-lived smoke tests.
 */
import { spawn } from 'node:child_process';
import { once } from 'node:events';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { setTimeout as delay } from 'node:timers/promises';
import { chromium } from 'playwright';
import { execFileSync } from 'node:child_process';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const repoRoot = path.resolve(__dirname, '..', '..');
const smokeHelper = path.resolve(__dirname, 'selection-controls-smoke.sh');

/** Framework-specific configuration. */
const FRAMEWORKS = {
  dioxus: {
    port: 4701,
    url: 'http://127.0.0.1',
    path: '/',
    automationKeys: ['checkbox', 'switch', 'radio', 'telemetry-log']
  },
  sycamore: {
    port: 4702,
    url: 'http://127.0.0.1',
    path: '/',
    automationKeys: ['checkbox', 'switch', 'radio', 'telemetry-log']
  },
  yew: {
    port: 4703,
    url: 'http://127.0.0.1',
    path: '/',
    automationKeys: ['checkbox', 'switch', 'radio', 'telemetry-log']
  },
  react: {
    port: 4704,
    url: 'http://127.0.0.1',
    path: '/',
    automationKeys: ['checkbox', 'switch', 'telemetry-log']
  }
};

const AUTOMATION_LOOKUP = (() => {
  const output = execFileSync(smokeHelper, ['--list-automation', '--format', 'json'], {
    cwd: repoRoot,
    encoding: 'utf8'
  }).trim();
  const ids = JSON.parse(output);
  const mapping = new Map();
  for (const id of ids) {
    if (id.endsWith('.checkbox')) {
      mapping.set('checkbox', id);
    } else if (id.endsWith('.switch')) {
      mapping.set('switch', id);
    } else if (id.endsWith('.radio')) {
      mapping.set('radio', id);
    } else if (id.endsWith('.telemetry-log')) {
      mapping.set('telemetry-log', id);
    }
  }
  return mapping;
})();

function parseArgs(argv) {
  const args = { framework: 'all' };
  const tokens = [...argv];
  while (tokens.length > 0) {
    const current = tokens.shift();
    if (current === '--framework') {
      const value = tokens.shift();
      if (!value) {
        throw new Error('--framework expects a value');
      }
      args.framework = value.toLowerCase();
    } else if (current === '--help' || current === '-h') {
      printUsage();
      process.exit(0);
    } else {
      throw new Error(`Unknown argument: ${current}`);
    }
  }
  return args;
}

function printUsage() {
  console.log(`Usage: node selection-controls-playwright.mjs [--framework <name|all>]\n\n` +
    `Frameworks:\n  ${Object.keys(FRAMEWORKS).join(', ')}\n` +
    `\nExamples:\n  node selection-controls-playwright.mjs\n  node selection-controls-playwright.mjs --framework react`);
}

async function waitForServer(url, timeoutMs = 120_000) {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    try {
      const response = await fetch(url, { method: 'GET' });
      if (response.ok) {
        return;
      }
    } catch (error) {
      // Ignore connection failures until timeout expires.
    }
    await delay(250);
  }
  throw new Error(`Server did not respond at ${url} within ${timeoutMs}ms`);
}

async function runFramework(name) {
  if (!FRAMEWORKS[name]) {
    throw new Error(`Unknown framework '${name}'. Supported: ${Object.keys(FRAMEWORKS).join(', ')}`);
  }
  const { port, url, path: urlPath, automationKeys } = FRAMEWORKS[name];
  const server = spawn(smokeHelper, [name, '--mode', 'serve', '--port', String(port)], {
    cwd: repoRoot,
    stdio: ['ignore', 'pipe', 'pipe']
  });

  server.stdout.on('data', (chunk) => {
    process.stdout.write(`[${name}][server] ${chunk}`);
  });
  server.stderr.on('data', (chunk) => {
    process.stderr.write(`[${name}][server] ${chunk}`);
  });

  const targetUrl = `${url}:${port}${urlPath}`;
  try {
    await waitForServer(targetUrl);

    const browser = await chromium.launch();
    try {
      const page = await browser.newPage();
      await page.goto(targetUrl, { waitUntil: 'networkidle' });

      for (const key of automationKeys) {
        const selector = AUTOMATION_LOOKUP.get(key);
        if (!selector) {
          throw new Error(`No automation ID registered for key '${key}'.`);
        }
        const locator = page.locator(`[data-automation-id="${selector}"]`);
        const count = await locator.count();
        if (count === 0) {
          throw new Error(`Framework '${name}' is missing automation selector '${selector}'.`);
        }
      }

      console.log(`[selection-controls][${name}] Verified automation selectors at ${targetUrl}`);
    } finally {
      await browser.close();
    }
  } finally {
    server.kill('SIGINT');
    await once(server, 'exit');
  }
}

async function main() {
  const args = parseArgs(process.argv.slice(2));
  const names = args.framework === 'all' ? Object.keys(FRAMEWORKS) : [args.framework];
  for (const name of names) {
    await runFramework(name);
  }
}

main().catch((error) => {
  console.error(`[selection-controls][playwright] ${error instanceof Error ? error.stack : error}`);
  process.exitCode = 1;
});
