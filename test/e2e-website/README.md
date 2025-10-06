# Docs end-to-end testing

## Running locally

1. Run `pnpm --dir docs dev` to start the development docs server. The Playwright suites rely on a fully hydrated docs instance so the embedded Sandpack previews can boot.
2. Export `PLAYWRIGHT_TEST_BASE_URL=http://127.0.0.1:3000` (or whichever host/port your docs server prints) before launching Playwright. CI pipelines typically inject a Netlify deploy preview URL into this variable.
3. Run `pnpm --dir docs exec -- playwright test --config test/e2e-website/playwright.config.ts` in a separate terminal to execute the smoke suites (`*.spec.ts`) inside `test/e2e-website`.

> Pass `--headed` to run tests in headed browsers, check out [Playwright CLI](https://playwright.dev/docs/intro#command-line) for more options.

### Quick-start gallery coverage

- `quick-start-gallery.spec.ts` drives the `/examples/quick-start-gallery` page, waits for the Sandpack iframe to hydrate, and asserts that the rendered call-to-action exposes `data-rustic-app-action="app-quick-start-primary"`, `data-rustic-analytics="docs.quick-start.button"`, and the visible label text from `QuickStartButtonGenerator.ts`.
- The scenario ensures docs, StackBlitz, and Rust quick-start flows continue sharing the same analytics hooks; failures surface under `target/logs/docs-playwright.log` when invoked via `cargo xtask docs-test`.

## CI

After Netlify deploys the preview site, the `netlify/functions/deploy-succeeded.js` hook calls CircleCI API to run the `e2e-website` workflow against the deployed URL.
