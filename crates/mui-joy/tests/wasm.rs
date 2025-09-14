use mui_joy::Button;
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
                <Button label="Add" {onclick} />
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
