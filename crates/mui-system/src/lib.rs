//! Core styling primitives for Material UI in Rust.
//!
//! This crate mirrors the structure of the upstream `packages/mui-system`
//! JavaScript package. It provides foundational building blocks like `Box`,
//! `Grid`, theming utilities and responsive helpers that higher level crates
//! can build upon.
//!
//! Features are gated so that downstream users only compile the code required
//! for their target framework (`yew`, `leptos`, ...).

pub mod style;
pub mod theme;
pub mod responsive;

#[cfg(feature = "yew")]
pub mod r#box;
#[cfg(feature = "yew")]
pub mod grid;
#[cfg(feature = "yew")]
pub mod theme_provider;

pub use responsive::{grid_span_to_percent, Responsive};
pub use theme::{Breakpoints, Palette, Theme};
#[cfg(feature = "yew")]
pub use r#box::Box;
#[cfg(feature = "yew")]
pub use grid::Grid;
#[cfg(feature = "yew")]
pub use theme_provider::{use_theme, ThemeProvider};

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
