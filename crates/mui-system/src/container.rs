use crate::style_props;

#[cfg(feature = "yew")]
mod yew_impl {
    use super::*;
    use yew::prelude::*;

    /// Properties for the [`Container`] component.
    #[derive(Properties, PartialEq)]
    pub struct ContainerProps {
        /// Optional maximum width of the container (e.g. `"1200px"`).
        #[prop_or_default]
        pub max_width: Option<String>,
        /// Additional style string to merge, following the MUI `sx` syntax.
        #[prop_or_default]
        pub sx: String,
        /// Child elements to display inside the container.
        #[prop_or_default]
        pub children: Children,
    }

    /// Centers content with an optional maximum width.
    #[function_component(Container)]
    pub fn container(props: &ContainerProps) -> Html {
        let mut style = String::new();
        if let Some(mw) = &props.max_width {
            style.push_str(&style_props! { margin_left: "auto", margin_right: "auto", max_width: mw.clone(), width: "100%" });
        } else {
            style.push_str(&style_props! { width: "100%" });
        }
        style.push_str(&props.sx);
        html! { <div style={style}>{ for props.children.iter() }</div> }
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
        #[prop(optional, into)] max_width: Option<String>,
        #[prop(optional, into)] sx: String,
        children: Children,
    ) -> impl IntoView {
        let mut style = String::new();
        if let Some(mw) = max_width {
            style.push_str(&style_props! { margin_left: "auto", margin_right: "auto", max_width: mw, width: "100%" });
        } else {
            style.push_str(&style_props! { width: "100%" });
        }
        style.push_str(&sx);
        view! { <div style=style>{children()}</div> }
    }
}

#[cfg(feature = "leptos")]
pub use leptos_impl::Container;
