#![cfg(feature = "yew")]
use yew::prelude::*;
use crate::responsive::grid_span_to_percent;

/// Simple grid item that computes its own width based on `span` and `columns`.
#[derive(Properties, PartialEq)]
pub struct GridProps {
    /// Total number of columns in the grid container.
    #[prop_or(12)]
    pub columns: u16,
    /// Number of columns this item should span.
    #[prop_or(12)]
    pub span: u16,
    /// Additional inline styles merged with the computed width.
    #[prop_or_default]
    pub style: String,
    /// Child elements of the grid item.
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Grid)]
pub fn grid(props: &GridProps) -> Html {
    let width = grid_span_to_percent(props.span, props.columns);
    let style = format!("{}width:{}%;", props.style, width);
    html! { <div style={style}>{ for props.children.iter() }</div> }
}
