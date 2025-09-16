//! Cross-framework helpers for rendering a themed `<header>` complete with
//! scoped class names and ARIA metadata.
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
//!   string directly to support complex responsive shorthands.
//! * `variant` - Determines the visual treatment. A modifier class using the
//!   [`BEM`](https://en.bem.info/methodology/) naming convention is produced so
//!   downstream CSS can hook into `mui-themed-header--plain` / `--outlined`
//!   without manual concatenation.
//!
//! ## Accessibility
//! Optional ARIA `role` and `aria-label` attributes are emitted to provide
//! assistive technologies with rich context about the header.  Centralising this
//! logic ensures the SSR-focused adapters (Dioxus & Sycamore) and the Leptos
//! component render identical accessibility metadata, eliminating a whole class
//! of drift bugs that are notoriously hard to catch in manual testing.

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

/// Formats inline styles for SSR adapters that cannot rely on live style
/// managers.  The values mirror those produced by the Leptos component so the
/// rendered markup remains visually consistent across frameworks.
#[cfg(any(feature = "dioxus", feature = "sycamore"))]
fn inline_style(tokens: &VisualTokens) -> String {
    format!(
        "color:{};padding:{};background-color:{};border-bottom:{};display:flex;align-items:center;gap:{};",
        tokens.text_color, tokens.padding, tokens.background, tokens.border, tokens.gap
    )
}

/// Collects HTML attributes shared across adapters.  `style` is optional so the
/// Leptos implementation can rely on generated CSS classes while SSR adapters
/// emit inline styling for deterministic output.
fn attribute_pairs(
    props: &ThemedProps,
    classes: String,
    style: Option<String>,
) -> Vec<(String, String)> {
    let mut attrs = collect_attributes(Some(classes), style.into_iter().map(|s| ("style", s)));
    if let Some(role) = &props.role {
        extend_attributes(&mut attrs, [("role", role.clone())]);
    }
    if let Some(label) = &props.aria_label {
        extend_attributes(&mut attrs, [("aria-label", label.clone())]);
    }
    attrs
}

/// Renders the final header markup using precomputed attributes.
fn render_header(props: &ThemedProps, classes: String, style: Option<String>) -> String {
    let attrs = attribute_pairs(props, classes, style);
    let attr_string = attributes_to_html(&attrs);
    format!("<header {}>{}</header>", attr_string, props.child)
}

/// Adapter targeting the [`leptos`](https://docs.rs/leptos) framework.
///
/// The implementation relies on [`css_with_theme!`](mui_styled_engine::css_with_theme)
/// so colour and spacing automatically track the active [`Theme`].  A scoped
/// style block is emitted alongside the `<header>` markup ensuring SSR output
/// remains deterministic even without a live style registry.
#[cfg(feature = "leptos")]
pub mod leptos {
    //! Leptos adapter that renders a themed `<header>` while exercising the
    //! `css_with_theme!` macro.  The generated CSS is inlined to make SSR easy
    //! to hydrate and still exposes the shared BEM class for additional
    //! enterprise customisation layers.
    use super::*;
    use mui_styled_engine_macros::css_with_theme;

    /// Render a themed `<header>` with ARIA metadata using Leptos.
    pub fn render(props: &ThemedProps) -> String {
        let tokens = resolve_visual_tokens(props);
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
        // Capture the generated stylesheet before unregistering the temporary
        // style handle.  The CSS is then inlined to keep the SSR adapter fully
        // self-contained.
        let stylesheet = style.get_style_str().to_string();
        let scoped = style.get_class_name().to_string();
        style.unregister();
        let classes = format!("{} {}", deterministic_class(props.variant), scoped);
        let header = render_header(props, classes, None);
        format!("<style>{}</style>{}", stylesheet, header)
    }
}

/// Adapter targeting the [`dioxus`](https://dioxuslabs.com) framework.
///
/// Delegates styling to [`resolve_visual_tokens`] ensuring the inline CSS and
/// BEM modifier class mirror the Leptos variant.  The adapter also wires the
/// optional ARIA `role`/`aria-label` attributes into the rendered `<header>` so
/// server rendered output remains accessible without additional plumbing.
#[cfg(feature = "dioxus")]
pub mod dioxus {
    use super::*;

    /// Render a themed `<header>` with ARIA metadata using Dioxus.
    pub fn render(props: &ThemedProps) -> String {
        let tokens = resolve_visual_tokens(props);
        let classes = deterministic_class(props.variant);
        let style = inline_style(&tokens);
        render_header(props, classes, Some(style))
    }
}

/// Adapter targeting the [`sycamore`](https://sycamore-rs.netlify.app) framework.
///
/// Delegates to the shared helper functions so that Sycamore's SSR adapter emits
/// the same inline styling, BEM modifier classes and ARIA metadata as the other
/// frameworks.  Keeping the logic central makes future automation (for example
/// generating documentation snippets) straightforward.
#[cfg(feature = "sycamore")]
pub mod sycamore {
    use super::*;

    /// Render a themed `<header>` with ARIA metadata using Sycamore.
    pub fn render(props: &ThemedProps) -> String {
        let tokens = resolve_visual_tokens(props);
        let classes = deterministic_class(props.variant);
        let style = inline_style(&tokens);
        render_header(props, classes, Some(style))
    }
}
