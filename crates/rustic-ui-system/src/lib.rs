//! Core styling primitives for Material UI in Rust.
//!
//! This crate mirrors the structure of the upstream `packag../rustic-ui-system`
//! JavaScript package. It provides foundational building blocks like `Box`,
//! `Grid`, theming utilities and responsive helpers that higher level crates
//! can build upon.
//!
//! Features are gated so that downstream users only compile the code required
//! for their target framework (`yew`, `leptos`, ...).

pub mod macros;
pub mod portal;
pub mod responsive;
mod scoped_class;
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

#[doc(hidden)]
pub use crate::theme_provider::use_theme;
#[cfg(any(feature = "yew", feature = "leptos"))]
pub use container::Container;
#[cfg(any(feature = "yew", feature = "leptos"))]
pub use grid::Grid;
pub use portal::{PortalFragment, PortalLayer, PortalMount};
#[cfg(any(feature = "yew", feature = "leptos"))]
pub use r#box::Box;
pub use responsive::{grid_span_to_percent, Responsive};
#[cfg(any(feature = "yew", feature = "leptos"))]
pub use stack::{Stack, StackDirection};
#[allow(unused_imports)]
pub use style::*;
#[doc(hidden)]
pub use stylist::{css, Style};
pub use theme::{Breakpoints, Palette, Theme};
extern crate self as rustic_ui_styled_engine;
#[cfg(all(not(feature = "yew"), feature = "leptos"))]
pub use theme_provider::ThemeProviderLeptos as ThemeProvider;
#[cfg(feature = "yew")]
pub use theme_provider::ThemeProviderYew as ThemeProvider;
pub use themed_element::{ThemedProps, Variant};
#[cfg(any(feature = "yew", feature = "leptos"))]
pub use typography::{Typography, TypographyVariant};

#[cfg(any(feature = "yew", feature = "leptos"))]
pub(crate) use scoped_class::ScopedClass;

/// Legacy no-op function retained to keep dependent crates compiling while
/// more features are ported. New functionality resides in the modules above.
pub fn placeholder() {}

#[cfg(feature = "compat-mui")]
#[doc = "Deprecated compatibility shim exposing the crate under the legacy `mui_system` name.\n\
Enable the `compat-mui` feature while you migrate imports to `rustic_ui_system`.\n\
This alias will be removed in a future pre-1.0 release."]
#[deprecated(
    since = "0.1.0",
    note = "Update imports to use `rustic_ui_system`. The `mui_system` alias will be removed."
)]
pub use crate as mui_system;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_span_math_is_sound() {
        assert!((grid_span_to_percent(3, 12) - 25.0).abs() < f32::EPSILON);
    }
}
