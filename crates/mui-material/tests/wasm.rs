#![cfg(feature = "yew")]

use mui_headless::checkbox::CheckboxState;
use mui_headless::list::{ListState, SelectionMode};
use mui_headless::radio::{RadioGroupState, RadioOrientation};
use mui_headless::switch::SwitchState;
use mui_material::checkbox::{self, CheckboxProps};
use mui_material::radio::{self, RadioGroupProps};
use mui_material::switch::{self, SwitchProps};
use mui_material::table::{self, TableColumn, TableProps, TableRow};
use mui_material::{AppBar, Button, Snackbar, TextField};
use mui_styled_engine::{Theme, ThemeProvider};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_test::*;
use yew::prelude::*;
use yew::AttrValue;
use yew::Renderer;

// Expose the axe-core helper so each test can easily perform an
// accessibility audit.  Centralizing the logic keeps individual tests
// focused on asserting behavior rather than plumbing.
mod axe;
use axe::axe_check;

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

// ---------------------------------------------------------------------------
// Additional interactive component tests exercising styles and accessibility.
// ---------------------------------------------------------------------------

/// Ensure that the AppBar injects themed styles and passes an axe-core audit.
#[wasm_bindgen_test(async)]
async fn app_bar_style_and_accessibility() {
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

    // The styled engine injects a <style> tag into <head>. Verify the expected
    // background color from the default theme is present so visual styling is
    // preserved across refactors.
    let head_html = document.head().unwrap().inner_html();
    assert!(head_html.contains("background: #1976d2"));

    axe_check(&mount).await;
}

/// Verify that buttons respond to keyboard interaction and are free of
/// accessibility violations.
#[wasm_bindgen_test(async)]
async fn button_keyboard_navigation() {
    use std::cell::Cell;
    use std::rc::Rc;

    let document = gloo_utils::document();
    let mount = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&mount).unwrap();

    #[function_component(App)]
    fn app() -> Html {
        html! {
            <ThemeProvider theme={Theme::default()}>
                <Button label="Submit" />
            </ThemeProvider>
        }
    }

    Renderer::<App>::with_root(mount.clone()).render();

    let button: web_sys::HtmlElement = mount
        .query_selector("button")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();

    // Track click invocation so we can assert keyboard activation works.
    let clicked = Rc::new(Cell::new(false));
    {
        let clicked = clicked.clone();
        let cb = Closure::<dyn FnMut(_)>::new(move |_e: web_sys::Event| {
            clicked.set(true);
        });
        button
            .add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())
            .unwrap();
        cb.forget();
    }

    button.focus().unwrap();
    let event = web_sys::KeyboardEvent::new_with_keyboard_event_init_dict(
        "keydown",
        web_sys::KeyboardEventInit::new().key("Enter"),
    )
    .unwrap();
    button.dispatch_event(&event).unwrap();

    assert!(clicked.get(), "Enter key should trigger click");
    axe_check(&mount).await;
}

/// Validate that the data table renders semantic roles and passes an axe audit.
#[wasm_bindgen_test(async)]
async fn table_accessibility_contract() {
    let document = gloo_utils::document();
    let mount = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&mount).unwrap();

    #[function_component(App)]
    fn app() -> Html {
        let props = TableProps::new(
            vec![
                TableColumn::new("Region"),
                TableColumn::new("Nodes").numeric(),
            ],
            vec![
                TableRow::new(vec!["us-central".into(), "64".into()]),
                TableRow::new(vec!["eu-west".into(), "48".into()]),
            ],
        )
        .with_selection_mode(SelectionMode::Single)
        .with_automation_id("wasm-table");
        let state = ListState::uncontrolled(props.rows.len(), &[], SelectionMode::Single);
        let markup = table::yew::render(&props, &state);

        html! {
            <ThemeProvider theme={Theme::default()}>
                { Html::from_html_unchecked(AttrValue::from(markup)) }
            </ThemeProvider>
        }
    }

    Renderer::<App>::with_root(mount.clone()).render();

    let table = mount
        .query_selector("table")
        .unwrap()
        .expect("table rendered");
    assert_eq!(table.get_attribute("role").unwrap(), "grid");
    assert!(table
        .get_attribute("data-automation-id")
        .unwrap()
        .contains("wasm-table"));

    axe_check(&mount).await;
}

/// Verify the checkbox control renders with accessible markup and passes the
/// axe-core audit in a browser environment.
#[wasm_bindgen_test(async)]
async fn checkbox_accessibility_audit() {
    let document = gloo_utils::document();
    let mount = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&mount).unwrap();

    #[function_component(App)]
    fn app() -> Html {
        let props = CheckboxProps::new("Email updates");
        let state = CheckboxState::uncontrolled(false, false);
        let markup =
            Html::from_html_unchecked(AttrValue::from(checkbox::yew::render(&props, &state)));
        html! {
            <ThemeProvider theme={Theme::default()}>
                {markup}
            </ThemeProvider>
        }
    }

    Renderer::<App>::with_root(mount.clone()).render();

    let checkbox = mount
        .query_selector("[role='checkbox']")
        .unwrap()
        .expect("checkbox rendered");
    assert_eq!(checkbox.get_attribute("aria-checked").unwrap(), "false");
    axe_check(&mount).await;
}

/// Validate the radio group exposes radiogroup semantics and is free of axe
/// violations.
#[wasm_bindgen_test(async)]
async fn radio_accessibility_audit() {
    let document = gloo_utils::document();
    let mount = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&mount).unwrap();

    #[function_component(App)]
    fn app() -> Html {
        let state = RadioGroupState::uncontrolled(
            vec!["Alpha".into(), "Beta".into(), "Gamma".into()],
            false,
            RadioOrientation::Horizontal,
            Some(0),
        );
        let props = RadioGroupProps::from_state(&state);
        let markup = Html::from_html_unchecked(AttrValue::from(radio::yew::render(&props, &state)));
        html! {
            <ThemeProvider theme={Theme::default()}>
                {markup}
            </ThemeProvider>
        }
    }

    Renderer::<App>::with_root(mount.clone()).render();

    let radios = mount.query_selector_all("[role='radio']").unwrap();
    assert_eq!(radios.length(), 3);
    axe_check(&mount).await;
}

/// Validate the switch renders and passes accessibility audit.
#[wasm_bindgen_test(async)]
async fn switch_accessibility_audit() {
    let document = gloo_utils::document();
    let mount = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&mount).unwrap();

    #[function_component(App)]
    fn app() -> Html {
        let mut state = SwitchState::uncontrolled(false, false);
        state.focus();
        let props = SwitchProps::new("Notifications");
        let markup =
            Html::from_html_unchecked(AttrValue::from(switch::yew::render(&props, &state)));
        html! {
            <ThemeProvider theme={Theme::default()}>
                {markup}
            </ThemeProvider>
        }
    }

    Renderer::<App>::with_root(mount.clone()).render();

    let switch_el = mount
        .query_selector("[role='switch']")
        .unwrap()
        .expect("switch rendered");
    assert_eq!(switch_el.get_attribute("aria-checked").unwrap(), "false");
    axe_check(&mount).await;
}

/// Confirm that the TextField computes inline styles based on theme tokens and
/// remains accessible.
#[wasm_bindgen_test(async)]
async fn text_field_styles_and_accessibility() {
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
        .expect("input rendered")
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap();
    // The component builds an inline style string. Validate that the default
    // primary color is present which proves theme integration works.
    let style = input.get_attribute("style").unwrap();
    assert!(style.contains("#1976d2"));

    axe_check(&mount).await;
}
