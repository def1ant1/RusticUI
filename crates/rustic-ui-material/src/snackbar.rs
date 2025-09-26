//! Snackbar component that surfaces transient feedback with theme aware styling
//! and ARIA metadata.
//!
//! [`css_with_theme!`](mui_styled_engine::css_with_theme) drives color,
//! spacing and border decisions from the active
//! [`Theme`](mui_styled_engine::Theme). The
//! [`style_helpers::themed_class`](crate::style_helpers::themed_class) helper
//! converts generated styles into scoped classes that every adapter consumes
//! while [`style_helpers::themed_attributes_html`](crate::style_helpers::themed_attributes_html)
//! builds ARIA-rich attribute strings for HTML-first renderers. Each variant
//! applies `role="status"` to announce messages politely to assistive
//! technologies.

#[cfg(any(feature = "yew", feature = "dioxus", feature = "sycamore"))]
use mui_styled_engine::{css_with_theme, use_theme, Style, Theme};

#[cfg(feature = "yew")]
use yew::prelude::*;

pub use crate::macros::{Color as SnackbarColor, Size as SnackbarSize, Variant as SnackbarVariant};

#[cfg(any(feature = "yew", feature = "dioxus", feature = "sycamore"))]
fn resolve_style(
    theme: &Theme,
    color: SnackbarColor,
    size: SnackbarSize,
    variant: SnackbarVariant,
) -> (String, &'static str, String) {
    let bg = match color {
        SnackbarColor::Primary => theme.palette.primary.clone(),
        SnackbarColor::Secondary => theme.palette.secondary.clone(),
    };
    let padding = match size {
        SnackbarSize::Small => "4px 8px",
        SnackbarSize::Medium => "8px 16px",
        SnackbarSize::Large => "16px 24px",
    };
    let border = match variant {
        SnackbarVariant::Outlined => format!("1px solid {}", bg.clone()),
        _ => String::from("none"),
    };
    (bg, padding, border)
}

#[cfg(any(feature = "yew", feature = "dioxus", feature = "sycamore"))]
fn snackbar_style(
    theme: &Theme,
    color: SnackbarColor,
    size: SnackbarSize,
    variant: SnackbarVariant,
) -> Style {
    let (bg, padding, border) = resolve_style(theme, color, size, variant);
    css_with_theme!(
        theme,
        r#"
        background: ${bg};
        color: #fff;
        padding: ${padding};
        border: ${border};
    "#,
        bg = bg,
        padding = padding,
        border = border
    )
}

#[cfg(any(feature = "dioxus", feature = "sycamore"))]
fn render_html(
    message: &str,
    color: SnackbarColor,
    size: SnackbarSize,
    variant: SnackbarVariant,
) -> String {
    let theme = use_theme();
    // Shared helper builds the final attribute string so SSR and client
    // adapters stay perfectly aligned.
    let attr_string = crate::style_helpers::themed_attributes_html(
        snackbar_style(&theme, color, size, variant),
        [("role", "status")],
    );
    format!("<div {}>{}</div>", attr_string, message)
}

#[cfg(feature = "yew")]
crate::material_component_props!(SnackbarProps {
    /// Message presented to the user.
    message: String,
});

#[cfg(feature = "yew")]
mod yew_impl {
    use super::*;

    /// Transient feedback component that briefly notifies the user about an
    /// operation.
    #[function_component(Snackbar)]
    pub fn snackbar(props: &SnackbarProps) -> Html {
        let theme = use_theme();
        // Shared helper keeps the scoped class consistent across server and
        // client integrations.
        let class = crate::style_helpers::themed_class(snackbar_style(
            &theme,
            props.color,
            props.size,
            props.variant,
        ));

        html! {
            <div class={class} role="status">{ &props.message }</div>
        }
    }
}

#[cfg(feature = "yew")]
pub use yew_impl::{Snackbar, SnackbarProps};

#[cfg(feature = "dioxus")]
mod dioxus_impl {
    use super::*;

    #[derive(Default, Clone, PartialEq)]
    pub struct SnackbarProps {
        pub message: String,
        pub color: SnackbarColor,
        pub size: SnackbarSize,
        pub variant: SnackbarVariant,
    }

    /// Render the snackbar into a HTML string using the shared helpers.
    pub fn render(props: &SnackbarProps) -> String {
        super::render_html(
            &props.message,
            props.color.clone(),
            props.size.clone(),
            props.variant.clone(),
        )
    }

    /// Backwards compatible shim invoking [`render`].
    #[deprecated(
        since = "0.1.0",
        note = "Use `render` to obtain the HTML string. This shim will be removed once native Dioxus components ship."
    )]
    pub fn Snackbar(props: SnackbarProps) -> String {
        render(&props)
    }
}

#[cfg(feature = "dioxus")]
pub use dioxus_impl::{render as render_dioxus, Snackbar, SnackbarProps};

#[cfg(feature = "sycamore")]
mod sycamore_impl {
    use super::*;

    #[derive(Default, Clone, PartialEq)]
    pub struct SnackbarProps {
        pub message: String,
        pub color: SnackbarColor,
        pub size: SnackbarSize,
        pub variant: SnackbarVariant,
    }

    /// Render the snackbar into a HTML string using the shared helpers.
    pub fn render(props: &SnackbarProps) -> String {
        super::render_html(
            &props.message,
            props.color.clone(),
            props.size.clone(),
            props.variant.clone(),
        )
    }

    /// Backwards compatible shim invoking [`render`].
    #[deprecated(
        since = "0.1.0",
        note = "Use `render` to obtain the HTML string. This shim will be removed once native Sycamore components ship."
    )]
    pub fn Snackbar(props: SnackbarProps) -> String {
        render(&props)
    }
}

#[cfg(feature = "sycamore")]
pub use sycamore_impl::{render as render_sycamore, Snackbar, SnackbarProps};
