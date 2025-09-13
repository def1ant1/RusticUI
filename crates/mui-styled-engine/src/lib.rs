//! Core styling utilities shared across Material UI Rust components.
//! 
//! This crate centralizes theming so higher level widgets only depend on a
//! single source for style information. It currently re-exports the theme
//! primitives from [`mui-system`].

pub use mui_system::theme::{Theme, Palette, Breakpoints};
#[cfg(feature = "yew")]
pub use mui_system::theme_provider::{ThemeProvider, use_theme};

/// Placeholder to prove the crate links correctly.
pub fn placeholder() {
    mui_system::placeholder();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_theme_available() {
        let t = Theme::default();
        assert_eq!(t.palette.primary, "#1976d2");
    }
}
