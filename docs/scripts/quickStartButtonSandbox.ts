import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { buildQuickStartButtonSandbox } from '../src/components/examples/QuickStartButtonGenerator';

/**
 * CLI helper that keeps the Sandpack snapshot in sync with the generator. CI can run this script
 * with `--check` to confirm the tracked JSON file matches the latest source, while local contributors
 * can pass `--write` to refresh the export after intentional changes.
 */
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const snapshotPath = path.resolve(__dirname, '../data/examples/quick-start-button-sandbox.json');

const sandbox = buildQuickStartButtonSandbox();

const snapshot = `${JSON.stringify(
  {
    title: sandbox.title,
    entryFile: sandbox.entryFile,
    previewFile: sandbox.previewFile,
    dependencies: sandbox.dependencies,
    files: sandbox.fileEntries,
  },
  null,
  2,
)}\n`;

const mode = process.argv.slice(2)[0] ?? '--print';

if (mode === '--write') {
  fs.mkdirSync(path.dirname(snapshotPath), { recursive: true });
  fs.writeFileSync(snapshotPath, snapshot, 'utf8');
  console.log(`Wrote ${snapshotPath}`);
} else if (mode === '--check') {
  if (!fs.existsSync(snapshotPath)) {
    console.error(`Missing snapshot at ${snapshotPath}. Run with --write to generate it.`);
    process.exitCode = 1;
  } else {
    const current = fs.readFileSync(snapshotPath, 'utf8');
    if (current !== snapshot) {
      console.error('Quick-start sandbox snapshot is stale. Run with --write to refresh.');
      process.exitCode = 1;
    } else {
      console.log('Quick-start sandbox snapshot is up to date.');
    }
  }
} else {
  console.log(snapshot);
}
