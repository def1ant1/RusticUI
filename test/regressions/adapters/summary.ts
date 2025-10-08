/**
 * Visual regression summary writer tailored for the adapter Playwright harness.
 *
 * The coverage dashboard expects a JSON file with snapshot counts. Rather than
 * duplicating logic in multiple workflows we encapsulate the bookkeeping in this
 * helper class. Tests call `recordSnapshot()` whenever they persist a screenshot
 * and the harness flushes everything to disk exactly once after the suite
 * completes.
 */

import * as fs from 'node:fs/promises';
import * as path from 'node:path';

export class VisualSummaryWriter {
  private readonly outputPath: string;
  private snapshots = 0;
  private differences = 0;
  private updated = 0;
  private skipped = 0;
  private readonly notes = new Set<string>();

  constructor(outputPath: string) {
    this.outputPath = outputPath;
  }

  recordSnapshot(): void {
    this.snapshots += 1;
  }

  recordDifference(): void {
    this.differences += 1;
  }

  recordUpdated(): void {
    this.updated += 1;
  }

  recordSkipped(): void {
    this.skipped += 1;
  }

  addNote(note: string): void {
    this.notes.add(note);
  }

  hasSnapshots(): boolean {
    return this.snapshots > 0;
  }

  async flush(): Promise<void> {
    if (!this.hasSnapshots()) {
      // Avoid leaving stale results behind if a previous run produced snapshots
      // but this one did not (for example when running locally without Storybook
      // builds). The coverage report will treat the missing file as a skipped
      // suite which is exactly what we want.
      await fs.rm(this.outputPath, { force: true });
      return;
    }

    const payload = {
      snapshots: this.snapshots,
      differences: this.differences,
      updated: this.updated,
      skipped: this.skipped,
      notes: Array.from(this.notes),
    };

    await fs.mkdir(path.dirname(this.outputPath), { recursive: true });
    await fs.writeFile(this.outputPath, `${JSON.stringify(payload, null, 2)}\n`);
  }
}
