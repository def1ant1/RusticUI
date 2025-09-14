use mui_system::theme_provider::use_theme;
use yew::prelude::*;

// Import shared enums and macros so all components stay aligned.
use crate::{joy_component_props, Color, Variant};

joy_component_props!(ButtonProps {
    /// Text displayed inside the button.
    label: String,
    /// Click handler for interactive behavior.
    onclick: Callback<MouseEvent>,
});

/// A basic Joy UI button.
#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let theme = use_theme();
    let color = match props.color {
        Color::Primary => theme.palette.primary.clone(),
        Color::Neutral => theme.palette.neutral.clone(),
        Color::Danger => theme.palette.danger.clone(),
    };
    let style = match props.variant {
        Variant::Solid => format!(
            "background:{};color:#fff;border-radius:{}px;",
            color, theme.joy.radius
        ),
        Variant::Soft => format!(
            "background:{}33;color:{};border-radius:{}px;",
            color, color, theme.joy.radius
        ),
        Variant::Outlined => format!(
            "color:{};border:1px solid {};background:transparent;border-radius:{}px;",
            color, color, theme.joy.radius
        ),
        Variant::Plain => format!(
            "color:{};background:transparent;border-radius:{}px;",
            color, theme.joy.radius
        ),
    };
    let onclick = props.onclick.clone();
    html! { <button style={style} onclick={onclick}>{ &props.label }</button> }
}
