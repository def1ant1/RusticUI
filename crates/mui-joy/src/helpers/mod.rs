//! Centralised Joy helper utilities shared across framework adapters.
//!
//! The helpers deliberately separate three concerns that previously lived
//! inside the individual component modules:
//!
//! * **Design token resolution** – everything that transforms the strongly
//!   typed [`Theme`](mui_system::theme::Theme) into inline CSS declarations now
//!   flows through [`resolve_surface_tokens`].  This keeps palette lookups,
//!   border radius calculations and future theming extensions reusable across
//!   the Yew/Dioxus/Leptos/Sycamore adapters without hand-maintaining the
//!   formatting logic in every component.
//! * **ARIA/automation metadata** – [`aria`] converts the tuples emitted by the
//!   headless state machines into framework friendly attribute values.  The
//!   translation lives in one place so enterprise automation suites can rely on
//!   consistent role/aria/data hooks across renderers.
//! * **Adapter glue** – [`yew::use_button_adapter`] and
//!   [`yew::use_chip_adapter`] wrap the [`mui_headless`] state machines inside
//!   ergonomic hooks.  They expose stable callbacks/events so feature teams can
//!   bolt on analytics, logging or custom styling without re-implementing the
//!   headless transitions.
//!
//! The module is intentionally verbose with inline documentation so future
//! contributors understand how to extend the helpers when new Joy primitives
//! are ported.  The goal is to eliminate repetitive boilerplate and make it
//! trivial to add enterprise grade behaviour to additional components.

use crate::{Color, Variant};
use mui_system::theme::{PaletteScheme, Theme};

/// Describes the resolved surface level design tokens for a Joy component.
///
/// Consumers pass the struct into [`SurfaceTokens::compose`] to produce inline
/// styles tailored for the specific component (buttons, chips, cards, etc).
#[derive(Clone, Debug, PartialEq)]
pub struct SurfaceTokens {
    /// Background color applied to the component surface.
    pub background: Option<String>,
    /// Foreground/text color emitted alongside the surface background.
    pub foreground: Option<String>,
    /// Border declaration matching the Joy variant.
    pub border: Option<String>,
    /// Outline declaration driven by the Joy focus tokens.
    pub focus_outline: Option<String>,
    /// Corner radius pulled from [`Theme::joy`].
    pub radius_px: u8,
}

impl SurfaceTokens {
    /// Compose a CSS style string using the resolved tokens and any additional
    /// declarations supplied by the caller (padding, layout, etc).
    pub fn compose<I>(&self, extra: I) -> String
    where
        I: IntoIterator<Item = (&'static str, String)>,
    {
        let mut segments = Vec::with_capacity(6);
        if let Some(background) = &self.background {
            segments.push(("background", background.clone()));
        }
        if let Some(foreground) = &self.foreground {
            segments.push(("color", foreground.clone()));
        }
        if let Some(border) = &self.border {
            segments.push(("border", border.clone()));
        }
        segments.push(("border-radius", format!("{}px", self.radius_px)));
        if let Some(outline) = &self.focus_outline {
            segments.push(("outline", outline.clone()));
            // Align focus outlines with Joy defaults so we get a subtle inset
            // shadow instead of the browser default thick blue ring.
            segments.push(("outline-offset", format!("-{}px", self.radius_px.min(4))));
        }
        segments.extend(extra);
        compose_inline_style(segments)
    }
}

/// Convenience helper for turning key/value pairs into an inline CSS string.
pub fn compose_inline_style<I>(pairs: I) -> String
where
    I: IntoIterator<Item = (&'static str, String)>,
{
    pairs
        .into_iter()
        .map(|(key, value)| format!("{key}:{value};"))
        .collect::<Vec<_>>()
        .join("")
}

/// Resolve the active palette entry for the requested Joy color.
///
/// Keeping the mapping centralised guarantees that every framework adapter,
/// documentation example, and automated test references the same palette
/// wiring.  When upstream Joy introduces an additional color (for example the
/// `Info` accent) we only need to extend this match arm and regenerate the
/// theme templates.
fn palette_color(theme: &Theme, color: Color) -> String {
    let palette: &PaletteScheme = theme.palette.active();
    match color {
        Color::Primary => palette.primary.clone(),
        Color::Neutral => palette.neutral.clone(),
        Color::Danger => palette.danger.clone(),
        Color::Success => palette.success.clone(),
        Color::Warning => palette.warning.clone(),
        Color::Info => palette.info.clone(),
    }
}

/// Convert a base color into an 8-digit hex string with the supplied alpha.
fn with_alpha(color: &str, alpha: &str) -> String {
    format!("{color}{alpha}")
}

/// Resolve Joy surface tokens for a given color + variant pairing.
pub fn resolve_surface_tokens(theme: &Theme, color: Color, variant: Variant) -> SurfaceTokens {
    let palette_color = palette_color(theme, color);
    let radius = theme.joy.radius;
    let focus_outline = Some(format!(
        "{}px solid {}",
        theme.joy.focus_thickness, palette_color
    ));

    match variant {
        Variant::Solid => SurfaceTokens {
            background: Some(palette_color.clone()),
            foreground: Some("#fff".to_string()),
            border: Some("none".to_string()),
            focus_outline,
            radius_px: radius,
        },
        Variant::Soft => SurfaceTokens {
            background: Some(with_alpha(&palette_color, "33")),
            foreground: Some(palette_color.clone()),
            border: Some("none".to_string()),
            focus_outline,
            radius_px: radius,
        },
        Variant::Outlined => SurfaceTokens {
            background: Some("transparent".to_string()),
            foreground: Some(palette_color.clone()),
            border: Some(format!("1px solid {}", palette_color)),
            focus_outline,
            radius_px: radius,
        },
        Variant::Plain => SurfaceTokens {
            background: Some("transparent".to_string()),
            foreground: Some(palette_color.clone()),
            border: Some("none".to_string()),
            focus_outline,
            radius_px: radius,
        },
    }
}

/// Pre-computed inline styles for the Joy `AspectRatio` primitive.
#[derive(Clone, Debug, PartialEq)]
pub struct AspectRatioStyles {
    /// Wrapper element style which enforces the padding-top hack.
    pub outer: String,
    /// Inner element style that pins the content to the wrapper.
    pub inner: String,
}

/// Resolve the inline styles required to maintain the provided aspect ratio.
pub fn resolve_aspect_ratio_styles(ratio: f32) -> AspectRatioStyles {
    assert!(ratio > 0.0, "aspect ratio must be positive");
    let outer = format!(
        "position:relative;width:100%;padding-top:{}%;",
        100.0 / ratio
    );
    let inner = "position:absolute;top:0;left:0;width:100%;height:100%;".to_string();
    AspectRatioStyles { outer, inner }
}

#[cfg(feature = "yew")]
mod aria {
    use mui_headless::aria;
    use mui_headless::chip::{ChipAttributes, ChipState};
    use yew::virtual_dom::AttrValue;

    use super::ChipAdapterConfig;

    /// Standardised ARIA attributes for Joy buttons.
    #[derive(Clone, Debug, PartialEq)]
    pub struct ButtonAria {
        pub role: AttrValue,
        pub aria_pressed: AttrValue,
    }

    impl ButtonAria {
        pub fn from_pairs(pairs: [(&'static str, &'static str); 2]) -> Self {
            let mut role = aria::role_button();
            let mut pressed = "false";
            for (name, value) in pairs {
                match name {
                    "role" => role = value,
                    "aria-pressed" => pressed = value,
                    _ => {}
                }
            }
            Self {
                role: AttrValue::from(role),
                aria_pressed: AttrValue::from(pressed),
            }
        }
    }

    /// Standardised ARIA attributes for Joy chips.
    #[derive(Clone, Debug, PartialEq)]
    pub struct ChipAria {
        pub role: AttrValue,
        pub aria_hidden: AttrValue,
        pub aria_disabled: Option<AttrValue>,
        pub data_disabled: Option<AttrValue>,
        pub id: Option<AttrValue>,
        pub aria_labelledby: Option<AttrValue>,
        pub aria_describedby: Option<AttrValue>,
    }

    impl ChipAria {
        pub fn from_state(state: &ChipState, config: &ChipAdapterConfig) -> Self {
            let mut builder = ChipAttributes::new(state);
            if let Some(id) = config.id.as_deref() {
                builder = builder.id(id);
            }
            if let Some(value) = config.labelled_by.as_deref() {
                builder = builder.labelled_by(value);
            }
            if let Some(value) = config.described_by.as_deref() {
                builder = builder.described_by(value);
            }

            let role = AttrValue::from(builder.role());
            let hidden = builder.hidden();
            let aria_hidden = AttrValue::from(hidden.1);
            let aria_disabled = builder.disabled().map(|(_, value)| AttrValue::from(value));
            let data_disabled = builder
                .data_disabled()
                .map(|(_, value)| AttrValue::from(value));
            let id = builder
                .id_attr()
                .map(|(_, value)| AttrValue::from(value.to_string()));
            let aria_labelledby = builder
                .labelledby()
                .map(|(_, value)| AttrValue::from(value.to_string()));
            let aria_describedby = builder
                .describedby()
                .map(|(_, value)| AttrValue::from(value.to_string()));

            Self {
                role,
                aria_hidden,
                aria_disabled,
                data_disabled,
                id,
                aria_labelledby,
                aria_describedby,
            }
        }
    }

    pub use ButtonAria as Button;
    pub use ChipAria as Chip;
}

#[cfg(feature = "yew")]
pub use aria::{Button as ButtonAria, Chip as ChipAria};

#[cfg(feature = "yew")]
mod yew_adapters {
    use std::time::Duration;

    use gloo_timers::callback::Interval;
    use mui_headless::button::ButtonState;
    use mui_headless::chip::{ChipChange, ChipConfig as HeadlessChipConfig, ChipState};
    use yew::prelude::*;

    use super::{aria, ChipAria};

    /// Configuration passed into [`use_button_adapter`].
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct ButtonAdapterConfig {
        pub disabled: bool,
        pub throttle: Option<Duration>,
    }

    /// Aggregated adapter output returned from [`use_button_adapter`].
    pub struct ButtonAdapter {
        pub onclick: Callback<MouseEvent>,
        pub aria: aria::ButtonAria,
        pub disabled: bool,
    }

    fn bump_counter(handle: &UseStateHandle<u64>) {
        let next = **handle + 1;
        handle.set(next);
    }

    /// Hook exposing the [`mui_headless::button::ButtonState`] state machine to Yew.
    #[hook]
    pub fn use_button_adapter(
        config: ButtonAdapterConfig,
        on_press: Callback<MouseEvent>,
    ) -> ButtonAdapter {
        let state = use_mut_ref(|| ButtonState::new(config.disabled, config.throttle));
        let rerender = use_state(|| 0u64);

        {
            let state = state.clone();
            let rerender = rerender.clone();
            use_effect_with(config.clone(), move |config: &ButtonAdapterConfig| {
                *state.borrow_mut() = ButtonState::new(config.disabled, config.throttle);
                bump_counter(&rerender);
                || ()
            });
        }

        let onclick = {
            let state = state.clone();
            let rerender = rerender.clone();
            let on_press = on_press.clone();
            Callback::from(move |event: MouseEvent| {
                let event_clone = event.clone();
                let mut triggered = false;
                state.borrow_mut().press(|_| {
                    triggered = true;
                    on_press.emit(event_clone);
                });
                if triggered {
                    bump_counter(&rerender);
                }
            })
        };

        let aria = {
            let state_ref = state.borrow();
            aria::ButtonAria::from_pairs(state_ref.aria_attributes())
        };

        ButtonAdapter {
            onclick,
            aria,
            disabled: config.disabled,
        }
    }

    /// Configuration for the Joy chip adapter.
    #[derive(Clone, Debug, PartialEq)]
    pub struct ChipAdapterConfig {
        pub dismissible: bool,
        pub disabled: bool,
        pub show_delay: Duration,
        pub hide_delay: Duration,
        pub delete_delay: Duration,
        pub id: Option<String>,
        pub labelled_by: Option<String>,
        pub described_by: Option<String>,
    }

    impl ChipAdapterConfig {
        pub fn headless_config(&self) -> HeadlessChipConfig {
            HeadlessChipConfig {
                show_delay: self.show_delay,
                hide_delay: self.hide_delay,
                delete_delay: self.delete_delay,
                dismissible: self.dismissible,
                disabled: self.disabled,
            }
        }
    }

    impl Default for ChipAdapterConfig {
        fn default() -> Self {
            Self {
                dismissible: false,
                disabled: false,
                show_delay: Duration::from_millis(0),
                hide_delay: Duration::from_millis(0),
                delete_delay: Duration::from_millis(0),
                id: None,
                labelled_by: None,
                described_by: None,
            }
        }
    }

    /// Adapter output consumed by the Joy chip component.
    pub struct ChipAdapter {
        pub visible: bool,
        pub controls_visible: bool,
        pub deleting: bool,
        pub aria: ChipAria,
        pub on_pointer_enter: Callback<MouseEvent>,
        pub on_pointer_leave: Callback<MouseEvent>,
        pub on_focus: Callback<FocusEvent>,
        pub on_blur: Callback<FocusEvent>,
        pub on_keydown: Callback<KeyboardEvent>,
        pub on_delete_click: Option<Callback<MouseEvent>>,
        pub disabled: bool,
    }

    fn process_chip_change(
        change: ChipChange,
        rerender: &UseStateHandle<u64>,
        delete_cb: Option<&Callback<MouseEvent>>,
        event: Option<MouseEvent>,
    ) {
        let mut needs_render = false;
        if change.controls_visible.is_some() || change.deletion_cancelled {
            needs_render = true;
        }
        if change.deleted {
            needs_render = true;
            if let (Some(callback), Some(event)) = (delete_cb, event) {
                callback.emit(event);
            }
        }
        if needs_render {
            bump_counter(rerender);
        }
    }

    /// Hook exposing the [`mui_headless::chip::ChipState`] state machine to Yew.
    #[hook]
    pub fn use_chip_adapter(
        config: ChipAdapterConfig,
        on_delete: Option<Callback<MouseEvent>>,
    ) -> ChipAdapter {
        let state = use_mut_ref(|| ChipState::new(config.headless_config()));
        let rerender = use_state(|| 0u64);

        {
            let state = state.clone();
            let rerender = rerender.clone();
            use_effect_with(config.clone(), move |config: &ChipAdapterConfig| {
                *state.borrow_mut() = ChipState::new(config.headless_config());
                bump_counter(&rerender);
                || ()
            });
        }

        {
            let state = state.clone();
            let rerender = rerender.clone();
            let delete_cb = on_delete.clone();
            use_effect(move || {
                let interval = Interval::new(32, move || {
                    let change = state.borrow_mut().poll();
                    process_chip_change(change, &rerender, delete_cb.as_ref(), None);
                });
                move || drop(interval)
            });
        }

        let on_pointer_enter = {
            let state = state.clone();
            let rerender = rerender.clone();
            let delete_cb = on_delete.clone();
            Callback::from(move |event: MouseEvent| {
                event.stop_propagation();
                let change = state.borrow_mut().pointer_enter();
                process_chip_change(change, &rerender, delete_cb.as_ref(), None);
            })
        };

        let on_pointer_leave = {
            let state = state.clone();
            let rerender = rerender.clone();
            let delete_cb = on_delete.clone();
            Callback::from(move |event: MouseEvent| {
                event.stop_propagation();
                let change = state.borrow_mut().pointer_leave();
                process_chip_change(change, &rerender, delete_cb.as_ref(), None);
            })
        };

        let on_focus = {
            let state = state.clone();
            let rerender = rerender.clone();
            let delete_cb = on_delete.clone();
            Callback::from(move |_event: FocusEvent| {
                let change = state.borrow_mut().focus();
                process_chip_change(change, &rerender, delete_cb.as_ref(), None);
            })
        };

        let on_blur = {
            let state = state.clone();
            let rerender = rerender.clone();
            let delete_cb = on_delete.clone();
            Callback::from(move |_event: FocusEvent| {
                let change = state.borrow_mut().blur();
                process_chip_change(change, &rerender, delete_cb.as_ref(), None);
            })
        };

        let on_keydown = {
            let state = state.clone();
            let rerender = rerender.clone();
            let delete_cb = on_delete.clone();
            Callback::from(move |event: KeyboardEvent| {
                let key = event.key();
                let change = match key.as_str() {
                    "Delete" | "Backspace" => {
                        event.prevent_default();
                        state.borrow_mut().request_delete()
                    }
                    "Escape" => {
                        event.prevent_default();
                        state.borrow_mut().escape()
                    }
                    _ => return,
                };
                process_chip_change(change, &rerender, delete_cb.as_ref(), None);
            })
        };

        let on_delete_click = on_delete.clone().map(|callback| {
            let state = state.clone();
            let rerender = rerender.clone();
            Callback::from(move |event: MouseEvent| {
                event.prevent_default();
                event.stop_propagation();
                let event_clone = event.clone();
                let change = state.borrow_mut().request_delete();
                process_chip_change(change, &rerender, Some(&callback), Some(event_clone));
            })
        });

        let aria = {
            let state_ref = state.borrow();
            super::aria::Chip::from_state(&state_ref, &config)
        };

        let state_ref = state.borrow();
        ChipAdapter {
            visible: state_ref.is_visible(),
            controls_visible: state_ref.controls_visible(),
            deleting: state_ref.deletion_pending(),
            aria,
            on_pointer_enter,
            on_pointer_leave,
            on_focus,
            on_blur,
            on_keydown,
            on_delete_click,
            disabled: config.disabled,
        }
    }

    pub use ButtonAdapter as Button;
    pub use ButtonAdapterConfig as ButtonConfig;
    pub use ChipAdapter as Chip;
    pub use ChipAdapterConfig as ChipConfig;
}

#[cfg(feature = "yew")]
pub use yew_adapters::{
    use_button_adapter, use_chip_adapter, Button as ButtonAdapter,
    ButtonConfig as ButtonAdapterConfig, Chip as ChipAdapter, ChipConfig as ChipAdapterConfig,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Color, Variant};
    use mui_system::theme::Theme;

    /// Ensure every color variant maps to a palette entry. All framework adapters rely on this
    /// helper so keeping the assertion here prevents drift between Yew, Leptos, Dioxus and
    /// Sycamore integrations.
    #[test]
    fn palette_color_covers_all_variants() {
        let theme = Theme::default();
        let palette = theme.palette.active();

        for color in Color::ALL {
            let resolved = palette_color(&theme, color);
            let expected = match color {
                Color::Primary => &palette.primary,
                Color::Neutral => &palette.neutral,
                Color::Danger => &palette.danger,
                Color::Success => &palette.success,
                Color::Warning => &palette.warning,
                Color::Info => &palette.info,
            };
            assert_eq!(resolved, *expected, "palette mismatch for {:?}", color);
        }
    }

    #[test]
    fn surface_tokens_resolve() {
        let theme = Theme::default();
        let tokens = resolve_surface_tokens(&theme, Color::Primary, Variant::Soft);
        assert_eq!(tokens.radius_px, theme.joy.radius);
        assert!(tokens.background.unwrap().ends_with("33"));
    }

    /// Regression test guaranteeing that the soft variant keeps its translucent background aligned
    /// with the base palette color for every Joy accent.
    #[test]
    fn soft_variant_applies_alpha_suffix_for_all_colors() {
        let theme = Theme::default();
        for color in Color::ALL {
            let tokens = resolve_surface_tokens(&theme, color, Variant::Soft);
            let base = palette_color(&theme, color);
            let expected = format!("{}33", base);
            assert_eq!(
                tokens.background.as_deref(),
                Some(expected.as_str()),
                "soft background should inherit {base} with alpha for {:?}",
                color
            );
        }
    }

    #[test]
    fn aspect_ratio_styles_are_generated() {
        let styles = resolve_aspect_ratio_styles(16.0 / 9.0);
        assert!(styles.outer.contains("padding-top"));
        assert!(styles.inner.contains("position:absolute"));
    }
}
