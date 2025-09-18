use crate::{responsive::Responsive, style, theme::Breakpoints};
use mui_utils::deep_merge;
use serde_json::{Map, Value};

/// Direction of children placement inside [`Stack`].
#[derive(Clone, PartialEq)]
pub enum StackDirection {
    Row,
    Column,
}

impl StackDirection {
    fn as_css_value(&self) -> &'static str {
        match self {
            StackDirection::Row => "row",
            StackDirection::Column => "column",
        }
    }
}

/// Shared descriptor mirroring [`crate::r#box::BoxStyleInputs`] so adapters can
/// forward borrowed responsive handles without cloning.  The optional
/// `StackDirection` is kept by value since it is a small enum and avoids a
/// lifetime parameter for ergonomic builder usage in tests.
pub struct StackStyleInputs<'a> {
    /// Orientation override. Defaults to [`StackDirection::Column`].
    pub direction: Option<StackDirection>,
    /// Responsive gap declaration applied via the modern `gap` property.
    pub spacing: Option<&'a Responsive<String>>,
    /// Optional flexbox alignment on the cross axis.
    pub align_items: Option<&'a str>,
    /// Optional flexbox alignment on the main axis.
    pub justify_content: Option<&'a str>,
    /// JSON overrides merged through the `sx` pipeline.
    pub sx: Option<&'a Value>,
}

/// Shared style assembly for framework adapters and integration tests.
#[doc(hidden)]
pub fn build_stack_style(
    width: u32,
    breakpoints: &Breakpoints,
    inputs: StackStyleInputs<'_>,
) -> String {
    let direction_value = inputs
        .direction
        .unwrap_or(StackDirection::Column)
        .as_css_value();
    let mut style_map = Map::new();
    style_map.insert("display".into(), Value::String("flex".into()));
    style_map.insert(
        "flex-direction".into(),
        Value::String(direction_value.into()),
    );

    if let Some(sp) = inputs.spacing {
        // Resolve the gap for the current viewport.  Using `gap` keeps both
        // horizontal and vertical spacing in sync, mirroring the ergonomic
        // defaults of the upstream Stack implementation.
        let resolved = sp.resolve(width, breakpoints);
        style_map.insert("gap".into(), Value::String(resolved));
    }
    if let Some(ai) = inputs.align_items {
        style_map.insert("align-items".into(), Value::String(ai.to_owned()));
    }
    if let Some(jc) = inputs.justify_content {
        style_map.insert("justify-content".into(), Value::String(jc.to_owned()));
    }

    let mut style_value = Value::Object(style_map);
    if let Some(sx) = inputs.sx {
        deep_merge(&mut style_value, sx.clone());
    }

    style::json_to_style_string(&style_value)
}

#[cfg(feature = "yew")]
mod yew_impl {
    use super::*;
    use yew::prelude::*;

    /// Properties for the [`Stack`] component.
    #[derive(Properties, PartialEq)]
    pub struct StackProps {
        /// Orientation of the stack. Defaults to vertical column layout.
        #[prop_or_default]
        pub direction: Option<StackDirection>,
        /// Gap between children. Accepts any CSS length value.
        #[prop_or_default]
        pub spacing: Option<Responsive<String>>,
        /// Align items on the cross axis.
        #[prop_or_default]
        pub align_items: Option<String>,
        /// Align items on the main axis.
        #[prop_or_default]
        pub justify_content: Option<String>,
        /// Additional arbitrary JSON merged into the generated CSS.
        #[prop_or_default]
        pub sx: Option<Value>,
        /// Child elements to render.
        #[prop_or_default]
        pub children: Children,
    }

    /// Minimal flexbox based stack layout.
    #[function_component(Stack)]
    pub fn stack(props: &StackProps) -> Html {
        let theme = crate::theme_provider::use_theme();
        let width = crate::responsive::viewport_width();
        let style_rules = build_stack_style(
            width,
            &theme.breakpoints,
            StackStyleInputs {
                direction: props.direction.clone(),
                spacing: props.spacing.as_ref(),
                align_items: props.align_items.as_deref(),
                justify_content: props.justify_content.as_deref(),
                sx: props.sx.as_ref(),
            },
        );
        // Reuse the shared scoped class helper so Stack benefits from CSS
        // caching and avoids inline declarations which break strict CSPs.
        let scoped = use_memo(style_rules, |css| {
            crate::ScopedClass::from_declarations(css.clone())
        });
        let class = scoped.class().to_string();
        html! { <div class={class}>{ for props.children.iter() }</div> }
    }
}

#[cfg(feature = "yew")]
pub use yew_impl::{Stack, StackProps};

#[cfg(feature = "leptos")]
mod leptos_impl {
    use super::*;
    use leptos::*;

    /// Leptos implementation of [`Stack`].
    #[component]
    pub fn Stack(
        #[prop(optional)] direction: Option<StackDirection>,
        #[prop(optional)] spacing: Option<Responsive<String>>,
        #[prop(optional, into)] align_items: Option<String>,
        #[prop(optional, into)] justify_content: Option<String>,
        #[prop(optional)] sx: Option<Value>,
        children: Children,
    ) -> impl IntoView {
        let theme = crate::theme_provider::use_theme();
        let width = crate::responsive::viewport_width();
        let style_rules = build_stack_style(
            width,
            &theme.breakpoints,
            StackStyleInputs {
                direction,
                spacing: spacing.as_ref(),
                align_items: align_items.as_deref(),
                justify_content: justify_content.as_deref(),
                sx: sx.as_ref(),
            },
        );
        // Store the scoped class in the runtime so Leptos keeps the CSS alive
        // until the component unmounts.
        let scoped = store_value(crate::ScopedClass::from_declarations(style_rules));
        let class = scoped.with_value(|class| class.class().to_string());
        view! { <div class=class>{children()}</div> }
    }
}

#[cfg(feature = "leptos")]
pub use leptos_impl::Stack;
