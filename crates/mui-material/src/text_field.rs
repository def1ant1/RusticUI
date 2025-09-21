//! Enterprise ready text input component powered by the headless
//! [`TextFieldState`](mui_headless::text_field::TextFieldState).
//!
//! The widget exposes adapters for Yew, Leptos, Dioxus and Sycamore. Shared
//! styling is expressed through [`css_with_theme!`](mui_styled_engine::css_with_theme)
//! so palette and spacing values derive from the active [`Theme`]. The
//! [`style_helpers::themed_class`](crate::style_helpers::themed_class) helper
//! converts styles into scoped classes ensuring each adapter references the
//! same generated CSS. For SSR adapters,
//! [`style_helpers::themed_attributes_html`](crate::style_helpers::themed_attributes_html)
//! serializes the themed class alongside ARIA and native input attributes so
//! hydration consistently matches server output. Optional `style_overrides`
//! allow callers to append raw declarations without abandoning the centralized
//! theme approach.
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
//! ## State-driven change management
//! Instead of wiring bespoke debounce timers per framework, adapters now invoke
//! [`TextFieldState::change`](mui_headless::text_field::TextFieldState::change),
//! [`TextFieldState::commit`](mui_headless::text_field::TextFieldState::commit)
//! and [`TextFieldState::reset`](mui_headless::text_field::TextFieldState::reset).
//! Each callback receives an owned snapshot (`TextFieldChangeEvent`,
//! `TextFieldCommitEvent`, `TextFieldResetEvent`) exposing debounced change
//! guidance, dirty/visited flags and analytics metadata so upstream code can
//! centralise data validation or instrumentation pipelines.
//!
//! ## Accessibility
//! Every adapter forwards the `aria-label` to ensure assistive technologies have
//! a human readable description. Placeholders and values are mirrored as native
//! attributes so browser autofill and screen readers behave consistently. The
//! shared attribute assembly also guarantees SSR output matches hydrated markup,
//! eliminating brittle QA issues in pre-production environments. ARIA flags and
//! analytics identifiers originate from the shared [`TextFieldState`] so SSR and
//! hydrated behaviour stay perfectly aligned.
#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]
use mui_headless::text_field::{
    TextFieldChangeEvent, TextFieldCommitEvent, TextFieldResetEvent, TextFieldState,
};
#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]
use mui_styled_engine::{css_with_theme, use_theme, Style, Theme};

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
fn resolve_style(
    color: TextFieldColor,
    size: TextFieldSize,
    variant: TextFieldVariant,
    style_overrides: Option<String>,
) -> Style {
    let theme = use_theme();
    let (color, font_size, border) = compute_parts(&theme, color, size, variant);
    let extra = style_overrides.unwrap_or_default();
    css_with_theme!(
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
    )
}

#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]
#[derive(Clone, Debug, PartialEq)]
struct TextFieldRenderSnapshot {
    value: String,
    dirty: bool,
    visited: bool,
    aria_invalid: Option<String>,
    aria_describedby: Option<String>,
    analytics_id: Option<String>,
    status_message: Option<String>,
}

#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]
impl TextFieldRenderSnapshot {
    /// Stringified representation of the dirty flag for attribute propagation.
    fn dirty_attr(&self) -> &'static str {
        if self.dirty {
            "true"
        } else {
            "false"
        }
    }

    /// Stringified representation of the visited flag for attribute propagation.
    fn visited_attr(&self) -> &'static str {
        if self.visited {
            "true"
        } else {
            "false"
        }
    }
}

#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]
fn snapshot_from_state(
    state: &TextFieldState,
    status_id: Option<&str>,
    analytics_id: Option<&str>,
) -> TextFieldRenderSnapshot {
    let builder = state.attributes();
    let builder = if let Some(id) = status_id {
        builder.status_id(id)
    } else {
        builder
    };
    let builder = if let Some(id) = analytics_id {
        builder.analytics_id(id)
    } else {
        builder
    };
    TextFieldRenderSnapshot {
        value: state.value().to_string(),
        dirty: state.dirty(),
        visited: state.visited(),
        aria_invalid: builder.aria_invalid().map(|(_, value)| value.to_string()),
        aria_describedby: builder
            .aria_describedby()
            .map(|(_, value)| value.to_string()),
        analytics_id: builder
            .data_analytics_id()
            .map(|(_, value)| value.to_string()),
        status_message: builder.status_message(),
    }
}

#[cfg(any(feature = "dioxus", feature = "sycamore"))]
fn ssr_input_attributes(
    snapshot: &TextFieldRenderSnapshot,
    placeholder: &str,
    aria_label: &str,
) -> Vec<(String, String)> {
    let mut attrs = vec![
        ("type".to_string(), "text".to_string()),
        ("value".to_string(), snapshot.value.clone()),
        ("placeholder".to_string(), placeholder.to_string()),
        ("aria-label".to_string(), aria_label.to_string()),
        ("data-dirty".to_string(), snapshot.dirty_attr().to_string()),
        (
            "data-visited".to_string(),
            snapshot.visited_attr().to_string(),
        ),
    ];
    if let Some(value) = &snapshot.aria_invalid {
        attrs.push(("aria-invalid".to_string(), value.clone()));
    }
    if let Some(value) = &snapshot.aria_describedby {
        attrs.push(("aria-describedby".to_string(), value.clone()));
    }
    if let Some(value) = &snapshot.analytics_id {
        attrs.push(("data-analytics-id".to_string(), value.clone()));
    }
    if let Some(value) = &snapshot.status_message {
        attrs.push(("data-status-message".to_string(), value.clone()));
    }
    attrs
}

#[cfg(any(feature = "yew", feature = "leptos"))]
mod shared_state_handle {
    use super::*;
    use std::cell::{Ref, RefCell, RefMut};
    use std::rc::Rc;

    /// Shared handle that grants adapters interior mutability over the
    /// [`TextFieldState`].  Wrapping the state inside `Rc<RefCell<_>>` allows
    /// multiple closures (input, blur, keyboard handlers) to coordinate without
    /// cloning the state machine.
    #[derive(Clone)]
    pub struct TextFieldStateHandle {
        inner: Rc<RefCell<TextFieldState>>,
    }

    impl TextFieldStateHandle {
        /// Construct a new handle from an owned state instance.
        pub fn new(state: TextFieldState) -> Self {
            Self {
                inner: Rc::new(RefCell::new(state)),
            }
        }

        /// Immutable access to the underlying state.
        pub fn borrow(&self) -> Ref<'_, TextFieldState> {
            self.inner.borrow()
        }

        /// Mutable access to the underlying state.
        pub fn borrow_mut(&self) -> RefMut<'_, TextFieldState> {
            self.inner.borrow_mut()
        }
    }

    impl From<TextFieldState> for TextFieldStateHandle {
        fn from(state: TextFieldState) -> Self {
            Self::new(state)
        }
    }

    impl PartialEq for TextFieldStateHandle {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.inner, &other.inner)
        }
    }
}

#[cfg(any(feature = "yew", feature = "leptos"))]
pub use shared_state_handle::TextFieldStateHandle;

#[cfg(feature = "yew")]
mod yew_impl {
    use super::*;
    use std::time::Duration;
    use wasm_bindgen::JsCast;
    use web_sys::{HtmlInputElement, KeyboardEvent};
    use yew::prelude::*;

    #[cfg(target_arch = "wasm32")]
    use mui_utils::debounce;

    /// Internal helper that memoizes a debounced change dispatcher.
    ///
    /// The Yew adapter historically delegated debouncing to `mui_utils::debounce`
    /// so consumers could throttle expensive validation or network operations.
    /// When the state-driven refactor landed that wiring was accidentally dropped,
    /// causing change callbacks to fire on every keystroke.  This struct stores
    /// the active callback and debounce interval so that we only rebuild the
    /// underlying timer when either input changes.  Each `emit` call forwards the
    /// latest [`TextFieldChangeEvent`] to the cached handler, preserving the
    /// original throttling semantics while still allowing the headless
    /// [`TextFieldState`] to own the authoritative value/flag bookkeeping.
    struct ChangeDispatcher {
        active_debounce: Option<Duration>,
        callback: Option<Callback<TextFieldChangeEvent>>,
        handler: Box<dyn FnMut(TextFieldChangeEvent)>,
    }

    impl ChangeDispatcher {
        /// Produce a dispatcher that performs no work until configured.
        fn new() -> Self {
            Self {
                active_debounce: None,
                callback: None,
                handler: Box::new(|_| {}),
            }
        }

        /// Ensure the dispatcher reflects the latest debounce metadata.
        fn ensure(
            &mut self,
            debounce_window: Option<Duration>,
            callback: Option<Callback<TextFieldChangeEvent>>,
        ) {
            let normalized = debounce_window.filter(|delay| !delay.is_zero());
            if self.active_debounce == normalized && self.callback == callback {
                return;
            }
            self.active_debounce = normalized;
            self.callback = callback.clone();
            self.handler = Self::build_handler(normalized, callback);
        }

        /// Forward the provided event through the cached handler.
        fn emit(&mut self, event: TextFieldChangeEvent) {
            (self.handler)(event);
        }

        /// Construct a concrete handler that optionally wraps the callback in a debounce timer.
        fn build_handler(
            debounce_window: Option<Duration>,
            callback: Option<Callback<TextFieldChangeEvent>>,
        ) -> Box<dyn FnMut(TextFieldChangeEvent)> {
            match callback {
                None => Box::new(|_| {}),
                Some(cb) => {
                    #[cfg(target_arch = "wasm32")]
                    {
                        if let Some(delay) = debounce_window {
                            let cb_inner = cb.clone();
                            let mut debounced = debounce(
                                move |event: TextFieldChangeEvent| {
                                    cb_inner.emit(event);
                                },
                                delay,
                            );
                            Box::new(move |event: TextFieldChangeEvent| {
                                debounced(event);
                            })
                        } else {
                            let cb_inner = cb.clone();
                            Box::new(move |event: TextFieldChangeEvent| {
                                cb_inner.emit(event);
                            })
                        }
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        let cb_inner = cb.clone();
                        Box::new(move |event: TextFieldChangeEvent| {
                            cb_inner.emit(event);
                        })
                    }
                }
            }
        }
    }

    /// Properties consumed by the Yew text field component.
    #[derive(Properties, Clone, PartialEq)]
    pub struct TextFieldProps {
        /// Shared state machine controlling value, dirty/visited flags and debounce metadata.
        pub state: TextFieldStateHandle,
        /// Optional placeholder hint rendered when the field is empty.
        #[prop_or_default]
        pub placeholder: AttrValue,
        /// Optional accessible label.
        #[prop_or_default]
        pub aria_label: AttrValue,
        /// Optional status element identifier used to link `aria-describedby`.
        #[prop_or_default]
        pub status_id: Option<AttrValue>,
        /// Optional analytics identifier forwarded as `data-analytics-id`.
        #[prop_or_default]
        pub analytics_id: Option<AttrValue>,
        /// Additional CSS declarations appended to the themed base style.
        #[prop_or_default]
        pub style_overrides: Option<String>,
        /// Visual color scheme.
        #[prop_or_default]
        pub color: TextFieldColor,
        /// Stylistic variant.
        #[prop_or_default]
        pub variant: TextFieldVariant,
        /// Component size.
        #[prop_or_default]
        pub size: TextFieldSize,
        /// Callback invoked when the value changes.
        #[prop_or_default]
        pub on_change: Option<Callback<TextFieldChangeEvent>>,
        /// Callback invoked when the field commits (blur or enter).
        #[prop_or_default]
        pub on_commit: Option<Callback<TextFieldCommitEvent>>,
        /// Callback invoked after the field resets (escape key).
        #[prop_or_default]
        pub on_reset: Option<Callback<TextFieldResetEvent>>,
    }

    /// Controlled text input field driven entirely by [`TextFieldState`].
    #[function_component(TextField)]
    pub fn text_field(props: &TextFieldProps) -> Html {
        let class = crate::style_helpers::themed_class(resolve_style(
            props.color.clone(),
            props.size.clone(),
            props.variant.clone(),
            props.style_overrides.clone(),
        ));

        // Internal counter used to trigger re-renders when the shared state mutates.
        let version = use_state(|| 0u64);

        let status_id = props.status_id.as_ref().map(|value| value.as_str());
        let analytics_id = props.analytics_id.as_ref().map(|value| value.as_str());
        let snapshot = {
            let state = props.state.borrow();
            snapshot_from_state(&state, status_id, analytics_id)
        };

        let aria_invalid: Option<AttrValue> = snapshot
            .aria_invalid
            .clone()
            .map(|value| AttrValue::from(value));
        let aria_describedby: Option<AttrValue> = snapshot
            .aria_describedby
            .clone()
            .map(|value| AttrValue::from(value));
        let data_status_message: Option<AttrValue> =
            snapshot.status_message.clone().map(AttrValue::from);
        let data_analytics_id: Option<AttrValue> =
            snapshot.analytics_id.clone().map(AttrValue::from);

        let change_dispatch = use_mut_ref(ChangeDispatcher::new);
        let on_change_cb = props.on_change.clone();
        let state_for_input = props.state.clone();
        let version_for_input = version.clone();
        let change_dispatch_for_input = change_dispatch.clone();
        let oninput = Callback::from(move |event: InputEvent| {
            let value = event
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
                .map(|el| el.value())
                .unwrap_or_default();
            let callback = on_change_cb.clone();
            {
                let mut state = state_for_input.borrow_mut();
                state.change(value, |snapshot| {
                    let event = TextFieldChangeEvent::from(snapshot);
                    let mut dispatcher = change_dispatch_for_input.borrow_mut();
                    dispatcher.ensure(event.debounce, callback.clone());
                    dispatcher.emit(event);
                });
            }
            let next = (*version_for_input).wrapping_add(1);
            version_for_input.set(next);
        });

        let on_commit_cb = props.on_commit.clone();
        let state_for_blur = props.state.clone();
        let version_for_blur = version.clone();
        let onblur = Callback::from(move |_event: FocusEvent| {
            let callback = on_commit_cb.clone();
            {
                let mut state = state_for_blur.borrow_mut();
                state.commit(|snapshot| {
                    if let Some(cb) = callback.clone() {
                        cb.emit(TextFieldCommitEvent::from(snapshot));
                    }
                });
            }
            let next = (*version_for_blur).wrapping_add(1);
            version_for_blur.set(next);
        });

        let commit_cb_key = props.on_commit.clone();
        let reset_cb_key = props.on_reset.clone();
        let state_for_keys = props.state.clone();
        let version_for_keys = version.clone();
        let onkeydown = Callback::from(move |event: KeyboardEvent| {
            let mut should_refresh = false;
            match event.key().as_str() {
                "Enter" => {
                    event.prevent_default();
                    let callback = commit_cb_key.clone();
                    {
                        let mut state = state_for_keys.borrow_mut();
                        state.commit(|snapshot| {
                            if let Some(cb) = callback.clone() {
                                cb.emit(TextFieldCommitEvent::from(snapshot));
                            }
                        });
                    }
                    should_refresh = true;
                }
                "Escape" => {
                    event.prevent_default();
                    let callback = reset_cb_key.clone();
                    {
                        let mut state = state_for_keys.borrow_mut();
                        state.reset(|snapshot| {
                            if let Some(cb) = callback.clone() {
                                cb.emit(TextFieldResetEvent::from(snapshot));
                            }
                        });
                    }
                    should_refresh = true;
                }
                _ => {}
            }
            if should_refresh {
                let next = (*version_for_keys).wrapping_add(1);
                version_for_keys.set(next);
            }
        });

        html! {
            <input
                class={class}
                value={AttrValue::from(snapshot.value.clone())}
                placeholder={props.placeholder.clone()}
                aria-label={props.aria_label.clone()}
                aria-invalid={aria_invalid}
                aria-describedby={aria_describedby}
                data-dirty={AttrValue::from(snapshot.dirty_attr())}
                data-visited={AttrValue::from(snapshot.visited_attr())}
                data-status-message={data_status_message}
                data-analytics-id={data_analytics_id}
                oninput={oninput}
                onblur={onblur}
                onkeydown={onkeydown}
            />
        }
    }
}

#[cfg(feature = "yew")]
pub use yew_impl::{TextField, TextFieldProps};

#[cfg(feature = "leptos")]
mod leptos_impl {
    use super::*;
    use leptos::{
        component, create_memo, create_signal,
        ev::{Event, FocusEvent, KeyboardEvent},
        event_target_value, view, IntoView, SignalGet, SignalSet, SignalUpdate,
    };
    use std::rc::Rc;

    /// Properties consumed by the Leptos text field component.
    #[derive(leptos::Props, Clone, PartialEq)]
    pub struct TextFieldProps {
        /// Shared state machine powering the input.
        pub state: TextFieldStateHandle,
        /// Optional placeholder text.
        #[prop(optional, into)]
        pub placeholder: Option<String>,
        /// Optional accessibility label.
        #[prop(optional, into)]
        pub aria_label: Option<String>,
        /// Optional status identifier for validation messages.
        #[prop(optional, into)]
        pub status_id: Option<String>,
        /// Optional analytics identifier forwarded to the DOM.
        #[prop(optional, into)]
        pub analytics_id: Option<String>,
        /// Additional CSS overrides appended to the themed style.
        #[prop(optional)]
        pub style_overrides: Option<String>,
        /// Visual color scheme.
        #[prop(optional)]
        pub color: Option<TextFieldColor>,
        /// Stylistic variant.
        #[prop(optional)]
        pub variant: Option<TextFieldVariant>,
        /// Component size.
        #[prop(optional)]
        pub size: Option<TextFieldSize>,
        /// Callback invoked whenever the value changes.
        #[prop(optional)]
        pub on_change: Option<Rc<dyn Fn(TextFieldChangeEvent)>>,
        /// Callback invoked on commit (blur or enter).
        #[prop(optional)]
        pub on_commit: Option<Rc<dyn Fn(TextFieldCommitEvent)>>,
        /// Callback invoked when the field resets (escape key).
        #[prop(optional)]
        pub on_reset: Option<Rc<dyn Fn(TextFieldResetEvent)>>,
    }

    /// Leptos variant mirroring the Yew implementation by driving behaviour from [`TextFieldState`].
    #[component]
    pub fn TextField(props: TextFieldProps) -> impl IntoView {
        let TextFieldProps {
            state,
            placeholder,
            aria_label,
            status_id,
            analytics_id,
            style_overrides,
            color,
            variant,
            size,
            on_change,
            on_commit,
            on_reset,
        } = props;

        let placeholder = placeholder.unwrap_or_default();
        let aria_label = aria_label.unwrap_or_default();
        let color = color.unwrap_or_default();
        let variant = variant.unwrap_or_default();
        let size = size.unwrap_or_default();

        let class = crate::style_helpers::themed_class(resolve_style(
            color.clone(),
            size.clone(),
            variant.clone(),
            style_overrides.clone(),
        ));

        let (version, set_version) = create_signal(0u64);
        let state_for_snapshot = state.clone();
        let status_id_for_snapshot = status_id.clone();
        let analytics_id_for_snapshot = analytics_id.clone();
        let snapshot = create_memo(move |_| {
            version.get();
            let state = state_for_snapshot.borrow();
            snapshot_from_state(
                &state,
                status_id_for_snapshot.as_deref(),
                analytics_id_for_snapshot.as_deref(),
            )
        });

        let change_cb = on_change.clone();
        let state_for_input = state.clone();
        let set_version_input = set_version.clone();
        let on_input_handler = move |ev: Event| {
            let value = event_target_value(&ev);
            let callback = change_cb.clone();
            {
                let mut state = state_for_input.borrow_mut();
                state.change(value, |snapshot| {
                    if let Some(cb) = callback.clone() {
                        cb(TextFieldChangeEvent::from(snapshot));
                    }
                });
            }
            set_version_input.update(|tick| *tick = tick.wrapping_add(1));
        };

        let commit_cb = on_commit.clone();
        let state_for_blur = state.clone();
        let set_version_blur = set_version.clone();
        let on_blur_handler = move |_ev: FocusEvent| {
            let callback = commit_cb.clone();
            {
                let mut state = state_for_blur.borrow_mut();
                state.commit(|snapshot| {
                    if let Some(cb) = callback.clone() {
                        cb(TextFieldCommitEvent::from(snapshot));
                    }
                });
            }
            set_version_blur.update(|tick| *tick = tick.wrapping_add(1));
        };

        let commit_cb_key = on_commit.clone();
        let reset_cb_key = on_reset.clone();
        let state_for_keys = state.clone();
        let set_version_keys = set_version.clone();
        let on_keydown_handler = move |ev: KeyboardEvent| {
            let mut should_refresh = false;
            match ev.key().as_str() {
                "Enter" => {
                    ev.prevent_default();
                    let callback = commit_cb_key.clone();
                    {
                        let mut state = state_for_keys.borrow_mut();
                        state.commit(|snapshot| {
                            if let Some(cb) = callback.clone() {
                                cb(TextFieldCommitEvent::from(snapshot));
                            }
                        });
                    }
                    should_refresh = true;
                }
                "Escape" => {
                    ev.prevent_default();
                    let callback = reset_cb_key.clone();
                    {
                        let mut state = state_for_keys.borrow_mut();
                        state.reset(|snapshot| {
                            if let Some(cb) = callback.clone() {
                                cb(TextFieldResetEvent::from(snapshot));
                            }
                        });
                    }
                    should_refresh = true;
                }
                _ => {}
            }
            if should_refresh {
                set_version_keys.update(|tick| *tick = tick.wrapping_add(1));
            }
        };

        view! {
            <input
                class=class
                prop:value=move || snapshot.get().value.clone()
                placeholder=placeholder.clone()
                aria-label=aria_label.clone()
                attr:aria-invalid=move || snapshot.get().aria_invalid.clone()
                attr:aria-describedby=move || snapshot.get().aria_describedby.clone()
                attr:data-dirty=move || snapshot.get().dirty_attr().to_string()
                attr:data-visited=move || snapshot.get().visited_attr().to_string()
                attr:data-status-message=move || snapshot.get().status_message.clone()
                attr:data-analytics-id=move || snapshot.get().analytics_id.clone()
                on:input=on_input_handler
                on:blur=on_blur_handler
                on:keydown=on_keydown_handler
            />
        }
    }
}

#[cfg(feature = "leptos")]
pub use leptos_impl::{TextField, TextFieldProps};

#[cfg(feature = "dioxus")]
pub mod dioxus {
    use super::*;

    /// Properties consumed by the Dioxus adapter.
    #[derive(Default, Clone, PartialEq)]
    pub struct TextFieldProps {
        /// Placeholder hint rendered when the field is empty.
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
        /// Optional status identifier used to link validation messaging.
        pub status_id: Option<String>,
        /// Optional analytics identifier mirrored to the DOM.
        pub analytics_id: Option<String>,
    }

    /// Render the text field into an `<input>` tag with themed styling and
    /// state-driven metadata.
    pub fn render(props: &TextFieldProps, state: &TextFieldState) -> String {
        let snapshot = snapshot_from_state(
            state,
            props.status_id.as_deref(),
            props.analytics_id.as_deref(),
        );
        let attr_string = crate::style_helpers::themed_attributes_html(
            resolve_style(
                props.color.clone(),
                props.size.clone(),
                props.variant.clone(),
                props.style_overrides.clone(),
            ),
            ssr_input_attributes(&snapshot, &props.placeholder, &props.aria_label),
        );
        format!("<input {attrs} />", attrs = attr_string)
    }
}

#[cfg(feature = "sycamore")]
pub mod sycamore {
    use super::*;

    /// Properties consumed by the Sycamore adapter.
    #[derive(Default, Clone, PartialEq)]
    pub struct TextFieldProps {
        /// Placeholder hint rendered when the field is empty.
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
        /// Optional status identifier used to link validation messaging.
        pub status_id: Option<String>,
        /// Optional analytics identifier mirrored to the DOM.
        pub analytics_id: Option<String>,
    }

    /// Render the text field into plain HTML with theme-derived styling and
    /// state-driven metadata.
    pub fn render(props: &TextFieldProps, state: &TextFieldState) -> String {
        let snapshot = snapshot_from_state(
            state,
            props.status_id.as_deref(),
            props.analytics_id.as_deref(),
        );
        let attr_string = crate::style_helpers::themed_attributes_html(
            resolve_style(
                props.color.clone(),
                props.size.clone(),
                props.variant.clone(),
                props.style_overrides.clone(),
            ),
            ssr_input_attributes(&snapshot, &props.placeholder, &props.aria_label),
        );
        format!("<input {attrs} />", attrs = attr_string)
    }
}

#[cfg(all(
    test,
    any(
        feature = "yew",
        feature = "leptos",
        feature = "dioxus",
        feature = "sycamore"
    )
))]
mod tests {
    use super::{snapshot_from_state, ssr_input_attributes};
    use mui_headless::text_field::TextFieldState;

    #[test]
    fn snapshot_reflects_dirty_and_visited_flags() {
        let mut state = TextFieldState::uncontrolled("seed", None);
        let first = snapshot_from_state(&state, None, None);
        assert_eq!(first.dirty_attr(), "false");
        assert_eq!(first.visited_attr(), "false");

        state.change("updated", |_| {});
        let after_change = snapshot_from_state(&state, None, None);
        assert_eq!(after_change.dirty_attr(), "true");
        assert_eq!(after_change.visited_attr(), "false");

        state.commit(|_| {});
        let after_commit = snapshot_from_state(&state, None, None);
        assert_eq!(after_commit.dirty_attr(), "true");
        assert_eq!(after_commit.visited_attr(), "true");
    }

    #[test]
    fn ssr_attributes_include_error_status() {
        let mut state = TextFieldState::uncontrolled("", None);
        state.set_errors(vec!["Required".into()]);
        let snapshot = snapshot_from_state(&state, Some("status"), Some("analytics-1"));
        let attrs = ssr_input_attributes(&snapshot, "Placeholder", "Label");
        assert!(attrs
            .iter()
            .any(|(k, v)| k == "aria-invalid" && v == "true"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == "aria-describedby" && v == "status"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == "data-analytics-id" && v == "analytics-1"));
        assert!(attrs.iter().any(|(k, _)| k == "data-status-message"));
    }
}
