use mui_styled_engine::use_theme;
#[cfg(target_arch = "wasm32")]
use mui_utils::debounce;
use mui_utils::deep_merge;
use serde_json::{json, Value};
#[cfg(target_arch = "wasm32")]
use std::time::Duration;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub use crate::macros::{
    Color as TextFieldColor, Size as TextFieldSize, Variant as TextFieldVariant,
};

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

/// Controlled text input field.
/// Styling is resolved through the shared [`Theme`] so colors and typography
/// remain consistent across the application.
#[function_component(TextField)]
pub fn text_field(props: &TextFieldProps) -> Html {
    let theme = use_theme();
    let color = match props.color {
        TextFieldColor::Primary => theme.palette.primary.clone(),
        TextFieldColor::Secondary => theme.palette.secondary.clone(),
    };
    let font_size = match props.size {
        TextFieldSize::Small => "0.8rem",
        TextFieldSize::Medium => "1rem",
        TextFieldSize::Large => "1.2rem",
    };
    let border = match props.variant {
        TextFieldVariant::Outlined => format!("1px solid {}", color),
        TextFieldVariant::Contained => format!("1px solid {}", color),
        TextFieldVariant::Text => "none".to_string(),
    };
    // Construct base style object which mirrors the themed defaults.
    // Representing style as JSON allows downstream consumers to declaratively
    // merge modifications without rewriting the entire CSS string.
    let mut style = json!({
        "color": color,
        "font-size": font_size,
        "border": border,
        "padding": "4px 8px",
    });
    if let Some(extra) = &props.style_overrides {
        // Merge overrides deeply; this makes incremental styling trivial.
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
    // Persist the debounced closure between renders to keep pending timers alive
    // and avoid re-allocating the inner state machine.
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
