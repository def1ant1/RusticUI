//! Simple container with a themed border and padding.
//!
//! ## Style generation & theme integration
//! * [`css_with_theme!`](rustic_ui_styled_engine::css_with_theme) centralizes styling.
//!   Both the border color and interior spacing are pulled from the active
//!   [`Theme`](rustic_ui_styled_engine::Theme) so applications remain visually
//!   consistent.
//! * [`style_helpers::themed_class`](crate::style_helpers::themed_class) wraps
//!   the generated [`Style`](rustic_ui_styled_engine::Style) and hands back the scoped
//!   class each adapter applies. Documenting the helper here keeps future
//!   modules aligned with the established lifecycle for styled engine handles.
//!
//! ## Accessibility hooks
//! Cards render as simple `<div>` containers and therefore intentionally omit
//! additional ARIA metadata. Surface areas that require labelled semantics (for
//! example dialogs or app bars) should lean on
//! [`style_helpers::themed_attributes_html`](crate::style_helpers::themed_attributes_html)
//! as shown in other modules. Recording that expectation here keeps the pattern
//! front-of-mind for teams composing more complex surfaces.

#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore",
))]
use rustic_ui_styled_engine::{css_with_theme, Style};

#[cfg(feature = "leptos")]
use leptos::Children;
#[cfg(feature = "yew")]
use yew::prelude::*;

#[cfg(any(feature = "yew", feature = "leptos"))]
use crate::material_props;

/// Generates the [`Style`] used to render the card with the active [`Theme`].
#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]
fn resolve_style() -> Style {
    css_with_theme!(
        r#"
        border: 1px solid ${border};
        padding: ${pad};
        "#,
        border = theme.palette.primary.clone(),
        pad = format!("{}px", theme.spacing(2))
    )
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
        // Shared helper keeps the scoped class consistent across all adapters.
        let class = crate::style_helpers::themed_class(resolve_style());
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
        // Shared helper keeps the scoped class consistent across all adapters.
        let class = crate::style_helpers::themed_class(resolve_style());
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
        // Shared helper keeps the scoped class consistent across all adapters.
        let class = crate::style_helpers::themed_class(resolve_style());
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
        // Shared helper keeps the scoped class consistent across all adapters.
        let class = crate::style_helpers::themed_class(resolve_style());
        format!("<div class=\"{}\">{}</div>", class, props.children)
    }
}
