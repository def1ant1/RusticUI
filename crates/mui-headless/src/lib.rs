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
//! and Joy stay aligned.
//!
//! The Material layer (`mui-material`) documents how these headless states are
//! rendered with shared theming, automation identifiers, and SSR safe markup.
//! See [`crates/mui-material/README.md`](../mui-material/README.md#feedback-primitives-tooltip--chip)
//! for a tour of the tooltip and chip primitives and the
//! `examples/feedback-*` blueprints that exercise them across Yew, Leptos,
//! Dioxus, and Sycamore adapters.

pub mod accordion;
pub mod aria;
pub mod autocomplete;
pub mod button;
pub mod checkbox;
pub mod chip;
pub mod dialog;
pub mod drawer;
pub mod interaction;
pub mod list;
pub mod menu;
pub mod popover;
pub mod radio;
pub mod select;
pub mod slider;
pub mod snackbar;
pub mod stepper;
pub mod switch;
pub mod tab;
pub mod tab_panel;
pub mod tabs;
pub mod text_field;
pub mod timing;
pub mod toggle_button_group;
pub mod tooltip;

mod selection;
mod toggle;
