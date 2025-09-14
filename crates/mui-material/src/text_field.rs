#[cfg(any(feature = "yew", feature = "dioxus", feature = "sycamore"))]
use mui_styled_engine::{use_theme, Theme};
#[cfg(target_arch = "wasm32")]
use mui_utils::debounce;
use mui_utils::deep_merge;
use serde_json::{json, Value};
#[cfg(target_arch = "wasm32")]
use std::time::Duration;
#[cfg(feature = "yew")]
use wasm_bindgen::JsCast;
#[cfg(feature = "yew")]
use web_sys::HtmlInputElement;
#[cfg(feature = "yew")]
use yew::prelude::*;

pub use crate::macros::{
    Color as TextFieldColor, Size as TextFieldSize, Variant as TextFieldVariant,
};

#[cfg(any(feature = "yew", feature = "dioxus", feature = "sycamore"))]
fn compute_parts(
    theme: &Theme,
    color: TextFieldColor,
    size: TextFieldSize,
    variant: TextFieldVariant,
) -> (String, &'static str, String) {
    let color = match color {
        TextFieldColor::Primary => theme.palette.primary.clone(),
        TextFieldColor::Secondary => theme.palette.secondary.clone(),
    };
    let font_size = match size {
        TextFieldSize::Small => "0.8rem",
        TextFieldSize::Medium => "1rem",
        TextFieldSize::Large => "1.2rem",
    };
    let border = match variant {
        TextFieldVariant::Outlined => format!("1px solid {}", color.clone()),
        TextFieldVariant::Contained => format!("1px solid {}", color.clone()),
        TextFieldVariant::Text => "none".to_string(),
    };
    (color, font_size, border)
}

#[cfg(feature = "yew")]
crate::material_component_props!(TextFieldProps {
    /// Current value displayed in the input element.
    value: String,
    /// Placeholder hint shown when the field is empty.
    placeholder: String,
    /// Accessibility label describing the purpose of the field.
    aria_label: String,
    /// Delay in milliseconds before `on_input` is invoked.
    debounce_ms: u64,
    /// Callback emitting the latest value after debouncing.
    on_input: Option<Callback<String>>,
    /// Optional style overrides expressed as JSON.
    style_overrides: Option<Value>,
});

#[cfg(feature = "yew")]
mod yew_impl {
    use super::*;

    /// Controlled text input field.
    #[function_component(TextField)]
    pub fn text_field(props: &TextFieldProps) -> Html {
        let theme = use_theme();
        let (color, font_size, border) = compute_parts(&theme, props.color, props.size, props.variant);
        let mut style = json!({
            "color": color,
            "font-size": font_size,
            "border": border,
            "padding": "4px 8px",
        });
        if let Some(extra) = &props.style_overrides {
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

        let on_input_cb = props.on_input.clone().unwrap_or_else(|| Callback::noop());
        let _debounce_ms = props.debounce_ms;
        #[cfg(target_arch = "wasm32")]
        let debounced = {
            let debounce_ms = _debounce_ms;
            use_mut_ref(move || {
                let cb = on_input_cb.clone();
                if debounce_ms > 0 {
                    Box::new(debounce(
                        move |v: String| cb.emit(v),
                        Duration::from_millis(debounce_ms),
                    )) as Box<dyn FnMut(String)>
                } else {
                    Box::new(move |v: String| cb.emit(v)) as Box<dyn FnMut(String)>
                }
            })
        };
        #[cfg(not(target_arch = "wasm32"))]
        let debounced = use_mut_ref(move || {
            let cb = on_input_cb.clone();
            Box::new(move |v: String| cb.emit(v)) as Box<dyn FnMut(String)>
        });
        let oninput = {
            let debounced = debounced.clone();
            Callback::from(move |e: InputEvent| {
                let value = e
                    .target()
                    .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
                    .map(|el| el.value())
                    .unwrap_or_default();
                (debounced.borrow_mut())(value);
            })
        };

        html! {
            <input
                style={style_str}
                value={props.value.clone()}
                placeholder={props.placeholder.clone()}
                aria-label={props.aria_label.clone()}
                oninput={oninput}
            />
        }
    }
}

#[cfg(feature = "yew")]
pub use yew_impl::{TextField, TextFieldProps};

#[cfg(feature = "dioxus")]
mod dioxus_impl {
    use super::*;

    #[derive(Default, Clone, PartialEq)]
    pub struct TextFieldProps {
        pub value: String,
        pub placeholder: String,
        pub aria_label: String,
        pub color: TextFieldColor,
        pub size: TextFieldSize,
        pub variant: TextFieldVariant,
    }

    pub fn TextField(props: TextFieldProps) {
        let theme = use_theme();
        let _ = compute_parts(&theme, props.color, props.size, props.variant);
        let _ = (props.value, props.placeholder, props.aria_label);
    }
}

#[cfg(feature = "dioxus")]
pub use dioxus_impl::{TextField, TextFieldProps};

#[cfg(feature = "sycamore")]
mod sycamore_impl {
    use super::*;

    #[derive(Default, Clone, PartialEq)]
    pub struct TextFieldProps {
        pub value: String,
        pub placeholder: String,
        pub aria_label: String,
        pub color: TextFieldColor,
        pub size: TextFieldSize,
        pub variant: TextFieldVariant,
    }

    pub fn TextField(props: TextFieldProps) {
        let theme = use_theme();
        let _ = compute_parts(&theme, props.color, props.size, props.variant);
        let _ = (props.value, props.placeholder, props.aria_label);
    }
}

#[cfg(feature = "sycamore")]
pub use sycamore_impl::{TextField, TextFieldProps};
