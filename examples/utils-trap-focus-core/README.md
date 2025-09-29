# Focus Trap Utilities Core

This crate centralizes the automation-friendly focus trap configuration shared
by the framework-specific utilities examples.  Calling
`utils_trap_focus_core::enterprise_story()` yields:

- a [`FocusTrapState`] preloaded with focusable node IDs, analytics markers, and
  loop configuration;
- SSR HTML for the start/end sentinels and modal surface; and
- theming information so hydration harnesses can wrap the snapshot with the
  same palette and typography overrides.

Downstream adapters (Yew, Leptos, Dioxus, Sycamore) import the story to render
matching sentinels at build time and during hydration.  Tests ensure the SSR
snapshot always includes the critical `data-rustic-focus-trap` hooks that QA
suites and monitoring agents rely on.
