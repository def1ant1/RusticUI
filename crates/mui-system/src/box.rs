use crate::{responsive::Responsive, style, theme::Theme};
use crate::theme_provider::use_theme;

#[cfg(feature = "yew")]
mod yew_impl {
    use super::*;
    use web_sys::window;
    use yew::prelude::*;

    /// Lightweight wrapper around a `div` that accepts system style properties.
    #[derive(Properties, PartialEq)]
    pub struct BoxProps {
        /// Responsive margin shorthand. Values cascade from `xs` upwards.
        #[prop_or_default]
        pub m: Option<Responsive<String>>,
        /// Responsive padding shorthand.
        #[prop_or_default]
        pub p: Option<Responsive<String>>,
        /// Optional `display` style.
        #[prop_or_default]
        pub display: Option<String>,
        /// Flexbox alignment of children on the cross axis.
        #[prop_or_default]
        pub align_items: Option<String>,
        /// Flexbox alignment of children on the main axis.
        #[prop_or_default]
        pub justify_content: Option<String>,
        /// Raw style string allowing arbitrary `sx` values.
        #[prop_or_default]
        pub sx: String,
        /// Elements to render inside the box.
        #[prop_or_default]
        pub children: Children,
    }

    #[function_component(Box)]
    pub fn box_component(props: &BoxProps) -> Html {
        let theme = use_theme();
        let width = window()
            .and_then(|w| w.inner_width().ok())
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0) as u32;
        let mut style_string = String::new();
        if let Some(m) = &props.m {
            style_string.push_str(&style::margin(m.resolve(width, &theme.breakpoints)));
        }
        if let Some(p) = &props.p {
            style_string.push_str(&style::padding(p.resolve(width, &theme.breakpoints)));
        }
        if let Some(d) = &props.display {
            style_string.push_str(&style::display(d.clone()));
        }
        if let Some(ai) = &props.align_items {
            style_string.push_str(&style::align_items(ai.clone()));
        }
        if let Some(jc) = &props.justify_content {
            style_string.push_str(&style::justify_content(jc.clone()));
        }
        style_string.push_str(&props.sx);
        html! { <div style={style_string}>{ for props.children.iter() }</div> }
    }
}

#[cfg(feature = "yew")]
pub use yew_impl::{Box, BoxProps};

#[cfg(feature = "leptos")]
mod leptos_impl {
    use super::*;
    use leptos::*;
    use web_sys::window;

    /// Leptos version of [`Box`] sharing the same props as the Yew variant.
    #[component]
    pub fn Box(
        #[prop(optional)] m: Option<Responsive<String>>,
        #[prop(optional)] p: Option<Responsive<String>>,
        #[prop(optional, into)] display: Option<String>,
        #[prop(optional, into)] align_items: Option<String>,
        #[prop(optional, into)] justify_content: Option<String>,
        #[prop(optional, into)] sx: String,
        children: Children,
    ) -> impl IntoView {
        let theme = use_theme();
        let width = window()
            .and_then(|w| w.inner_width().ok())
            .and_then(|v| v.as_f64().ok())
            .unwrap_or(0.0) as u32;
        let mut style_string = String::new();
        if let Some(m) = m {
            style_string.push_str(&style::margin(m.resolve(width, &theme.breakpoints)));
        }
        if let Some(p) = p {
            style_string.push_str(&style::padding(p.resolve(width, &theme.breakpoints)));
        }
        if let Some(d) = display {
            style_string.push_str(&style::display(d));
        }
        if let Some(ai) = align_items {
            style_string.push_str(&style::align_items(ai));
        }
        if let Some(jc) = justify_content {
            style_string.push_str(&style::justify_content(jc));
        }
        style_string.push_str(&sx);
        view! { <div style=style_string>{children()}</div> }
    }
}

#[cfg(feature = "leptos")]
pub use leptos_impl::Box;
