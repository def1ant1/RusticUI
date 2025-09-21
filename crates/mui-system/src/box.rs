use crate::{responsive::Responsive, style, theme::Breakpoints};
use mui_utils::deep_merge;
use serde_json::{Map, Value};

/// Lightweight descriptor passed into [`build_box_style`].  The struct keeps the
/// rendering code terse because the framework adapters can forward borrowed
/// references rather than cloning every [`Responsive`] value.
pub struct BoxStyleInputs<'a> {
    pub margin: Option<&'a Responsive<String>>,
    pub padding: Option<&'a Responsive<String>>,
    pub font_size: Option<&'a Responsive<String>>,
    pub font_weight: Option<&'a Responsive<String>>,
    pub line_height: Option<&'a Responsive<String>>,
    pub letter_spacing: Option<&'a Responsive<String>>,
    pub color: Option<&'a Responsive<String>>,
    pub background_color: Option<&'a Responsive<String>>,
    pub width: Option<&'a Responsive<String>>,
    pub height: Option<&'a Responsive<String>>,
    pub min_width: Option<&'a Responsive<String>>,
    pub max_width: Option<&'a Responsive<String>>,
    pub min_height: Option<&'a Responsive<String>>,
    pub max_height: Option<&'a Responsive<String>>,
    pub position: Option<&'a Responsive<String>>,
    pub top: Option<&'a Responsive<String>>,
    pub right: Option<&'a Responsive<String>>,
    pub bottom: Option<&'a Responsive<String>>,
    pub left: Option<&'a Responsive<String>>,
    pub display: Option<&'a str>,
    pub align_items: Option<&'a str>,
    pub justify_content: Option<&'a str>,
    pub sx: Option<&'a Value>,
}

fn insert_declaration(map: &mut Map<String, Value>, declaration: String) {
    if let Some((prop, value)) = declaration.trim_end_matches(';').split_once(':') {
        map.insert(prop.to_owned(), Value::String(value.to_owned()));
    }
}

fn apply_responsive_style<F>(
    styles: &mut Map<String, Value>,
    width: u32,
    breakpoints: &Breakpoints,
    value: Option<&Responsive<String>>,
    builder: F,
) where
    F: Fn(String) -> String,
{
    if let Some(responsive) = value {
        let resolved = responsive.resolve(width, breakpoints);
        insert_declaration(styles, builder(resolved));
    }
}

/// Assembles the inline CSS string for [`Box`] based on the supplied responsive
/// props. Centralising the resolver keeps behaviour identical across the Yew
/// and Leptos adapters and gives the integration tests something deterministic
/// to exercise. The grouped sections mirror the Issue 13 enhancements—spacing,
/// typography, sizing, colour and positioning each route through
/// `Responsive::resolve` so automated breakpoint handling stays declarative.
#[doc(hidden)]
pub fn build_box_style(
    width: u32,
    breakpoints: &Breakpoints,
    inputs: BoxStyleInputs<'_>,
) -> String {
    let mut style_map = Map::new();

    // Spacing ----------------------------------------------------------------
    apply_responsive_style(&mut style_map, width, breakpoints, inputs.margin, |value| {
        style::margin(value)
    });
    apply_responsive_style(
        &mut style_map,
        width,
        breakpoints,
        inputs.padding,
        |value| style::padding(value),
    );

    // Typography -------------------------------------------------------------
    apply_responsive_style(
        &mut style_map,
        width,
        breakpoints,
        inputs.font_size,
        |value| style::font_size(value),
    );
    apply_responsive_style(
        &mut style_map,
        width,
        breakpoints,
        inputs.font_weight,
        |value| style::font_weight(value),
    );
    apply_responsive_style(
        &mut style_map,
        width,
        breakpoints,
        inputs.line_height,
        |value| style::line_height(value),
    );
    apply_responsive_style(
        &mut style_map,
        width,
        breakpoints,
        inputs.letter_spacing,
        |value| style::letter_spacing(value),
    );

    // Sizing -----------------------------------------------------------------
    apply_responsive_style(&mut style_map, width, breakpoints, inputs.width, |value| {
        style::width(value)
    });
    apply_responsive_style(&mut style_map, width, breakpoints, inputs.height, |value| {
        style::height(value)
    });
    apply_responsive_style(
        &mut style_map,
        width,
        breakpoints,
        inputs.min_width,
        |value| style::min_width(value),
    );
    apply_responsive_style(
        &mut style_map,
        width,
        breakpoints,
        inputs.max_width,
        |value| style::max_width(value),
    );
    apply_responsive_style(
        &mut style_map,
        width,
        breakpoints,
        inputs.min_height,
        |value| style::min_height(value),
    );
    apply_responsive_style(
        &mut style_map,
        width,
        breakpoints,
        inputs.max_height,
        |value| style::max_height(value),
    );

    // Color ------------------------------------------------------------------
    apply_responsive_style(&mut style_map, width, breakpoints, inputs.color, |value| {
        style::color(value)
    });
    apply_responsive_style(
        &mut style_map,
        width,
        breakpoints,
        inputs.background_color,
        |value| style::background_color(value),
    );

    // Positioning ------------------------------------------------------------
    apply_responsive_style(
        &mut style_map,
        width,
        breakpoints,
        inputs.position,
        |value| style::position(value),
    );
    apply_responsive_style(&mut style_map, width, breakpoints, inputs.top, |value| {
        style::top(value)
    });
    apply_responsive_style(&mut style_map, width, breakpoints, inputs.right, |value| {
        style::right(value)
    });
    apply_responsive_style(&mut style_map, width, breakpoints, inputs.bottom, |value| {
        style::bottom(value)
    });
    apply_responsive_style(&mut style_map, width, breakpoints, inputs.left, |value| {
        style::left(value)
    });

    // Layout toggles that remain non-responsive for now ----------------------
    if let Some(display) = inputs.display {
        insert_declaration(&mut style_map, style::display(display));
    }
    if let Some(ai) = inputs.align_items {
        insert_declaration(&mut style_map, style::align_items(ai));
    }
    if let Some(jc) = inputs.justify_content {
        insert_declaration(&mut style_map, style::justify_content(jc));
    }

    let mut style_value = Value::Object(style_map);
    if let Some(sx) = inputs.sx {
        // Merge JSON overrides after the generated declarations so caller
        // provided automation remains authoritative when conflicts appear.
        deep_merge(&mut style_value, sx.clone());
    }

    style::json_to_style_string(&style_value)
}

#[cfg(feature = "yew")]
mod yew_impl {
    use super::*;
    use crate::theme_provider::use_theme;
    use yew::prelude::*;

    /// Properties for the [`Box`] component when targeting Yew.
    #[derive(Properties, PartialEq)]
    pub struct BoxProps {
        /// Responsive margin shorthand. Values cascade from `xs` upwards.
        #[prop_or_default]
        pub m: Option<Responsive<String>>,
        /// Responsive padding shorthand.
        #[prop_or_default]
        pub p: Option<Responsive<String>>,
        /// Responsive font size declarations covering the Issue 13 typography additions.
        #[prop_or_default]
        pub font_size: Option<Responsive<String>>,
        /// Font weight responsive overrides.
        #[prop_or_default]
        pub font_weight: Option<Responsive<String>>,
        /// Responsive line height adjustments.
        #[prop_or_default]
        pub line_height: Option<Responsive<String>>,
        /// Responsive letter spacing adjustments.
        #[prop_or_default]
        pub letter_spacing: Option<Responsive<String>>,
        /// Responsive foreground color configuration.
        #[prop_or_default]
        pub color: Option<Responsive<String>>,
        /// Responsive background color configuration.
        #[prop_or_default]
        pub background_color: Option<Responsive<String>>,
        /// Responsive width declarations.
        #[prop_or_default]
        pub width: Option<Responsive<String>>,
        /// Responsive height declarations.
        #[prop_or_default]
        pub height: Option<Responsive<String>>,
        /// Minimum width per breakpoint.
        #[prop_or_default]
        pub min_width: Option<Responsive<String>>,
        /// Maximum width per breakpoint.
        #[prop_or_default]
        pub max_width: Option<Responsive<String>>,
        /// Minimum height per breakpoint.
        #[prop_or_default]
        pub min_height: Option<Responsive<String>>,
        /// Maximum height per breakpoint.
        #[prop_or_default]
        pub max_height: Option<Responsive<String>>,
        /// Responsive position mode (`static`, `absolute`, ...).
        #[prop_or_default]
        pub position: Option<Responsive<String>>,
        /// Top offset for positioned layouts.
        #[prop_or_default]
        pub top: Option<Responsive<String>>,
        /// Right offset for positioned layouts.
        #[prop_or_default]
        pub right: Option<Responsive<String>>,
        /// Bottom offset for positioned layouts.
        #[prop_or_default]
        pub bottom: Option<Responsive<String>>,
        /// Left offset for positioned layouts.
        #[prop_or_default]
        pub left: Option<Responsive<String>>,
        /// Optional `display` style.
        #[prop_or_default]
        pub display: Option<String>,
        /// Flexbox alignment of children on the cross axis.
        #[prop_or_default]
        pub align_items: Option<String>,
        /// Flexbox alignment of children on the main axis.
        #[prop_or_default]
        pub justify_content: Option<String>,
        /// Optional JSON blob merged with the generated styles via `sx`.
        #[prop_or_default]
        pub sx: Option<Value>,
        /// Elements to render inside the box.
        #[prop_or_default]
        pub children: Children,
    }

    #[function_component(Box)]
    pub fn box_component(props: &BoxProps) -> Html {
        let theme = use_theme();
        let viewport = crate::responsive::viewport_width();
        let style_rules = build_box_style(
            viewport,
            &theme.breakpoints,
            BoxStyleInputs {
                margin: props.m.as_ref(),
                padding: props.p.as_ref(),
                font_size: props.font_size.as_ref(),
                font_weight: props.font_weight.as_ref(),
                line_height: props.line_height.as_ref(),
                letter_spacing: props.letter_spacing.as_ref(),
                color: props.color.as_ref(),
                background_color: props.background_color.as_ref(),
                width: props.width.as_ref(),
                height: props.height.as_ref(),
                min_width: props.min_width.as_ref(),
                max_width: props.max_width.as_ref(),
                min_height: props.min_height.as_ref(),
                max_height: props.max_height.as_ref(),
                position: props.position.as_ref(),
                top: props.top.as_ref(),
                right: props.right.as_ref(),
                bottom: props.bottom.as_ref(),
                left: props.left.as_ref(),
                display: props.display.as_deref(),
                align_items: props.align_items.as_deref(),
                justify_content: props.justify_content.as_deref(),
                sx: props.sx.as_ref(),
            },
        );
        // Register the declarations with the styled engine so Box participates
        // in the same caching and SSR pathways as our Material components.
        let scoped = use_memo(style_rules, |css| {
            crate::ScopedClass::from_declarations(css.clone())
        });
        let class = scoped.class().to_string();
        html! { <div class={class}>{ for props.children.iter() }</div> }
    }
}

#[cfg(feature = "yew")]
pub use yew_impl::{Box, BoxProps};

#[cfg(feature = "leptos")]
mod leptos_impl {
    use super::*;
    use crate::theme_provider::use_theme;
    use leptos::*;

    /// Leptos version of [`Box`] sharing the same responsive props as the Yew variant.
    #[component]
    pub fn Box(
        #[prop(optional)] m: Option<Responsive<String>>,
        #[prop(optional)] p: Option<Responsive<String>>,
        #[prop(optional)] font_size: Option<Responsive<String>>,
        #[prop(optional)] font_weight: Option<Responsive<String>>,
        #[prop(optional)] line_height: Option<Responsive<String>>,
        #[prop(optional)] letter_spacing: Option<Responsive<String>>,
        #[prop(optional)] color: Option<Responsive<String>>,
        #[prop(optional)] background_color: Option<Responsive<String>>,
        #[prop(optional)] width: Option<Responsive<String>>,
        #[prop(optional)] height: Option<Responsive<String>>,
        #[prop(optional)] min_width: Option<Responsive<String>>,
        #[prop(optional)] max_width: Option<Responsive<String>>,
        #[prop(optional)] min_height: Option<Responsive<String>>,
        #[prop(optional)] max_height: Option<Responsive<String>>,
        #[prop(optional)] position: Option<Responsive<String>>,
        #[prop(optional)] top: Option<Responsive<String>>,
        #[prop(optional)] right: Option<Responsive<String>>,
        #[prop(optional)] bottom: Option<Responsive<String>>,
        #[prop(optional)] left: Option<Responsive<String>>,
        #[prop(optional, into)] display: Option<String>,
        #[prop(optional, into)] align_items: Option<String>,
        #[prop(optional, into)] justify_content: Option<String>,
        #[prop(optional)] sx: Option<Value>,
        children: Children,
    ) -> impl IntoView {
        let theme = use_theme();
        let viewport = crate::responsive::viewport_width();
        let style_rules = build_box_style(
            viewport,
            &theme.breakpoints,
            BoxStyleInputs {
                margin: m.as_ref(),
                padding: p.as_ref(),
                font_size: font_size.as_ref(),
                font_weight: font_weight.as_ref(),
                line_height: line_height.as_ref(),
                letter_spacing: letter_spacing.as_ref(),
                color: color.as_ref(),
                background_color: background_color.as_ref(),
                width: width.as_ref(),
                height: height.as_ref(),
                min_width: min_width.as_ref(),
                max_width: max_width.as_ref(),
                min_height: min_height.as_ref(),
                max_height: max_height.as_ref(),
                position: position.as_ref(),
                top: top.as_ref(),
                right: right.as_ref(),
                bottom: bottom.as_ref(),
                left: left.as_ref(),
                display: display.as_deref(),
                align_items: align_items.as_deref(),
                justify_content: justify_content.as_deref(),
                sx: sx.as_ref(),
            },
        );
        // Mirror the Yew integration by registering the CSS once and reusing
        // the class for subsequent renders, keeping hydration output stable.
        let scoped = store_value(crate::ScopedClass::from_declarations(style_rules));
        let class = scoped.with_value(|class| class.class().to_string());
        view! { <div class=class>{children()}</div> }
    }
}

#[cfg(feature = "leptos")]
pub use leptos_impl::Box;
