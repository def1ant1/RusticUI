//! Experimental widgets for the Rust port of Material UI.
//!
//! This crate hosts pre-release components that require real world
//! feedback before graduating into the stable crates.  Each module is
//! guarded behind a Cargo feature flag to keep compile times lean and to
//! signal the unstable nature of the APIs.
//!
//! The design favors pluggable abstractions (e.g. [`adapters::DateAdapter`]) so
//! downstream applications can swap implementations without touching
//! widget logic.  This is intended to scale to enterprise grade usage
//! where different teams may standardize on different date/time crates. Each
//! widget lives behind a feature flag (`autocomplete`, `date-picker`,
//! `data-grid`, `tree-view`, `timeline`, `time-picker`, `masonry`,
//! `localization`) to minimize compile times and manual toggling.

pub mod adapters;

#[cfg(feature = "localization")]
pub mod localization;

#[cfg(feature = "autocomplete")]
pub mod autocomplete;

#[cfg(feature = "date-picker")]
pub mod date_picker;

#[cfg(feature = "data-grid")]
pub mod data_grid;

#[cfg(feature = "time-picker")]
pub mod time_picker;

#[cfg(feature = "masonry")]
pub mod masonry;

#[cfg(feature = "tree-view")]
pub mod tree_view;

#[cfg(feature = "timeline")]
pub mod timeline;
