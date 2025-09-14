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

/// Render a grid item as a `<div>` with a calculated percentage width.
///
/// Using percentages keeps the layout responsive without forcing the browser
/// to recalculate layout on every resize event.  The function component keeps
/// the render path simple and therefore easy to inline by the optimizer.
#[function_component(Grid)]
pub fn grid(props: &GridProps) -> Html {
    // Convert the span to a percentage so it scales with the parent container.
    let width = grid_span_to_percent(props.span, props.columns);
    // Merge caller supplied styles with the computed width.
    let style = format!("{}width:{}%;", props.style, width);
    html! { <div style={style}>{ for props.children.iter() }</div> }
}
