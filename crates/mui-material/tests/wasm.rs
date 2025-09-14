#![cfg(feature = "yew")]

use mui_material::{AppBar, Button, Snackbar, TextField};
use mui_styled_engine::{Theme, ThemeProvider};
use wasm_bindgen_test::*;
use yew::prelude::*;
use yew::Renderer;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn button_renders_with_theme_color() {
    let document = gloo_utils::document();
    let mount = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&mount).unwrap();

    #[function_component(App)]
    fn app() -> Html {
        html! {
            <ThemeProvider theme={Theme::default()}>
                <Button label="Hello" />
            </ThemeProvider>
        }
    }

    Renderer::<App>::with_root(mount.clone()).render();

    let button = mount
        .query_selector("button")
        .unwrap()
        .expect("button rendered");
    assert_eq!(button.text_content().unwrap(), "Hello");
}

#[wasm_bindgen_test]
fn app_bar_renders_title() {
    let document = gloo_utils::document();
    let mount = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&mount).unwrap();

    #[function_component(App)]
    fn app() -> Html {
        html! {
            <ThemeProvider theme={Theme::default()}>
                <AppBar title="Dashboard" aria_label="main navigation" />
            </ThemeProvider>
        }
    }

    Renderer::<App>::with_root(mount.clone()).render();

    let header = mount
        .query_selector("header")
        .unwrap()
        .expect("app bar rendered");
    assert_eq!(
        header.get_attribute("aria-label").unwrap(),
        "main navigation"
    );
}

#[wasm_bindgen_test]
fn text_field_sets_placeholder() {
    let document = gloo_utils::document();
    let mount = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&mount).unwrap();

    #[function_component(App)]
    fn app() -> Html {
        html! {
            <ThemeProvider theme={Theme::default()}>
                <TextField value="" placeholder="Name" aria_label="name" />
            </ThemeProvider>
        }
    }

    Renderer::<App>::with_root(mount.clone()).render();

    let input = mount
        .query_selector("input")
        .unwrap()
        .expect("input rendered");
    assert_eq!(input.get_attribute("placeholder").unwrap(), "Name");
}

#[wasm_bindgen_test]
fn snackbar_announces_message() {
    let document = gloo_utils::document();
    let mount = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&mount).unwrap();

    #[function_component(App)]
    fn app() -> Html {
        html! {
            <ThemeProvider theme={Theme::default()}>
                <Snackbar message="Saved" />
            </ThemeProvider>
        }
    }

    Renderer::<App>::with_root(mount.clone()).render();

    let div = mount
        .query_selector("div[role='status']")
        .unwrap()
        .expect("snackbar rendered");
    assert_eq!(div.text_content().unwrap(), "Saved");
}
