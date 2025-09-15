//! Simple container with a themed border and padding.
//!
//! The card demonstrates how `css_with_theme!` centralizes styling. Both the
//! border color and interior spacing are pulled from the active
//! [`Theme`](mui_styled_engine::Theme) so applications remain visually
//! consistent. The generated class is attached to the root `<div>` element in
//! every adapter which keeps markup lean and avoids repetitive inline styles.

use mui_styled_engine::css_with_theme;

#[cfg(feature = "leptos")]
use leptos::Children;
#[cfg(feature = "yew")]
use yew::prelude::*;

use crate::material_props;

/// Generates a scoped CSS class using the active [`Theme`].
#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]
fn resolve_class() -> String {
    let style = css_with_theme!(
        r#"
        border: 1px solid ${border};
        padding: ${pad};
        "#,
        border = theme.palette.primary.clone(),
        pad = format!("{}px", theme.spacing(2))
    );
    style.get_class_name().to_string()
}

// ---------------------------------------------------------------------------
// Shared Yew/Leptos props
// ---------------------------------------------------------------------------

#[cfg(any(feature = "yew", feature = "leptos"))]
material_props!(CardProps {
    /// Content of the card.
    children: Children,
});

// ---------------------------------------------------------------------------
// Yew adapter
// ---------------------------------------------------------------------------

#[cfg(feature = "yew")]
mod yew_impl {
    use super::*;

    /// Simple container with themed border and padding.
    #[function_component(Card)]
    pub fn card(props: &CardProps) -> Html {
        let class = resolve_class();
        html! { <div class={class}>{ for props.children.iter() }</div> }
    }
}

#[cfg(feature = "yew")]
pub use yew_impl::Card;

// ---------------------------------------------------------------------------
// Leptos adapter
// ---------------------------------------------------------------------------

#[cfg(feature = "leptos")]
mod leptos_impl {
    use super::*;
    use leptos::*;

    /// Leptos variant rendering a `<div>` with theme-derived styling.
    #[component]
    pub fn Card(props: CardProps) -> impl IntoView {
        let class = resolve_class();
        view! { <div class=class>{props.children()}</div> }
    }
}

#[cfg(feature = "leptos")]
pub use leptos_impl::Card;

#[cfg(any(feature = "yew", feature = "leptos"))]
pub use CardProps;

// ---------------------------------------------------------------------------
// Dioxus adapter
// ---------------------------------------------------------------------------

#[cfg(feature = "dioxus")]
pub mod dioxus {
    use super::*;

    /// Properties consumed by the Dioxus adapter.
    #[derive(Default, Clone, PartialEq)]
    pub struct CardProps {
        /// Inner HTML rendered inside the card.
        pub children: String,
    }

    /// Render the card into a `<div>` tag with a theme-derived class.
    pub fn render(props: &CardProps) -> String {
        let class = super::resolve_class();
        format!("<div class=\"{}\">{}</div>", class, props.children)
    }
}

// ---------------------------------------------------------------------------
// Sycamore adapter
// ---------------------------------------------------------------------------

#[cfg(feature = "sycamore")]
pub mod sycamore {
    use super::*;

    /// Properties consumed by the Sycamore adapter.
    #[derive(Default, Clone, PartialEq)]
    pub struct CardProps {
        /// Inner HTML rendered inside the card.
        pub children: String,
    }

    /// Render the card into plain HTML with a themed class.
    pub fn render(props: &CardProps) -> String {
        let class = super::resolve_class();
        format!("<div class=\"{}\">{}</div>", class, props.children)
    }
}
