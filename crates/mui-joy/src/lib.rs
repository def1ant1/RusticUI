//! Joy UI component library.
//!
//! This crate mirrors the structure of `mui-material` but implements
//! components and tokens from the Joy design system. The goal is to provide
//! a fully typed Rust API that can scale with additional components.

pub mod button;
pub mod macros;

pub use button::{Button, ButtonColor, ButtonProps, ButtonVariant};
