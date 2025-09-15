use crate::{responsive::Responsive, style, style_props, theme::Breakpoints};

/// Builds the inline style string for the container based on the current
/// viewport width. The helper is used by all framework adapters and ensures the
/// integration tests exercise the exact same resolution logic.
#[doc(hidden)]
pub fn build_container_style(
    width: u32,
    breakpoints: &Breakpoints,
    max_width: Option<&Responsive<String>>,
    sx: &str,
) -> String {
    let mut style = if let Some(mw) = max_width {
        let resolved = mw.resolve(width, breakpoints);
        let mut base = style_props! { margin_left: "auto", margin_right: "auto", width: "100%" };
        base.push_str(&style::max_width(resolved));
        base
    } else {
        style_props! { width: "100%" }
    };
    style.push_str(sx);
    style
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
        let theme = crate::theme_provider::use_theme();
        let width = crate::responsive::viewport_width();
        let style = build_container_style(
            width,
            &theme.breakpoints,
            props.max_width.as_ref(),
            &props.sx,
        );
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
        #[prop(optional)] max_width: Option<Responsive<String>>,
        #[prop(optional, into)] sx: String,
        children: Children,
    ) -> impl IntoView {
        let theme = crate::theme_provider::use_theme();
        let width = crate::responsive::viewport_width();
        let style = build_container_style(width, &theme.breakpoints, max_width.as_ref(), &sx);
        view! { <div style=style>{children()}</div> }
    }
}

#[cfg(feature = "leptos")]
pub use leptos_impl::Container;
