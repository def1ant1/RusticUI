use crate::{
    responsive::{grid_span_to_percent, Responsive},
    style,
    theme::Breakpoints,
};
use rustic_ui_utils::deep_merge;
use serde_json::{Map, Value};

fn insert_declaration(map: &mut Map<String, Value>, declaration: String) {
    if let Some((prop, value)) = declaration.trim_end_matches(';').split_once(':') {
        map.insert(prop.to_owned(), Value::String(value.to_owned()));
    }
}

/// Shared descriptor passed into [`build_grid_style`].  The struct mirrors the
/// ergonomics established by [`crate::r#box::BoxStyleInputs`], letting each
/// adapter borrow responsive props without cloning.  Keeping this contract in
/// sync across the stack ensures enterprise teams can automate style
/// generation, and it documents every hook that participates in responsive
/// calculations.
pub struct GridStyleInputs<'a> {
    /// Responsive column configuration for the surrounding grid container.
    pub columns: Option<&'a Responsive<u16>>,
    /// Responsive column span for the current item.
    pub span: Option<&'a Responsive<u16>>,
    /// Optional flexbox alignment on the main axis.
    pub justify_content: Option<&'a str>,
    /// Optional flexbox alignment on the cross axis.
    pub align_items: Option<&'a str>,
    /// Declarative JSON overrides merged through the `sx` pipeline.
    pub sx: Option<&'a Value>,
}

/// Shared styling routine leveraged by every framework specific grid
/// implementation and the integration tests.  Centralising the logic keeps the
/// breakpoint cascade identical regardless of whether the consumer renders via
/// Yew, Leptos or a headless test harness.
#[doc(hidden)]
pub fn build_grid_style(
    width: u32,
    breakpoints: &Breakpoints,
    inputs: GridStyleInputs<'_>,
) -> String {
    // Compute the active column model for the current viewport.  We lean on the
    // `Responsive::constant` helper so callers can omit props entirely and
    // still reuse the same resolution pipeline used for explicit overrides.
    let default_columns = Responsive::constant(12);
    let resolved_columns = inputs
        .columns
        .map(|value| value.resolve(width, breakpoints))
        .unwrap_or_else(|| default_columns.resolve(width, breakpoints));
    let default_span = Responsive::constant(12);
    let resolved_span = inputs
        .span
        .map(|value| value.resolve(width, breakpoints))
        .unwrap_or_else(|| default_span.resolve(width, breakpoints));
    let width_percent = grid_span_to_percent(resolved_span, resolved_columns);

    let mut style_map = Map::new();
    insert_declaration(&mut style_map, style::width(format!("{}%", width_percent)));

    if let Some(jc) = inputs.justify_content {
        insert_declaration(&mut style_map, style::justify_content(jc));
    }
    if let Some(ai) = inputs.align_items {
        insert_declaration(&mut style_map, style::align_items(ai));
    }

    let mut style_value = Value::Object(style_map);
    if let Some(sx) = inputs.sx {
        // Retain the deep merge behaviour so JSON overrides replace the
        // generated declarations when keys collide.
        deep_merge(&mut style_value, sx.clone());
    }

    style::json_to_style_string(&style_value)
}

#[cfg(feature = "yew")]
mod yew_impl {
    use super::*;
    use crate::theme_provider::use_theme_yew as use_theme;
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
        /// Additional JSON driven styles merged with the computed width.
        #[prop_or_default]
        pub sx: Option<Value>,
        /// Child elements of the grid item.
        #[prop_or_default]
        pub children: Children,
    }

    /// Render a grid item as a `<div>` with a calculated percentage width.
    #[function_component(Grid)]
    pub fn grid(props: &GridProps) -> Html {
        let theme = use_theme();
        let width = crate::responsive::viewport_width();
        let style_rules = build_grid_style(
            width,
            &theme.breakpoints,
            GridStyleInputs {
                columns: props.columns.as_ref(),
                span: props.span.as_ref(),
                justify_content: props.justify_content.as_deref(),
                align_items: props.align_items.as_deref(),
                sx: props.sx.as_ref(),
            },
        );
        // Promote the computed declarations into a scoped class so server
        // renderers and client frameworks share the same CSS payload.
        let scoped = use_memo(style_rules, |css| {
            crate::ScopedClass::from_declarations(css.clone())
        });
        let class = scoped.class().to_string();
        html! { <div class={class}>{ for props.children.iter() }</div> }
    }
}

#[cfg(feature = "yew")]
pub use yew_impl::{Grid, GridProps};

#[cfg(feature = "leptos")]
mod leptos_impl {
    use super::*;
    use crate::theme_provider::use_theme_leptos as use_theme;
    use leptos::*;

    /// Leptos implementation of [`Grid`].
    #[component]
    pub fn Grid(
        #[prop(optional)] columns: Option<Responsive<u16>>,
        #[prop(optional)] span: Option<Responsive<u16>>,
        #[prop(optional, into)] justify_content: Option<String>,
        #[prop(optional, into)] align_items: Option<String>,
        #[prop(optional)] sx: Option<Value>,
        children: Children,
    ) -> impl IntoView {
        let theme = use_theme();
        let width_px = crate::responsive::viewport_width();
        let style_rules = build_grid_style(
            width_px,
            &theme.breakpoints,
            GridStyleInputs {
                columns: columns.as_ref(),
                span: span.as_ref(),
                justify_content: justify_content.as_deref(),
                align_items: align_items.as_deref(),
                sx: sx.as_ref(),
            },
        );
        // Persist the scoped style for Leptos just like the Yew variant.
        let scoped = store_value(crate::ScopedClass::from_declarations(style_rules));
        let class = scoped.with_value(|class| class.class().to_string());
        view! { <div class=class>{children()}</div> }
    }
}

#[cfg(feature = "leptos")]
pub use leptos_impl::Grid;
