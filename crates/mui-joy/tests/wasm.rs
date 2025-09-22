#![cfg(feature = "yew")]
//! Browser integration tests exercising the Yew adapters.
//!
//! The suite is compiled conditionally so that enabling Leptos, Dioxus or
//! Sycamore without the Yew feature still produces a clean build focussed on
//! the framework-neutral prop definitions.

use mui_joy::{AspectRatio, Button, Chip, Color, Variant};
use mui_system::theme_provider::ThemeProvider;
use mui_system::Theme;
use wasm_bindgen_test::*;
use yew::prelude::*;
use yew::Renderer;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn button_clicks_increment() {
    let document = gloo_utils::document();
    let mount = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&mount).unwrap();

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
                <div id="count">{*count}</div>
            </ThemeProvider>
        }
    }

    Renderer::<App>::with_root(mount.clone()).render();

    let button = mount.query_selector("button").unwrap().unwrap();
    let event = web_sys::Event::new("click").unwrap();
    button.dispatch_event(&event).unwrap();

    let count = mount.query_selector("#count").unwrap().unwrap();
    assert_eq!(count.text_content().unwrap(), "1");
}

#[wasm_bindgen_test]
fn chip_delete_triggers_callback() {
    let document = gloo_utils::document();
    let mount = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&mount).unwrap();

    #[function_component(App)]
    fn app() -> Html {
        let count = use_state(|| 0);
        let on_delete = {
            let count = count.clone();
            Callback::from(move |_| count.set(*count + 1))
        };
        html! {
            <ThemeProvider theme={Theme::default()}>
                <Chip label="Tag" on_delete={Some(on_delete)} />
                <div id="count">{*count}</div>
            </ThemeProvider>
        }
    }

    Renderer::<App>::with_root(mount.clone()).render();

    let btn = mount.query_selector("button").unwrap().unwrap();
    let event = web_sys::Event::new("click").unwrap();
    btn.dispatch_event(&event).unwrap();

    let count = mount.query_selector("#count").unwrap().unwrap();
    assert_eq!(count.text_content().unwrap(), "1");
}

#[wasm_bindgen_test]
fn aspect_ratio_sets_padding() {
    let document = gloo_utils::document();
    let mount = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&mount).unwrap();

    #[function_component(App)]
    fn app() -> Html {
        html! {
            <AspectRatio ratio={16.0 / 9.0}>
                <img src="https://via.placeholder.com/160x90" alt="placeholder" />
            </AspectRatio>
        }
    }

    Renderer::<App>::with_root(mount.clone()).render();

    let outer = mount.first_element_child().unwrap();
    let style = outer.get_attribute("style").unwrap();
    assert!(style.contains("56.25%"));
}
