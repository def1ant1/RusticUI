import { test as base, expect } from '@playwright/test';
import { TestFixture } from './playwright.config';

const test = base.extend<TestFixture>({});

test.describe('Quick-start gallery docs smoke test', () => {
  test('Sandpack preview hydrates the shared quick-start CTA with analytics hooks', async ({ page }) => {
    // Navigate directly via baseURL-relative path so CI and local runs can point PLAYWRIGHT_TEST_BASE_URL
    // at either a Netlify deploy preview or a `pnpm docs:dev` server without further edits.
    await page.goto('/examples/quick-start-gallery');

    // Sandpack streams markup through an iframe; use a wildcard title selector to stay resilient against
    // upstream title casing tweaks while we wait for the React preview to hydrate.
    const previewFrame = page.frameLocator('iframe[title*="Sandpack"]');

    // The generator renders an H1 immediately once hydration finishes. Waiting on the heading makes sure the
    // iframe finished booting the React tree before we assert on the CTA attributes.
    await expect(
      previewFrame.getByRole('heading', { name: 'RusticUI quick-start CTA' }),
    ).toBeVisible();

    // The shared QuickStartButton component always renders a Material <Button component="a"> anchor so the
    // accessible role resolves to "link". Assert against the deterministic label to guarantee the Sandpack
    // preview pulled data from `QuickStartButtonGenerator.ts` instead of stale markup.
    const primaryCta = previewFrame.getByRole('link', {
      name: 'Bootstrap the shared Material shell',
    });
    await expect(primaryCta).toBeVisible();

    // Downstream automation relies on the analytics and app-action attributes. Matching the generator output
    // here prevents regressions if the component ever changes its props or default analytics identifier.
    await expect(primaryCta).toHaveAttribute('data-rustic-app-action', 'app-quick-start-primary');
    await expect(primaryCta).toHaveAttribute('data-rustic-analytics', 'docs.quick-start.button');

    // Double-check the rendered text to catch situations where translations or markup changes accidentally
    // strip the human-facing label that doc copy references.
    await expect(primaryCta).toHaveText('Bootstrap the shared Material shell');
  });
});
