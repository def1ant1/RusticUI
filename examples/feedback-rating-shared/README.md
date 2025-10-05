# Feedback rating shared harness

This binary produces the SSR fragment consumed by the framework-specific rating
adapters.  Run `cargo run --package feedback-rating-shared` to print the
hydration baseline, or pipe the output into snapshot tests for React, Yew,
Leptos, Sycamore, or Dioxus orchestrators.

```bash
cargo run --package feedback-rating-shared > dist/rating.html
```

The example uses [`RatingState::uncontrolled`](../../crates/rustic-ui-headless/src/rating.rs)
to manage hover previews, half-step precision, and analytics metadata before
rendering the themed stars via the Material adapter.  Update the
`RatingAdapterProps` to experiment with controlled flows or per-framework
automation identifiers.
