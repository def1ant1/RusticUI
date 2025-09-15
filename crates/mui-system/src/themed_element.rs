//! Cross-framework helpers for rendering a themed `<div>` with class names
//! and ARIA metadata.
//!
//! The module exposes lightweight adapters for Leptos, Dioxus and Sycamore.
//! Each adapter delegates to a shared [`render_html`] function that resolves
//! colors and spacing from the active [`Theme`].  A variant specific class name
//! is attached to the element so custom CSS can target the rendering without
//! repetitive boilerplate. Optional ARIA `role` and `aria-label` attributes are
//! emitted to provide additional context to assistive technologies.
//!
//! ## Styling logic
//! * `color` - Defaults to [`Theme::palette.primary`]; callers can override it
//!   to match brand requirements.
//! * `padding` - Raw CSS padding value.  Defaults to `0` when not supplied.
//! * `variant` - High level style variant influencing the generated class
//!   (e.g. `mui-plain` vs. `mui-outlined`).
//!
//! By centralising style computation, downstream crates can avoid repeating
//! manual string concatenation and instead reuse the same logic across multiple
//! front-end frameworks.
//!
//! ## Accessibility
//! Providing an explicit ARIA `role` and human friendly `aria-label` ensures
//! screen readers correctly announce the purpose of the element.  This keeps the
//! generated markup inclusive out of the box while still allowing projects to
//! opt into more advanced semantics when required.

use crate::theme_provider::use_theme;

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

/// Resolve styling and class names based on the [`Theme`] and provided
/// [`ThemedProps`].  The function centralises style generation so each adapter
/// can focus on framework specific rendering.
fn render_html(props: &ThemedProps) -> String {
    let theme = use_theme();
    let color = props
        .color
        .clone()
        .unwrap_or_else(|| theme.palette.primary.clone());
    let padding = props.padding.clone().unwrap_or_else(|| "0".into());
    let class = match props.variant {
        Variant::Plain => "mui-plain",
        Variant::Outlined => "mui-outlined",
    };
    let mut attrs = vec![
        format!(r#"class="{}""#, class),
        format!(r#"style="color:{};padding:{};""#, color, padding),
    ];
    if let Some(role) = &props.role {
        attrs.push(format!(r#"role="{}""#, role));
    }
    if let Some(label) = &props.aria_label {
        attrs.push(format!(r#"aria-label="{}""#, label));
    }
    format!("<div {}>{}</div>", attrs.join(" "), props.child)
}

/// Adapter targeting the [`leptos`](https://docs.rs/leptos) framework.
///
/// Simply forwards to [`render_html`] so that all frameworks share the same
/// styling logic and accessibility guarantees.
#[cfg(feature = "leptos")]
pub mod leptos {
    use super::*;

    /// Render a themed `<div>` with ARIA metadata using Leptos.
    pub fn render(props: &ThemedProps) -> String {
        super::render_html(props)
    }
}

/// Adapter targeting the [`dioxus`](https://dioxuslabs.com) framework.
///
/// Delegates to [`render_html`] to minimise repetitive logic.
#[cfg(feature = "dioxus")]
pub mod dioxus {
    use super::*;

    /// Render a themed `<div>` with ARIA metadata using Dioxus.
    pub fn render(props: &ThemedProps) -> String {
        super::render_html(props)
    }
}

/// Adapter targeting the [`sycamore`](https://sycamore-rs.netlify.app) framework.
///
/// Delegates to [`render_html`] to provide consistent output.
#[cfg(feature = "sycamore")]
pub mod sycamore {
    use super::*;

    /// Render a themed `<div>` with ARIA metadata using Sycamore.
    pub fn render(props: &ThemedProps) -> String {
        super::render_html(props)
    }
}
