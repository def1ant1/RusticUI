//! Headless foundation for MUI components.
//!
//! This crate exposes state machines and ARIA attribute helpers that are
//! shared across framework specific adapters.  Rendering logic lives in
//! higher level crates which consume these primitives.  Beyond the existing
//! [`button`] machine, the crate now ships specialized state for selection
//! controls – [`checkbox`], [`radio`] and [`switch`] – along with [`interaction`]
//! primitives for keyboard orchestration.

pub mod aria;
pub mod button;
pub mod checkbox;
pub mod interaction;
pub mod radio;
pub mod switch;

mod toggle;
