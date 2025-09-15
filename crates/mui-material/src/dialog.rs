//! Minimal dialog container demonstrating theme-aware styling and accessibility.
//!
//! The component derives its visual appearance from the active
//! [`Theme`](mui_styled_engine::Theme) via the [`css_with_theme!`]
//! macro. Both the border color and interior padding are pulled from the
//! theme's palette and spacing scale so applications stay visually
//! consistent. The resulting scoped style is attached as a class to the
//! root `<div>` element. Each adapter also wires up ARIA attributes—
//! `role="dialog"` and `aria-modal="true"`—and accepts an `aria_label`
//! to ensure assistive technologies can announce the region accurately.

use mui_styled_engine::css_with_theme;

#[cfg(feature = "leptos")]
use leptos::Children;
#[cfg(feature = "yew")]
use yew::prelude::*;

use crate::material_props;

/// Generates a CSS class scoped to this dialog using the active [`Theme`].
///
/// The class is derived once per render and applied to the `<div>` element in
/// every framework adapter which keeps styling logic centralized and easy to
/// maintain.
#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]
fn resolve_class() -> String {
    let style = css_with_theme!(
        r#"
        border: 2px solid ${border};
        padding: ${pad};
        "#,
        // Pull colors and spacing from the theme so consumers only tweak
        // global tokens instead of individual components.
        border = theme.palette.secondary.clone(),
        pad = format!("{}px", theme.spacing(3))
    );
    style.get_class_name().to_string()
}

// ---------------------------------------------------------------------------
// Shared Yew/Leptos props
// ---------------------------------------------------------------------------

#[cfg(any(feature = "yew", feature = "leptos"))]
material_props!(DialogProps {
    /// Whether the dialog is shown.
    open: bool,
    /// Dialog contents rendered inside the container.
    children: Children,
    /// Accessible label announced by assistive technologies.
    aria_label: String,
});

// ---------------------------------------------------------------------------
// Yew adapter
// ---------------------------------------------------------------------------

#[cfg(feature = "yew")]
mod yew_impl {
    use super::*;

    /// Minimal dialog implementation that toggles visibility and wires up
    /// accessibility attributes.
    #[function_component(Dialog)]
    pub fn dialog(props: &DialogProps) -> Html {
        if !props.open {
            return Html::default();
        }
        // Generate a theme-aware class once and attach it to the `<div>`.
        let class = resolve_class();
        html! {
            <div class={class} role="dialog" aria-modal="true" aria-label={props.aria_label.clone()}>
                { for props.children.iter() }
            </div>
        }
    }
}

#[cfg(feature = "yew")]
pub use yew_impl::Dialog;

// ---------------------------------------------------------------------------
// Leptos adapter
// ---------------------------------------------------------------------------

#[cfg(feature = "leptos")]
mod leptos_impl {
    use super::*;
    use leptos::*;

    /// Leptos variant mirroring the Yew implementation.
    #[component]
    pub fn Dialog(props: DialogProps) -> impl IntoView {
        if !props.open {
            return view! {};
        }
        let class = resolve_class();
        view! {
            <div class=class role="dialog" aria-modal="true" aria-label=props.aria_label>
                {props.children()}
            </div>
        }
    }
}

#[cfg(feature = "leptos")]
pub use leptos_impl::Dialog;

#[cfg(any(feature = "yew", feature = "leptos"))]
pub use DialogProps;

// ---------------------------------------------------------------------------
// Dioxus adapter
// ---------------------------------------------------------------------------

#[cfg(feature = "dioxus")]
pub mod dioxus {
    use super::*;

    /// Properties consumed by the Dioxus adapter. The struct intentionally
    /// mirrors the fields used by other frameworks so business logic remains
    /// consistent across integrations.
    #[derive(Default, Clone, PartialEq)]
    pub struct DialogProps {
        /// Whether the dialog is shown.
        pub open: bool,
        /// Child markup rendered inside the dialog.
        pub children: String,
        /// Accessible label announced by assistive technologies.
        pub aria_label: String,
    }

    /// Render the dialog into a `<div>` tag using a theme-derived class and
    /// standard ARIA attributes.
    pub fn render(props: &DialogProps) -> String {
        if !props.open {
            return String::new();
        }
        let class = super::resolve_class();
        format!(
            "<div class=\"{}\" role=\"dialog\" aria-modal=\"true\" aria-label=\"{}\">{}</div>",
            class, props.aria_label, props.children
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
    #[derive(Default, Clone, PartialEq)]
    pub struct DialogProps {
        /// Whether the dialog is shown.
        pub open: bool,
        /// Child markup rendered inside the dialog.
        pub children: String,
        /// Accessible label announced by assistive technologies.
        pub aria_label: String,
    }

    /// Render the dialog into plain HTML with themed styling and ARIA
    /// attributes for accessibility.
    pub fn render(props: &DialogProps) -> String {
        if !props.open {
            return String::new();
        }
        let class = super::resolve_class();
        format!(
            "<div class=\"{}\" role=\"dialog\" aria-modal=\"true\" aria-label=\"{}\">{}</div>",
            class, props.aria_label, props.children
        )
    }
}
