import { test, expect } from '@playwright/test';

// Smoke test verifying that the React hydration surface exposes telemetry updates as expected.
test('checkbox telemetry is streamed in order', async ({ page }) => {
  await page.goto('/');
  const log = page.locator('[data-testid="telemetry-log"] li');
  await page.getByRole('checkbox', { name: 'Receive Alerts' }).check();
  await expect(log).toHaveCount(2);
  const entries = await log.allTextContents();
  expect(entries[0]).toContain('mount');
  expect(entries[1]).toContain('user');
});
