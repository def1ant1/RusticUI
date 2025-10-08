/**
 * Chromatic runner for adapter Storybooks.
 *
 * CI calls this script after Playwright finishes capturing local screenshots. It
 * publishes the same Storybook builds to Chromatic so designers can review diffs
 * without pulling artifacts. The script deliberately shells out to the Chromatic
 * CLI rather than re-implementing the protocol – we merely standardise the
 * command-line arguments and environment variables.
 */

import { spawn } from 'node:child_process';
import * as fs from 'node:fs/promises';
import * as path from 'node:path';
import { fileURLToPath } from 'node:url';
import process from 'node:process';
import { loadAdapterStorybooks } from './manifest';

const currentDirectory = path.dirname(fileURLToPath(import.meta.url));
const packageDir = currentDirectory;
const workspaceRoot = path.resolve(path.join(packageDir, '..', '..', '..'));
const chromaticBin = path.join(packageDir, 'node_modules', '.bin', 'chromatic');

function resolveProjectToken(storybookId: string): string | undefined {
  const explicitKey = `CHROMATIC_PROJECT_TOKEN_${storybookId.replace(/[^A-Z0-9]/gi, '_').toUpperCase()}`;
  return process.env[explicitKey] ?? process.env.CHROMATIC_PROJECT_TOKEN;
}

async function runChromatic(args: readonly string[]): Promise<void> {
  try {
    await fs.access(chromaticBin);
  } catch (error) {
    throw new Error(
      `Chromatic CLI not installed at ${chromaticBin}. Did you run "pnpm --dir test/regressions/adapters install"?`,
    );
  }

  await new Promise<void>((resolve, reject) => {
    const child = spawn(chromaticBin, args, {
      stdio: 'inherit',
      env: {
        ...process.env,
        // Normalise Chromatic cache location so repeated runs are faster.
        CHROMATIC_CACHE_DIR: process.env.CHROMATIC_CACHE_DIR ?? path.join(workspaceRoot, '.chromatic-cache'),
      },
    });

    child.on('exit', (code) => {
      if (code === 0) {
        resolve();
      } else {
        reject(new Error(`Chromatic exited with code ${code}`));
      }
    });
    child.on('error', reject);
  });
}

async function publishStorybooks(): Promise<void> {
  const storybooks = await loadAdapterStorybooks();
  if (!storybooks.length) {
    console.warn('No adapter Storybooks discovered. Skipping Chromatic publish step.');
    return;
  }

  for (const storybook of storybooks) {
    const token = resolveProjectToken(storybook.id);
    if (!token) {
      console.warn(
        `Skipping Chromatic upload for ${storybook.id}. Provide CHROMATIC_PROJECT_TOKEN or CHROMATIC_PROJECT_TOKEN_${storybook.id.toUpperCase()}.`,
      );
      continue;
    }

    const args = [
      '--project-token',
      token,
      '--storybook-build-dir',
      storybook.staticDir,
      '--no-interactive',
      '--exit-once-uploaded',
    ];

    if (process.env.CHROMATIC_BRANCH) {
      args.push('--branch', process.env.CHROMATIC_BRANCH);
    }
    if (process.env.CHROMATIC_COMMIT) {
      args.push('--commit', process.env.CHROMATIC_COMMIT);
    }
    if (process.env.CHROMATIC_SHA) {
      args.push('--sha', process.env.CHROMATIC_SHA);
    }

    console.log(`Publishing ${storybook.id} to Chromatic…`);
    await runChromatic(args);
  }
}

publishStorybooks().catch((error) => {
  console.error('Chromatic publishing failed', error);
  process.exitCode = 1;
});
