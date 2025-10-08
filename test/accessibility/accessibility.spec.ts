import AxeBuilder from '@axe-core/playwright';
import { expect, test } from '@playwright/test';
import * as fs from 'node:fs/promises';
import * as path from 'node:path';
import { accessibilityTargets, ImpactGate } from './targets';

type Impact = 'minor' | 'moderate' | 'serious' | 'critical';

const impactWeights: Record<Impact, number> = {
  minor: 0,
  moderate: 1,
  serious: 2,
  critical: 3,
};

const sanitize = (value: string) => value.replace(/[^a-z0-9]+/gi, '-').replace(/(^-|-$)/g, '').toLowerCase();

function resolveThreshold(impact?: ImpactGate): number {
  if (!impact || impact === 'none') {
    return -1;
  }
  return impactWeights[impact as Impact] ?? -1;
}

test.describe('RusticUI documentation accessibility', () => {
  for (const target of accessibilityTargets) {
    test(`axe audit :: ${target.slug}`, async ({ page }, testInfo) => {
      const resolvedPath = target.path.startsWith('http')
        ? target.path
        : target.path.startsWith('/')
        ? target.path
        : `/${target.path}`;

      const response = await page.goto(resolvedPath, { waitUntil: 'networkidle' });
      expect(response, `Failed to load ${resolvedPath}`).not.toBeNull();

      if (target.readySelector) {
        await page.locator(target.readySelector).first().waitFor({ state: 'visible' });
      }

      if (target.postReadyWaitMs) {
        await page.waitForTimeout(target.postReadyWaitMs);
      }

      const axeBuilder = new AxeBuilder({ page }).withTags(['wcag2a', 'wcag2aa']);

      const allowedRuleRelaxations = new Set(target.axe?.allowedViolations ?? []);

      const results = await axeBuilder.analyze();
      const threshold = resolveThreshold(target.axe?.maxAllowedImpact);

      const blocking = results.violations.filter((violation) => {
        if (allowedRuleRelaxations.has(violation.id)) {
          return false;
        }
        if (!violation.impact) {
          return true;
        }
        const weight = impactWeights[violation.impact as Impact];
        if (weight === undefined) {
          return true;
        }
        return weight > threshold;
      });

      const warnings = results.violations.filter((violation) => !blocking.includes(violation));

      const slug = sanitize(target.slug);
      const axeOutputPath = testInfo.outputPath(`${slug}.axe.json`);
      await fs.mkdir(path.dirname(axeOutputPath), { recursive: true });
      await fs.writeFile(axeOutputPath, JSON.stringify(results, null, 2), 'utf8');
      await testInfo.attach('axe-results', {
        path: axeOutputPath,
        contentType: 'application/json',
      });

      const markdownLines = [
        `# Axe summary for ${target.slug}`,
        '',
        `- URL: ${resolvedPath}`,
        `- Blocking violations: ${blocking.length}`,
        `- Non-blocking warnings: ${warnings.length}`,
      ];
      if (allowedRuleRelaxations.size > 0) {
        markdownLines.push(
          `- Allowed rule relaxations: ${Array.from(allowedRuleRelaxations).join(', ')}`,
        );
      }
      if (target.notes?.length) {
        markdownLines.push('', '## Context', ...target.notes.map((line) => `- ${line}`));
      }
      if (blocking.length) {
        markdownLines.push('', '## Blocking violations');
        for (const violation of blocking) {
          markdownLines.push(`- **${violation.id}** (${violation.impact ?? 'unknown'}): ${violation.help}`);
          violation.nodes.forEach((node) => {
            markdownLines.push(
              `  - Target: ${node.target.join(' > ')} – ${node.failureSummary?.trim() ?? 'See axe payload'}`,
            );
          });
        }
      }
      if (warnings.length) {
        markdownLines.push('', '## Warnings');
        for (const violation of warnings) {
          markdownLines.push(`- **${violation.id}** (${violation.impact ?? 'unknown'}): ${violation.help}`);
        }
      }

      const markdownPath = testInfo.outputPath(`${slug}.md`);
      await fs.writeFile(markdownPath, markdownLines.join('\n'), 'utf8');
      await testInfo.attach('summary', { path: markdownPath, contentType: 'text/markdown' });

      testInfo.annotations.push({
        type: 'a11y',
        description: `${target.slug}: ${blocking.length} blocking / ${warnings.length} warnings`,
      });

      expect(blocking, () => {
        const formatted = blocking
          .map((violation) => `${violation.id} (${violation.impact ?? 'unknown'}) -> ${violation.help}`)
          .join('\n');
        return formatted.length ? formatted : 'axe-core reported violations';
      }).toEqual([]);
    });
  }
});
