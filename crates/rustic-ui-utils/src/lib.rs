#![forbid(unsafe_code)]
//! General purpose utilities shared across the `rustic_ui_*` crates.
//!
//! NOTE: we leave this explicit migration note to guide downstream consumers
//! through the legacy MUI-to-`rustic_ui_*` rename so aliases can be retired once
//! applications finish the transition.
//!
//! The goal of this crate is to centralize tiny helpers behind
//! zero-cost, highly generic abstractions. Functions are organized in
//! separate modules so downstream crates can depend on only what they
//! need and the compiler can aggressively optimize away unused code.
//!
//! # Modules
//! * [`accessibility`] - compose ARIA rich HTML attribute collections.
//! * [`debounce`] - delay execution until a burst of calls has
//!   subsided.
//! * [`throttle`] - ensure a function runs at most once per interval.
//! * [`deep_merge`] - recursively merge JSON-like values.
//! * [`compose_classes`] - build CSS class strings for component slots.
//!
//! # Examples
//! ```
//! use rustic_ui_utils::{deep_merge, compose_classes};
//! use serde_json::json;
//! use std::collections::HashMap;
//!
//! let mut data = json!({"a": 1});
//! deep_merge(&mut data, json!({"b": 2}));
//! assert_eq!(data, json!({"a": 1, "b": 2}));
//!
//! let mut slots = HashMap::new();
//! slots.insert("root".to_string(), vec![Some("root".to_string())]);
//! let classes = compose_classes(&slots, |s| format!("My-{s}"), None);
//! assert_eq!(classes.get("root"), Some(&"My-root".to_string()));
//! ```
//!
//! Future utilities can extend this crate to keep application code DRY
//! and encourage reuse across the ecosystem.

pub mod accessibility;
pub mod compose_classes;
pub mod debounce;
pub mod deep_merge;
pub mod throttle;

pub use accessibility::{attributes_to_html, collect_attributes, extend_attributes};
pub use compose_classes::compose_classes;
pub use debounce::debounce;
pub use deep_merge::deep_merge;
pub use throttle::throttle;

#[cfg(feature = "compat-mui")]
#[doc = "Deprecated compatibility shim exposing the crate under the legacy `mui_utils` name.\n\
Enable the `compat-mui` feature only while migrating to `rustic_ui_utils`.\n\
The alias will be removed once downstream projects complete the rename."]
#[deprecated(
    since = "0.1.0",
    note = "Use `rustic_ui_utils`. The `mui_utils` alias is temporary and will be removed."
)]
pub use crate as mui_utils;
