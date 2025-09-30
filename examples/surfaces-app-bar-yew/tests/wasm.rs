use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use yew::Renderer;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn renders_automation_attributes() {
    let document = web_sys::window().unwrap().document().unwrap();
    let root = document.create_element("div").unwrap();
    root.set_id("test-root");
    document.body().unwrap().append_child(&root).unwrap();

    Renderer::<surfaces_app_bar_yew::App>::with_root(root.into()).render();

    let header = document
        .query_selector("header[data-component='rustic_ui_app-bar']")
        .unwrap()
        .expect("app bar rendered");
    assert_eq!(
        header
            .get_attribute("data-analytics-view-id")
            .as_deref(),
        Some("nav.operations.view")
    );
    assert_eq!(
        header
            .get_attribute("data-automation-id")
            .as_deref(),
        Some("rustic-app-bar-operations-console")
    );
}
