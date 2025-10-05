//! Headless foundation for MUI components.
//!
//! This crate exposes state machines and ARIA attribute helpers that are
//! shared across framework specific adapters.  Rendering logic lives in
//! higher level crates which consume these primitives.  Beyond the existing
//! [`button`] machine, the crate now ships specialized state for selection
//! controls – [`checkbox`], [`radio`] and [`switch`] – along with data display
//! helpers such as [`list`], [`menu`] and the accessible [`tabs`] family which
//! includes [`tab`] and [`tab_panel`] attribute builders.  Layout driven
//! components such as [`drawer`] also reuse the centralized accessibility
//! primitives.  The [`interaction`] primitives expose keyboard orchestration
//! shared across each state machine.  New Joy focused primitives including
//! [`accordion`], [`autocomplete`], [`slider`], [`snackbar`], [`stepper`] and
//! [`toggle_button_group`] build on the same deterministic rules so Material
//! and Joy stay aligned.  Newly added infrastructure primitives –
//! [`click_away`], [`collapsible_region`], and [`focus_trap`] – codify
//! concurrency-safe focus orchestration for overlays and disclosure widgets.
//! Experimental loop instrumentation lives in [`unstable_trap_focus`] behind an
//! `unstable` feature gate so enterprise teams can evaluate telemetry-heavy
//! behaviours before they harden into the stable focus APIs.
//! See the respective module documentation for detailed explanations of the
//! concurrency guarantees, focus-loop handling, and accessibility contracts.
//!
//! The Material layer (`rustic_ui_material`) documents how these headless states are
//! rendered with shared theming, automation identifiers, and SSR safe markup.
//! See [`crates/rustic-ui-material/README.md`](../rustic-ui-material/README.md#feedback-primitives-tooltip--chip)
//! for a tour of the tooltip and chip primitives and the
//! `examples/feedback-*` blueprints that exercise them across Yew, Leptos,
//! Dioxus, and Sycamore adapters.

pub mod accordion;
#[cfg(feature = "feedback")]
pub mod alert;
pub mod app_bar;
pub mod aria;
pub mod autocomplete;
pub mod avatar;
#[cfg(feature = "feedback")]
pub mod backdrop;
pub mod badge;
pub mod bottom_navigation;
pub mod r#box;
pub mod breadcrumbs;
pub mod button;
pub mod checkbox;
pub mod chip;
#[cfg(feature = "progress")]
pub mod circular_progress;
pub mod click_away;
pub mod collapsible_region;
pub mod container;
pub mod dialog;
pub mod divider;
pub mod drawer;
pub mod focus_trap;
#[cfg(feature = "forms")]
pub mod form_control;
pub mod grid;
pub mod hidden;
pub mod image_list;
#[cfg(feature = "forms")]
pub mod input_adornment;
#[cfg(feature = "forms")]
pub mod input_base;
pub mod interaction;
pub mod layout;
#[cfg(feature = "progress")]
pub mod linear_progress;
pub mod link;
pub mod list;
pub mod menu;
pub mod modal;
pub mod pagination;
pub mod paper;
pub mod popover;
pub mod popper;
pub mod portal;
pub mod radio;
pub mod select;
#[cfg(feature = "progress")]
pub mod skeleton;
pub mod slider;
pub mod snackbar;
pub mod speed_dial;
pub mod stack;
pub mod stepper;
pub mod switch;
pub mod tab;
pub mod tab_panel;
pub mod tabs;
pub mod text_field;
pub mod timing;
pub mod toggle_button_group;
pub mod tooltip;
pub mod transition;
pub mod typography;
#[cfg(feature = "unstable")]
pub mod unstable_trap_focus;

mod selection;
mod toggle;

#[cfg(feature = "forms")]
pub use input_base::{
    InputAnalyticsEvent, InputAnalyticsEventKind, InputChange, InputChangeEvent, InputCommit,
    InputCommitEvent, InputControlBuilder, InputControlBundle, InputReset, InputResetEvent,
    InputSelection, InputState,
};
pub use selection::ControlStrategy;

#[cfg(feature = "compat-mui")]
#[doc = "Deprecated compatibility shim exposing the crate under the legacy `mui_headless` name.\n\
Enable the `compat-mui` feature only while migrating to `rustic_ui_headless`.\n\
The alias will be removed once downstream crates finish the transition."]
#[deprecated(
    since = "0.1.0",
    note = "Use `rustic_ui_headless` going forward. The `mui_headless` alias is temporary."
)]
pub use crate as mui_headless;
