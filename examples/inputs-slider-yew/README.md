# inputs-slider-yew

Automation-first Yew harness that exercises the headless [`SliderState`](../../crates/rustic-ui-headless/src/slider.rs)
and Material renderer. CI compiles the crate for the host target to capture SSR
markup and for `wasm32-unknown-unknown` to ensure the Yew integration links
successfully.

## Running locally

```bash
wasm-pack build --target web --out-dir dist
```

The host build prints the serialized slider markup which downstream automation
can diff during regression testing.
