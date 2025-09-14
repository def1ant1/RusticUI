use mui_system::theme_provider::use_theme;
use yew::prelude::*;

use crate::{joy_component_props, Color, Variant};

joy_component_props!(CardProps {
    /// Nested content displayed within the card body.
    children: Children,
});

/// Simple container mirroring Joy UI's Card component.
///
/// The implementation intentionally focuses on the core layout primitives
/// so it can serve as a foundation for more advanced features such as
/// headers, footers or media sections.
#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    let theme = use_theme();
    // Resolve the color from the active theme's palette.
    let color = match props.color {
        Color::Primary => theme.palette.primary.clone(),
        Color::Neutral => theme.palette.neutral.clone(),
        Color::Danger => theme.palette.danger.clone(),
    };
    // Basic styling demonstrating Joy's variant system.
    let style = match props.variant {
        Variant::Solid => format!(
            "background:{};padding:16px;border-radius:{}px;",
            color, theme.joy.radius
        ),
        Variant::Soft => format!(
            "background:{}33;padding:16px;border-radius:{}px;",
            color, theme.joy.radius
        ),
        Variant::Outlined => format!(
            "border:1px solid {};padding:16px;border-radius:{}px;",
            color, theme.joy.radius
        ),
        Variant::Plain => format!("padding:16px;border-radius:{}px;", theme.joy.radius),
    };
    html! { <div style={style}>{ for props.children.iter() }</div> }
}
