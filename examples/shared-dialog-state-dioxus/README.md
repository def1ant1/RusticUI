# Shared Dialog State – Dioxus Blueprint

This blueprint confirms that the shared overlay state can be reused in a
[Dioxus](https://dioxuslabs.com/) application without sacrificing automation
hooks or validation semantics.  The script provisions a ready-to-run project
that targets the web renderer so teams can exercise the same dialog/popover/text
field flows found in the Yew and Leptos demos.

## Capabilities

- **State parity** – `SharedOverlayState` is stored inside a Dioxus `use_state`
  handle so requests to `request_dialog_open`, `toggle_popover`, and
  `commit_text` mutate the same state machine used in every other framework.
- **Automation-first markup** – attribute helpers from the shared crate are
  converted into `data-*` and `aria-*` attributes in the `rsx!` tree so QA suites
  receive identical hooks.
- **Lifecycle journaling** – every interaction appends notes to an ordered list
  rendered from a Dioxus state vector, making it trivial to audit behaviour or
  forward events to observability stacks.
- **Anchor diagram broadcast** – the ASCII anchor diagram is printed once during
  startup for cross-team alignment on collision handling.

## Bootstrapping

```bash
./examples/shared-dialog-state-dioxus/scripts/bootstrap.sh
cd target/shared-dialog-state-dioxus-demo
cargo install dioxus-cli # if not already installed
DX_PLATFORM=web dx serve
```

The generated crate already depends on `shared-dialog-state-core`, includes
verbose inline comments, and emits deterministic automation attributes that
mirror the other frameworks.  The workspace structure makes it easy to add
server-side rendering or desktop targets later without rewriting the shared
state orchestration.
