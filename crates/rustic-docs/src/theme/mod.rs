//! Shared theming helpers for the RusticUI documentation portal.
//!
//! The module exposes palette/typography snapshots alongside high level
//! Leptos components so the rest of the crate can consume RusticUI primitives
//! without duplicating boilerplate.  Each helper is heavily documented to make
//! it trivial for enterprise adopters to extend the theme in their own
//! automation pipelines.

pub mod palette;
pub mod surfaces;
pub mod typography;

pub use palette::{baseline_palettes, palette_for_scheme, PaletteSnapshot};
pub use surfaces::{DocsAppBar, DocsSurface, DocsThemeShell, ThemeToggleControl};
pub use typography::{typography_scale, TypographySnapshot};
