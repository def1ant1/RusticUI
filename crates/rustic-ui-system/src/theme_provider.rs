use crate::theme::{ColorScheme, Theme};
#[cfg(any(feature = "yew", feature = "leptos"))]
use rustic_ui_styled_engine_macros::css_with_theme;

/// Returns the canonical Material Design theme used across the workspace.
///
/// Centralising this logic ensures automation (`cargo xtask generate-theme`),
/// documentation snippets and runtime behaviour always agree on the exact set
/// of palette, typography and spacing tokens exposed to applications.
pub fn material_theme() -> Theme {
    Theme::default()
}

/// Returns the Material theme pre-configured for light mode.
pub fn material_theme_light() -> Theme {
    material_theme_for_scheme(ColorScheme::Light)
}

/// Returns the Material theme pre-configured for dark mode.
pub fn material_theme_dark() -> Theme {
    material_theme_for_scheme(ColorScheme::Dark)
}

/// Builds the canonical Material theme but forces the initial color scheme to
/// the supplied mode.  This helper is the recommended entrypoint for
/// automated pipelines that need to render artifacts for each scheme without
/// duplicating override logic.
pub fn material_theme_for_scheme(scheme: ColorScheme) -> Theme {
    let mut theme = material_theme();
    theme.palette.initial_color_scheme = scheme;
    theme
}

/// Updates the provided [`Theme`] in-place so its active color scheme switches
/// to the requested variant.  Returning the same handle keeps builder style
/// flows ergonomic.
pub fn theme_with_color_scheme(mut theme: Theme, scheme: ColorScheme) -> Theme {
    theme.palette.initial_color_scheme = scheme;
    theme
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

    let active_scheme = theme.palette.initial_color_scheme;
    let active_palette = theme.palette.active();
    let light_palette = &theme.palette.light;
    let dark_palette = &theme.palette.dark;
    let joy_focus_color = theme.joy.focus_color_from_palette(active_palette);
    let joy_focus_outline = theme.joy.focus_outline_for_color(&joy_focus_color);
    let joy_focus_shadow = theme.joy.focus_shadow_for_color(&joy_focus_color);

    format!(
        "/* Global baseline generated from the strongly typed Material theme.\n   Enterprise operators: adjust the `data-mui-color-scheme` attribute on the document element to flip between modes without rebuilding CSS. */\nhtml {{\n    box-sizing: border-box;\n    font-family: {};\n    font-size: {}px;\n    -webkit-font-smoothing: antialiased;\n    -moz-osx-font-smoothing: grayscale;\n    color-scheme: {};\n    background-color: {};\n    color: {};\n}}\n\n*, *::before, *::after {{\n    box-sizing: inherit;\n}}\n\n:root {{\n    color-scheme: {};\n    /* Joy automation hook: the custom properties below stay in sync with `cargo xtask generate-theme --joy`. */\n    --joy-radius: {}px;\n    --joy-focus-outline: {};\n    --joy-focus-shadow: {};\n}}\n\nbody {{\n    margin: 0;\n    min-height: 100vh;\n    font-family: {};\n    font-size: {}px;\n    line-height: {};\n    background-color: {};\n    color: {};\n}}\n\nstrong, b {{\n    font-weight: {};\n}}\n\ncode, pre {{\n    font-family: {};\n}}\n\n/* Data attribute selectors keep automated deployments deterministic by allowing infrastructure to force a mode before JS boots. */\n[data-mui-color-scheme='light'] html,\n[data-mui-color-scheme='light'] body {{\n    background-color: {};\n    color: {};\n}}\n\n[data-mui-color-scheme='light'] :root {{\n    color-scheme: light;\n}}\n\n[data-mui-color-scheme='dark'] html,\n[data-mui-color-scheme='dark'] body {{\n    background-color: {};\n    color: {};\n}}\n\n[data-mui-color-scheme='dark'] :root {{\n    color-scheme: dark;\n}}\n\n/* Respect end-user preference media queries so SSR output automatically matches OS settings even before hydration. */\n@media (prefers-color-scheme: dark) {{\n    :root {{\n        color-scheme: dark;\n    }}\n\n    html, body {{\n        background-color: {};\n        color: {};\n    }}\n}}\n\n@media (prefers-color-scheme: light) {{\n    :root {{\n        color-scheme: light;\n    }}\n\n    html, body {{\n        background-color: {};\n        color: {};\n    }}\n}}\n",
        theme.typography.font_family,
        html_font_size,
        active_scheme.as_str(),
        active_palette.background_default,
        active_palette.text_primary,
        active_scheme.as_str(),
        theme.joy.radius,
        joy_focus_outline,
        joy_focus_shadow,
        theme.typography.font_family,
        body_font_size,
        line_height,
        active_palette.background_default,
        active_palette.text_primary,
        theme.typography.font_weight_bold,
        theme.typography.font_family_monospace,
        light_palette.background_default,
        light_palette.text_primary,
        dark_palette.background_default,
        dark_palette.text_primary,
        dark_palette.background_default,
        dark_palette.text_primary,
        light_palette.background_default,
        light_palette.text_primary,
    )
}

#[cfg(target_arch = "wasm32")]
fn detect_user_prefers_dark() -> Option<bool> {
    web_sys::window()
        .and_then(|window| window.match_media("(prefers-color-scheme: dark)").ok())
        .flatten()
        .map(|media| media.matches())
}

#[cfg(not(target_arch = "wasm32"))]
fn detect_user_prefers_dark() -> Option<bool> {
    None
}

#[cfg(target_arch = "wasm32")]
fn push_color_scheme_to_dom(scheme: ColorScheme) {
    if let Some(document) = web_sys::window().and_then(|window| window.document()) {
        if let Some(root) = document.document_element() {
            let _ = root.set_attribute("data-mui-color-scheme", scheme.as_str());
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn push_color_scheme_to_dom(_scheme: ColorScheme) {}

#[cfg(feature = "yew")]
mod yew_impl {
    use super::*;
    use stylist::yew::Global;
    use yew::prelude::*;

    /// Rich handle returned by [`use_material_color_scheme`] offering helpers to
    /// synchronise UI state, CSS baselines and theme overrides.
    #[derive(Clone)]
    pub struct UseMaterialColorScheme {
        state: UseStateHandle<ColorScheme>,
    }

    impl UseMaterialColorScheme {
        /// Returns the currently resolved [`ColorScheme`].
        pub fn scheme(&self) -> ColorScheme {
            *self.state
        }

        /// Overwrite the active scheme.
        pub fn set(&self, scheme: ColorScheme) {
            self.state.set(scheme);
        }

        /// Convenience helper returning a [`Callback`] that can be passed to
        /// buttons or switches.
        pub fn setter(&self) -> Callback<ColorScheme> {
            let state = self.state.clone();
            Callback::from(move |scheme| state.set(scheme))
        }

        /// Flip between light and dark.
        pub fn toggle(&self) {
            self.state.set(self.scheme().toggled());
        }

        /// Applies the currently selected scheme to the provided [`Theme`]
        /// returning a cloned instance with the correct mode encoded.
        pub fn apply_to(&self, theme: Theme) -> Theme {
            theme_with_color_scheme(theme, self.scheme())
        }
    }

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

    /// Manages the document level colour scheme attribute while exposing a
    /// state handle applications can bind to toggles or store in persistence
    /// layers.  The hook honours user preferences via `matchMedia` on the
    /// initial render and keeps the DOM attribute in sync so CSS selectors flip
    /// immediately even before the component tree re-renders.
    #[hook]
    pub fn use_material_color_scheme() -> UseMaterialColorScheme {
        let theme = use_theme();
        let initial = detect_user_prefers_dark()
            .map(|prefers_dark| {
                if prefers_dark {
                    ColorScheme::Dark
                } else {
                    ColorScheme::Light
                }
            })
            .unwrap_or(theme.palette.initial_color_scheme);

        let state = use_state(|| initial);

        // Ensure the DOM attribute mirrors the current scheme so the CSS reset
        // can rely on `[data-mui-color-scheme]` selectors.
        {
            let state = state.clone();
            use_effect_with(*state, move |scheme: &ColorScheme| {
                push_color_scheme_to_dom(*scheme);
                || ()
            });
        }

        UseMaterialColorScheme { state }
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
        let preview_color = material_theme().palette.active().text_primary.clone();
        let sentinel = css_with_theme!(
            r#"
                color: ${text_color};
            "#,
            text_color = preview_color.clone()
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
    use_material_color_scheme, use_theme, CssBaseline, CssBaselineProps, GlobalStyles,
    ThemeProvider, ThemeProviderProps, UseMaterialColorScheme,
};

#[cfg(feature = "yew")]
pub use yew_impl::{
    use_material_color_scheme as use_material_color_scheme_yew, use_theme as use_theme_yew,
    CssBaseline as CssBaselineYew, CssBaselineProps as CssBaselinePropsYew,
    GlobalStyles as GlobalStylesYew, ThemeProvider as ThemeProviderYew,
    ThemeProviderProps as ThemeProviderPropsYew,
    UseMaterialColorScheme as UseMaterialColorSchemeYew,
};

#[cfg(feature = "leptos")]
mod leptos_impl {
    use super::*;
    use leptos::*;

    /// Handle returned by [`use_material_color_scheme`] for Leptos adapters.
    #[derive(Clone, Copy)]
    pub struct MaterialColorSchemeHandle {
        scheme: RwSignal<ColorScheme>,
    }

    impl MaterialColorSchemeHandle {
        /// Current colour scheme.
        pub fn scheme(&self) -> ColorScheme {
            self.scheme.get()
        }

        /// Expose a read-only signal for UI bindings.
        pub fn signal(&self) -> ReadSignal<ColorScheme> {
            self.scheme.read_only()
        }

        /// Imperatively update the active scheme.
        pub fn set(&self, scheme: ColorScheme) {
            self.scheme.set(scheme);
        }

        /// Toggle helper mirroring the Yew implementation.
        pub fn toggle(&self) {
            self.scheme.update(|current| *current = current.toggled());
        }

        /// Apply the scheme to a cloned [`Theme`].
        pub fn apply_to(&self, theme: Theme) -> Theme {
            theme_with_color_scheme(theme, self.scheme())
        }
    }

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
        let preview_color = material_theme().palette.active().text_primary.clone();
        let sentinel = css_with_theme!(
            r#"
                color: ${text_color};
            "#,
            text_color = preview_color.clone()
        );
        sentinel.unregister();

        let theme = use_theme();
        let mut css = material_css_baseline_from_theme(&theme);
        if let Some(extra) = additional_css {
            css.push_str(&extra);
        }
        view! { <style>{css}</style> }
    }

    /// Leptos hook mirroring [`use_material_color_scheme`] for Yew.  Returns a
    /// handle that drives UI elements and keeps the DOM attribute in sync for
    /// the generated CSS selectors.
    pub fn use_material_color_scheme() -> MaterialColorSchemeHandle {
        let theme = use_theme();
        let initial = detect_user_prefers_dark()
            .map(|prefers_dark| {
                if prefers_dark {
                    ColorScheme::Dark
                } else {
                    ColorScheme::Light
                }
            })
            .unwrap_or(theme.palette.initial_color_scheme);

        let scheme = create_rw_signal(initial);

        create_effect(move |_| {
            push_color_scheme_to_dom(scheme.get());
        });

        MaterialColorSchemeHandle { scheme }
    }

    /// Alias kept for API parity between frameworks.
    pub use CssBaseline as GlobalStyles;
}

#[cfg(feature = "leptos")]
pub use leptos_impl::MaterialColorSchemeHandle;

#[cfg(all(feature = "leptos", not(feature = "yew")))]
pub use leptos_impl::{
    use_material_color_scheme, use_theme, CssBaseline, GlobalStyles, ThemeProvider,
};

#[cfg(feature = "leptos")]
pub use leptos_impl::{
    use_material_color_scheme as use_material_color_scheme_leptos, use_theme as use_theme_leptos,
    CssBaseline as CssBaselineLeptos, GlobalStyles as GlobalStylesLeptos,
    ThemeProvider as ThemeProviderLeptos,
};

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
