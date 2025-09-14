#![cfg(feature = "yew")]
use crate::style_props;
use yew::prelude::*;

/// Properties for the [`Container`] component.
#[derive(Properties, PartialEq)]
pub struct ContainerProps {
    /// Optional maximum width of the container (e.g. `"1200px"`).
    #[prop_or_default]
    pub max_width: Option<String>,
    /// Additional style string to merge.
    #[prop_or_default]
    pub style: String,
    /// Child elements to display inside the container.
    #[prop_or_default]
    pub children: Children,
}

/// Centers content with an optional maximum width.
#[function_component(Container)]
pub fn container(props: &ContainerProps) -> Html {
    let mut style = String::new();
    if let Some(mw) = &props.max_width {
        style.push_str(&style_props! { margin_left: "auto", margin_right: "auto", max_width: mw.clone(), width: "100%" });
    } else {
        style.push_str(&style_props! { width: "100%" });
    }
    style.push_str(&props.style);
    html! { <div style={style}>{ for props.children.iter() }</div> }
}
