use mui_system::theme_provider::use_theme;
use yew::prelude::*;

use crate::helpers::{resolve_surface_tokens, SurfaceTokens};
use crate::{joy_component_props, Color, Variant};

joy_component_props!(CardProps {
    /// Nested content displayed within the card body.
    children: Children,
});

/// Joy UI card built on top of shared design token helpers.
///
/// # Design tokens
/// * [`helpers::resolve_surface_tokens`](crate::helpers::resolve_surface_tokens) aligns the
///   background/border styling with Joy variants.
/// * [`Theme::spacing`](mui_system::theme::Theme::spacing) drives the padding rhythm.
///
/// # Headless state contract
/// Cards are purely presentational today and therefore do not bind to a headless state machine.
/// Downstream adapters simply render the resolved tokens making this component safe to reuse
/// across Yew, Leptos, Dioxus, and Sycamore.
#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    let theme = use_theme();

    let surface: SurfaceTokens =
        resolve_surface_tokens(&theme, props.color.clone(), props.variant.clone());
    let padding = format!("{}px", theme.spacing(2));
    let style = surface.compose([
        ("padding", padding),
        ("display", "block".to_string()),
        ("box-sizing", "border-box".to_string()),
    ]);

    html! { <div style={style}>{ for props.children.iter() }</div> }
}
