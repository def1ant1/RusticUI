use std::time::Duration;

use mui_system::theme_provider::use_theme;
use yew::prelude::*;

use crate::helpers::{
    resolve_surface_tokens, use_button_adapter, ButtonAdapterConfig, SurfaceTokens,
};
// Import shared enums and macros so all components stay aligned.
use crate::{joy_component_props, Color, Variant};

joy_component_props!(ButtonProps {
    /// Text displayed inside the button.
    label: String,
    /// Click handler for interactive behavior.
    onclick: Callback<MouseEvent>,
    /// Optional throttle window (in milliseconds) guarding against accidental double clicks.
    throttle_ms: Option<u64>,
    /// Whether the button should be disabled.
    disabled: bool,
});

/// Joy UI button rendering the [`mui_headless::button::ButtonState`] machine.
///
/// # Design tokens
/// * [`helpers::resolve_surface_tokens`](crate::helpers::resolve_surface_tokens) pulls the
///   palette entry for [`Color`] and merges it with the Joy border radius + focus tokens.
/// * [`Theme::spacing`](mui_system::theme::Theme::spacing) is leveraged for consistent padding.
///
/// # Headless state contract
/// The component delegates click orchestration to
/// [`mui_headless::button::ButtonState`], exposed to Yew via
/// [`helpers::use_button_adapter`](crate::helpers::use_button_adapter). The adapter mirrors the
/// same state transitions across future Leptos/Dioxus/Sycamore bindings ensuring SSR + hydration
/// consistency.
#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let theme = use_theme();

    // Resolve design tokens once so we can build the final inline styles declaratively.
    let surface: SurfaceTokens =
        resolve_surface_tokens(&theme, props.color.clone(), props.variant.clone());
    let padding = format!("{}px {}px", theme.spacing(1), theme.spacing(2));

    let adapter = use_button_adapter(
        ButtonAdapterConfig {
            disabled: props.disabled,
            throttle: props.throttle_ms.map(Duration::from_millis),
        },
        props.onclick.clone(),
    );

    let mut extra = vec![
        ("padding", padding),
        ("border", "none".to_string()),
        (
            "cursor",
            if adapter.disabled {
                "not-allowed".to_string()
            } else {
                "pointer".to_string()
            },
        ),
        (
            "transition",
            "background 120ms ease, color 120ms ease".to_string(),
        ),
    ];
    let style = surface.compose(extra.drain(..));

    html! {
        <button
            style={style}
            role={adapter.aria.role.clone()}
            aria-pressed={adapter.aria.aria_pressed.clone()}
            disabled={adapter.disabled}
            onclick={adapter.onclick}
        >
            { &props.label }
        </button>
    }
}
