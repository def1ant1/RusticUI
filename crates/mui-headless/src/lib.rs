//! Headless foundation for MUI components.
//!
//! This crate exposes state machines and ARIA attribute helpers that are
//! shared across framework specific adapters.  Rendering logic lives in
//! higher level crates which consume these primitives.

pub mod aria;
pub mod button;
