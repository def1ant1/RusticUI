import { defineConfig, devices } from '@playwright/test';
import * as path from 'node:path';
import { fileURLToPath } from 'node:url';

const currentDirectory = path.dirname(fileURLToPath(import.meta.url));
const defaultHost = process.env.RUSTIC_UI_ACCESSIBILITY_HOST ?? '127.0.0.1';
const defaultPort = Number(process.env.RUSTIC_UI_ACCESSIBILITY_PORT ?? '4321');
const baseURL =
  process.env.RUSTIC_UI_ACCESSIBILITY_BASE_URL ?? `http://${defaultHost}:${defaultPort}`;
const resultsDir =
  process.env.RUSTIC_UI_ACCESSIBILITY_RESULTS_DIR ??
  path.join(currentDirectory, '..', '..', 'test-results', 'accessibility');
const shouldLaunchWebServer =
  process.env.RUSTIC_UI_ACCESSIBILITY_SKIP_WEB_SERVER === '1' ? false : true;
const webServerCommand =
  process.env.RUSTIC_UI_ACCESSIBILITY_WEB_COMMAND ??
  `pnpm --dir docs run dev -- --hostname ${defaultHost} --port ${defaultPort}`;
const webServerTimeout = Number(
  process.env.RUSTIC_UI_ACCESSIBILITY_WEB_TIMEOUT ?? `${4 * 60 * 1000}`,
);

export default defineConfig({
  testDir: currentDirectory,
  testMatch: 'accessibility.spec.ts',
  fullyParallel: false,
  workers: Number(process.env.PLAYWRIGHT_WORKERS ?? (process.env.CI ? '2' : '1')),
  timeout: Number(process.env.RUSTIC_UI_ACCESSIBILITY_TEST_TIMEOUT ?? `${3 * 60 * 1000}`),
  expect: { timeout: 30_000 },
  reporter: [
    ['list'],
    ['json', { outputFile: path.join(resultsDir, 'summary.json') }],
    ['html', { outputFolder: path.join(resultsDir, 'html'), open: 'never' }],
  ],
  outputDir: resultsDir,
  use: {
    baseURL,
    headless: true,
    trace: 'retain-on-failure',
    video: 'retain-on-failure',
    screenshot: 'only-on-failure',
  },
  projects: [
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'], viewport: { width: 1440, height: 900 } },
    },
  ],
  webServer: shouldLaunchWebServer
    ? {
        command: webServerCommand,
        url: baseURL,
        reuseExistingServer: process.env.CI ? false : true,
        timeout: webServerTimeout,
        stdout: 'pipe',
        stderr: 'pipe',
      }
    : undefined,
});
