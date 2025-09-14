use mui_styled_engine::{css_with_theme, use_theme};
use yew::prelude::*;

pub use crate::macros::{
    Color as TextFieldColor, Size as TextFieldSize, Variant as TextFieldVariant,
};

crate::material_component_props!(TextFieldProps {
    /// Current value displayed in the input element.
    value: String,
    /// Placeholder hint shown when the field is empty.
    placeholder: String,
    /// Accessibility label describing the purpose of the field.
    aria_label: String,
});

/// Controlled text input field.
/// Styling is resolved through the shared [`Theme`] so colors and typography
/// remain consistent across the application.
#[function_component(TextField)]
pub fn text_field(props: &TextFieldProps) -> Html {
    let theme = use_theme();
    let color = match props.color {
        TextFieldColor::Primary => theme.palette.primary.clone(),
        TextFieldColor::Secondary => theme.palette.secondary.clone(),
    };
    let font_size = match props.size {
        TextFieldSize::Small => "0.8rem",
        TextFieldSize::Medium => "1rem",
        TextFieldSize::Large => "1.2rem",
    };
    let border = match props.variant {
        TextFieldVariant::Outlined => format!("1px solid {}", color),
        TextFieldVariant::Contained => format!("1px solid {}", color),
        TextFieldVariant::Text => "none".to_string(),
    };
    let style = css_with_theme!(
        theme,
        r#"
        color: ${color};
        font-size: ${font_size};
        border: ${border};
        padding: 4px 8px;
    "#,
        color = color,
        font_size = font_size,
        border = border
    );
    let class = style.get_class_name().to_string();

    html! {
        <input
            class={class}
            value={props.value.clone()}
            placeholder={props.placeholder.clone()}
            aria-label={props.aria_label.clone()}
        />
    }
}
