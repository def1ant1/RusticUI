//! Minimal dialog container demonstrating theme-aware styling and accessibility.
//!
//! ## Style composition
//! * [`css_with_theme!`](mui_styled_engine::css_with_theme) powers every
//!   adapter. The macro exposes a `theme` binding so border colours pull from
//!   `theme.palette.secondary` while padding respects `theme.spacing(3)`.
//!   Wrapping the declaration inside [`style_helpers::themed_class`](crate::style_helpers::themed_class)
//!   produces a deterministic class name that can be safely reused across
//!   renders without leaking duplicate strings.
//! * Each framework module calls back into [`resolve_style`] so client side
//!   components (Yew/Leptos) and server-side renderers (Leptos/Dioxus/Sycamore)
//!   receive the identical scoped class. This keeps brand styling consistent
//!   even when applications mix rendering strategies for pre-production smoke
//!   tests or hybrid deployments.
//!
//! ## Accessibility toggling
//! * Rendering is gated on the `open` flag. When `open` is `false` adapters
//!   emit no markup which keeps hidden content out of the accessibility tree
//!   and mirrors the behaviour of Material UI's JavaScript implementation.
//! * When `open` flips to `true` every adapter renders a `<div>` decorated
//!   with `role="dialog"`, `aria-modal="true"` and the caller supplied
//!   `aria_label`. Screen readers can then accurately announce the region and
//!   understand that focus should remain trapped inside the modal until it is
//!   dismissed.
//!
//! Each framework module is intentionally tiny and delegates styling to
//! [`resolve_style`] which centralizes theme lookups. Frameworks that render raw
//! HTML strings reuse
//! [`style_helpers::themed_attributes_html`](crate::style_helpers::themed_attributes_html)
//! to attach the ARIA metadata without duplicating string concatenation logic.
//! This shared machinery significantly reduces repetitive setup when scaling to
//! multiple enterprise applications.

#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore",
))]
use mui_styled_engine::{css_with_theme, Style};

#[cfg(feature = "leptos")]
use leptos::Children;
#[cfg(feature = "yew")]
use yew::prelude::*;

#[cfg(any(feature = "yew", feature = "leptos"))]
use crate::material_props;

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

/// Shared helper wiring framework agnostic ARIA metadata into the dialog.
///
/// String-based adapters (Leptos SSR/Dioxus/Sycamore) delegate to this function
/// so the `role="dialog"` and `aria-modal="true"` attributes are emitted in a
/// consistent order. Returning a `String` keeps the helpers friendly for
/// snapshot tests and other automation harnesses that reason about serialized
/// HTML.
#[cfg(any(feature = "leptos", feature = "dioxus", feature = "sycamore"))]
fn render_open_dialog_html(aria_label: &str, child: &str) -> String {
    // Centralize ARIA metadata via the shared helper so every SSR adapter emits
    // identical markup and future attributes can be added in a single place.
    let attr_string = crate::style_helpers::themed_attributes_html(
        resolve_style(),
        [
            ("role", "dialog"),
            ("aria-modal", "true"),
            ("aria-label", aria_label),
        ],
    );
    format!(
        "<div {attrs}>{child}</div>",
        attrs = attr_string,
        child = child
    )
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
    ///
    /// When `open` is `false` an empty node is returned to keep the dialog out
    /// of the DOM and accessibility tree. When `open` is `true` a `<div>` with
    /// `role="dialog"` and `aria-modal="true"` is emitted so assistive
    /// technologies understand focus is trapped within the region.
    #[function_component(Dialog)]
    pub fn dialog(props: &DialogProps) -> Html {
        if !props.open {
            return Html::default();
        }
        // Generate a theme-aware class once and attach it to the `<div>`.
        let class = crate::style_helpers::themed_class(resolve_style());
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
    ///
    /// Closed dialogs return an empty view while open dialogs emit the
    /// styled `<div>` with ARIA metadata.
    #[component]
    pub fn Dialog(props: DialogProps) -> impl IntoView {
        if !props.open {
            return view! {};
        }
        let class = crate::style_helpers::themed_class(resolve_style());
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
// Leptos SSR adapter
// ---------------------------------------------------------------------------

#[cfg(feature = "leptos")]
pub mod leptos {
    use super::*;

    /// Properties consumed by the Leptos SSR adapter. We intentionally mirror
    /// the structure of other SSR focused modules so applications can swap
    /// adapters without re-mapping state or accessibility metadata.
    #[derive(Default, Clone, PartialEq)]
    pub struct DialogProps {
        /// Whether the dialog should be rendered.
        pub open: bool,
        /// Raw HTML/text representing the dialog contents.
        pub children: String,
        /// Accessible label announced by assistive technologies.
        pub aria_label: String,
    }

    /// Render the dialog into a HTML string using `css_with_theme!` for
    /// styling. Closed dialogs return an empty string so hidden regions never
    /// reach the accessibility tree.
    pub fn render(props: &DialogProps) -> String {
        if !props.open {
            return String::new();
        }
        super::render_open_dialog_html(&props.aria_label, &props.children)
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
    /// standard ARIA attributes. Closed dialogs yield an empty string so hidden
    /// content is never announced by screen readers.
    pub fn render(props: &DialogProps) -> String {
        if !props.open {
            return String::new();
        }
        super::render_open_dialog_html(&props.aria_label, &props.children)
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
    /// attributes for accessibility. If `open` is `false` an empty string is
    /// returned to avoid leaving off-screen content in the markup.
    pub fn render(props: &DialogProps) -> String {
        if !props.open {
            return String::new();
        }
        super::render_open_dialog_html(&props.aria_label, &props.children)
    }
}
