#![cfg(feature = "yew")]
use yew::prelude::*;

/// Lightweight wrapper around a `div` that accepts style properties.
#[derive(Properties, PartialEq)]
pub struct BoxProps {
    /// Inline CSS style string typically generated via the `style_props!` macro.
    #[prop_or_default]
    pub style: String,
    /// Child elements to render inside the container.
    #[prop_or_default]
    pub children: Children,
}

/// Basic building block analogous to the `Box` component from MUI's JS
/// ecosystem. Additional props and behaviors can be layered on over time.
#[function_component(Box)]
pub fn box_component(props: &BoxProps) -> Html {
    html! { <div style={props.style.clone()}>{ for props.children.iter() }</div> }
}
