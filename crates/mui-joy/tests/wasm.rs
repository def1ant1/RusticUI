// Module included from tests/mod.rs when the Yew feature is enabled.
//! Browser integration tests exercising the Yew adapters.
//!
//! The suite is compiled conditionally so that enabling Leptos, Dioxus or
//! Sycamore without the Yew feature still produces a clean build focussed on
//! the framework-neutral prop definitions.

mod axe;

use axe::axe_check;
use mui_joy::{AspectRatio, Button, Chip, Color, Variant};
use mui_system::theme_provider::ThemeProvider;
use mui_system::Theme;
use wasm_bindgen_test::*;
use yew::prelude::*;
use yew::Renderer;

wasm_bindgen_test_configure!(run_in_browser);

/// Create and append a DOM mount point that individual tests can reuse.
///
/// Centralizing the scaffolding keeps each test focussed on behaviour and
/// accessibility assertions rather than the mechanics of DOM construction.
fn mount_host() -> web_sys::Element {
    let document = gloo_utils::document();
    let mount = document
        .create_element("div")
        .expect("failed to create mount point");
    document
        .body()
        .expect("missing <body> element")
        .append_child(&mount)
        .expect("failed to insert mount point");
    mount
}

/// Remove the temporary mount point created via [`mount_host`].  This ensures
/// every test leaves the DOM in a known-good state, which is critical when the
/// wasm test harness runs suites sequentially in the same document.
fn teardown_host(mount: &web_sys::Element) {
    let document = gloo_utils::document();
    let mount_node: web_sys::Node = mount.clone().into();
    document
        .body()
        .expect("missing <body> element")
        .remove_child(&mount_node)
        .expect("failed to remove mount point");
}

#[wasm_bindgen_test]
fn button_clicks_increment() {
    let mount = mount_host();

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

    teardown_host(&mount);
}

#[wasm_bindgen_test]
fn chip_delete_triggers_callback() {
    let mount = mount_host();

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

    teardown_host(&mount);
}

#[wasm_bindgen_test]
fn aspect_ratio_sets_padding() {
    let mount = mount_host();

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

    teardown_host(&mount);
}

#[wasm_bindgen_test]
async fn button_is_accessible() {
    let mount = mount_host();

    #[function_component(App)]
    fn app() -> Html {
        html! {
            <ThemeProvider theme={Theme::default()}>
                <Button label="Add" color={Color::Primary} variant={Variant::Solid} />
            </ThemeProvider>
        }
    }

    Renderer::<App>::with_root(mount.clone()).render();
    axe_check(&mount).await;
    teardown_host(&mount);
}

#[wasm_bindgen_test]
async fn chip_is_accessible() {
    let mount = mount_host();

    #[function_component(App)]
    fn app() -> Html {
        html! {
            <ThemeProvider theme={Theme::default()}>
                <Chip label="Filter" color={Color::Success} variant={Variant::Soft} />
            </ThemeProvider>
        }
    }

    Renderer::<App>::with_root(mount.clone()).render();
    axe_check(&mount).await;
    teardown_host(&mount);
}

#[wasm_bindgen_test]
async fn aspect_ratio_is_accessible() {
    let mount = mount_host();

    #[function_component(App)]
    fn app() -> Html {
        html! {
            <AspectRatio ratio={4.0 / 3.0}>
                <img src="https://via.placeholder.com/400x300" alt="placeholder" />
            </AspectRatio>
        }
    }

    Renderer::<App>::with_root(mount.clone()).render();
    axe_check(&mount).await;
    teardown_host(&mount);
}
