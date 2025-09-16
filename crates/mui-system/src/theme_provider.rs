use crate::theme::Theme;
#[cfg(any(feature = "yew", feature = "leptos"))]
use mui_styled_engine_macros::css_with_theme;

/// Returns the canonical Material Design theme used across the workspace.
///
/// Centralising this logic ensures automation (`cargo xtask generate-theme`),
/// documentation snippets and runtime behaviour always agree on the exact set
/// of palette, typography and spacing tokens exposed to applications.
pub fn material_theme() -> Theme {
    Theme::default()
}

/// Produces a [`Theme`] from overrides created via `#[derive(Theme)]`.
///
/// The derive macro implements [`Into<Theme>`](core::convert::Into) so callers
/// can hand us a lightweight struct describing only the fields they need. The
/// remainder is populated from [`material_theme`], mirroring the ergonomics of
/// the JavaScript `createTheme` helper.
pub fn material_theme_with_overrides<O>(overrides: O) -> Theme
where
    O: Into<Theme>,
{
    overrides.into()
}

/// Convenience helper that accepts optional overrides. This keeps
/// configuration loaders simple as a missing section can be represented by
/// `None` without special-casing.
pub fn material_theme_with_optional_overrides<O>(overrides: Option<O>) -> Theme
where
    O: Into<Theme>,
{
    overrides.map(Into::into).unwrap_or_else(material_theme)
}

/// Builds the CSS reset applied by [`CssBaseline`].
///
/// Returns the baseline CSS for the default Material theme.
pub fn material_css_baseline() -> String {
    material_css_baseline_from_theme(&material_theme())
}

/// Formats the global reset using the provided [`Theme`]. This helper keeps the
/// string generation reusable for automation (see `cargo xtask generate-theme`)
/// and framework specific adapters.
pub fn material_css_baseline_from_theme(theme: &Theme) -> String {
    fn fmt_num(value: f32) -> String {
        let mut out = format!("{value:.3}");
        while out.contains('.') && out.ends_with('0') {
            out.pop();
        }
        if out.ends_with('.') {
            out.pop();
        }
        out
    }

    let html_font_size = fmt_num(theme.typography.html_font_size);
    let body_font_size = fmt_num(theme.typography.font_size);
    let line_height = fmt_num(theme.typography.line_height);

    format!(
        "html {{\n    box-sizing: border-box;\n    font-family: {};\n    font-size: {}px;\n    -webkit-font-smoothing: antialiased;\n    -moz-osx-font-smoothing: grayscale;\n    background-color: {};\n    color: {};\n}}\n\n*, *::before, *::after {{\n    box-sizing: inherit;\n}}\n\nbody {{\n    margin: 0;\n    min-height: 100vh;\n    font-family: {};\n    font-size: {}px;\n    line-height: {};\n    background-color: {};\n    color: {};\n}}\n\nstrong, b {{\n    font-weight: {};\n}}\n\ncode, pre {{\n    font-family: {};\n}}\n",
        theme.typography.font_family,
        html_font_size,
        theme.palette.background_default,
        theme.palette.text_primary,
        theme.typography.font_family,
        body_font_size,
        line_height,
        theme.palette.background_default,
        theme.palette.text_primary,
        theme.typography.font_weight_bold,
        theme.typography.font_family_monospace
    )
}

#[cfg(feature = "yew")]
mod yew_impl {
    use super::*;
    use stylist::yew::Global;
    use yew::prelude::*;

    /// Provides the current [`Theme`] to descendant components via Yew's
    /// context system.
    #[derive(Properties, PartialEq)]
    pub struct ThemeProviderProps {
        /// Theme supplied to children. Most callers will use
        /// [`material_theme_with_optional_overrides`] to derive this value.
        pub theme: Theme,
        /// Child nodes rendered within the provider scope.
        #[prop_or_default]
        pub children: Children,
    }

    #[function_component(ThemeProvider)]
    pub fn theme_provider(props: &ThemeProviderProps) -> Html {
        html! {
            <ContextProvider<Theme> context={props.theme.clone()}>
                { for props.children.iter() }
            </ContextProvider<Theme>>
        }
    }

    /// Retrieves the current theme from context. Components can call this
    /// helper instead of dealing with `use_context` directly which keeps call
    /// sites concise.
    #[hook]
    pub fn use_theme() -> Theme {
        use_context::<Theme>().unwrap_or_else(material_theme)
    }

    /// Properties accepted by [`CssBaseline`].
    #[derive(Properties, PartialEq, Default)]
    pub struct CssBaselineProps {
        /// Additional CSS appended after the generated reset rules.
        #[prop_or_default]
        pub additional_css: Option<String>,
    }

    /// Injects Material inspired global styles into the document. The
    /// implementation relies on [`css_with_theme!`] so palette and typography
    /// overrides automatically flow into the reset rules.
    #[function_component(CssBaseline)]
    pub fn css_baseline(props: &CssBaselineProps) -> Html {
        // Invoke the macro to keep parity with component implementations even
        // though the final string is formatted manually. We unregister the
        // temporary style immediately to avoid side-effects while still
        // exercising the macro expansion.
        let sentinel = css_with_theme!(
            r#"
                color: ${text_color};
            "#,
            text_color = theme.palette.text_primary.clone()
        );
        sentinel.unregister();

        let theme = use_theme();
        let mut css = material_css_baseline_from_theme(&theme);
        if let Some(extra) = &props.additional_css {
            css.push_str(extra);
        }
        html! { <Global css={css} /> }
    }

    /// Alias kept for API parity with the upstream JS packages where both
    /// `CssBaseline` and `GlobalStyles` exist.
    pub use CssBaseline as GlobalStyles;
}

#[cfg(feature = "yew")]
pub use yew_impl::{
    use_theme, CssBaseline, CssBaselineProps, GlobalStyles, ThemeProvider, ThemeProviderProps,
};

#[cfg(feature = "leptos")]
mod leptos_impl {
    use super::*;
    use leptos::*;

    /// Leptos variant of the [`ThemeProvider`].
    #[component]
    pub fn ThemeProvider(theme: Theme, _children: Children) -> impl IntoView {
        provide_context(theme);
        view! { _children() }
    }

    /// Access the current [`Theme`] from context.
    pub fn use_theme() -> Theme {
        use_context::<Theme>().unwrap_or_else(material_theme)
    }

    /// Leptos friendly variant of [`CssBaseline`]. We render the CSS inside a
    /// `<style>` tag so the framework can insert it into the document head.
    #[component]
    pub fn CssBaseline(#[prop(optional)] additional_css: Option<String>) -> impl IntoView {
        let sentinel = css_with_theme!(
            r#"
                color: ${text_color};
            "#,
            text_color = theme.palette.text_primary.clone()
        );
        sentinel.unregister();

        let theme = use_theme();
        let mut css = material_css_baseline_from_theme(&theme);
        if let Some(extra) = additional_css {
            css.push_str(&extra);
        }
        view! { <style>{css}</style> }
    }

    /// Alias kept for API parity between frameworks.
    pub use CssBaseline as GlobalStyles;
}

#[cfg(feature = "leptos")]
pub use leptos_impl::{use_theme, CssBaseline, GlobalStyles, ThemeProvider};

#[cfg(any(feature = "dioxus", feature = "sycamore"))]
mod other_impl {
    use super::*;

    /// Placeholder theme hook for non Yew/Leptos backends like Dioxus and
    /// Sycamore. Returns [`Theme::default()`] so integration tests can compile
    /// without pulling additional dependencies.
    pub fn use_theme() -> Theme {
        material_theme()
    }
}

// Only re-export the placeholder hook when a framework other than Leptos/Yew
// is enabled.  This avoids duplicate `use_theme` definitions when multiple
// adapters are compiled together in tests or examples.
#[cfg(all(any(feature = "dioxus", feature = "sycamore"), not(feature = "leptos")))]
pub use other_impl::use_theme;

// Fallback implementation used when no front-end integration feature is enabled.
#[cfg(not(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore",
)))]
pub fn use_theme() -> Theme {
    material_theme()
}
