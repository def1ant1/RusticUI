#[cfg(any(feature = "yew", feature = "dioxus", feature = "sycamore"))]
use mui_styled_engine::{css_with_theme, use_theme, Theme};

#[cfg(feature = "yew")]
use yew::prelude::*;

// Re-export the shared enums under component specific names.  This keeps the
// public API ergonomic while still centralizing the actual definitions in
// `macros.rs`.
pub use crate::macros::{Color as AppBarColor, Size as AppBarSize, Variant as AppBarVariant};

#[cfg(any(feature = "yew", feature = "dioxus", feature = "sycamore"))]
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

#[cfg(feature = "yew")]
crate::material_component_props!(AppBarProps {
    /// Title displayed inside the app bar.
    title: String,
    /// Accessible label announced by screen readers describing the app bar.
    aria_label: String,
});

#[cfg(feature = "yew")]
mod yew_impl {
    use super::*;

    /// High level navigation bar rendered at the top of the application.
    #[function_component(AppBar)]
    pub fn app_bar(props: &AppBarProps) -> Html {
        let theme = use_theme();
        let (bg, height) = resolve_style(&theme, props.color, props.size);
        let style = css_with_theme!(
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
        );
        let class = style.get_class_name().to_string();

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
pub use yew_impl::{AppBar, AppBarProps};

#[cfg(feature = "dioxus")]
mod dioxus_impl {
    use super::*;

    #[derive(Default, Clone, PartialEq)]
    pub struct AppBarProps {
        pub title: String,
        pub aria_label: String,
        pub color: AppBarColor,
        pub size: AppBarSize,
    }

    pub fn AppBar(_props: AppBarProps) {
        let theme = use_theme();
        let _ = resolve_style(&theme, _props.color, _props.size);
    }
}

#[cfg(feature = "dioxus")]
pub use dioxus_impl::{AppBar, AppBarProps};

#[cfg(feature = "sycamore")]
mod sycamore_impl {
    use super::*;

    #[derive(Default, Clone, PartialEq)]
    pub struct AppBarProps {
        pub title: String,
        pub aria_label: String,
        pub color: AppBarColor,
        pub size: AppBarSize,
    }

    pub fn AppBar(_props: AppBarProps) {
        let theme = use_theme();
        let _ = resolve_style(&theme, _props.color, _props.size);
    }
}

#[cfg(feature = "sycamore")]
pub use sycamore_impl::{AppBar, AppBarProps};
