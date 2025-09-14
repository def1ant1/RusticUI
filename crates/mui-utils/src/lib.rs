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
//! * [`deep_merge`] - recursively merge JSON-like values.
//! * [`compose_classes`] - build CSS class strings for component slots.
//!
//! # Examples
//! ```
//! use mui_utils::{deep_merge, compose_classes};
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

pub mod compose_classes;
pub mod debounce;
pub mod deep_merge;
pub mod throttle;

pub use compose_classes::compose_classes;
pub use debounce::debounce;
pub use deep_merge::deep_merge;
pub use throttle::throttle;
