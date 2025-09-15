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
pub mod style;
pub mod theme;
// Cross framework element demonstrating themed styling and ARIA metadata.
// The module is gated behind feature specific adapters to keep compilation
// lean while still allowing reuse across front-end targets.
pub mod themed_element;

pub mod r#box;
pub mod container;
pub mod grid;
pub mod stack;
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
#[cfg(any(feature = "yew", feature = "leptos"))]
pub use stack::{Stack, StackDirection};
#[allow(unused_imports)]
pub use style::*;
pub use theme::{Breakpoints, Palette, Theme};
#[cfg(all(
    any(feature = "dioxus", feature = "sycamore"),
    not(any(feature = "yew", feature = "leptos"))
))]
pub use theme_provider::use_theme;
#[cfg(feature = "yew")]
pub use theme_provider::{use_theme, ThemeProvider};
#[cfg(all(feature = "leptos", not(feature = "yew")))]
pub use theme_provider::{use_theme, ThemeProvider};
pub use themed_element::{ThemedProps, Variant};
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
