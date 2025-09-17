use crate::{responsive::Responsive, style, style_props, theme::Breakpoints};

/// Direction of children placement inside [`Stack`].
#[derive(Clone, PartialEq)]
pub enum StackDirection {
    Row,
    Column,
}

impl StackDirection {
    fn as_css_value(&self) -> &'static str {
        match self {
            StackDirection::Row => "row",
            StackDirection::Column => "column",
        }
    }
}

/// Shared style assembly for framework adapters and integration tests.
#[doc(hidden)]
pub fn build_stack_style(
    width: u32,
    breakpoints: &Breakpoints,
    direction: Option<StackDirection>,
    spacing: Option<&Responsive<String>>,
    align_items: Option<&str>,
    justify_content: Option<&str>,
    sx: &str,
) -> String {
    let direction_value = direction.unwrap_or(StackDirection::Column).as_css_value();
    let mut style = style_props! { display: "flex", flex_direction: direction_value };

    if let Some(sp) = spacing {
        // Resolve the gap for the current viewport.  Using `gap` keeps both
        // horizontal and vertical spacing in sync, mirroring the ergonomic
        // defaults of the upstream Stack implementation.
        let resolved = sp.resolve(width, breakpoints);
        style.push_str(&style_props! { gap: resolved });
    }
    if let Some(ai) = align_items {
        style.push_str(&style::align_items(ai));
    }
    if let Some(jc) = justify_content {
        style.push_str(&style::justify_content(jc));
    }

    style.push_str(sx);
    style
}

#[cfg(feature = "yew")]
mod yew_impl {
    use super::*;
    use yew::prelude::*;

    /// Properties for the [`Stack`] component.
    #[derive(Properties, PartialEq)]
    pub struct StackProps {
        /// Orientation of the stack. Defaults to vertical column layout.
        #[prop_or_default]
        pub direction: Option<StackDirection>,
        /// Gap between children. Accepts any CSS length value.
        #[prop_or_default]
        pub spacing: Option<Responsive<String>>,
        /// Align items on the cross axis.
        #[prop_or_default]
        pub align_items: Option<String>,
        /// Align items on the main axis.
        #[prop_or_default]
        pub justify_content: Option<String>,
        /// Additional arbitrary style string merged into the generated CSS.
        #[prop_or_default]
        pub sx: String,
        /// Child elements to render.
        #[prop_or_default]
        pub children: Children,
    }

    /// Minimal flexbox based stack layout.
    #[function_component(Stack)]
    pub fn stack(props: &StackProps) -> Html {
        let theme = crate::theme_provider::use_theme();
        let width = crate::responsive::viewport_width();
        let style_rules = build_stack_style(
            width,
            &theme.breakpoints,
            props.direction.clone(),
            props.spacing.as_ref(),
            props.align_items.as_deref(),
            props.justify_content.as_deref(),
            &props.sx,
        );
        // Reuse the shared scoped class helper so Stack benefits from CSS
        // caching and avoids inline declarations which break strict CSPs.
        let scoped = use_memo(style_rules, |css| {
            crate::ScopedClass::from_declarations(css.clone())
        });
        let class = scoped.class().to_string();
        html! { <div class={class}>{ for props.children.iter() }</div> }
    }
}

#[cfg(feature = "yew")]
pub use yew_impl::{Stack, StackProps};

#[cfg(feature = "leptos")]
mod leptos_impl {
    use super::*;
    use leptos::*;

    /// Leptos implementation of [`Stack`].
    #[component]
    pub fn Stack(
        #[prop(optional)] direction: Option<StackDirection>,
        #[prop(optional)] spacing: Option<Responsive<String>>,
        #[prop(optional, into)] align_items: Option<String>,
        #[prop(optional, into)] justify_content: Option<String>,
        #[prop(optional, into)] sx: String,
        children: Children,
    ) -> impl IntoView {
        let theme = crate::theme_provider::use_theme();
        let width = crate::responsive::viewport_width();
        let style_rules = build_stack_style(
            width,
            &theme.breakpoints,
            direction,
            spacing.as_ref(),
            align_items.as_deref(),
            justify_content.as_deref(),
            &sx,
        );
        // Store the scoped class in the runtime so Leptos keeps the CSS alive
        // until the component unmounts.
        let scoped = store_value(crate::ScopedClass::from_declarations(style_rules));
        let class = scoped.with_value(|class| class.class().to_string());
        view! { <div class=class>{children()}</div> }
    }
}

#[cfg(feature = "leptos")]
pub use leptos_impl::Stack;
