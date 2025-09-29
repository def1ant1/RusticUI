use rustic_ui_headless::slider::{SliderConfig, SliderState};
use rustic_ui_material::{render_slider, SliderAdapterProps};
#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

/// Host-target entry point used when running the example in CI for smoke tests.
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let state = SliderState::new(SliderConfig::default());
    let markup = render_slider(&SliderAdapterProps::new(&state));
    println!("SSR markup:\n{}", markup.html);
}

/// WASM entry point bootstrapping the Yew renderer.
#[cfg(target_arch = "wasm32")]
fn main() {
    yew::Renderer::<App>::new().render();
}

/// Minimal Yew component that demonstrates using the shared slider renderer.
#[cfg(target_arch = "wasm32")]
#[function_component(App)]
fn app() -> Html {
    let state = SliderState::new(SliderConfig::default());
    let markup = render_slider(&SliderAdapterProps::new(&state));
    Html::from_html_unchecked(markup.html.into())
}
