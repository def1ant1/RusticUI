//! Experimental widgets for the Rust port of Material UI.
//!
//! This crate hosts pre-release components that require real world
//! feedback before graduating into the stable crates.  Each module is
//! guarded behind a Cargo feature flag to keep compile times lean and to
//! signal the unstable nature of the APIs.
//!
//! The design favors pluggable abstractions (e.g. [`DateAdapter`]) so
//! downstream applications can swap implementations without touching
//! widget logic.  This is intended to scale to enterprise grade usage
//! where different teams may standardize on different date/time crates.

pub mod adapters;
pub mod localization;

#[cfg(feature = "date-picker")]
pub mod date_picker;

#[cfg(feature = "time-picker")]
pub mod time_picker;

