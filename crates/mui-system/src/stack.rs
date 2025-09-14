use crate::{style, style_props};

/// Direction of children placement inside [`Stack`].
#[derive(Clone, PartialEq)]
pub enum StackDirection {
    Row,
    Column,
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
        pub spacing: Option<String>,
        /// Align items on the cross axis.
        #[prop_or_default]
        pub align_items: Option<String>,
        /// Align items on the main axis.
        #[prop_or_default]
        pub justify_content: Option<String>,
        /// Additional arbitrary style string merged into the generated CSS.
        #[prop_or_default]
        pub sx: String,
        /// Child elements to render.
        #[prop_or_default]
        pub children: Children,
    }

    /// Minimal flexbox based stack layout.
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
        if let Some(ai) = &props.align_items {
            style.push_str(&style::align_items(ai.clone()));
        }
        if let Some(jc) = &props.justify_content {
            style.push_str(&style::justify_content(jc.clone()));
        }
        style.push_str(&props.sx);
        html! { <div style={style}>{ for props.children.iter() }</div> }
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
        #[prop(optional, into)] spacing: Option<String>,
        #[prop(optional, into)] align_items: Option<String>,
        #[prop(optional, into)] justify_content: Option<String>,
        #[prop(optional, into)] sx: String,
        children: Children,
    ) -> impl IntoView {
        let direction = match direction {
            Some(StackDirection::Row) => "row",
            _ => "column",
        };
        let mut style = style_props! { display: "flex", flex_direction: direction };
        if let Some(sp) = spacing {
            style.push_str(&style_props! { gap: sp });
        }
        if let Some(ai) = align_items {
            style.push_str(&style::align_items(ai));
        }
        if let Some(jc) = justify_content {
            style.push_str(&style::justify_content(jc));
        }
        style.push_str(&sx);
        view! { <div style=style>{children()}</div> }
    }
}

#[cfg(feature = "leptos")]
pub use leptos_impl::Stack;
