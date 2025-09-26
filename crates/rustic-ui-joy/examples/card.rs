//! Demonstrates Card, Chip and AspectRatio components working together.
//! Run with `cargo run -p mui-joy --example card --features yew`.

use rustic_ui_joy::{AspectRatio, Card, Chip, Color, Variant};
use rustic_ui_system::theme_provider::ThemeProvider;
use rustic_ui_system::Theme;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let deleted = use_state(|| false);
    let on_delete = {
        let deleted = deleted.clone();
        Callback::from(move |_| deleted.set(true))
    };
    html! {
        <ThemeProvider theme={Theme::default()}>
            <Card color={Color::Info} variant={Variant::Soft}>
                <AspectRatio ratio={16.0 / 9.0}>
                    <img src="https://via.placeholder.com/300" alt="placeholder" />
                </AspectRatio>
                { if !*deleted {
                    html! { <Chip label="Deletable" color={Color::Success} on_delete={Some(on_delete)} /> }
                } else {
                    html! { <span>{"Deleted"}</span> }
                }}
            </Card>
        </ThemeProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
