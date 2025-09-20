//! Headless foundation for MUI components.
//!
//! This crate exposes state machines and ARIA attribute helpers that are
//! shared across framework specific adapters.  Rendering logic lives in
//! higher level crates which consume these primitives.  Beyond the existing
//! [`button`] machine, the crate now ships specialized state for selection
//! controls – [`checkbox`], [`radio`] and [`switch`] – along with data display
//! helpers such as [`list`] and [`menu`]. The [`interaction`] primitives expose
//! keyboard orchestration shared across each state machine.

pub mod aria;
pub mod button;
pub mod checkbox;
pub mod drawer;
pub mod interaction;
pub mod list;
pub mod menu;
pub mod radio;
pub mod select;
pub mod switch;
pub mod tab;
pub mod tab_panel;
pub mod tabs;

mod selection;
mod toggle;
