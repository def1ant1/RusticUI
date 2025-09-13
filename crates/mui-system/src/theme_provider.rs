#![cfg(feature = "yew")]
use yew::prelude::*;
use crate::theme::Theme;

/// Provides the current [`Theme`] to descendant components via Yew's
/// `ContextProvider` mechanism.
#[derive(Properties, PartialEq)]
pub struct ThemeProviderProps {
    pub theme: Theme,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(ThemeProvider)]
pub fn theme_provider(props: &ThemeProviderProps) -> Html {
    html! {
        <ContextProvider<Theme> context={props.theme.clone()}>
            { for props.children.iter() }
        </ContextProvider<Theme>>
    }
}

/// Retrieves the current theme from context. Components can call this helper
/// instead of dealing with `use_context` directly which keeps the call sites
/// concise.
#[cfg(feature = "yew")]
pub fn use_theme() -> Theme {
    use_context::<Theme>().unwrap_or_default()
}
