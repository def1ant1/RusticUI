# Styling Benchmarks

This document records a simple micro-benchmark comparing Rust style generation
via `mui-styled-engine` with the JavaScript `@emotion/css` implementation.

| Implementation | Iterations | Total Time | Approx. per Style |
|----------------|-----------:|-----------:|------------------:|
| Rust (`Style::new(css!())`) | 47,000,000 | 5.0 s | ~106 ns |
| JS (`@emotion/css`) | 100,000 | 49.9 ms | ~498 ns |

## Repository benches

`cargo xtask bench` now shells through Criterion benches under each crate. The
Material harness (`crates/rustic-ui-material/benches/material_render.rs`) tracks both the
`ButtonState -> themed HTML` pipeline **and** the drawer surface renderer. The
former guards the hot button adapters while the latter captures how navigation
surfaces stitch automation attributes, responsive tokens and theming into SSR-ready
markup. The headless harness (`crates/rustic-ui-headless/benches/transition.rs`) measures the
single-overlay lifecycle plus a pooled scenario representing clustered overlays so
enter/exit bookkeeping stays constant as new analytics hooks land. Both benches include
detailed notes inline so contributors can extend the coverage without rediscovering the
intended measurement strategy.

> Measurements were taken on the CI container using `criterion` for Rust and a
> simple Node.js loop for the JS implementation. Values are indicative only but
> demonstrate the zero-cost nature of the Rust approach.
