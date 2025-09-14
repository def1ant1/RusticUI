#[cfg(any(feature = "yew", feature = "dioxus", feature = "sycamore"))]
use mui_styled_engine::{use_theme, Theme};

#[cfg(feature = "yew")]
use yew::prelude::*;

use crate::material_props;

#[cfg(any(feature = "yew", feature = "dioxus", feature = "sycamore"))]
fn resolve_style(theme: &Theme) -> String {
    format!(
        "border:1px solid {};padding:{}px;",
        theme.palette.primary,
        theme.spacing(2)
    )
}

#[cfg(feature = "yew")]
material_props!(CardProps {
    /// Content of the card.
    children: Children,
});

#[cfg(feature = "yew")]
mod yew_impl {
    use super::*;

    /// Simple container with themed border and padding.
    #[function_component(Card)]
    pub fn card(props: &CardProps) -> Html {
        let theme = use_theme();
        let style = resolve_style(&theme);
        html! { <div style={style}>{ for props.children.iter() }</div> }
    }
}

#[cfg(feature = "yew")]
pub use yew_impl::{Card, CardProps};

#[cfg(feature = "dioxus")]
mod dioxus_impl {
    use super::*;

    #[derive(Default, Clone, PartialEq)]
    pub struct CardProps;

    pub fn Card(_props: CardProps) {
        let theme = use_theme();
        let _ = resolve_style(&theme);
    }
}

#[cfg(feature = "dioxus")]
pub use dioxus_impl::{Card, CardProps};

#[cfg(feature = "sycamore")]
mod sycamore_impl {
    use super::*;

    #[derive(Default, Clone, PartialEq)]
    pub struct CardProps;

    pub fn Card(_props: CardProps) {
        let theme = use_theme();
        let _ = resolve_style(&theme);
    }
}

#[cfg(feature = "sycamore")]
pub use sycamore_impl::{Card, CardProps};
