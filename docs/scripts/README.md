# Docs automation entry points

The legacy Next.js build relied on `buildServiceWorker.js` and
`generateTemplateScreenshots.ts`. Those Node scripts have been replaced by
Rust-native tasks to keep the automation story consistent with the rest of the
workspace:

- `cargo xtask docs-assets --mode service-worker` – copies `docs/src/sw.js` to
  `docs/export/sw.js` and prepends a timestamp banner. The task mirrors the
  previous behaviour but now participates in the same observability and error
  handling surface as other `xtask` commands.
- `cargo xtask docs-assets --mode screenshots` – scans the template directories
  and emits `docs/public/static/screenshots/manifest.json`. The manifest records
  the target URLs, output paths, and supported modes (light/dark/default) so CI
  jobs can drive screenshot capture without bespoke Playwright scripts.

Both commands honour `DEPLOY_PREVIEW` when constructing URLs and accept
`--project` to scope the output to a specific documentation family.
