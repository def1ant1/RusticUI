//! Simple example showcasing the Joy `Button` component.
//!
//! Run with:
//! `cargo run -p mui-joy --example button --features yew`

use mui_joy::{Button, Color, Variant};
use mui_system::theme_provider::ThemeProvider;
use mui_system::Theme;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let count = use_state(|| 0);
    let onclick = {
        let count = count.clone();
        Callback::from(move |_| count.set(*count + 1))
    };
    let theme = Theme::default();
    let swatches = Color::ALL
        .iter()
        .map(|color| {
            let label = format!("{} action", color.as_str());
            html! {
                <Button
                    label={label}
                    color={*color}
                    variant={Variant::Solid}
                    onclick={onclick.clone()}
                />
            }
        })
        .collect::<Html>();

    html! {
        <ThemeProvider theme={theme}>
            <div style="display:flex;gap:12px;flex-wrap:wrap;align-items:center;">
                { swatches }
            </div>
            <p>{ format!("Clicks: {}", *count) }</p>
        </ThemeProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
