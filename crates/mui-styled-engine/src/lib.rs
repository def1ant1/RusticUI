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
//! // `css_with_theme!` automatically retrieves the current theme and exposes a
//! // `theme` binding that can be used in value substitutions.
//! let style = css_with_theme!(r#"color: ${p};"#, p = theme.palette.primary.clone());
//! assert!(!style.get_class_name().is_empty());
//! ```

pub use mui_system::theme::{Breakpoints, Palette, Theme};
pub use mui_system::theme_provider::use_theme;
#[cfg(feature = "yew")]
pub use mui_system::theme_provider::ThemeProvider;
#[cfg(feature = "leptos")]
pub use mui_system::theme_provider::ThemeProvider;
// Re-export procedural macros so crate users only depend on one package.
pub use mui_styled_engine_macros::{css_with_theme, styled_component, Theme};
// Ensure procedural macros can reference this crate as `mui_styled_engine` even
// when used internally.
extern crate self as mui_styled_engine;

mod ssr;
pub use ssr::*;

mod context;
pub use context::*;

pub use stylist::{css, global_style, Style, StyleSource};

#[cfg(feature = "yew")]
mod yew_integration {
    use super::*;
    use yew::prelude::*;

    /// Properties accepted by [`StyledEngineProvider`].
    #[derive(Properties, PartialEq)]
    pub struct StyledEngineProviderProps {
        /// Theme made available to all children.
        #[prop_or_default]
        pub theme: Theme,
        /// Optional registry allowing callers to reuse style collections across
        /// renders (useful during SSR). If omitted a fresh registry is created
        /// per provider instance.
        #[prop_or_default]
        pub registry: Option<StyleRegistry>,
        /// Child components that require styling.
        #[prop_or_default]
        pub children: Children,
    }

    /// Yew component that wires [`ThemeProvider`] with a shared [`StyleRegistry`].
    ///
    /// The registry is exposed via a [`ContextProvider`] so nested components can
    /// obtain the [`StyleManager`] and record CSS as they render. After the tree
    /// is rendered on the server, calling [`StyledEngineProvider::flush_styles`]
    /// will return all accumulated `<style>` blocks.
    pub struct StyledEngineProvider {
        registry: StyleRegistry,
    }

    impl StyledEngineProvider {
        /// Drains collected styles from the internal registry.
        pub fn flush_styles(&self) -> String {
            self.registry.flush_styles()
        }
    }

    impl Component for StyledEngineProvider {
        type Message = ();
        type Properties = StyledEngineProviderProps;

        fn create(ctx: &Context<Self>) -> Self {
            let registry = ctx
                .props()
                .registry
                .clone()
                .unwrap_or_else(|| StyleRegistry::new(ctx.props().theme.clone()));
            Self { registry }
        }

        fn changed(&mut self, ctx: &Context<Self>, _old: &Self::Properties) -> bool {
            // Always refresh the registry on prop change to avoid leaking styles
            // from previous renders. Callers can supply an existing registry via
            // props if they want to retain styles across renders.
            self.registry = ctx
                .props()
                .registry
                .clone()
                .unwrap_or_else(|| StyleRegistry::new(ctx.props().theme.clone()));
            true
        }

        fn view(&self, ctx: &Context<Self>) -> Html {
            html! {
                <ContextProvider<StyleRegistry> context={self.registry.clone()}>
                    <ThemeProvider theme={ctx.props().theme.clone()}>
                        { for ctx.props().children.iter() }
                    </ThemeProvider>
                </ContextProvider<StyleRegistry>>
            }
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
        struct Custom {
            palette: Palette,
        }
        let custom = Custom {
            palette: Palette {
                primary: "#fff".into(),
                ..Palette::default()
            },
        };
        let t = custom.into_theme();
        assert_eq!(t.palette.primary, "#fff");
    }

    #[test]
    fn theme_derive_handles_nested_and_option() {
        struct PaletteOverride {
            primary: String,
        }
        impl From<PaletteOverride> for Palette {
            fn from(p: PaletteOverride) -> Self {
                Palette { primary: p.primary, ..Palette::default() }
            }
        }
        #[derive(Theme)]
        struct Wrapper {
            palette: Option<PaletteOverride>,
        }
        let t = Wrapper {
            palette: Some(PaletteOverride { primary: "#000".into() }),
        }
        .into_theme();
        assert_eq!(t.palette.primary, "#000");
    }
}
