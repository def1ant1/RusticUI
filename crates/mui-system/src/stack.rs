#![cfg(feature = "yew")]
use crate::style_props;
use yew::prelude::*;

/// Direction of children placement inside [`Stack`].
#[derive(Clone, PartialEq)]
pub enum StackDirection {
    Row,
    Column,
}

/// Properties for the [`Stack`] component.
#[derive(Properties, PartialEq)]
pub struct StackProps {
    /// Orientation of the stack. Defaults to vertical column layout.
    #[prop_or_default]
    pub direction: Option<StackDirection>,
    /// Gap between children. Accepts any CSS length value.
    #[prop_or_default]
    pub spacing: Option<String>,
    /// Additional arbitrary style string merged into the generated CSS.
    #[prop_or_default]
    pub style: String,
    /// Child elements to render.
    #[prop_or_default]
    pub children: Children,
}

/// Minimal flexbox based stack layout.
///
/// ```rust
/// use mui_system::Stack;
/// use mui_system::style_props;
/// # #[cfg(feature = "yew")]
/// # fn render() -> yew::Html {
/// html! {<Stack spacing={Some("8px".into())}>{"item"}</Stack>}
/// # }
/// ```
#[function_component(Stack)]
pub fn stack(props: &StackProps) -> Html {
    let direction = match props.direction {
        Some(StackDirection::Row) => "row",
        _ => "column",
    };
    let mut style = style_props! { display: "flex", flex_direction: direction };
    if let Some(spacing) = &props.spacing {
        style.push_str(&style_props! { gap: spacing.clone() });
    }
    style.push_str(&props.style);
    html! { <div style={style}>{ for props.children.iter() }</div> }
}
