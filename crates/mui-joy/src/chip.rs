use std::time::Duration;

use mui_system::theme_provider::use_theme;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

use crate::helpers::{
    compose_inline_style, resolve_surface_tokens, use_chip_adapter, ChipAdapterConfig, SurfaceTokens,
};
use crate::{joy_component_props, Color, Variant};

joy_component_props!(ChipProps {
    /// Text displayed within the chip.
    label: String,
    /// Optional handler invoked when the delete icon is clicked.
    on_delete: Option<Callback<MouseEvent>>,
    /// Explicit toggle for the trailing delete affordance. Defaults to `on_delete.is_some()`.
    dismissible: Option<bool>,
    /// Whether the chip is interactive.
    disabled: bool,
    /// Optional DOM id to link analytics tooling or automation scripts.
    id: Option<String>,
    /// Optional `aria-labelledby` reference id.
    aria_labelledby: Option<String>,
    /// Optional `aria-describedby` reference id.
    aria_describedby: Option<String>,
    /// Delay before the trailing controls become visible (milliseconds).
    show_delay_ms: Option<u64>,
    /// Delay before hiding the trailing controls (milliseconds).
    hide_delay_ms: Option<u64>,
    /// Grace period before a deletion is committed (milliseconds).
    delete_delay_ms: Option<u64>,
});

/// Joy UI chip that wires into [`mui_headless::chip::ChipState`].
///
/// # Design tokens
/// * [`helpers::resolve_surface_tokens`](crate::helpers::resolve_surface_tokens) aligns color and
///   border treatments with Joy's variant system and propagates the shared focus outline tokens.
/// * The helper derived styles are augmented with layout hints (`display`, `padding`, etc.) so the
///   component stays consistent across adapters without hand-maintained CSS.
///
/// # Headless state contract
/// The component renders the state exposed by [`mui_headless::chip::ChipState`] via
/// [`helpers::use_chip_adapter`](crate::helpers::use_chip_adapter). Hover/focus/delete semantics
/// remain identical across the Yew implementation and future Leptos/Dioxus/Sycamore bindings.
#[function_component(Chip)]
pub fn chip(props: &ChipProps) -> Html {
    let theme = use_theme();

    let surface: SurfaceTokens = resolve_surface_tokens(&theme, props.color.clone(), props.variant.clone());
    let dismissible = props.dismissible.unwrap_or_else(|| props.on_delete.is_some());
    let config = ChipAdapterConfig {
        dismissible: dismissible && props.on_delete.is_some(),
        disabled: props.disabled,
        show_delay: props
            .show_delay_ms
            .map(Duration::from_millis)
            .unwrap_or_else(|| Duration::from_millis(0)),
        hide_delay: props
            .hide_delay_ms
            .map(Duration::from_millis)
            .unwrap_or_else(|| Duration::from_millis(0)),
        delete_delay: props
            .delete_delay_ms
            .map(Duration::from_millis)
            .unwrap_or_else(|| Duration::from_millis(0)),
        id: props.id.clone(),
        labelled_by: props.aria_labelledby.clone(),
        described_by: props.aria_describedby.clone(),
    };
    let adapter = use_chip_adapter(config, props.on_delete.clone());

    if !adapter.visible {
        return Html::default();
    }

    let mut extra = vec![
        ("display", "inline-flex".to_string()),
        ("align-items", "center".to_string()),
        ("gap", "4px".to_string()),
        ("padding", "4px 8px".to_string()),
        ("user-select", "none".to_string()),
    ];
    extra.push((
        "cursor",
        if adapter.disabled {
            "not-allowed".to_string()
        } else {
            "pointer".to_string()
        },
    ));
    if adapter.disabled {
        extra.push(("opacity", "0.6".to_string()));
    }
    let style = surface.compose(extra);

    let tabindex: AttrValue = if adapter.disabled {
        AttrValue::from("-1")
    } else {
        AttrValue::from("0")
    };

    let delete_button = if (adapter.controls_visible || adapter.deleting)
        && adapter.on_delete_click.is_some()
    {
        let mut delete_style = vec![
            ("background", "transparent".to_string()),
            ("border", "none".to_string()),
            ("padding", "0".to_string()),
            ("margin-left", "4px".to_string()),
        ];
        delete_style.push((
            "cursor",
            if adapter.disabled {
                "not-allowed".to_string()
            } else {
                "pointer".to_string()
            },
        ));
        let delete_style = compose_inline_style(delete_style);
        let onclick = adapter.on_delete_click.as_ref().unwrap().clone();
        html! {
            <button
                type="button"
                style={delete_style}
                aria-label="Remove chip"
                disabled={adapter.disabled}
                onclick={onclick}
            >
                {"×"}
            </button>
        }
    } else {
        Html::default()
    };

    html! {
        <span
            style={style}
            role={adapter.aria.role.clone()}
            aria-hidden={adapter.aria.aria_hidden.clone()}
            aria-disabled={adapter.aria.aria_disabled.clone()}
            data-disabled={adapter.aria.data_disabled.clone()}
            id={adapter.aria.id.clone()}
            aria-labelledby={adapter.aria.aria_labelledby.clone()}
            aria-describedby={adapter.aria.aria_describedby.clone()}
            tabindex={tabindex}
            onmouseenter={adapter.on_pointer_enter}
            onmouseleave={adapter.on_pointer_leave}
            onfocus={adapter.on_focus}
            onblur={adapter.on_blur}
            onkeydown={adapter.on_keydown}
        >
            { &props.label }
            { delete_button }
        </span>
    }
}
