# mui-headless

Deterministic component state machines designed for SSR friendly rendering,
enterprise automation hooks, and ergonomic framework adapters. Every public API
is extensively documented so portal renderers, hydration pipelines, and QA
suites can share the same mental model without reverse engineering internal
callbacks.

## Select state machine quick reference

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
- `SelectState::set_option_count(count)` keeps the disabled vector in sync with
  dynamic collections and clamps out-of-range indices. This avoids panics when
  async data loaders swap entire result sets.
- Navigation (`on_key`, `on_typeahead`) and selection (`select`,
  `select_highlighted`) helpers automatically skip disabled options and suppress
  callbacks so analytics hooks do not receive impossible interactions.

### Example (framework agnostic)

```rust
use mui_headless::select::SelectState;
use mui_headless::interaction::ControlKey;
use mui_headless::selection::ControlStrategy;

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
```

### Testing strategy

Unit tests live alongside the implementations (`src/select.rs`) and document how
navigation, typeahead fallback, and controlled/uncontrolled sync behave when
options are disabled. Integration tests in `mui-material` assert that every
framework adapter emits `aria-disabled`/`data-disabled` attributes so SSR output
stays deterministic. Run the workspace suites with:

```bash
cargo test -p mui-headless
cargo test -p mui-material --all-features
```

### Automation-friendly design notes

- State machines prefer `Vec<bool>` bookkeeping over closures so they remain
  `Clone` for deterministic SSR snapshots.
- Methods never panic on out-of-bounds indices; instead they clamp and early
  return, making them safe to call from generated UI code.
- Callbacks are invoked only for enabled options ensuring analytics pipelines
  do not log interactions end users never saw.
