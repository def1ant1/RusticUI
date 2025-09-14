#![forbid(unsafe_code)]
//! General purpose utilities shared across the `mui-*` crates.
//!
//! The goal of this crate is to centralize tiny helpers behind
//! zero-cost, highly generic abstractions. Functions are organized in
//! separate modules so downstream crates can depend on only what they
//! need and the compiler can aggressively optimize away unused code.
//!
//! # Modules
//! * [`debounce`] - delay execution until a burst of calls has
//!   subsided.
//! * [`throttle`] - ensure a function runs at most once per interval.
//!
//! Future utilities can extend this crate to keep application code DRY
//! and encourage reuse across the ecosystem.

pub mod debounce;
pub mod throttle;

pub use debounce::debounce;
pub use throttle::throttle;
