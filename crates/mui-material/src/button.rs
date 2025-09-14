#[cfg(feature = "leptos")]
use mui_styled_engine::css_with_theme;
#[cfg(any(feature = "yew", feature = "leptos"))]
use mui_styled_engine::use_theme;

// Re-export shared enums under component specific names so the public API
// remains ergonomic while the underlying definitions stay centralized.
pub use crate::macros::{Color as ButtonColor, Size as ButtonSize, Variant as ButtonVariant};

#[cfg(feature = "yew")]
mod yew_impl {
    use super::*;
    use mui_utils::{deep_merge, throttle};
    use serde_json::{json, Value};
    use std::time::Duration;
    use yew::prelude::*;

    crate::material_component_props!(ButtonProps {
        /// Text displayed inside the button.
        label: String,
        /// Whether the button is disabled and non-interactive.
        disabled: bool,
        /// Callback invoked when the button is clicked.
        on_click: Option<Callback<MouseEvent>>,
        /// Minimum number of milliseconds between invocations of `on_click`.
        throttle_ms: u64,
        /// Optional style overrides expressed as a JSON object.  Keys map to CSS
        /// properties written in kebab-case.
        style_overrides: Option<Value>,
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
            ButtonVariant::Outlined => (
                "transparent".into(),
                palette.clone(),
                format!("1px solid {}", palette),
            ),
            ButtonVariant::Text => ("transparent".into(), palette.clone(), "none".into()),
        };
        // Base styles derived from the theme. Represented as JSON so they can
        // be merged with user provided overrides using `deep_merge` for a fully
        // declarative styling pipeline.
        let mut style = json!({
            "background": bg,
            "color": color,
            "border": border,
            "padding": padding,
            "cursor": if props.disabled { "default" } else { "pointer" },
            "opacity": if props.disabled { 0.5 } else { 1.0 },
        });
        if let Some(extra) = &props.style_overrides {
            // Merge overrides deeply so callers can tweak any subset of
            // properties without re-specifying the entire object.
            deep_merge(&mut style, extra.clone());
        }
        let style_str = style
            .as_object()
            .map(|m| {
                m.iter()
                    .map(|(k, v)| {
                        let val = v
                            .as_str()
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| v.to_string());
                        format!("{k}: {val};")
                    })
                    .collect::<String>()
            })
            .unwrap_or_default();

        let onclick_cb = props.on_click.clone().unwrap_or_else(|| Callback::noop());
        let throttle_ms = props.throttle_ms;
        // Store the throttled callback in `use_mut_ref` so the same closure is
        // reused across renders – avoiding reallocation on every render cycle.
        let throttled = use_mut_ref(move || {
            let cb = onclick_cb.clone();
            if throttle_ms > 0 {
                Box::new(throttle(
                    move |ev: MouseEvent| cb.emit(ev),
                    Duration::from_millis(throttle_ms),
                )) as Box<dyn FnMut(MouseEvent)>
            } else {
                Box::new(move |ev: MouseEvent| cb.emit(ev)) as Box<dyn FnMut(MouseEvent)>
            }
        });
        let onclick = {
            let throttled = throttled.clone();
            Callback::from(move |ev: MouseEvent| {
                (throttled.borrow_mut())(ev);
            })
        };
        html! {
            <button
                style={style_str}
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
            ButtonVariant::Outlined => (
                "transparent".into(),
                palette.clone(),
                format!("1px solid {}", palette),
            ),
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
