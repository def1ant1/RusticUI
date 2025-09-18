use crate::{responsive::Responsive, style, theme::Breakpoints};
use mui_utils::deep_merge;
use serde_json::{Map, Value};

/// Lightweight descriptor mirroring the ergonomics of [`crate::r#box::BoxStyleInputs`].
/// Enterprise applications frequently centralise layout rules, so allowing the
/// adapters to pass borrowed responsive handles keeps cloning to a minimum and
/// makes the automation surface explicit.
pub struct ContainerStyleInputs<'a> {
    /// Optional responsive maximum width declaration for the container.
    pub max_width: Option<&'a Responsive<String>>,
    /// Arbitrary JSON overrides merged through the `sx` pipeline.
    pub sx: Option<&'a Value>,
}

/// Builds the inline style string for the container based on the current
/// viewport width. The helper is used by all framework adapters and ensures the
/// integration tests exercise the exact same resolution logic.
#[doc(hidden)]
pub fn build_container_style(
    width: u32,
    breakpoints: &Breakpoints,
    inputs: ContainerStyleInputs<'_>,
) -> String {
    let mut style_map = Map::new();
    style_map.insert("width".into(), Value::String("100%".into()));

    if let Some(mw) = inputs.max_width {
        let resolved = mw.resolve(width, breakpoints);
        style_map.insert("margin-left".into(), Value::String("auto".into()));
        style_map.insert("margin-right".into(), Value::String("auto".into()));
        style_map.insert("max-width".into(), Value::String(resolved));
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

    /// Properties for the [`Container`] component.
    #[derive(Properties, PartialEq)]
    pub struct ContainerProps {
        /// Optional maximum width of the container (e.g. `"1200px"`).
        #[prop_or_default]
        pub max_width: Option<Responsive<String>>,
        /// Additional JSON overrides to merge, following the MUI `sx` syntax.
        #[prop_or_default]
        pub sx: Option<Value>,
        /// Child elements to display inside the container.
        #[prop_or_default]
        pub children: Children,
    }

    /// Centers content with an optional maximum width.
    #[function_component(Container)]
    pub fn container(props: &ContainerProps) -> Html {
        let theme = crate::theme_provider::use_theme();
        let width = crate::responsive::viewport_width();
        let style_rules = build_container_style(
            width,
            &theme.breakpoints,
            ContainerStyleInputs {
                max_width: props.max_width.as_ref(),
                sx: props.sx.as_ref(),
            },
        );
        // Convert the inline style string into a scoped class so the styled
        // engine can deduplicate rules and we avoid sprinkling CSP sensitive
        // `style` attributes across enterprise deployments.
        let scoped = use_memo(style_rules, |css| {
            // Dynamic `sx` strings produce arbitrary declarations so we register
            // the combined CSS directly with the styled engine.
            crate::ScopedClass::from_declarations(css.clone())
        });
        let class = scoped.class().to_string();
        html! { <div class={class}>{ for props.children.iter() }</div> }
    }
}

#[cfg(feature = "yew")]
pub use yew_impl::{Container, ContainerProps};

#[cfg(feature = "leptos")]
mod leptos_impl {
    use super::*;
    use leptos::*;

    /// Leptos variant of [`Container`].
    #[component]
    pub fn Container(
        #[prop(optional)] max_width: Option<Responsive<String>>,
        #[prop(optional)] sx: Option<Value>,
        children: Children,
    ) -> impl IntoView {
        let theme = crate::theme_provider::use_theme();
        let width = crate::responsive::viewport_width();
        let style_rules = build_container_style(
            width,
            &theme.breakpoints,
            ContainerStyleInputs {
                max_width: max_width.as_ref(),
                sx: sx.as_ref(),
            },
        );
        // Persist the scoped class for the component lifetime so the stylist
        // registry keeps the CSS mounted until Leptos disposes the view.
        let scoped = store_value(crate::ScopedClass::from_declarations(style_rules));
        let class = scoped.with_value(|class| class.class().to_string());
        view! { <div class=class>{children()}</div> }
    }
}

#[cfg(feature = "leptos")]
pub use leptos_impl::Container;
