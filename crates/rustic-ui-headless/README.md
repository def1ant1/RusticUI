# rustic_ui_headless

Deterministic component state machines designed for SSR friendly rendering,
enterprise automation hooks, and ergonomic framework adapters. Every public API
is extensively documented so portal renderers, hydration pipelines, and QA
suites can share the same mental model without reverse engineering internal
callbacks.

## Select state machine quick reference

`app_bar::AppBarState` centralises navigation banner metadata. The builder style
API exposes HTML/SVG attribute helpers with consistent automation identifiers,
and optional analytics hooks (`data-analytics-view-id`,
`data-analytics-interaction-id`) so Material adapters can emit the same SSR
markup as their client-side counterparts without duplicating telemetry logic.

`SelectState` powers listbox-style widgets (selects, combo boxes, virtualized
menus). The state machine now tracks which options are disabled alongside the
open/selected/highlighted bookkeeping so adapters can declaratively toggle
interactivity during SSR and client renders.

- `SelectState::set_option_disabled(index, bool)` updates the internal
  `Vec<bool>` that mirrors `option_count`. The helper automatically advances the
  highlight/selection to the nearest enabled option in uncontrolled mode so end
  users never land on inert entries.
- `SelectState::is_option_enabled(index)` and
  `SelectState::is_option_disabled(index)` expose read access for renderers that
  want to emit `aria-disabled` or `data-disabled` attributes without reimplementing
  the toggle logic.
- `SelectState::option_accessibility_attributes(index)` builds the `role="option"`
  metadata and conditionally appends disabled cues so adapters only need to
  extend the returned vector with automation IDs or custom data hooks.
- `SelectState::set_option_count(count)` keeps the disabled vector in sync with
  dynamic collections and clamps out-of-range indices. This avoids panics when
  async data loaders swap entire result sets.
- Navigation (`on_key`, `on_typeahead`) and selection (`select`,
  `select_highlighted`) helpers automatically skip disabled options and suppress
  callbacks so analytics hooks do not receive impossible interactions.

### Example (framework agnostic)

```rust
use rustic_ui_headless::select::SelectState;
use rustic_ui_headless::interaction::ControlKey;
use rustic_ui_headless::selection::ControlStrategy;

let mut state = SelectState::new(
    4,             // options rendered
    Some(0),       // initial selection
    false,         // popover closed by default
    ControlStrategy::Uncontrolled,
    ControlStrategy::Uncontrolled,
);

// Disable the third option (index 2) and rely on the state machine to advance
// highlight/selection without firing callbacks.
state.set_option_disabled(2, true);
assert!(state.is_option_disabled(2));

// Keyboard navigation skips disabled entries automatically.
let next = state.on_key(ControlKey::ArrowDown, |_| {});
assert_eq!(next, Some(3));

// Attribute builders centralize the `role`/disabled bookkeeping so adapters can
// append framework specific metadata without duplicating logic.
let attrs = state.option_accessibility_attributes(1);
assert!(attrs.iter().any(|(k, v)| k == &"role" && v == "option"));
assert!(attrs.iter().any(|(k, v)| k == &"aria-disabled" && v == "true"));
```

## Menu state machine quick reference

`MenuState` powers menu button widgets (`role="menu"` + `menuitem`). The state
machine mirrors the select implementation by tracking disabled items alongside
the open/highlight bookkeeping so adapters can declaratively toggle
interactivity during SSR and client hydration.

- `MenuState::set_item_disabled(index, bool)` flips the internal `Vec<bool>` and
  automatically advances the highlight to the nearest enabled entry when the
  menu manages focus (uncontrolled mode). Disabled items therefore never trap
  keyboard users even if RBAC rules or async data loads promote an action to a
  read-only state mid-session.
- `MenuState::is_item_enabled(index)`/`is_item_disabled(index)` expose read
  access for renderers that need to emit `aria-disabled` or
  `data-disabled` attributes without recalculating the bookkeeping.
- `MenuState::item_accessibility_attributes(index)` mirrors the select helper by
  returning the `role="menuitem"` tuple and optional disabled metadata ready to
  be extended with framework specific automation hooks.
- `MenuState::set_item_count(count)` resizes the disabled vector so dynamic
  collections stay in sync. Clamping prevents out-of-bounds indices when async
  loaders replace the entire menu payload.
- Navigation helpers (`ensure_highlight`, `on_key`, `on_typeahead`) skip disabled
  items automatically and `activate_highlighted` suppresses callbacks if the
  highlight resolves to an inert entry. Analytics hooks therefore never observe
  impossible activations.

### Example (framework agnostic)

```rust
use rustic_ui_headless::menu::MenuState;
use rustic_ui_headless::interaction::ControlKey;
use rustic_ui_headless::selection::ControlStrategy;

let mut state = MenuState::new(
    3,              // items rendered
    false,          // menu closed by default
    ControlStrategy::Uncontrolled,
    ControlStrategy::Uncontrolled,
);

// Disable the middle action and rely on the state machine to skip it during
// keyboard navigation.
state.set_item_disabled(1, true);
assert!(state.is_item_disabled(1));

// Arrow keys automatically jump to the next enabled entry.
assert_eq!(state.on_key(ControlKey::ArrowDown), Some(2));

// Activation callbacks never fire for disabled indices.
state.activate_highlighted(|_| panic!("disabled items should not activate"));

// Menu attribute builders emit `role` and disabled metadata on demand.
let attrs = state.item_accessibility_attributes(1);
assert!(attrs.iter().any(|(k, v)| k == &"role" && v == "menuitem"));
assert!(attrs.iter().any(|(k, v)| k == &"aria-disabled" && v == "true"));
```

### Testing strategy

Unit tests live alongside the implementations (`src/select.rs`) and document how
navigation, typeahead fallback, and controlled/uncontrolled sync behave when
options are disabled. Integration tests in `rustic_ui_material` assert that every
framework adapter emits `aria-disabled`/`data-disabled` attributes so SSR output
stays deterministic. Run the workspace suites with:

```bash
cargo test -p rustic_ui_headless
cargo test -p rustic_ui_material --all-features
```

### Automation-friendly design notes

- State machines prefer `Vec<bool>` bookkeeping over closures so they remain
  `Clone` for deterministic SSR snapshots.
- Methods never panic on out-of-bounds indices; instead they clamp and early
  return, making them safe to call from generated UI code.
- Callbacks are invoked only for enabled options ensuring analytics pipelines
  do not log interactions end users never saw.

## Experimental focus loop instrumentation (`unstable_trap_focus`)

Enterprise overlays often need to study how keyboard users interact with focus
loops before locking the telemetry model into a long-lived API.  The
`unstable_trap_focus` module wraps [`FocusTrapState`](src/focus_trap.rs) with
loop counters, direction metadata, and optional observers so QA automation can
feed the data into dashboards without sprinkling ad-hoc hooks across
renderers.  The helper is intentionally behind the `unstable` feature flag and
may change shape between releases – wire it in through thin integration layers
so migrating back to the stable `focus_trap` APIs is a one-line change once the
instrumentation hardens.

```rust
use rustic_ui_headless::interaction::ControlKey;
use rustic_ui_headless::unstable_trap_focus::{
    FocusLoopDirection, UnstableFocusTrapState,
};

let mut trap = UnstableFocusTrapState::new(true);
trap.set_focusables(["trigger", "close"]);
trap.register_focus(Some("close"));

// Observe how users wrap between sentinels.
trap.set_loop_observer(Some(|event| {
    println!(
        "loop #{}, direction: {:?}, analytics tag: {:?}",
        event.occurrence, event.direction, event.analytics_tag
    );
}));

let disposition = trap.handle_key(ControlKey::ArrowRight);
assert!(matches!(disposition, rustic_ui_headless::focus_trap::FocusDisposition::Focus("trigger")));
assert_eq!(trap.loop_event_count(), 1);
```

When the instrumentation stabilizes the observer hooks and data attributes will
move into the canonical `focus_trap` module.  Until then we recommend keeping
the wrapper isolated inside analytics services (or feature-gated crates) so the
integration surface stays easy to refactor.

## Dialog state machine deep dive

`DialogState` coordinates open/close transitions, focus trap bookkeeping, and
analytics metadata across SSR and hydration. The lifecycle phases map directly
to automation hooks so QA suites can assert the same transitions across Yew,
Leptos, Dioxus, Sycamore, or any custom renderer.

```
┌───────────────┐       open()        ┌──────────────┐
│ DialogPhase:: │ ───────────────▶ │ DialogPhase:: │
│ Closed        │                   │ Opening       │
└──────┬────────┘ ◀─────────────── │ └────┬─────────┘
       │        close()            │      │ finish_open()
       │                           ▼      ▼
       │                   ┌──────────────┐
       └────────────────── │ DialogPhase::│
                           │ Open         │
                           └──────────────┘
```

- `DialogState::open` and `DialogState::close` emit intents without mutating the
  internal phase when controlled. Call `sync_open` and `finish_open`/`finish_close`
  after animations to keep analytics and focus trap metadata aligned with the
  rendered output.
- `DialogState::surface_attributes()` centralises `role`, `aria-modal`,
  `data-state`, and `data-transition` tuples so adapters only append
  framework-specific identifiers (for example automation IDs).
- The shared [`ANCHOR_DIAGRAM`](../../examples/shared-dialog-state-core/src/lib.rs)
  constant illustrates how dialogs coordinate with `PopoverState` to keep
  floating surfaces anchored to deterministic DOM nodes.

## Popover geometry and anchor orchestration

`PopoverState` exposes deterministic anchor bookkeeping so SSR, hydration, and
runtime collision detection all share the same placement data. Controlled
popovers simply forward intents to parent controllers which then call
`sync_open` and optionally `resolve_with` to run custom collision detection.

```
┌───────────────┐   set_anchor_metadata   ┌────────────────────────────┐
│ Anchor id +   │ ─────────────────────▶ │ Analytics & portal helpers │
│ geometry      │                        └──────────┬─────────────────┘
└──────┬────────┘                                     │
       │                  toggle/open/close           │ render with
       ▼                                             ▼
┌──────────────┐    resolve_with()    ┌────────────────────────────┐
│ Popover open │ ───────────────────▶ │ data-preferred/resolved    │
│ flag         │                      │ attributes                  │
└──────────────┘                      └────────────────────────────┘
```

- Store anchor geometry via `set_anchor_metadata(Some(id), Some(AnchorGeometry))`
  so collision detection has the same bounding box data on the server and the
  client.
- Use `surface_attributes().analytics_id("...")` to emit deterministic
  `data-analytics-id` hooks that automation suites can reuse across frameworks.
- `resolve_with` keeps the preferred placement unless the provided resolver
  returns a different position. The returned `CollisionOutcome` is mirrored in
  `data-resolved-placement`, simplifying screenshot and telemetry comparisons.

## Text field validation lifecycle

`TextFieldState` tracks value, dirty/visited flags, validation errors, and
debounce windows in one place. The state machine works the same in every
framework because controlled changes always flow through `change` →
`sync_value` → `commit`/`reset`.

```
change(value) ──▶ dirty? ──▶ commit() ──▶ errors? ──▶ analytics/logging
                               ▲                      │
                               │                      ▼
                            reset() ──────────────── clear
```

- `TextFieldState::change` emits a `TextFieldChange` snapshot that includes the
  debounce interval so adapters can throttle expensive operations without
  duplicating timers.
- `TextFieldState::commit` marks the field as visited and returns whether
  validation errors are currently applied. Call `set_errors` before `commit`
  when performing synchronous validation so `has_errors` reflects the latest
  status.
- `TextFieldState::attributes()` returns reusable `aria-invalid`,
  `aria-describedby`, `data-dirty`, and `data-visited` tuples. Feeding these into
  markup helpers keeps automation selectors identical across frameworks.

## Sample orchestration

```rust
use rustic_ui_headless::dialog::DialogState;
use rustic_ui_headless::popover::{AnchorGeometry, PopoverPlacement, PopoverState};
use rustic_ui_headless::text_field::TextFieldState;
use std::time::Duration;

let mut dialog = DialogState::controlled();
let mut popover = PopoverState::controlled(PopoverPlacement::Bottom);
popover.set_anchor_metadata(
    Some("shared-popover-anchor"),
    Some(AnchorGeometry { x: 320.0, y: 640.0, width: 240.0, height: 48.0 }),
);
let mut text_field = TextFieldState::controlled("Northwind Traders", Some(Duration::from_millis(250)));

dialog.open(|open| assert!(open));
dialog.sync_open(true);
dialog.finish_open();

popover.toggle(|open| assert!(open));
popover.sync_open(true);
popover.resolve_with(|geometry, preferred| {
    if geometry.y + geometry.height > 600.0 {
        PopoverPlacement::Top
    } else {
        preferred
    }
});

text_field.change("Northwind Fabrics", |snapshot| {
    assert!(snapshot.dirty);
    assert_eq!(snapshot.debounce.map(|d| d.as_millis()), Some(250));
});
text_field.set_errors(vec!["Company name must be at least 3 characters.".into()]);
text_field.commit(|snapshot| assert!(snapshot.has_errors));
```

The example mirrors the automation-centric blueprints in
`examples/shared-dialog-state-*` by combining dialog transitions, popover
geometry, and text-field validation into a single deterministic flow.

## Architectural rationale for the new utility suite

- **Click-away orchestration** – `click_away::ClickAwayState` holds a lazily
  initialised event subscription that toggles pointer capture on demand. We keep
  the subscription detached until `arm()` is called so SSR snapshots do not
  attempt to access global browser primitives. The state exposes `should_close`
  rather than firing ad-hoc closures which keeps hydration deterministic across
  Yew, Leptos, Dioxus, and Sycamore adapters.
- **Focus trap timelines** – `focus_trap::FocusTrapState` mirrors the dialog
  lifecycle but records a full transition timeline (`Opening`, `Open`,
  `Closing`). Material renderers serialise this into `data-transition` markers so
  Playwright and axe-core audits can assert the same order without waiting on
  animation frames. We purposefully separate intent (`request_focus_within`) from
  the reconciliation (`commit_focus_within`) so tests can time-travel through the
  state machine.
- **Global telemetry** – `telemetry::EventStream` centralises the capture of
  high-value analytics (open counts, dismissal reasons, focus violations) and
  exposes pure data records. Adapters forward the records to framework-specific
  loggers, but the core utility stays `no_std` friendly for server-side runs.

These decisions guarantee the utilities stay cloneable, replayable, and easily
serialised for SSR pipelines while still exposing the hooks Material renderers
expect.

## Troubleshooting the utility suite

1. **Unexpected focus escapes** – enable `FocusTrapState::diagnostics()` to
   return the last known active element and transition timeline. In adapters, log
   the diagnostics whenever a trap is re-armed to confirm hydration captured the
   correct node.
2. **Click-away loops** – confirm `ClickAwayState::is_armed()` returns `false`
   before hydration. If server renders initialise the state too early, gate the
   call behind `is_hydrated` or reuse the adapters' `arm_on_mount` helper so the
   subscription is deferred until the browser API becomes available.
3. **Telemetry gaps** – when analytics dashboards miss events, run the
   automation example `cargo xtask examples --group automation --release` and
   inspect the generated `automation-events.ndjson`. The headless crate emits the
   stream into `target/rustic-ui-automation/` so you can diff the expected
   records against your integration.

## Observability and automation hooks

- Every state machine exposes `automation_attributes()` that return ready-to-log
  `(&'static str, String)` tuples. Material adapters extend the tuples with
  component-specific IDs before rendering, keeping SSR/CSR diffs flat.
- `EventStream::drain_with` is intentionally synchronous. Feed the drained batch
  into your logging sink inside the render loop (for SSR) or schedule it via
  microtasks (for CSR) to avoid reordering analytics events.
- Run `cargo test -p rustic-ui-headless -- --include-ignored` locally when adding
  new instrumentation. The ignored tests replay hydration edge cases (for
  example `focus_trap_replay.rs`) and ensure that new logging does not introduce
  borrow conflicts or timing issues.
