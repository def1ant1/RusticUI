use crate::{
    responsive::{grid_span_to_percent, Responsive},
    style,
    theme::Breakpoints,
};

/// Shared styling routine leveraged by every framework specific grid
/// implementation and the integration tests.  Centralising the logic keeps the
/// breakpoint cascade identical regardless of whether the consumer renders via
/// Yew, Leptos or a headless test harness.
#[doc(hidden)]
pub fn build_grid_style(
    width: u32,
    breakpoints: &Breakpoints,
    columns: Option<&Responsive<u16>>,
    span: Option<&Responsive<u16>>,
    justify_content: Option<&str>,
    align_items: Option<&str>,
    sx: &str,
) -> String {
    // Compute the active column model for the current viewport.  We lean on the
    // `Responsive::constant` helper so callers can omit props entirely and
    // still reuse the same resolution pipeline used for explicit overrides.
    let default_columns = Responsive::constant(12);
    let resolved_columns = columns
        .map(|value| value.resolve(width, breakpoints))
        .unwrap_or_else(|| default_columns.resolve(width, breakpoints));
    let default_span = Responsive::constant(12);
    let resolved_span = span
        .map(|value| value.resolve(width, breakpoints))
        .unwrap_or_else(|| default_span.resolve(width, breakpoints));
    let width_percent = grid_span_to_percent(resolved_span, resolved_columns);

    let mut style_string = String::from(sx);
    style_string.push_str(&format!("width:{}%;", width_percent));

    if let Some(jc) = justify_content {
        style_string.push_str(&style::justify_content(jc));
    }
    if let Some(ai) = align_items {
        style_string.push_str(&style::align_items(ai));
    }

    style_string
}

#[cfg(feature = "yew")]
mod yew_impl {
    use super::*;
    use yew::prelude::*;

    /// Simple grid item that computes its own width based on `span` and `columns`.
    #[derive(Properties, PartialEq)]
    pub struct GridProps {
        /// Total number of columns in the grid container. `None` defaults to 12
        /// so existing layouts continue to render exactly as before.
        #[prop_or_default]
        pub columns: Option<Responsive<u16>>,
        /// Number of columns this item should span at each breakpoint.
        #[prop_or_default]
        pub span: Option<Responsive<u16>>,
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
        let theme = crate::theme_provider::use_theme();
        let width = crate::responsive::viewport_width();
        let style_string = build_grid_style(
            width,
            &theme.breakpoints,
            props.columns.as_ref(),
            props.span.as_ref(),
            props.justify_content.as_deref(),
            props.align_items.as_deref(),
            &props.sx,
        );
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
        #[prop(optional)] columns: Option<Responsive<u16>>,
        #[prop(optional)] span: Option<Responsive<u16>>,
        #[prop(optional, into)] justify_content: Option<String>,
        #[prop(optional, into)] align_items: Option<String>,
        #[prop(optional, into)] sx: String,
        children: Children,
    ) -> impl IntoView {
        let theme = crate::theme_provider::use_theme();
        let width_px = crate::responsive::viewport_width();
        let style_string = build_grid_style(
            width_px,
            &theme.breakpoints,
            columns.as_ref(),
            span.as_ref(),
            justify_content.as_deref(),
            align_items.as_deref(),
            &sx,
        );
        view! { <div style=style_string>{children()}</div> }
    }
}

#[cfg(feature = "leptos")]
pub use leptos_impl::Grid;
