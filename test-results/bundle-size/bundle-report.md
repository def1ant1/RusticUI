# RusticUI bundle cost report

Generated at 2025-10-07T04:28:36.117327731+00:00 (unix: 1759811316).

Artifacts compiled with release profile under `target/bundle-report`.

| Scenario | Crate | Features | Size (KiB) | Δ KiB | Δ % | Artifact | Notes |
|----------|-------|----------|-----------:|------:|----:|----------|-------|
| Headless (default) | rustic-ui-headless | default features | 2805.51 | 0.00 | 0.00 | `target/bundle-report/release/deps/librustic_ui_headless-bd7b6b1d76841d8d.rlib` | Full optional surface mirroring the published crate defaults. |
| Headless (forms + feedback) | rustic-ui-headless | forms, feedback | 2805.51 | 0.00 | 0.00 | `target/bundle-report/release/deps/librustic_ui_headless-bd7b6b1d76841d8d.rlib` | Adds snackbar, rating and feedback primitives. |
| Headless (forms + progress) | rustic-ui-headless | forms, progress | 2805.51 | 0.00 | 0.00 | `target/bundle-report/release/deps/librustic_ui_headless-bd7b6b1d76841d8d.rlib` | Includes determinate and indeterminate progress indicators. |
| Headless (forms core) | rustic-ui-headless | forms | 2805.51 | 0.00 | 0.00 | `target/bundle-report/release/deps/librustic_ui_headless-bd7b6b1d76841d8d.rlib` | Baseline surface with form controls enabled. Modules like select/text-field require this feature to compile. |
| Material (default) | rustic-ui-material | default features | 4450.71 | 0.00 | 0.00 | `target/bundle-report/release/deps/librustic_ui_material-253dcceb9c7236eb.rlib` | Full optional surface mirroring the published crate defaults. |
| Material (forms + feedback) | rustic-ui-material | forms, feedback | 4450.71 | 0.00 | 0.00 | `target/bundle-report/release/deps/librustic_ui_material-253dcceb9c7236eb.rlib` | Activates alert, backdrop and snackbar renderers. |
| Material (forms + progress) | rustic-ui-material | forms, progress | 4450.71 | 0.00 | 0.00 | `target/bundle-report/release/deps/librustic_ui_material-253dcceb9c7236eb.rlib` | Adds linear/circular progress components and skeleton loaders. |
| Material (forms core) | rustic-ui-material | forms | 4450.71 | 0.00 | 0.00 | `target/bundle-report/release/deps/librustic_ui_material-253dcceb9c7236eb.rlib` | Baseline Material renderers require form controls; this matches the smallest supported feature matrix. |

## Methodology

- Sizes capture release-mode .rlib artifacts compiled on the CI host triple.
- Run `cargo xtask bundle-report` to refresh the data before shipping feature-flag changes.
- The generated Markdown feeds docs/performance/bundle-costs.md so engineering docs stay in sync with telemetry.
- Baseline measurements enable the `forms` feature because headless/material crates reference shared form utilities even wh
en other flags are disabled.
