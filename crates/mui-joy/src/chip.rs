use mui_system::theme_provider::use_theme;
use yew::prelude::*;

use crate::{joy_component_props, Color, Variant};

joy_component_props!(ChipProps {
    /// Text displayed within the chip.
    label: String,
    /// Optional handler invoked when the delete icon is clicked.
    on_delete: Option<Callback<MouseEvent>>,
});

/// A small piece of information, often used as an input or tag.
#[function_component(Chip)]
pub fn chip(props: &ChipProps) -> Html {
    let theme = use_theme();
    let color = match props.color {
        Color::Primary => theme.palette.primary.clone(),
        Color::Neutral => theme.palette.neutral.clone(),
        Color::Danger => theme.palette.danger.clone(),
    };
    let style = match props.variant {
        Variant::Solid => format!(
            "background:{};color:#fff;border-radius:{}px;padding:4px 8px;",
            color, theme.joy.radius
        ),
        Variant::Soft => format!(
            "background:{}33;color:{};border-radius:{}px;padding:4px 8px;",
            color, color, theme.joy.radius
        ),
        Variant::Outlined => format!(
            "border:1px solid {};color:{};border-radius:{}px;padding:4px 8px;",
            color, color, theme.joy.radius
        ),
        Variant::Plain => format!(
            "color:{};border-radius:{}px;padding:4px 8px;",
            color, theme.joy.radius
        ),
    };
    let delete_button = props.on_delete.as_ref().map(|cb| {
        html! { <button style="margin-left:4px" onclick={cb.clone()}>{{"×"}}</button> }
    });
    html! { <span style={style}>{ &props.label }{ delete_button }</span> }
}
