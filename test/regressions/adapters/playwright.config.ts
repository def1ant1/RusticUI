import { defineConfig } from '@playwright/test';
import * as path from 'node:path';
import { fileURLToPath } from 'node:url';

const currentDirectory = path.dirname(fileURLToPath(import.meta.url));

export default defineConfig({
  testDir: currentDirectory,
  testMatch: 'adapter-visual-regressions.spec.ts',
  /* Running serially keeps resource usage predictable and avoids launching more
   * than one static server at a time. */
  fullyParallel: false,
  workers: 1,
  timeout: 120_000,
  reporter: [['list']],
  use: {
    viewport: { width: 1280, height: 720 },
    colorScheme: 'light',
    javaScriptEnabled: true,
    ignoreHTTPSErrors: true,
    trace: 'on-first-retry',
  },
  outputDir: path.join(currentDirectory, '../../..', 'test-results', 'playwright-adapters'),
});
