//! Debounced text input component with theme-aware styling.
//!
//! The widget exposes adapters for Yew, Leptos, Dioxus and Sycamore. Shared
//! styling is expressed through [`css_with_theme!`](mui_styled_engine::css_with_theme)
//! so palette and spacing values derive from the active [`Theme`]. The
//! [`style_helpers::themed_class`](crate::style_helpers::themed_class) helper
//! converts styles into scoped classes ensuring each adapter references the
//! same generated CSS. Optional `style_overrides` allow callers to append raw
//! declarations without abandoning the centralized theme approach.
//!
//! ## Theme-driven styling
//! * **Palette integration** – colour variants map directly to
//!   [`Theme::palette`](mui_styled_engine::Theme) entries ensuring a primary
//!   field instantly reflects brand accents while secondary variants pick up the
//!   complementary tone.
//! * **Sizing** – font sizes align with Material defaults (`0.8rem`, `1rem`,
//!   `1.2rem`) so transitions between components remain visually cohesive. The
//!   generated CSS also standardises padding to mirror Material spacing tokens.
//! * **Border variants** – outlined and contained options share consistent
//!   border thickness while the text variant strips borders entirely. Because the
//!   logic lives in Rust, all frameworks share the same canonical treatment.
//!
//! ## Debounced input handling
//! Yew and Leptos adapters expose an optional `debounce_ms` prop that pipes user
//! input through [`mui_utils::debounce`], dramatically reducing chatter during
//! high-speed typing. Dioxus and Sycamore reuse the same styling helpers so the
//! CSS class is deterministic; upstream applications can reuse the
//! `data-debounce-ms` metadata emitted by `mui_system::themed_element` when
//! pairing SSR and client-side hydration flows.
//!
//! ## Accessibility
//! Every adapter forwards the `aria-label` to ensure assistive technologies have
//! a human readable description. Placeholders and values are mirrored as native
//! attributes so browser autofill and screen readers behave consistently. The
//! shared attribute assembly also guarantees SSR output matches hydrated markup,
//! eliminating brittle QA issues in pre-production environments.
#[cfg(feature = "leptos")]
use leptos::*;
#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]
use mui_styled_engine::{css_with_theme, use_theme, Theme};
#[cfg(target_arch = "wasm32")]
use mui_utils::debounce;
#[cfg(target_arch = "wasm32")]
use std::time::Duration;
#[cfg(feature = "leptos")]
use std::{cell::RefCell, rc::Rc};
#[cfg(feature = "yew")]
use wasm_bindgen::JsCast;
#[cfg(feature = "yew")]
use web_sys::HtmlInputElement;
#[cfg(feature = "yew")]
use yew::prelude::*;

pub use crate::macros::{
    Color as TextFieldColor, Size as TextFieldSize, Variant as TextFieldVariant,
};

#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]
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

#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]
fn resolve_class(
    color: TextFieldColor,
    size: TextFieldSize,
    variant: TextFieldVariant,
    style_overrides: Option<String>,
) -> String {
    let theme = use_theme();
    let (color, font_size, border) = compute_parts(&theme, color, size, variant);
    let extra = style_overrides.unwrap_or_default();
    crate::style_helpers::themed_class(css_with_theme!(
        theme,
        r#"
        color: ${color};
        font-size: ${font_size};
        border: ${border};
        padding: 4px 8px;
        ${extra}
        "#,
        color = color,
        font_size = font_size,
        border = border,
        extra = extra
    ))
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
    /// Additional CSS declarations appended to the themed base style.
    style_overrides: Option<String>,
});

#[cfg(feature = "yew")]
mod yew_impl {
    use super::*;

    /// Controlled text input field.
    ///
    /// Styling is centralized through [`css_with_theme!`] so palette colors,
    /// font sizes and borders track the active [`Theme`]. Optional `style_overrides`
    /// allow callers to append raw CSS snippets. When `debounce_ms` is greater
    /// than zero input events are delayed to avoid spamming the `on_input`
    /// callback. The `aria_label` is forwarded to ensure assistive technologies
    /// announce the purpose of the field.
    #[function_component(TextField)]
    pub fn text_field(props: &TextFieldProps) -> Html {
        let class = resolve_class(
            props.color,
            props.size,
            props.variant,
            props.style_overrides.clone(),
        );

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
                class={class}
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

#[cfg(feature = "leptos")]
mod leptos_impl {
    use super::*;
    use leptos::{ev::Event, event_target_value};

    /// Leptos variant rendering an accessible `<input>` element.
    ///
    /// Theme tokens drive colors, fonts and borders through [`css_with_theme!`].
    /// Optional `style_overrides` are interpolated into the same style block so
    /// callers can tweak presentation without abandoning the theme. An optional
    /// debounced callback limits rapid-fire updates while the `aria-label`
    /// ensures assistive technologies describe the field.
    #[component]
    pub fn TextField(
        #[prop(optional)] value: String,
        #[prop(optional)] placeholder: String,
        #[prop(optional)] aria_label: String,
        #[prop(optional)] debounce_ms: u64,
        #[prop(optional)] on_input: Option<Rc<dyn Fn(String)>>,
        #[prop(optional)] style_overrides: Option<String>,
        #[prop(optional)] color: TextFieldColor,
        #[prop(optional)] variant: TextFieldVariant,
        #[prop(optional)] size: TextFieldSize,
    ) -> impl IntoView {
        let class = resolve_class(color, size, variant, style_overrides.clone());
        let on_input_cb = on_input.unwrap_or_else(|| Rc::new(|_| {}));
        #[cfg(target_arch = "wasm32")]
        let debounced = {
            let cb = on_input_cb.clone();
            Rc::new(RefCell::new(if debounce_ms > 0 {
                Box::new(debounce(
                    move |v: String| cb(v),
                    Duration::from_millis(debounce_ms),
                )) as Box<dyn FnMut(String)>
            } else {
                Box::new(move |v: String| cb(v)) as Box<dyn FnMut(String)>
            }))
        };
        #[cfg(not(target_arch = "wasm32"))]
        let debounced = Rc::new(RefCell::new({
            let cb = on_input_cb.clone();
            Box::new(move |v: String| cb(v)) as Box<dyn FnMut(String)>
        }));
        let on_input_handler = {
            let debounced = debounced.clone();
            move |ev: Event| {
                let value = event_target_value(&ev);
                (debounced.borrow_mut())(value);
            }
        };
        view! {
            <input
                class=class
                value=value
                placeholder=placeholder
                aria-label=aria_label
                on:input=on_input_handler
            />
        }
    }
}

#[cfg(feature = "leptos")]
pub use leptos_impl::TextField;

#[cfg(feature = "dioxus")]
pub mod dioxus {
    use super::*;

    /// Properties consumed by the Dioxus adapter.
    #[derive(Default, Clone, PartialEq)]
    pub struct TextFieldProps {
        /// Current value displayed in the input element.
        pub value: String,
        /// Placeholder hint shown when the field is empty.
        pub placeholder: String,
        /// Accessibility label describing the purpose of the field.
        pub aria_label: String,
        /// Visual color scheme from the theme.
        pub color: TextFieldColor,
        /// Font sizing variant.
        pub size: TextFieldSize,
        /// Border style variant.
        pub variant: TextFieldVariant,
        /// Additional CSS declarations appended to the generated class.
        pub style_overrides: Option<String>,
    }

    /// Render the text field into an `<input>` tag with themed styling and
    /// `aria-label` metadata for accessibility.
    pub fn render(props: &TextFieldProps) -> String {
        let class = resolve_class(
            props.color.clone(),
            props.size.clone(),
            props.variant.clone(),
            props.style_overrides.clone(),
        );
        format!(
            "<input class=\"{}\" value=\"{}\" placeholder=\"{}\" aria-label=\"{}\" />",
            class, props.value, props.placeholder, props.aria_label
        )
    }
}

#[cfg(feature = "sycamore")]
pub mod sycamore {
    use super::*;

    /// Properties consumed by the Sycamore adapter.
    #[derive(Default, Clone, PartialEq)]
    pub struct TextFieldProps {
        /// Current value displayed in the input element.
        pub value: String,
        /// Placeholder hint shown when the field is empty.
        pub placeholder: String,
        /// Accessibility label describing the purpose of the field.
        pub aria_label: String,
        /// Visual color scheme from the theme.
        pub color: TextFieldColor,
        /// Font sizing variant.
        pub size: TextFieldSize,
        /// Border style variant.
        pub variant: TextFieldVariant,
        /// Additional CSS declarations appended to the generated class.
        pub style_overrides: Option<String>,
    }

    /// Render the text field into plain HTML with a theme-derived class and
    /// `aria-label` metadata for accessibility.
    pub fn render(props: &TextFieldProps) -> String {
        let class = resolve_class(
            props.color.clone(),
            props.size.clone(),
            props.variant.clone(),
            props.style_overrides.clone(),
        );
        format!(
            "<input class=\"{}\" value=\"{}\" placeholder=\"{}\" aria-label=\"{}\" />",
            class, props.value, props.placeholder, props.aria_label
        )
    }
}
