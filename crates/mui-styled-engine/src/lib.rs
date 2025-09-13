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

pub use stylist::{css, global_style, Style, StyleSource};

/// Macro helper that exposes the [`stylist::css!`] macro while ensuring the
/// provided theme is available inside the style block.  The macro simply
/// forwards all tokens to `css!` but marks the theme binding, allowing the
/// compiler to enforce that a [`Theme`] is passed.
#[macro_export]
macro_rules! css_with_theme {
    ($theme:expr, $($tt:tt)*) => {{
        let _t: &$crate::Theme = &$theme; // type check only
        stylist::Style::new(stylist::css!{ $($tt)* }).expect("valid css")
    }};
}

#[cfg(feature = "yew")]
mod yew_integration {
    use super::*;
    use stylist::yew::{GlobalStyle, StyleManager, StyleSheet, StyleSource};
    use yew::prelude::*;

    /// Injects global CSS rules into the document head. The styles are scoped
    /// by [`stylist`] ensuring they will not conflict with other components.
    #[derive(Properties, PartialEq)]
    pub struct GlobalStylesProps {
        /// Raw CSS rules to be injected globally.
        pub styles: String,
    }

    #[function_component(GlobalStyles)]
    pub fn global_styles(props: &GlobalStylesProps) -> Html {
        let gs = GlobalStyle::new(props.styles.clone()).expect("valid CSS");
        gs.to_tag()
    }

    /// Provides a [`StyleManager`] and [`Theme`] context to all child
    /// components.  When a `manager` is supplied it will be used to collect
    /// styles for server side rendering.
    #[derive(Properties, PartialEq)]
    pub struct StyledEngineProviderProps {
        /// Optional style manager used during SSR to collect styles.
        #[prop_or_default]
        pub manager: Option<StyleManager>,
        /// Theme made available to all children.
        #[prop_or_default]
        pub theme: Theme,
        /// Child components that require styling.
        #[prop_or_default]
        pub children: Children,
    }

    #[function_component(StyledEngineProvider)]
    pub fn styled_engine_provider(props: &StyledEngineProviderProps) -> Html {
        let mgr = props.manager.clone().unwrap_or_default();
        html! {
            <stylist::yew::StyleRoot manager={mgr}>
                <ThemeProvider theme={props.theme.clone()}>
                    { for props.children.iter() }
                </ThemeProvider>
            </stylist::yew::StyleRoot>
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
}

