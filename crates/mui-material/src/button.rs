#[cfg(any(feature = "yew", feature = "leptos"))]
use mui_styled_engine::{css_with_theme, use_theme};

// Re-export shared enums under component specific names so the public API
// remains ergonomic while the underlying definitions stay centralized.
pub use crate::macros::{Color as ButtonColor, Size as ButtonSize, Variant as ButtonVariant};

#[cfg(feature = "yew")]
mod yew_impl {
    use super::*;
    use yew::prelude::*;

    crate::material_component_props!(ButtonProps {
        /// Text displayed inside the button.
        label: String,
        /// Whether the button is disabled and non-interactive.
        disabled: bool,
        /// Callback invoked when the button is clicked.
        on_click: Option<Callback<MouseEvent>>,
    });

    /// Yew implementation of [`Button`].
    #[function_component(Button)]
    pub fn button(props: &ButtonProps) -> Html {
        let theme = use_theme();
        let palette = match props.color {
            ButtonColor::Primary => theme.palette.primary.clone(),
            ButtonColor::Secondary => theme.palette.secondary.clone(),
        };
        let padding = match props.size {
            ButtonSize::Small => "2px 8px",
            ButtonSize::Medium => "4px 16px",
            ButtonSize::Large => "8px 24px",
        };
        let (bg, color, border) = match props.variant {
            ButtonVariant::Contained => (palette.clone(), "#fff".to_string(), "none".to_string()),
            ButtonVariant::Outlined => ("transparent".into(), palette.clone(), format!("1px solid {}", palette)),
            ButtonVariant::Text => ("transparent".into(), palette.clone(), "none".into()),
        };
        let style = css_with_theme!(
            theme,
            r#"
            background: ${bg};
            color: ${color};
            border: ${border};
            padding: ${padding};
            cursor: ${cursor};
            opacity: ${opacity};
        "#,
            bg = bg,
            color = color,
            border = border,
            padding = padding,
            cursor = if props.disabled { "default" } else { "pointer" },
            opacity = if props.disabled { 0.5 } else { 1.0 },
        );
        let class = style.get_class_name().to_string();
        let onclick = props.on_click.clone().unwrap_or_else(|| Callback::noop());
        html! {
            <button
                class={class}
                type="button"
                disabled={props.disabled}
                onclick={onclick}
            >{ &props.label }</button>
        }
    }
}

#[cfg(feature = "yew")]
pub use yew_impl::{Button, ButtonProps};

#[cfg(feature = "leptos")]
mod leptos_impl {
    use super::*;
    use leptos::*;

    #[derive(Clone, PartialEq, Props, Default)]
    pub struct ButtonProps {
        #[prop(into)]
        pub label: String,
        #[prop(optional)]
        pub color: ButtonColor,
        #[prop(optional)]
        pub variant: ButtonVariant,
        #[prop(optional)]
        pub size: ButtonSize,
        #[prop(optional)]
        pub disabled: bool,
        #[prop(optional)]
        pub on_click: Option<Box<dyn Fn(ev::MouseEvent) + 'static>>,
    }

    /// Leptos implementation of [`Button`].
    #[component]
    pub fn Button(props: ButtonProps) -> impl IntoView {
        let theme = use_theme();
        let palette = match props.color {
            ButtonColor::Primary => theme.palette.primary.clone(),
            ButtonColor::Secondary => theme.palette.secondary.clone(),
        };
        let padding = match props.size {
            ButtonSize::Small => "2px 8px",
            ButtonSize::Medium => "4px 16px",
            ButtonSize::Large => "8px 24px",
        };
        let (bg, color, border) = match props.variant {
            ButtonVariant::Contained => (palette.clone(), "#fff".to_string(), "none".to_string()),
            ButtonVariant::Outlined => ("transparent".into(), palette.clone(), format!("1px solid {}", palette)),
            ButtonVariant::Text => ("transparent".into(), palette.clone(), "none".into()),
        };
        let style = css_with_theme!(
            theme,
            r#"
            background: ${bg};
            color: ${color};
            border: ${border};
            padding: ${padding};
            cursor: ${cursor};
            opacity: ${opacity};
        "#,
            bg = bg,
            color = color,
            border = border,
            padding = padding,
            cursor = if props.disabled { "default" } else { "pointer" },
            opacity = if props.disabled { 0.5 } else { 1.0 },
        );
        let class = style.get_class_name().to_string();
        let on_click = props.on_click;
        view! {
            <button
                class=class
                type="button"
                disabled=props.disabled
                on:click=move |ev| if let Some(cb) = &on_click { cb(ev) }
            >{props.label}</button>
        }
    }
}

#[cfg(feature = "leptos")]
pub use leptos_impl::{Button, ButtonProps};
