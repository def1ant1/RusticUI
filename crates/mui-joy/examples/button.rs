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
    html! {
        <ThemeProvider theme={Theme::default()}>
            <Button label="Add" color={Color::Primary} variant={Variant::Solid} {onclick} />
            <p>{ format!("Clicks: {}", *count) }</p>
        </ThemeProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
