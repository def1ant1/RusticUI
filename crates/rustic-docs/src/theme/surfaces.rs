//! Higher level Leptos components that wrap the shared Material theme.
//!
//! Each helper hides the raw `ThemeProvider` wiring so content modules can
//! focus on layout concerns.  The components surface palette/typography
//! diagnostics inline which doubles as living documentation for enterprise
//! adopters inspecting how RusticUI primitives react to colour-scheme toggles.

use leptos::{create_memo, view, Children, IntoView, SignalGet};
use rustic_ui_system::theme::ColorScheme;
use rustic_ui_system::theme_provider::{
    material_theme, use_material_color_scheme, use_theme, CssBaseline, ThemeProvider,
};

use super::palette::{baseline_palettes, palette_for_scheme};
use super::typography::typography_scale;

/// Root shell embedding the Rustic Material theme and CSS baseline.
#[allow(
    missing_docs,
    reason = "Leptos macro emits generated prop structs without doc attributes."
)]
#[leptos::component]
pub fn DocsThemeShell(children: Children) -> impl IntoView {
    let theme = material_theme();
    view! {
        <ThemeProvider theme>
            <CssBaseline />
            {children()}
        </ThemeProvider>
    }
}

/// Application bar rendered using Material palette tokens.
#[allow(
    missing_docs,
    reason = "Leptos macro emits generated prop structs without doc attributes."
)]
#[leptos::component]
pub fn DocsAppBar(
    #[doc = "Primary title rendered inside the bar."]
    #[prop(into)]
    title: String,
    #[doc = "Supporting text describing the active page."]
    #[prop(into)]
    subtitle: String,
) -> impl IntoView {
    let theme = use_theme();
    let palette = palette_for_scheme(theme.palette.initial_color_scheme);
    let typography = typography_scale();
    view! {
        <header
            style=format!(
                "display:flex;flex-direction:column;gap:0.25rem;padding:1rem 1.25rem;background:{};color:{};border-radius:0.75rem;",
                palette.primary,
                palette.text_secondary
            )
        >
            <h1
                style=format!(
                    "margin:0;font-family:{};font-size:{:.3}rem;letter-spacing:0.05em;text-transform:uppercase;",
                    typography.font_family,
                    typography.heading_scale[0]
                )
            >
                {title}
            </h1>
            <p style="margin:0;font-size:0.9rem;opacity:0.85;">{subtitle}</p>
        </header>
    }
}

/// Themed container wrapping content using Material-inspired tokens.
#[allow(
    missing_docs,
    reason = "Leptos macro emits generated prop structs without doc attributes."
)]
#[leptos::component]
pub fn DocsSurface(
    #[doc = "Human friendly section title rendered using the typography ramp."]
    #[prop(into)]
    title: String,
    #[doc = "Short description explaining the intent of the surface."]
    #[prop(into)]
    description: String,
    #[doc = "Nested content rendered inside the card."] children: Children,
) -> impl IntoView {
    let theme = use_theme();
    let palette = palette_for_scheme(theme.palette.initial_color_scheme);
    let typography = typography_scale();
    view! {
        <article
            style=format!(
                "background:{};color:{};padding:1.25rem;border-radius:1rem;box-shadow:0 8px 24px rgba(0,0,0,0.08);",
                palette.surface,
                palette.text_primary
            )
        >
            <header style="display:flex;flex-direction:column;gap:0.25rem;margin-bottom:0.75rem;">
                <h2
                    style=format!(
                        "margin:0;font-family:{};font-size:{:.3}rem;",
                        typography.font_family,
                        typography.heading_scale[1]
                    )
                >
                    {title}
                </h2>
                <p style="margin:0;color:rgba(0,0,0,0.65);">{description}</p>
            </header>
            <section style="display:grid;gap:0.75rem;">
                {children()}
            </section>
        </article>
    }
}

/// Interactive palette inspector that toggles the Material colour scheme.
#[allow(
    missing_docs,
    reason = "Leptos macro emits generated prop structs without doc attributes."
)]
#[leptos::component]
pub fn ThemeToggleControl() -> impl IntoView {
    let handle = use_material_color_scheme();
    let scheme_signal = handle.signal();
    let palette = create_memo(move |_| palette_for_scheme(scheme_signal.get()));
    let typography = typography_scale().clone();

    view! {
        <div
            style="display:grid;gap:0.5rem;padding:0.75rem;border-radius:0.75rem;background:rgba(0,0,0,0.04);"
        >
            <p style="margin:0;font-weight:600;">{"Theme diagnostics"}</p>
            <p
                style=format!(
                    "margin:0;font-family:{};font-size:{:.3}rem;",
                    typography.font_family,
                    typography.body_font_size_px
                )
            >
                {move || format!("Primary tone: {}", palette.get().primary)}
            </p>
            <p style="margin:0;">{move || format!("Surface colour: {}", palette.get().surface)}</p>
            <p style="margin:0;">{move || format!("Text contrast: {} / {}", palette.get().text_primary, palette.get().text_secondary)}</p>
            <div style="display:flex;gap:0.75rem;align-items:center;">
                <span style=move || format!("background:{};width:32px;height:24px;border-radius:4px;display:inline-block;", palette.get().background)></span>
                <span style=move || format!("background:{};width:32px;height:24px;border-radius:4px;display:inline-block;", palette.get().surface)></span>
                <span style=move || format!("background:linear-gradient(90deg, {} 0%, {} 100%);width:48px;height:24px;border-radius:4px;display:inline-block;", palette.get().text_primary, palette.get().text_secondary)></span>
            </div>
            <button
                style="margin-top:0.5rem;padding:0.5rem 1rem;border-radius:8px;border:none;cursor:pointer;background:#1f2933;color:white;"
                on:click=move |_| handle.toggle()
            >
                {move || format!("Toggle to {} mode", match palette.get().scheme {
                    ColorScheme::Light => "dark",
                    ColorScheme::Dark => "light",
                })}
            </button>
            <footer style="font-size:0.75rem;color:rgba(0,0,0,0.6);">
                {move || {
                    baseline_palettes()
                        .iter()
                        .map(|snapshot| format!("{}:{}", snapshot.scheme.as_str(), snapshot.primary))
                        .collect::<Vec<_>>()
                        .join(" • ")
                }}
            </footer>
        </div>
    }
}
