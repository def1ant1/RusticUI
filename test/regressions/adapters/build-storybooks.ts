import * as fs from 'node:fs/promises';
import * as path from 'node:path';
import { spawn } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const currentDirectory = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(path.join(currentDirectory, '../../..'));
const examplesRoot = path.join(workspaceRoot, 'examples');

interface StorybookTarget {
  directory: string;
  script: string;
}

async function discoverStorybookScripts(): Promise<StorybookTarget[]> {
  const entries = await fs
    .readdir(examplesRoot, { withFileTypes: true })
    .catch(() => []);
  const candidates: StorybookTarget[] = [];

  for (const entry of entries) {
    if (!entry.isDirectory()) {
      continue;
    }

    const packageJsonPath = path.join(examplesRoot, entry.name, 'package.json');
    try {
      const raw = await fs.readFile(packageJsonPath, 'utf8');
      const pkg = JSON.parse(raw) as { scripts?: Record<string, string> };
      const scripts = pkg.scripts ?? {};
      const scriptName = ['storybook:build', 'build:storybook', 'storybook'].find((name) => scripts[name]);
      if (scriptName) {
        candidates.push({
          directory: path.join(examplesRoot, entry.name),
          script: scriptName,
        });
      }
    } catch (error: unknown) {
      if ((error as NodeJS.ErrnoException).code !== 'ENOENT') {
        console.warn(`Failed to inspect ${packageJsonPath}`, error);
      }
    }
  }

  return candidates;
}

async function runScript(target: StorybookTarget): Promise<void> {
  console.log(`Building Storybook via pnpm --dir ${target.directory} run ${target.script}`);

  await new Promise<void>((resolve, reject) => {
    const child = spawn(
      'pnpm',
      ['--dir', target.directory, 'run', target.script],
      {
        stdio: 'inherit',
        env: { ...process.env },
      },
    );

    child.on('exit', (code) => {
      if (code === 0) {
        resolve();
      } else {
        reject(new Error(`Storybook build failed with exit code ${code}`));
      }
    });
    child.on('error', reject);
  });
}

async function main(): Promise<void> {
  const storybooks = await discoverStorybookScripts();
  if (!storybooks.length) {
    console.warn('No Storybook build scripts discovered under examples/. Skipping build step.');
    return;
  }

  for (const storybook of storybooks) {
    await runScript(storybook);
  }
}

main().catch((error) => {
  console.error('Storybook build orchestration failed', error);
  process.exitCode = 1;
});
