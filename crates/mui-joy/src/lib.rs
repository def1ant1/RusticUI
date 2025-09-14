//! Joy UI component library.
//!
//! This crate mirrors the structure of `mui-material` but implements
//! components and tokens from the Joy design system. The goal is to provide
//! a fully typed Rust API that can scale with additional components.

pub mod aspect_ratio;
pub mod button;
pub mod card;
pub mod chip;
pub mod macros;

pub use aspect_ratio::{AspectRatio, AspectRatioProps};
pub use button::{Button, ButtonProps};
pub use card::{Card, CardProps};
pub use chip::{Chip, ChipProps};
pub use macros::{Color, Variant};
