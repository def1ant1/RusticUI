//! Minimal dialog container demonstrating theme-aware styling and accessibility.
//!
//! ## State driven rendering
//! * Every adapter consumes a [`DialogState`](rustic_ui_headless::dialog::DialogState)
//!   so the surface/backdrop visibility, focus trap analytics and escape key
//!   telemetry remain authoritative. Passing the state object keeps server-side
//!   rendering (SSR) and client-side rendering (CSR) perfectly aligned because
//!   both environments observe the same lifecycle phases and transition
//!   metadata.
//! * Attribute builders returned by `rustic_ui_headless` – such as
//!   [`DialogSurfaceAttributes`](rustic_ui_headless::dialog::DialogSurfaceAttributes)
//!   and [`DialogBackdropAttributes`](rustic_ui_headless::dialog::DialogBackdropAttributes)
//!   – feed into shared helpers that produce automation friendly `data-*`
//!   tuples. This ensures analytics pipelines and integration tests receive the
//!   focus-trap and transition markers that enterprise deployments rely on.
//!
//! ## Style composition
//! * [`css_with_theme!`](rustic_ui_styled_engine::css_with_theme) powers every
//!   adapter. The macro exposes a `theme` binding so border colours pull from
//!   `theme.palette.secondary` while padding respects `theme.spacing(3)`.
//!   Wrapping the declaration inside
//!   [`style_helpers::themed_class`](crate::style_helpers::themed_class) produces
//!   a deterministic class name that can be safely reused across renders without
//!   leaking duplicate strings.
//! * Each framework module calls back into [`resolve_style`] so client side
//!   components (Yew/Leptos) and server-side renderers (Leptos/Dioxus/Sycamore)
//!   receive the identical scoped class. This keeps brand styling consistent
//!   even when applications mix rendering strategies for pre-production smoke
//!   tests or hybrid deployments.
//!
//! Centralising style and accessibility helpers drastically reduces repetitive
//! setup when scaling to multiple enterprise applications because adapters only
//! forward the [`DialogState`] snapshot plus optional attribute overrides.

#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore",
))]
use rustic_ui_headless::dialog::{DialogBackdropAttributes, DialogState, DialogSurfaceAttributes};
#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore",
))]
use rustic_ui_styled_engine::{css_with_theme, Style};

#[cfg(feature = "leptos")]
use leptos::children::Children;
#[cfg(feature = "yew")]
use yew::prelude::*;

/// Generates the [`Style`] scoped to this dialog using the active [`Theme`].
///
/// [`css_with_theme!`] exposes a `theme` binding allowing palette and spacing
/// values to be substituted directly inside the CSS template. The class is
/// derived once per render and applied to the `<div>` element in every
/// framework adapter which keeps styling logic centralized and easy to
/// maintain.
#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]
fn resolve_style() -> Style {
    css_with_theme!(
        r#"
        border: 2px solid ${border};
        padding: ${pad};
        "#,
        // Pull colors and spacing from the theme so consumers only tweak
        // global tokens instead of individual components.
        border = theme.palette.secondary.clone(),
        pad = format!("{}px", theme.spacing(3))
    )
}

// ---------------------------------------------------------------------------
// Shared attribute helpers
// ---------------------------------------------------------------------------

/// Declarative overrides applied to the [`DialogSurfaceAttributes`] builder.
///
/// The struct intentionally stores owned `String` values so enterprise
/// orchestrators can construct automation-friendly identifiers once and clone
/// them across CSR/SSR adapters.  Each field maps directly to a builder method
/// on [`DialogSurfaceAttributes`], drastically reducing boilerplate inside
/// framework integrations.
#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore",
))]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DialogSurfaceOptions {
    /// Optional DOM id for the dialog surface.
    pub id: Option<String>,
    /// Optional `aria-labelledby` reference describing the dialog.
    pub labelled_by: Option<String>,
    /// Optional `aria-describedby` reference with supporting copy.
    pub described_by: Option<String>,
    /// Optional analytics tag exposed via `data-analytics-id`.
    pub analytics_id: Option<String>,
}

#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore",
))]
fn apply_surface_options<'a>(
    mut attrs: DialogSurfaceAttributes<'a>,
    options: &'a DialogSurfaceOptions,
) -> DialogSurfaceAttributes<'a> {
    if let Some(id) = &options.id {
        attrs = attrs.id(id);
    }
    if let Some(labelled_by) = &options.labelled_by {
        attrs = attrs.labelled_by(labelled_by);
    }
    if let Some(described_by) = &options.described_by {
        attrs = attrs.described_by(described_by);
    }
    if let Some(analytics) = &options.analytics_id {
        attrs = attrs.analytics_id(analytics);
    }
    attrs
}

/// Converts the surface attribute builder into automation-friendly key/value
/// pairs.
#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore",
))]
#[must_use]
pub fn dialog_surface_attributes(
    attrs: DialogSurfaceAttributes<'_>,
    aria_label: Option<&str>,
) -> Vec<(String, String)> {
    let mut pairs = Vec::with_capacity(10);
    pairs.push(("role".into(), attrs.role().into()));
    let (aria_modal_key, aria_modal_value) = attrs.aria_modal();
    pairs.push((aria_modal_key.into(), aria_modal_value.into()));
    if let Some((key, value)) = attrs.id_attr() {
        pairs.push((key.into(), value.into()));
    }
    if let Some((key, value)) = attrs.aria_labelledby() {
        pairs.push((key.into(), value.into()));
    }
    if let Some((key, value)) = attrs.aria_describedby() {
        pairs.push((key.into(), value.into()));
    }
    if let Some(label) = aria_label {
        if !label.is_empty() {
            pairs.push(("aria-label".into(), label.into()));
        }
    }
    let (state_key, state_value) = attrs.data_state();
    pairs.push((state_key.into(), state_value.into()));
    if let Some((key, value)) = attrs.data_transition() {
        pairs.push((key.into(), value.into()));
    }
    let (trap_key, trap_value) = attrs.data_focus_trap();
    pairs.push((trap_key.into(), trap_value.into()));
    if let Some((key, value)) = attrs.data_analytics_id() {
        pairs.push((key.into(), value.into()));
    }
    pairs
}

/// Converts the backdrop attribute builder into automation-friendly key/value
/// pairs so orchestrators can wire telemetry consistently across adapters.
#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore",
))]
#[must_use]
pub fn dialog_backdrop_attributes(attrs: DialogBackdropAttributes<'_>) -> Vec<(String, String)> {
    let mut pairs = Vec::with_capacity(3);
    let (state_key, state_value) = attrs.data_state();
    pairs.push((state_key.into(), state_value.into()));
    pairs.push(("data-visible".into(), attrs.is_visible().to_string()));
    pairs
}

#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore",
))]
fn surface_attribute_pairs(
    attrs: DialogSurfaceAttributes<'_>,
    aria_label: Option<&str>,
) -> Vec<(String, String)> {
    dialog_surface_attributes(attrs, aria_label)
}

/// Shared helper wiring framework agnostic ARIA metadata into the dialog.
///
/// String-based adapters (Leptos SSR/Dioxus/Sycamore) delegate to this function
/// so the full automation payload is emitted in a deterministic order. Returning
/// a `String` keeps the helpers friendly for snapshot tests and other
/// automation harnesses that reason about serialized HTML.
#[cfg(any(feature = "leptos", feature = "dioxus", feature = "sycamore"))]
fn render_dialog_surface_html(
    attrs: DialogSurfaceAttributes<'_>,
    aria_label: Option<&str>,
    child: &str,
) -> String {
    crate::render_helpers::render_element_html(
        "div",
        resolve_style(),
        surface_attribute_pairs(attrs, aria_label),
        child,
    )
}

// ---------------------------------------------------------------------------
// React SSR adapter
// ---------------------------------------------------------------------------

/// React integrations lean on the serialized markup for parity checks and
/// hydration validation. Exposing a thin adapter that mirrors the server-side
/// renderers keeps framework comparisons straightforward without duplicating
/// style or accessibility wiring.
#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]
pub mod react {
    use super::*;

    /// Properties accepted by the React oriented renderer. The struct mirrors
    /// the SSR adapters to keep orchestration consistent across frameworks.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct DialogProps {
        /// Dialog state machine controlling visibility and analytics hooks.
        pub state: DialogState,
        /// Optional attribute overrides applied to the dialog surface.
        pub surface: DialogSurfaceOptions,
        /// Serialized child markup rendered inside the dialog.
        pub children: String,
        /// Optional accessible label announced by assistive technologies.
        pub aria_label: Option<String>,
    }

    /// Render the dialog surface using the shared SSR helper so React output
    /// matches the strings produced by Leptos/Dioxus/Sycamore adapters.
    pub fn render(props: &DialogProps) -> String {
        if !props.state.is_open() {
            return String::new();
        }
        let surface_attrs = apply_surface_options(props.state.surface_attributes(), &props.surface);
        super::render_dialog_surface_html(
            surface_attrs,
            props.aria_label.as_deref(),
            &props.children,
        )
    }
}

// ---------------------------------------------------------------------------
// Yew adapter
// ---------------------------------------------------------------------------

#[cfg(feature = "yew")]
mod yew_impl {
    use super::*;
    use std::rc::Rc;
    use yew::virtual_dom::VNode;

    /// Properties consumed by the Yew dialog component.
    #[derive(Properties, Clone, PartialEq)]
    pub struct DialogProps {
        /// Dialog state machine powering visibility, analytics and focus trap
        /// toggles.
        pub state: Rc<DialogState>,
        /// Optional surface attribute overrides such as element ids or
        /// analytics hooks.
        #[prop_or_default]
        pub surface: DialogSurfaceOptions,
        /// Optional accessible label when no heading is available.
        #[prop_or_default]
        pub aria_label: Option<AttrValue>,
        /// Dialog contents rendered inside the container.
        #[prop_or_default]
        pub children: Children,
    }

    fn apply_surface_attributes(tag: &mut yew::virtual_dom::VTag, attrs: Vec<(String, String)>) {
        for (key, value) in attrs {
            match key.as_str() {
                "role" => tag.add_attribute("role", value),
                "aria-modal" => tag.add_attribute("aria-modal", value),
                "id" => tag.add_attribute("id", value),
                "aria-labelledby" => tag.add_attribute("aria-labelledby", value),
                "aria-describedby" => tag.add_attribute("aria-describedby", value),
                "aria-label" => tag.add_attribute("aria-label", value),
                "data-state" => tag.add_attribute("data-state", value),
                "data-transition" => tag.add_attribute("data-transition", value),
                "data-focus-trap" => tag.add_attribute("data-focus-trap", value),
                "data-analytics-id" => tag.add_attribute("data-analytics-id", value),
                unexpected => {
                    debug_assert!(
                        false,
                        "unhandled dialog attribute `{}`; please update the Yew adapter",
                        unexpected
                    );
                }
            }
        }
    }

    /// Minimal dialog implementation that toggles visibility and wires up
    /// accessibility and analytics attributes directly from the [`DialogState`].
    #[function_component(Dialog)]
    pub fn dialog(props: &DialogProps) -> Html {
        if !props.state.is_open() {
            return Html::default();
        }
        let class = crate::style_helpers::themed_class(resolve_style());
        let aria_label = props.aria_label.as_ref().map(|value| value.as_str());
        let surface_attrs = apply_surface_options(props.state.surface_attributes(), &props.surface);
        let attrs = surface_attribute_pairs(surface_attrs, aria_label);
        let mut node = html! {
            <div class={class}>
                { for props.children.iter() }
            </div>
        };
        if let VNode::VTag(ref mut tag) = node {
            apply_surface_attributes(tag, attrs);
        }
        node
    }
}

#[cfg(feature = "yew")]
pub use yew_impl::{Dialog, DialogProps};

// ---------------------------------------------------------------------------
// Leptos adapter
// ---------------------------------------------------------------------------

#[cfg(feature = "leptos")]
mod leptos_impl {
    use super::*;
    use leptos::*;

    /// Properties consumed by the Leptos dialog component.
    #[derive(leptos::Props, Clone, PartialEq)]
    pub struct DialogProps {
        /// Dialog state machine powering visibility, transitions and analytics.
        #[prop(into)]
        pub state: DialogState,
        /// Optional accessible label when headings are not available.
        #[prop(optional, into)]
        pub aria_label: Option<String>,
        /// Optional surface attribute overrides propagated to the element.
        #[prop(optional)]
        pub surface: Option<DialogSurfaceOptions>,
        /// Dialog contents rendered inside the container.
        pub children: Children,
    }

    /// Leptos variant mirroring the Yew implementation by deriving attributes
    /// directly from the [`DialogState`].
    #[component]
    pub fn Dialog(props: DialogProps) -> impl IntoView {
        let DialogProps {
            state,
            aria_label,
            surface,
            children,
        } = props;
        if !state.is_open() {
            return view! {};
        }
        let class = crate::style_helpers::themed_class(resolve_style());
        let surface = surface.unwrap_or_default();
        let surface_attrs = apply_surface_options(state.surface_attributes(), &surface);
        let attrs = surface_attribute_pairs(surface_attrs, aria_label.as_deref());
        let mut element = leptos::html::div().class(class);
        for (key, value) in attrs {
            element = element.attr(key, value);
        }
        element.child(children()).into_view()
    }
}

#[cfg(feature = "leptos")]
pub use leptos_impl::{Dialog, DialogProps};

// ---------------------------------------------------------------------------
// Leptos SSR adapter
// ---------------------------------------------------------------------------

#[cfg(feature = "leptos")]
pub mod leptos {
    use super::*;

    /// Properties consumed by the Leptos SSR adapter. We intentionally mirror
    /// the structure of other SSR focused modules so applications can swap
    /// adapters without re-mapping state or accessibility metadata.
    #[derive(Clone, PartialEq)]
    pub struct DialogProps {
        /// Dialog state machine powering visibility and analytics metadata.
        pub state: DialogState,
        /// Attribute overrides applied to the dialog surface.
        pub surface: DialogSurfaceOptions,
        /// Raw HTML/text representing the dialog contents.
        pub children: String,
        /// Optional accessible label announced by assistive technologies.
        pub aria_label: Option<String>,
    }

    impl Default for DialogProps {
        fn default() -> Self {
            Self {
                state: DialogState::uncontrolled(false),
                surface: DialogSurfaceOptions::default(),
                children: String::new(),
                aria_label: None,
            }
        }
    }

    /// Render the dialog into a HTML string using `css_with_theme!` for
    /// styling. Closed dialogs return an empty string so hidden regions never
    /// reach the accessibility tree.
    pub fn render(props: &DialogProps) -> String {
        if !props.state.is_open() {
            return String::new();
        }
        let surface_attrs = apply_surface_options(props.state.surface_attributes(), &props.surface);
        super::render_dialog_surface_html(
            surface_attrs,
            props.aria_label.as_deref(),
            &props.children,
        )
    }
}

// ---------------------------------------------------------------------------
// Dioxus adapter
// ---------------------------------------------------------------------------

#[cfg(feature = "dioxus")]
pub mod dioxus {
    use super::*;

    /// Properties consumed by the Dioxus adapter. The struct intentionally
    /// mirrors the fields used by other frameworks so business logic remains
    /// consistent across integrations.
    #[derive(Clone, PartialEq)]
    pub struct DialogProps {
        /// Dialog state machine powering visibility and analytics metadata.
        pub state: DialogState,
        /// Attribute overrides applied to the dialog surface.
        pub surface: DialogSurfaceOptions,
        /// Child markup rendered inside the dialog.
        pub children: String,
        /// Optional accessible label announced by assistive technologies.
        pub aria_label: Option<String>,
    }

    impl Default for DialogProps {
        fn default() -> Self {
            Self {
                state: DialogState::uncontrolled(false),
                surface: DialogSurfaceOptions::default(),
                children: String::new(),
                aria_label: None,
            }
        }
    }

    /// Render the dialog into a `<div>` tag using a theme-derived class and
    /// standard ARIA attributes. Closed dialogs yield an empty string so hidden
    /// content is never announced by screen readers.
    pub fn render(props: &DialogProps) -> String {
        if !props.state.is_open() {
            return String::new();
        }
        let surface_attrs = apply_surface_options(props.state.surface_attributes(), &props.surface);
        super::render_dialog_surface_html(
            surface_attrs,
            props.aria_label.as_deref(),
            &props.children,
        )
    }
}

// ---------------------------------------------------------------------------
// Sycamore adapter
// ---------------------------------------------------------------------------

#[cfg(feature = "sycamore")]
pub mod sycamore {
    use super::*;

    /// Sycamore variant of the [`Dialog`] with identical fields to other
    /// adapters to minimize repetitive setup.
    #[derive(Clone, PartialEq)]
    pub struct DialogProps {
        /// Dialog state machine powering visibility and analytics metadata.
        pub state: DialogState,
        /// Attribute overrides applied to the dialog surface.
        pub surface: DialogSurfaceOptions,
        /// Child markup rendered inside the dialog.
        pub children: String,
        /// Optional accessible label announced by assistive technologies.
        pub aria_label: Option<String>,
    }

    impl Default for DialogProps {
        fn default() -> Self {
            Self {
                state: DialogState::uncontrolled(false),
                surface: DialogSurfaceOptions::default(),
                children: String::new(),
                aria_label: None,
            }
        }
    }

    /// Render the dialog into plain HTML with themed styling and ARIA
    /// attributes for accessibility. If `open` is `false` an empty string is
    /// returned to avoid leaving off-screen content in the markup.
    pub fn render(props: &DialogProps) -> String {
        if !props.state.is_open() {
            return String::new();
        }
        let surface_attrs = apply_surface_options(props.state.surface_attributes(), &props.surface);
        super::render_dialog_surface_html(
            surface_attrs,
            props.aria_label.as_deref(),
            &props.children,
        )
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
    use super::*;

    #[test]
    fn surface_attribute_pairs_capture_state_metadata() {
        let mut state = DialogState::uncontrolled(true);
        state.open(|_| {});
        let options = DialogSurfaceOptions::default();
        let attrs = apply_surface_options(state.surface_attributes(), &options);
        let pairs = surface_attribute_pairs(attrs, None);
        let lookup = |key: &str| {
            pairs
                .iter()
                .find(|(candidate, _)| candidate == key)
                .map(|(_, value)| value.as_str())
        };
        assert_eq!(lookup("role"), Some("dialog"));
        assert_eq!(lookup("aria-modal"), Some("true"));
        assert_eq!(lookup("data-state"), Some("open"));
        assert_eq!(lookup("data-focus-trap"), Some("active"));
    }

    #[test]
    fn render_html_includes_custom_surface_overrides() {
        let mut state = DialogState::uncontrolled(true);
        state.open(|_| {});
        let mut options = DialogSurfaceOptions::default();
        options.id = Some("checkout-dialog".into());
        options.labelled_by = Some("checkout-title".into());
        options.described_by = Some("checkout-copy".into());
        options.analytics_id = Some("dialog-analytics".into());
        let label = Some("Review your order");
        let builder = apply_surface_options(state.surface_attributes(), &options);
        let pairs = surface_attribute_pairs(builder.clone(), label);
        let html = render_dialog_surface_html(builder, label, "<p>Contents</p>");
        for (key, value) in pairs {
            let needle = format!("{key}=\"{value}\"");
            assert!(html.contains(&needle), "missing `{needle}` in `{html}`");
        }
    }
}
