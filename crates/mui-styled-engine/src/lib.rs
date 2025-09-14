//! Styling engine for Material UI Rust components.
//!
//! This crate wires together the [`stylist`](https://crates.io/crates/stylist)
//! CSS-in-Rust engine with the [`mui-system`] theme primitives so that
//! components can define scoped styles at compile time.  It provides a thin
//! wrapper around stylist with helpers for theme aware styling, global style
//! injection and server side rendering (SSR) integration.
//!
//! # Examples
//!
//! ```rust
//! use mui_styled_engine::{css_with_theme, Theme};
//!
//! let theme = Theme::default();
//! let style = css_with_theme!(theme, r#"color: ${color};"#, color = theme.palette.primary.clone());
//! assert!(!style.get_class_name().is_empty());
//! ```

pub use mui_system::theme::{Breakpoints, Palette, Theme};
#[cfg(feature = "yew")]
pub use mui_system::theme_provider::{use_theme, ThemeProvider};
// Re-export procedural macros so crate users only depend on one package.
pub use mui_styled_engine_macros::{styled_component, Theme as Theme};
// Ensure procedural macros can reference this crate as `mui_styled_engine` even
// when used internally.
extern crate self as mui_styled_engine;

mod ssr;
pub use ssr::*;

pub use stylist::{css, global_style, Style, StyleSource};

/// Macro helper that exposes the [`stylist::css!`] macro while ensuring the
/// provided theme is available inside the style block.  The macro simply
/// forwards all tokens to `css!` but marks the theme binding, allowing the
/// compiler to enforce that a [`Theme`] is passed.
#[macro_export]
macro_rules! css_with_theme {
    ($theme:expr, $($tt:tt)*) => {{
        let _t: &$crate::Theme = &$theme; // type check only
        $crate::Style::new($crate::css!{ $($tt)* }).expect("valid css")
    }};
}

#[cfg(feature = "yew")]
mod yew_integration {
    use super::*;
    use yew::prelude::*;

    /// Provides a [`Theme`] context to all child components.  This simplified
    /// provider can be expanded in the future to collect styles for SSR, but
    /// for now it keeps compilation light-weight.
    #[derive(Properties, PartialEq)]
    pub struct StyledEngineProviderProps {
        /// Theme made available to all children.
        #[prop_or_default]
        pub theme: Theme,
        /// Child components that require styling.
        #[prop_or_default]
        pub children: Children,
    }

    #[function_component(StyledEngineProvider)]
    pub fn styled_engine_provider(props: &StyledEngineProviderProps) -> Html {
        html! {
            <ThemeProvider theme={props.theme.clone()}>
                { for props.children.iter() }
            </ThemeProvider>
        }
    }
}

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

    #[test]
    fn styles_are_scoped() {
        let a = Style::new(css!(r#"color:red;"#)).unwrap();
        let b = Style::new(css!(r#"color:blue;"#)).unwrap();
        assert_ne!(a.get_class_name(), b.get_class_name());
    }

    #[test]
    fn theme_values_can_be_injected() {
        let theme = Theme::default();
        let style = css_with_theme!(theme, r#"color: ${c};"#, c = theme.palette.primary.clone());
        assert!(style.get_style_str().contains(&theme.palette.primary));
    }

    #[test]
    fn ssr_collects_styles() {
        use stylist::Style;
        let out = render_with_style(|mgr| {
            let style = Style::new_with_manager(css!("color:red;"), mgr).unwrap();
            format!("<div class=\"{}\"></div>", style.get_class_name())
        });
        assert!(out.styles.contains("color: red"), "{}", out.styles);
        assert!(out.html.contains("class"));
    }

    #[test]
    fn theme_can_be_derived() {
        #[derive(Theme)]
        struct Custom { palette: Palette }
        let custom = Custom { palette: Palette { primary: "#fff".into(), ..Palette::default() } };
        let t = custom.into_theme();
        assert_eq!(t.palette.primary, "#fff");
    }
}
