# Styling Benchmarks

This document records a simple micro-benchmark comparing Rust style generation
via `mui-styled-engine` with the JavaScript `@emotion/css` implementation.

| Implementation | Iterations | Total Time | Approx. per Style |
|----------------|-----------:|-----------:|------------------:|
| Rust (`Style::new(css!())`) | 47,000,000 | 5.0 s | ~106 ns |
| JS (`@emotion/css`) | 100,000 | 49.9 ms | ~498 ns |

## Repository benches

`cargo xtask bench` now shells through Criterion benches under each crate. The
Material harness (`crates/rustic-ui-material/benches/material_render.rs`) focuses on
the `ButtonState -> themed HTML` pipeline so regressions in CSS generation or
adapter wiring surface immediately. The headless harness
(`crates/rustic-ui-headless/benches/transition.rs`) stress-tests the overlay
transition state machine to ensure enter/exit bookkeeping stays constant as new
surfaces layer on analytics hooks. Both benches include detailed notes inline so
contributors can extend the coverage without rediscovering the intended
measurement strategy.

> Measurements were taken on the CI container using `criterion` for Rust and a
> simple Node.js loop for the JS implementation. Values are indicative only but
> demonstrate the zero-cost nature of the Rust approach.
