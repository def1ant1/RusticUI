# Styling Benchmarks

This document records a simple micro-benchmark comparing Rust style generation
via `mui-styled-engine` with the JavaScript `@emotion/css` implementation.

| Implementation | Iterations | Total Time | Approx. per Style |
|----------------|-----------:|-----------:|------------------:|
| Rust (`Style::new(css!())`) | 47,000,000 | 5.0 s | ~106 ns |
| JS (`@emotion/css`) | 100,000 | 49.9 ms | ~498 ns |

> Measurements were taken on the CI container using `criterion` for Rust and a
> simple Node.js loop for the JS implementation. Values are indicative only but
> demonstrate the zero-cost nature of the Rust approach.
