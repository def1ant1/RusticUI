# MUI SSR Accessibility Example

Enterprise-grade applications often require server-side rendering (SSR) for
fast first paint and search engine visibility.  This example shows how to
render a small Yew application on the server while respecting accessibility
best practices.

The sample renders an `AppBar` component with an `aria-label` attribute so
assistive technologies can properly describe the navigation landmark.

Run the example with:

```bash
cd examples/rustic_ui_ssr_accessibility
cargo run
```
