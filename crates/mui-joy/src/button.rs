use mui_system::theme_provider::use_theme;
use yew::prelude::*;

use crate::{joy_enum, joy_props};

joy_enum!(ButtonColor {
    Primary,
    Neutral,
    Danger
});
joy_enum!(ButtonVariant {
    Solid,
    Soft,
    Outlined,
    Plain
});

joy_props!(ButtonProps {
    /// Text displayed inside the button.
    label: String,
    /// Visual color scheme of the button.
    color: ButtonColor,
    /// Variant controlling the button's background and border.
    variant: ButtonVariant,
    /// Click handler for interactive behavior.
    onclick: Callback<MouseEvent>,
});

/// A basic Joy UI button.
#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let theme = use_theme();
    let color = match props.color {
        ButtonColor::Primary => theme.palette.primary.clone(),
        ButtonColor::Neutral => theme.palette.neutral.clone(),
        ButtonColor::Danger => theme.palette.danger.clone(),
    };
    let style = match props.variant {
        ButtonVariant::Solid => format!(
            "background:{};color:#fff;border-radius:{}px;",
            color, theme.joy.radius
        ),
        ButtonVariant::Soft => format!(
            "background:{}33;color:{};border-radius:{}px;",
            color, color, theme.joy.radius
        ),
        ButtonVariant::Outlined => format!(
            "color:{};border:1px solid {};background:transparent;border-radius:{}px;",
            color, color, theme.joy.radius
        ),
        ButtonVariant::Plain => format!(
            "color:{};background:transparent;border-radius:{}px;",
            color, theme.joy.radius
        ),
    };
    let onclick = props.onclick.clone();
    html! { <button style={style} onclick={onclick}>{ &props.label }</button> }
}
