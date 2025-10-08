import { test } from '@playwright/test';
import * as fs from 'node:fs/promises';
import * as path from 'node:path';
import { fileURLToPath } from 'node:url';
import { loadAdapterStorybooks, AdapterStorybookTarget } from './manifest';
import { startStaticServer, StaticServer } from './staticServer';
import { VisualSummaryWriter } from './summary';

const currentDirectory = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(path.join(currentDirectory, '../../..'));
const screenshotRoot = path.join(workspaceRoot, 'test-results', 'visual-regressions', 'adapters');
const summaryPath = path.join(workspaceRoot, 'test-results', 'visual-regressions-adapters.json');

const summary = new VisualSummaryWriter(summaryPath);
summary.addNote('Adapter Storybook snapshots captured via test/regressions/adapters.');
summary.addNote('Chromatic publishes the same builds for external reviewers.');

test.describe.configure({ mode: 'serial' });

test.beforeAll(async () => {
  await fs.rm(screenshotRoot, { recursive: true, force: true });
  await fs.mkdir(screenshotRoot, { recursive: true });
});

test.afterAll(async () => {
  await summary.flush();
});

function sanitizeStoryId(storyId: string): string {
  return storyId.replace(/[^a-zA-Z0-9-_]+/g, '-');
}

async function exerciseStorybook(target: AdapterStorybookTarget): Promise<void> {
  let server: StaticServer | undefined;
  let baseUrl: string | undefined;
  const storybookScreenshotDir = path.join(screenshotRoot, target.id);

  test.beforeAll(async () => {
    server = await startStaticServer(target.staticDir);
    baseUrl = server.baseUrl;
    await fs.mkdir(storybookScreenshotDir, { recursive: true });
    summary.addNote(`Storybook ${target.id} served from ${target.staticDir}.`);
  });

  test.afterAll(async () => {
    if (server) {
      await server.stop();
    }
  });

  for (const story of target.stories) {
    test(`${target.id} – ${story.id}`, async ({ page }, testInfo) => {
      if (!baseUrl) {
        test.skip(true, 'Static server not initialised.');
        summary.recordSkipped();
        return;
      }

      const storyUrl = `${baseUrl}/iframe.html?id=${encodeURIComponent(story.id)}&viewMode=story`;
      await page.goto(storyUrl, { waitUntil: 'networkidle' });

      const settleTimeout = target.settleTimeoutMs ?? 100;
      if (settleTimeout > 0) {
        await page.waitForTimeout(settleTimeout);
      }

      const screenshotPath = path.join(storybookScreenshotDir, `${sanitizeStoryId(story.id)}.png`);
      await page.screenshot({
        path: screenshotPath,
        animations: 'disabled',
        fullPage: true,
      });

      summary.recordSnapshot();
      testInfo.attach(`${story.id}-screenshot`, {
        path: screenshotPath,
        contentType: 'image/png',
      });
    });
  }
}

async function registerStorybookSuites(): Promise<void> {
  const storybooks = await loadAdapterStorybooks();

  if (!storybooks.length) {
    test.describe('adapter storybooks', () => {
      test('no storybooks discovered', () => {
        test.skip(true, 'Run build-storybook for the adapter examples before executing Playwright.');
      });
    });
    return;
  }

  for (const storybook of storybooks) {
    test.describe(storybook.id, () => {
      void exerciseStorybook(storybook);
    });
  }
}

await registerStorybookSuites();
