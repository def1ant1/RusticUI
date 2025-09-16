//! Cross-framework helpers for rendering a themed `<div>` complete with
//! scoped class names, generated CSS and ARIA metadata.
//!
//! The module exposes lightweight adapters for Leptos, Dioxus and Sycamore.
//! Each adapter resolves spacing and colors from the active [`Theme`] before
//! wiring the values into attributes and CSS.  By centralising the work here we
//! avoid hand-writing slightly different class name builders and ARIA maps per
//! framework – a small but compounding win when building enterprise scale
//! design systems.
//!
//! ## Styling logic
//! * `color` - Defaults to [`Theme::palette.primary`] so call sites inherit the
//!   design system's primary accent unless a bespoke value is supplied.
//! * `padding` - Falls back to `theme.spacing(2)` (converted to pixels) to keep
//!   breathing room consistent with Material defaults. Projects can override the
//!   string directly to support complex responsive shorthands while still
//!   benefiting from the theme scale.
//! * `border` - `Variant::Outlined` uses the theme's secondary text colour to
//!   render a subtle divider. Plain variants disable the border entirely so the
//!   element can blend with surrounding layout chrome.
//! * `variant` - Determines the visual treatment. A modifier class using the
//!   [`BEM`](https://en.bem.info/methodology/) naming convention is produced so
//!   downstream CSS can hook into `mui-themed-header--plain` / `--outlined`
//!   without manual concatenation. The scoped class generated via
//!   [`css_with_theme!`](mui_styled_engine_macros::css_with_theme) is appended to
//!   the list so automation can target the component using deterministic names.
//!
//! ## Accessibility
//! Optional ARIA `role` and `aria-label` attributes are emitted to provide
//! assistive technologies with rich context about the container.  The helpers
//! funnel every attribute through [`mui_utils::collect_attributes`] so the same
//! metadata order is preserved across adapters, which keeps hydration and
//! snapshot tests stable and documents exactly how accessibility data is
//! applied to the rendered `<div>`.

use crate::theme_provider::use_theme;
use mui_utils::{attributes_to_html, collect_attributes, extend_attributes};

/// Available visual variants for the themed element.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Variant {
    /// Minimal styling with no border.
    Plain,
    /// Outlined style often used for emphasis.
    Outlined,
}

impl Default for Variant {
    fn default() -> Self {
        Variant::Plain
    }
}

impl Variant {
    /// Returns the modifier portion used in BEM style class names.
    fn modifier(self) -> &'static str {
        match self {
            Variant::Plain => "plain",
            Variant::Outlined => "outlined",
        }
    }
}

/// Properties shared by all adapter implementations.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ThemedProps {
    /// Optional text color. Defaults to the theme's primary palette color.
    pub color: Option<String>,
    /// Optional padding value applied to the element.
    pub padding: Option<String>,
    /// Style variant determining the generated class name.
    pub variant: Variant,
    /// ARIA role announced by assistive technologies.
    pub role: Option<String>,
    /// Human readable label exposed via `aria-label`.
    pub aria_label: Option<String>,
    /// Inner HTML/text rendered inside the element.
    pub child: String,
}

/// Scoped CSS class prefix used by every adapter.  Centralising the constant
/// avoids subtle typos when new integrations are added in the future.
const BASE_CLASS: &str = "mui-themed-header";

/// Convenience type holding precomputed visual tokens.  The helpers below share
/// this struct so that colour, padding and border calculations remain consistent
/// regardless of which adapter triggered the work.
#[derive(Clone, Debug)]
struct VisualTokens {
    text_color: String,
    padding: String,
    background: String,
    border: String,
    gap: String,
}

/// Scoped style artefact produced by [`css_with_theme!`].
///
/// The struct records both the generated class name and the CSS string so SSR
/// adapters can inline the stylesheet while client side frameworks simply reuse
/// the class. Having a dedicated type keeps future automation (for example
/// generating documentation snippets) straightforward.
#[derive(Clone, Debug)]
struct ScopedStyle {
    class: String,
    stylesheet: String,
}

/// Resolves theme driven styling tokens, applying sensible defaults where
/// callers omitted a value.
fn resolve_visual_tokens(props: &ThemedProps) -> VisualTokens {
    let theme = use_theme();
    // Default to the primary palette colour – a safe baseline for enterprise
    // shells where brand accents dominate top level headers.
    let text_color = props
        .color
        .clone()
        .unwrap_or_else(|| theme.palette.primary.clone());
    // Provide a predictable padding default based on the spacing scale so the
    // layout feels harmonious even without explicit configuration.
    let padding = props
        .padding
        .clone()
        .unwrap_or_else(|| format!("{}px", theme.spacing(2)));
    let gap = format!("{}px", theme.spacing(1));
    let (background, border) = match props.variant {
        Variant::Plain => (theme.palette.background_default, "none".to_string()),
        Variant::Outlined => (
            theme.palette.background_paper,
            format!("1px solid {}", theme.palette.text_secondary),
        ),
    };
    VisualTokens {
        text_color,
        padding,
        background,
        border,
        gap,
    }
}

/// Builds a deterministic class list using a BEM style modifier.
fn deterministic_class(variant: Variant) -> String {
    format!("{BASE_CLASS} {BASE_CLASS}--{}", variant.modifier())
}

/// Generates a scoped CSS class and stylesheet using the active [`Theme`].
#[cfg(any(feature = "leptos", feature = "dioxus", feature = "sycamore"))]
fn scoped_style(tokens: &VisualTokens) -> ScopedStyle {
    use mui_styled_engine_macros::css_with_theme;

    // Drive every declaration from theme tokens so updates to palette/spacing
    // cascade through the component without touching presentation code.
    let style = css_with_theme!(
        r#"
            color: ${text_color};
            padding: ${padding};
            background-color: ${background};
            border-bottom: ${border};
            display: flex;
            align-items: center;
            gap: ${gap};
        "#,
        text_color = tokens.text_color.clone(),
        padding = tokens.padding.clone(),
        background = tokens.background.clone(),
        border = tokens.border.clone(),
        gap = tokens.gap.clone()
    );

    let stylesheet = style.get_style_str().to_string();
    let class = style.get_class_name().to_string();
    // Immediately unregister the temporary handle so the style registry remains
    // free of duplicates when multiple adapters render concurrently.
    style.unregister();

    ScopedStyle { class, stylesheet }
}

/// Resolves the deterministic BEM class and scoped theme class in one go.
#[cfg(any(feature = "leptos", feature = "dioxus", feature = "sycamore"))]
fn themed_classes(props: &ThemedProps) -> (String, ScopedStyle) {
    let tokens = resolve_visual_tokens(props);
    let scoped = scoped_style(&tokens);
    let classes = format!("{} {}", deterministic_class(props.variant), scoped.class);
    (classes, scoped)
}

/// Collects HTML attributes shared across adapters.
///
/// The helper funnels the caller supplied class list through
/// [`mui_utils::collect_attributes`] before layering optional ARIA role/label
/// metadata on top. Returning a `Vec` keeps the structure ergonomic for
/// [`attributes_to_html`], SSR renderers and potential future automation.
fn attribute_pairs(props: &ThemedProps, classes: String) -> Vec<(String, String)> {
    let mut attrs = collect_attributes(Some(classes), core::iter::empty::<(String, String)>());
    if let Some(role) = &props.role {
        extend_attributes(&mut attrs, [("role", role.clone())]);
    }
    if let Some(label) = &props.aria_label {
        extend_attributes(&mut attrs, [("aria-label", label.clone())]);
    }
    attrs
}

/// Renders the final container markup, optionally prefixing an inline
/// `<style>` tag for SSR scenarios.
fn render_div(props: &ThemedProps, classes: String, stylesheet: Option<String>) -> String {
    let attrs = attribute_pairs(props, classes);
    let attr_string = attributes_to_html(&attrs);
    let markup = format!("<div {}>{}</div>", attr_string, props.child);
    if let Some(css) = stylesheet {
        format!("<style>{}</style>{}", css, markup)
    } else {
        markup
    }
}

/// Adapter targeting the [`leptos`](https://docs.rs/leptos) framework.
///
/// The implementation relies on [`css_with_theme!`](mui_styled_engine::css_with_theme)
/// so colour and spacing automatically track the active [`Theme`].  A scoped
/// style block is emitted alongside the `<div>` markup ensuring SSR output
/// remains deterministic even without a live style registry.
#[cfg(feature = "leptos")]
pub mod leptos {
    //! Leptos adapter that renders a themed `<div>` while exercising the
    //! shared styling helpers. The generated CSS is inlined to make SSR easy
    //! to hydrate and still exposes the shared BEM class for additional
    //! enterprise customisation layers.
    use super::*;

    /// Render a themed `<div>` with ARIA metadata using Leptos.
    pub fn render(props: &ThemedProps) -> String {
        let (classes, scoped) = themed_classes(props);
        // Feed the generated stylesheet back into the markup so SSR output and
        // client side hydration share the exact same CSS payload.
        render_div(props, classes, Some(scoped.stylesheet))
    }
}

/// Adapter targeting the [`dioxus`](https://dioxuslabs.com) framework.
///
/// Delegates styling to [`resolve_visual_tokens`] ensuring the scoped stylesheet
/// and BEM modifier class mirror the Leptos variant.  The adapter also wires the
/// optional ARIA `role`/`aria-label` attributes into the rendered `<div>` so
/// server rendered output remains accessible without additional plumbing.
#[cfg(feature = "dioxus")]
pub mod dioxus {
    use super::*;

    /// Render a themed `<div>` with ARIA metadata using Dioxus.
    pub fn render(props: &ThemedProps) -> String {
        // Share the same scoped stylesheet as the other adapters so string based
        // renderers remain perfectly in sync with client side components.
        let (classes, scoped) = themed_classes(props);
        render_div(props, classes, Some(scoped.stylesheet))
    }
}

/// Adapter targeting the [`sycamore`](https://sycamore-rs.netlify.app) framework.
///
/// Delegates to the shared helper functions so that Sycamore's SSR adapter emits
/// the same scoped styling, BEM modifier classes and ARIA metadata as the other
/// frameworks.  Keeping the logic central makes future automation (for example
/// generating documentation snippets) straightforward.
#[cfg(feature = "sycamore")]
pub mod sycamore {
    use super::*;

    /// Render a themed `<div>` with ARIA metadata using Sycamore.
    pub fn render(props: &ThemedProps) -> String {
        let (classes, scoped) = themed_classes(props);
        render_div(props, classes, Some(scoped.stylesheet))
    }
}
