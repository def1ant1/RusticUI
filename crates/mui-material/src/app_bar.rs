use mui_styled_engine::{css_with_theme, use_theme};
use yew::prelude::*;

// Re-export the shared enums under component specific names.  This keeps the
// public API ergonomic while still centralizing the actual definitions in
// `macros.rs`.
pub use crate::macros::{Color as AppBarColor, Size as AppBarSize, Variant as AppBarVariant};

crate::material_component_props!(AppBarProps {
    /// Title displayed inside the app bar.
    title: String,
    /// Accessible label announced by screen readers describing the app bar.
    aria_label: String,
});

/// High level navigation bar rendered at the top of the application.
/// All styling derives from the active [`Theme`] via [`mui-styled-engine`]
/// ensuring a single source of truth.
#[function_component(AppBar)]
pub fn app_bar(props: &AppBarProps) -> Html {
    let theme = use_theme();
    // Resolve dynamic values from the theme.  In a real implementation these
    // would map to palette definitions, typography settings, etc.
    let bg = match props.color {
        AppBarColor::Primary => theme.palette.primary.clone(),
        AppBarColor::Secondary => theme.palette.secondary.clone(),
    };
    let height = match props.size {
        AppBarSize::Small => "48px",
        AppBarSize::Medium => "64px",
        AppBarSize::Large => "80px",
    };
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
