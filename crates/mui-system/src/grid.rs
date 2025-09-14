use crate::{responsive::grid_span_to_percent, style};

#[cfg(feature = "yew")]
mod yew_impl {
    use super::*;
    use yew::prelude::*;

    /// Simple grid item that computes its own width based on `span` and `columns`.
    #[derive(Properties, PartialEq)]
    pub struct GridProps {
        /// Total number of columns in the grid container.
        #[prop_or(12)]
        pub columns: u16,
        /// Number of columns this item should span.
        #[prop_or(12)]
        pub span: u16,
        /// Flexbox alignment on the main axis when using a flex container.
        #[prop_or_default]
        pub justify_content: Option<String>,
        /// Flexbox alignment on the cross axis.
        #[prop_or_default]
        pub align_items: Option<String>,
        /// Additional inline styles merged with the computed width.
        #[prop_or_default]
        pub sx: String,
        /// Child elements of the grid item.
        #[prop_or_default]
        pub children: Children,
    }

    /// Render a grid item as a `<div>` with a calculated percentage width.
    #[function_component(Grid)]
    pub fn grid(props: &GridProps) -> Html {
        let width = grid_span_to_percent(props.span, props.columns);
        let mut style_string = format!("{}width:{}%;", props.sx, width);
        if let Some(jc) = &props.justify_content {
            style_string.push_str(&style::justify_content(jc.clone()));
        }
        if let Some(ai) = &props.align_items {
            style_string.push_str(&style::align_items(ai.clone()));
        }
        html! { <div style={style_string}>{ for props.children.iter() }</div> }
    }
}

#[cfg(feature = "yew")]
pub use yew_impl::{Grid, GridProps};

#[cfg(feature = "leptos")]
mod leptos_impl {
    use super::*;
    use leptos::*;

    /// Leptos implementation of [`Grid`].
    #[component]
    pub fn Grid(
        #[prop(optional)] columns: Option<u16>,
        #[prop(optional)] span: Option<u16>,
        #[prop(optional, into)] justify_content: Option<String>,
        #[prop(optional, into)] align_items: Option<String>,
        #[prop(optional, into)] sx: String,
        children: Children,
    ) -> impl IntoView {
        let columns = columns.unwrap_or(12);
        let span = span.unwrap_or(12);
        let width = grid_span_to_percent(span, columns);
        let mut style_string = format!("{}width:{}%;", sx, width);
        if let Some(jc) = justify_content {
            style_string.push_str(&style::justify_content(jc));
        }
        if let Some(ai) = align_items {
            style_string.push_str(&style::align_items(ai));
        }
        view! { <div style=style_string>{children()}</div> }
    }
}

#[cfg(feature = "leptos")]
pub use leptos_impl::Grid;
