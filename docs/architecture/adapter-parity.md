# Adapter Parity

_Last updated 2025-10-07T16:01:32Z via `cargo xtask parity-report`._

The tables below enumerate which framework adapters ship for each component. Material adapters are discovered by scanning the adapter modules under `crates/rustic-ui-material/src`, and the Joy rows come from the Yew-first modules declared in `crates/rustic-ui-joy/src/lib.rs`. Parity is validated by the cross-adapter regression suites such as [`button_adapters.rs`](../../crates/rustic-ui-material/tests/button_adapters.rs) and [`joy_yew.rs`](../../crates/rustic-ui-material/tests/joy_yew.rs). Run `cargo xtask parity-report` after adding or removing adapters, and `cargo xtask parity-report --check` in CI, so this dashboard stays in sync.

## Material adapters

- React adapters: 23/35
- Yew adapters: 28/35
- Leptos adapters: 29/35
- Dioxus adapters: 35/35
- Sycamore adapters: 35/35

| Component | React | Yew | Leptos | Dioxus | Sycamore |
| --- | --- | --- | --- | --- | --- |
| App Bar | ⬜ | ⬜ | ⬜ | ✅ | ✅ |
| Bottom Navigation | ✅ | ✅ | ✅ | ✅ | ✅ |
| Box | ✅ | ✅ | ✅ | ✅ | ✅ |
| Breadcrumbs | ✅ | ✅ | ✅ | ✅ | ✅ |
| Button | ✅ | ✅ | ✅ | ✅ | ✅ |
| Card | ⬜ | ⬜ | ⬜ | ✅ | ✅ |
| Checkbox | ✅ | ✅ | ✅ | ✅ | ✅ |
| Chip | ✅ | ✅ | ✅ | ✅ | ✅ |
| Click Away | ⬜ | ✅ | ✅ | ✅ | ✅ |
| Collapsible | ⬜ | ✅ | ✅ | ✅ | ✅ |
| Container | ✅ | ✅ | ✅ | ✅ | ✅ |
| Dialog | ✅ | ⬜ | ✅ | ✅ | ✅ |
| Divider | ✅ | ✅ | ✅ | ✅ | ✅ |
| Drawer | ✅ | ✅ | ✅ | ✅ | ✅ |
| Focus Trap | ⬜ | ⬜ | ⬜ | ✅ | ✅ |
| Grid | ✅ | ✅ | ✅ | ✅ | ✅ |
| Hidden | ✅ | ✅ | ✅ | ✅ | ✅ |
| Image List | ✅ | ✅ | ✅ | ✅ | ✅ |
| Input Base | ✅ | ⬜ | ⬜ | ✅ | ✅ |
| Link | ✅ | ✅ | ✅ | ✅ | ✅ |
| List | ⬜ | ✅ | ✅ | ✅ | ✅ |
| Menu | ⬜ | ✅ | ✅ | ✅ | ✅ |
| Pagination | ✅ | ✅ | ✅ | ✅ | ✅ |
| Radio | ✅ | ✅ | ✅ | ✅ | ✅ |
| Rating | ✅ | ✅ | ✅ | ✅ | ✅ |
| Select | ⬜ | ✅ | ✅ | ✅ | ✅ |
| Speed Dial | ✅ | ✅ | ✅ | ✅ | ✅ |
| Stack | ✅ | ✅ | ✅ | ✅ | ✅ |
| Stepper | ✅ | ✅ | ✅ | ✅ | ✅ |
| Switch | ✅ | ✅ | ✅ | ✅ | ✅ |
| Table | ⬜ | ✅ | ✅ | ✅ | ✅ |
| Tabs | ✅ | ✅ | ✅ | ✅ | ✅ |
| Text Field | ⬜ | ⬜ | ⬜ | ✅ | ✅ |
| Tooltip | ⬜ | ✅ | ✅ | ✅ | ✅ |
| Unstable Trap Focus | ⬜ | ⬜ | ⬜ | ✅ | ✅ |


## Joy adapters

- React adapters: 0/10
- Yew adapters: 10/10
- Leptos adapters: 0/10
- Dioxus adapters: 0/10
- Sycamore adapters: 0/10

| Component | React | Yew | Leptos | Dioxus | Sycamore |
| --- | --- | --- | --- | --- | --- |
| Accordion | ⬜ | ✅ | ⬜ | ⬜ | ⬜ |
| Aspect Ratio | ⬜ | ✅ | ⬜ | ⬜ | ⬜ |
| Autocomplete | ⬜ | ✅ | ⬜ | ⬜ | ⬜ |
| Button | ⬜ | ✅ | ⬜ | ⬜ | ⬜ |
| Card | ⬜ | ✅ | ⬜ | ⬜ | ⬜ |
| Chip | ⬜ | ✅ | ⬜ | ⬜ | ⬜ |
| Slider | ⬜ | ✅ | ⬜ | ⬜ | ⬜ |
| Snackbar | ⬜ | ✅ | ⬜ | ⬜ | ⬜ |
| Stepper | ⬜ | ✅ | ⬜ | ⬜ | ⬜ |
| Toggle Button Group | ⬜ | ✅ | ⬜ | ⬜ | ⬜ |

