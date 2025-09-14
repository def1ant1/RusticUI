//! Core styling primitives for Material UI in Rust.
//!
//! This crate mirrors the structure of the upstream `packages/mui-system`
//! JavaScript package. It provides foundational building blocks like `Box`,
//! `Grid`, theming utilities and responsive helpers that higher level crates
//! can build upon.
//!
//! Features are gated so that downstream users only compile the code required
//! for their target framework (`yew`, `leptos`, ...).

pub mod macros;
pub mod responsive;
pub mod theme;
pub mod style;

#[cfg(any(feature = "yew", feature = "leptos"))]
pub mod r#box;
#[cfg(any(feature = "yew", feature = "leptos"))]
pub mod container;
#[cfg(any(feature = "yew", feature = "leptos"))]
pub mod grid;
#[cfg(any(feature = "yew", feature = "leptos"))]
pub mod stack;
#[cfg(any(feature = "yew", feature = "leptos"))]
pub mod theme_provider;
#[cfg(any(feature = "yew", feature = "leptos"))]
pub mod typography;

#[cfg(any(feature = "yew", feature = "leptos"))]
pub use container::Container;
#[cfg(any(feature = "yew", feature = "leptos"))]
pub use grid::Grid;
#[cfg(any(feature = "yew", feature = "leptos"))]
pub use r#box::Box;
pub use responsive::{grid_span_to_percent, Responsive};
#[allow(unused_imports)]
pub use style::*;
#[cfg(any(feature = "yew", feature = "leptos"))]
pub use stack::{Stack, StackDirection};
pub use theme::{Breakpoints, Palette, Theme};
#[cfg(any(feature = "yew", feature = "leptos"))]
pub use theme_provider::{use_theme, ThemeProvider};
#[cfg(any(feature = "yew", feature = "leptos"))]
pub use typography::{Typography, TypographyVariant};

/// Legacy no-op function retained to keep dependent crates compiling while
/// more features are ported. New functionality resides in the modules above.
pub fn placeholder() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_span_math_is_sound() {
        assert!((grid_span_to_percent(3, 12) - 25.0).abs() < f32::EPSILON);
    }
}
