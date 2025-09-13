use yew::prelude::*;
use mui_styled_engine::use_theme;

use crate::{material_enum, material_props};

material_enum!(ButtonColor { Primary, Secondary });
material_enum!(ButtonVariant { Text, Contained, Outlined });

material_props!(ButtonProps {
    /// Text displayed inside the button.
    label: String,
    /// Visual color scheme of the button.
    color: ButtonColor,
    /// Variant controlling the button's background and border.
    variant: ButtonVariant,
});

/// A basic Material Design button.
#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let theme = use_theme();
    let color = match props.color {
        ButtonColor::Primary => theme.palette.primary.clone(),
        ButtonColor::Secondary => theme.palette.secondary.clone(),
    };
    let style = match props.variant {
        ButtonVariant::Contained => format!("background:{};color:#fff;", color),
        ButtonVariant::Outlined => format!("color:{};border:1px solid {}", color, color),
        ButtonVariant::Text => format!("color:{};", color),
    };
    html! { <button style={style}>{ &props.label }</button> }
}
