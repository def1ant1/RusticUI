//! Material themed application bar that demonstrates centralized styling and
//! accessibility metadata.
//!
//! All adapters derive their visual appearance from
//! [`css_with_theme!`](mui_styled_engine::css_with_theme) so palette and sizing
//! decisions track the active [`Theme`](mui_styled_engine::Theme). The shared
//! [`style_helpers::themed_class`](crate::style_helpers::themed_class) helper
//! converts those styles into scoped class names while
//! [`style_helpers::themed_attributes_html`](crate::style_helpers::themed_attributes_html)
//! assembles ARIA rich attribute sets for SSR oriented adapters. Each framework
//! specific module layers the correct role and `aria-label` metadata onto a
//! semantic `<header>` so screen readers announce the bar as the application's
//! primary banner region.

#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]
use mui_styled_engine::{css_with_theme, use_theme, Style, Theme};

#[cfg(feature = "yew")]
use yew::prelude::*;

// Re-export the shared enums under component specific names.  This keeps the
// public API ergonomic while still centralizing the actual definitions in
// `macros.rs`.
pub use crate::macros::{Color as AppBarColor, Size as AppBarSize, Variant as AppBarVariant};

#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]
fn resolve_style(theme: &Theme, color: AppBarColor, size: AppBarSize) -> (String, &'static str) {
    let bg = match color {
        AppBarColor::Primary => theme.palette.primary.clone(),
        AppBarColor::Secondary => theme.palette.secondary.clone(),
    };
    let height = match size {
        AppBarSize::Small => "48px",
        AppBarSize::Medium => "64px",
        AppBarSize::Large => "80px",
    };
    (bg, height)
}

#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]
fn app_bar_style(theme: &Theme, color: AppBarColor, size: AppBarSize) -> Style {
    let (bg, height) = resolve_style(theme, color, size);
    css_with_theme!(
        theme,
        r#"
        background: ${bg};
        height: ${height};
        display: flex;
        align-items: center;
        padding: 0 16px;
    "#,
        bg = bg,
        height = height
    )
}

#[cfg(any(feature = "yew", feature = "leptos"))]
crate::material_component_props!(AppBarProps {
    /// Title displayed inside the app bar.
    title: String,
    /// Accessible label announced by screen readers describing the app bar.
    aria_label: String,
});

#[cfg(feature = "yew")]
mod yew_impl {
    //! Yew adapter rendering the [`AppBar`] as a semantic `<header>` element.
    //!
    //! Styling is resolved through [`css_with_theme!`] which pulls palette
    //! values from the active [`Theme`]. The resulting class is applied to the
    //! `<header>` element along with an explicit `role="banner"` and
    //! configurable `aria-label` to aid screen readers.
    use super::*;

    /// High level navigation bar rendered at the top of the application.
    #[function_component(AppBar)]
    pub fn app_bar(props: &AppBarProps) -> Html {
        let theme = use_theme();
        let class =
            crate::style_helpers::themed_class(app_bar_style(&theme, props.color, props.size));

        html! {
            <header
                class={class}
                role="banner"
                aria-label={props.aria_label.clone()}
            >
                { &props.title }
            </header>
        }
    }
}

#[cfg(feature = "yew")]
pub use yew_impl::AppBar;

#[cfg(feature = "leptos")]
mod leptos_impl {
    //! Leptos adapter rendering the [`AppBar`] as a semantic `<header>` element.
    use super::*;
    use leptos::*;

    /// High level navigation bar rendered at the top of the application.
    ///
    /// The component resolves colors and sizing from the active [`Theme`] and
    /// attaches an ARIA `role` and `aria-label` so assistive technologies can
    /// announce the region accurately.
    #[component]
    pub fn AppBar(props: AppBarProps) -> impl IntoView {
        let theme = use_theme();
        let class =
            crate::style_helpers::themed_class(app_bar_style(&theme, props.color, props.size));

        view! {
            <header class=class role="banner" aria-label=props.aria_label>
                {props.title}
            </header>
        }
    }
}

#[cfg(feature = "leptos")]
pub use leptos_impl::AppBar;

#[cfg(any(feature = "yew", feature = "leptos"))]
pub use AppBarProps;

/// Adapter targeting the [`dioxus`] framework.
///
/// Generates a themed `<header>` element and wires up ARIA attributes so the
/// navigation region is announced correctly by assistive technologies.
#[cfg(feature = "dioxus")]
pub mod dioxus {
    use super::*;

    /// Properties consumed by the Dioxus adapter. The struct intentionally
    /// mirrors the fields used by other frameworks so business logic remains
    /// consistent across integrations.
    #[derive(Default, Clone, PartialEq)]
    pub struct AppBarProps {
        /// Title displayed inside the app bar.
        pub title: String,
        /// Accessible label announced by assistive technologies.
        pub aria_label: String,
        /// Themed color palette applied to the background.
        pub color: AppBarColor,
        /// Height variant influencing overall bar size.
        pub size: AppBarSize,
    }

    /// Render the app bar into a `<header>` tag using a theme derived class.
    ///
    /// [`css_with_theme!`] resolves palette values from the active [`Theme`]
    /// which keeps styles centralized and easily overridable. The generated
    /// class is merged with ARIA metadata so screen readers can announce the
    /// banner role and label.
    pub fn render(props: &AppBarProps) -> String {
        let theme = use_theme();
        // Compose the attributes with the shared helper so HTML-first adapters
        // stay aligned with the declarative front-end integrations.
        let attr_string = crate::style_helpers::themed_attributes_html(
            app_bar_style(&theme, props.color.clone(), props.size.clone()),
            [("role", "banner"), ("aria-label", props.aria_label.clone())],
        );
        format!("<header {}>{}</header>", attr_string, props.title)
    }
}

/// Adapter targeting the [`sycamore`] framework.
///
/// Produces an accessible `<header>` with classes derived from the active
/// [`Theme`] and optional `aria-label` metadata.
#[cfg(feature = "sycamore")]
pub mod sycamore {
    use super::*;

    /// Sycamore variant of [`AppBar`] sharing identical props with other
    /// adapters to minimize repetitive setup across frameworks.
    #[derive(Default, Clone, PartialEq)]
    pub struct AppBarProps {
        /// Title displayed inside the app bar.
        pub title: String,
        /// Accessible label describing the banner region.
        pub aria_label: String,
        /// Background color pulled from the theme palette.
        pub color: AppBarColor,
        /// Height variant controlling the overall size.
        pub size: AppBarSize,
    }

    /// Render the app bar into plain HTML with themed styling and ARIA
    /// attributes for accessibility.
    pub fn render(props: &AppBarProps) -> String {
        let theme = use_theme();
        // Compose the attributes with the shared helper so HTML-first adapters
        // stay aligned with the declarative front-end integrations.
        let attr_string = crate::style_helpers::themed_attributes_html(
            app_bar_style(&theme, props.color.clone(), props.size.clone()),
            [("role", "banner"), ("aria-label", props.aria_label.clone())],
        );
        format!("<header {}>{}</header>", attr_string, props.title)
    }
}
