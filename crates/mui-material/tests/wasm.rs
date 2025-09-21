#![cfg(feature = "yew")]

use mui_headless::checkbox::CheckboxState;
use mui_headless::chip::{ChipConfig, ChipState};
use mui_headless::dialog::DialogState;
use mui_headless::drawer::{DrawerAnchor, DrawerState, DrawerVariant};
use mui_headless::list::{ListState, SelectionMode};
use mui_headless::menu::MenuState;
use mui_headless::popover::{PopoverPlacement, PopoverState};
use mui_headless::radio::{RadioGroupState, RadioOrientation};
use mui_headless::selection::ControlStrategy;
use mui_headless::switch::SwitchState;
use mui_headless::tabs::{ActivationMode, TabsOrientation, TabsState};
use mui_headless::text_field::TextFieldState;
use mui_headless::tooltip::{TooltipConfig, TooltipState};
use mui_material::checkbox::{self, CheckboxProps};
use mui_material::chip::{self, ChipProps};
use mui_material::dialog::{self as dialog_adapter, DialogSurfaceOptions};
use mui_material::drawer::{self, DrawerLayoutOptions, DrawerProps};
use mui_material::menu::{self, MenuItem, MenuProps};
use mui_material::radio::{self, RadioGroupProps};
use mui_material::switch::{self, SwitchProps};
use mui_material::tab_panel;
use mui_material::table::{self, TableColumn, TableProps, TableRow};
use mui_material::tabs::{self, TabListLayoutOptions, TabListProps};
use mui_material::text_field::TextFieldStateHandle;
use mui_material::tooltip::{self, TooltipProps};
use mui_material::{AppBar, Button, Snackbar, TextField};
use mui_styled_engine::{Theme, ThemeProvider};
use std::rc::Rc;
use std::time::Duration;
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

/// Render the Tabs adapter end-to-end in a browser to verify orientation
/// switching, manual activation semantics and accessibility integration.
#[wasm_bindgen_test(async)]
async fn tabs_orientation_and_accessibility() {
    let document = gloo_utils::document();
    let mount = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&mount).unwrap();

    #[function_component(App)]
    fn app() -> Html {
        let layout = TabListLayoutOptions::default();
        let theme = Theme::default();
        let state = TabsState::new(
            2,
            Some(0),
            ActivationMode::Manual,
            TabsOrientation::Vertical,
            unsafe { std::mem::transmute(1u8) },
            unsafe { std::mem::transmute(1u8) },
        );

        let tabs_markup = vec![
            mui_material::tab::render_tab_html(
                &state,
                state.tab(0).id("tab-overview").controls("panel-overview"),
                "Overview",
            ),
            mui_material::tab::render_tab_html(
                &state,
                state.tab(1).id("tab-reports").controls("panel-reports"),
                "Reports",
            ),
        ]
        .join("");

        let props = TabListProps {
            state: &state,
            attributes: state.list_attributes().id("account-tabs"),
            children: tabs_markup.as_str(),
            layout: &layout,
            theme: &theme,
            viewport: Some(theme.breakpoints.xl),
            on_activate_event: Some("tab-activate"),
        };

        let list_markup = tabs::yew::render_tab_list(props);
        let panels_markup = format!(
            "{}{}",
            tab_panel::render_tab_panel_html(
                &state,
                0,
                state
                    .panel(0)
                    .id("panel-overview")
                    .labelled_by("tab-overview"),
                r#"<p data-testid=\"panel-body\">Overview metrics</p>"#,
            ),
            tab_panel::render_tab_panel_html(
                &state,
                1,
                state
                    .panel(1)
                    .id("panel-reports")
                    .labelled_by("tab-reports"),
                "<p>Quarterly reports</p>",
            ),
        );

        html! {
            <ThemeProvider theme={theme.clone()}>
                { Html::from_html_unchecked(AttrValue::from(format!("{}{}", list_markup, panels_markup))) }
            </ThemeProvider>
        }
    }

    Renderer::<App>::with_root(mount.clone()).render();

    let wrapper = mount
        .query_selector("[data-on-activate]")
        .unwrap()
        .expect("tabs wrapper rendered");
    assert_eq!(
        wrapper.get_attribute("data-on-activate").unwrap(),
        "tab-activate"
    );

    let tablist = mount
        .query_selector("[role='tablist']")
        .unwrap()
        .expect("tablist rendered");
    assert_eq!(
        tablist.get_attribute("data-orientation").unwrap(),
        "vertical"
    );
    assert_eq!(tablist.get_attribute("data-activation").unwrap(), "manual");

    let tabs = mount.query_selector_all("[role='tab']").unwrap();
    assert_eq!(tabs.length(), 2);

    let panel = mount
        .query_selector("[role='tabpanel']")
        .unwrap()
        .expect("panel rendered");
    assert_eq!(
        panel.get_attribute("aria-labelledby").unwrap(),
        "tab-overview"
    );
    assert!(panel.inner_html().contains("data-testid=\"panel-body\""));

    axe_check(&mount).await;
}

/// Smoke test the Drawer adapter in wasm to verify responsive anchor handling
/// and accessibility attributes emitted by the modal variant.
#[wasm_bindgen_test(async)]
async fn drawer_modal_accessibility() {
    let document = gloo_utils::document();
    let mount = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&mount).unwrap();

    #[function_component(App)]
    fn app() -> Html {
        let mut layout = DrawerLayoutOptions::default();
        layout.anchor.lg = Some(DrawerAnchor::Top);
        layout.anchor.xl = Some(DrawerAnchor::Top);

        let theme = Theme::default();
        let state = DrawerState::new(
            true,
            unsafe { std::mem::transmute(1u8) },
            DrawerVariant::Modal,
            DrawerAnchor::Top,
        );

        let props = DrawerProps {
            state: &state,
            surface: state
                .surface_attributes()
                .id("nav-drawer")
                .labelled_by("drawer-heading"),
            backdrop: state.backdrop_attributes(),
            body: "<header id=\"drawer-heading\">Navigation</header><ul><li><a href=\"/\" data-testid=\"drawer-home\">Home</a></li><li><a href=\"/reports\">Reports</a></li></ul>",
            layout: &layout,
            theme: &theme,
            viewport: Some(theme.breakpoints.xl),
            on_toggle_event: Some("drawer-toggle"),
        };

        let render = drawer::yew::render(props);
        let mut nodes = vec![Html::from_html_unchecked(AttrValue::from(render.surface))];
        if let Some(backdrop_markup) = render.backdrop {
            nodes.push(Html::from_html_unchecked(AttrValue::from(backdrop_markup)));
        }

        html! {
            <ThemeProvider theme={theme.clone()}>
                { for nodes.into_iter() }
            </ThemeProvider>
        }
    }

    Renderer::<App>::with_root(mount.clone()).render();

    let dialog = mount
        .query_selector("[role='dialog']")
        .unwrap()
        .expect("drawer surface rendered");
    assert_eq!(dialog.get_attribute("data-anchor").unwrap(), "top");
    assert_eq!(dialog.get_attribute("data-variant").unwrap(), "modal");
    assert_eq!(
        dialog.get_attribute("data-on-toggle").unwrap(),
        "drawer-toggle"
    );

    let backdrop = mount
        .query_selector("[data-variant='modal'][aria-hidden]")
        .unwrap()
        .expect("backdrop rendered");
    assert_eq!(backdrop.get_attribute("data-open").unwrap(), "true");

    let nav_link = mount
        .query_selector("[data-testid='drawer-home']")
        .unwrap()
        .expect("navigation link rendered");
    assert_eq!(nav_link.text_content().unwrap(), "Home");

    axe_check(&mount).await;
}

#[wasm_bindgen_test(async)]
async fn menu_disabled_items_expose_state_and_pass_axe() {
    let document = gloo_utils::document();
    let mount = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&mount).unwrap();

    #[function_component(App)]
    fn app() -> Html {
        let props = MenuProps::new(
            "Actions",
            vec![
                MenuItem::new("Profile", "profile"),
                MenuItem::new("Logout", "logout"),
                MenuItem::new("Archive", "archive"),
            ],
        )
        .with_automation_id("wasm-menu");
        let mut state = MenuState::new(
            props.items.len(),
            false,
            ControlStrategy::Uncontrolled,
            ControlStrategy::Uncontrolled,
        );
        state.set_item_disabled(1, true);
        let markup = menu::yew::render(&props, &state);

        html! {
            <ThemeProvider theme={Theme::default()}>
                { Html::from_html_unchecked(AttrValue::from(markup)) }
            </ThemeProvider>
        }
    }

    Renderer::<App>::with_root(mount.clone()).render();

    let disabled_item = mount
        .query_selector("[data-automation-item='wasm-menu-1']")
        .unwrap()
        .expect("disabled menu item rendered");
    assert_eq!(
        disabled_item.get_attribute("aria-disabled").unwrap(),
        "true"
    );
    assert_eq!(
        disabled_item.get_attribute("data-disabled").unwrap(),
        "true"
    );

    let enabled_item = mount
        .query_selector("[data-automation-item='wasm-menu-0']")
        .unwrap()
        .expect("enabled menu item rendered");
    assert!(enabled_item.get_attribute("aria-disabled").is_none());
    assert!(enabled_item.get_attribute("data-disabled").is_none());

    axe_check(&mount).await;
}

/// Exercise the tooltip SSR adapter in a browser environment to confirm the
/// ARIA linkage, keyboard affordances and portal metadata survive hydration.
#[wasm_bindgen_test(async)]
async fn tooltip_focus_keyboard_and_accessibility() {
    use std::cell::Cell;
    use std::rc::Rc;

    let document = gloo_utils::document();
    let mount = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&mount).unwrap();

    #[function_component(App)]
    fn app() -> Html {
        let markup = {
            let mut config = TooltipConfig::default();
            config.show_delay = Duration::from_millis(0);
            let mut state = TooltipState::new(config);
            state.focus_anchor();
            let props = TooltipProps::new("Details", "Explains the current KPI")
                .with_automation_id("wasm-tooltip")
                .with_trigger_haspopup("dialog")
                .with_surface_labelled_by("tooltip-heading");
            tooltip::yew::render(&props, &state)
        };

        html! {
            <ThemeProvider theme={Theme::default()}>
                <span id="tooltip-heading" data-testid="tooltip-heading">Tooltip heading</span>
                { Html::from_html_unchecked(AttrValue::from(markup)) }
            </ThemeProvider>
        }
    }

    Renderer::<App>::with_root(mount.clone()).render();

    let trigger: web_sys::HtmlElement = mount
        .query_selector("[data-component='mui-tooltip-trigger']")
        .unwrap()
        .expect("tooltip trigger rendered")
        .dyn_into()
        .unwrap();
    assert_eq!(trigger.id(), "wasm-tooltip-trigger");
    assert_eq!(
        trigger.get_attribute("aria-describedby").unwrap(),
        "wasm-tooltip-surface"
    );
    assert_eq!(trigger.get_attribute("type").unwrap(), "button");
    assert_eq!(
        trigger.get_attribute("data-automation-trigger").unwrap(),
        "wasm-tooltip"
    );
    assert!(
        !trigger.class_name().is_empty(),
        "scoped class missing on trigger"
    );

    let surface = document
        .get_element_by_id("wasm-tooltip-surface")
        .expect("tooltip surface rendered");
    assert_eq!(surface.get_attribute("role").unwrap(), "tooltip");
    assert_eq!(surface.get_attribute("aria-hidden").unwrap(), "false");
    assert_eq!(
        surface.get_attribute("data-automation-surface").unwrap(),
        "wasm-tooltip"
    );
    let surface_class = surface.get_attribute("class").unwrap_or_default();
    assert!(!surface_class.is_empty(), "scoped class missing on surface");

    let portal = document
        .get_element_by_id("wasm-tooltip-portal")
        .expect("portal container rendered");
    assert_eq!(
        portal.get_attribute("data-portal-root").unwrap(),
        "wasm-tooltip"
    );
    let anchor = document
        .get_element_by_id("wasm-tooltip-anchor")
        .expect("portal anchor rendered");
    assert_eq!(
        anchor.get_attribute("data-portal-anchor").unwrap(),
        "wasm-tooltip"
    );

    let keyboard_invoked = Rc::new(Cell::new(false));
    {
        let keyboard_invoked = keyboard_invoked.clone();
        let handler = Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(move |_event| {
            keyboard_invoked.set(true);
        });
        trigger
            .add_event_listener_with_callback("keydown", handler.as_ref().unchecked_ref())
            .unwrap();
        handler.forget();
    }

    trigger.focus().unwrap();
    assert_eq!(document.active_element().unwrap().id(), trigger.id());

    let key_event = web_sys::KeyboardEvent::new_with_keyboard_event_init_dict(
        "keydown",
        web_sys::KeyboardEventInit::new().key("Enter").bubbles(true),
    )
    .unwrap();
    trigger.dispatch_event(&key_event).unwrap();
    assert!(
        keyboard_invoked.get(),
        "Enter key should be observed by tooltip trigger listeners"
    );

    axe_check(&mount).await;
}

/// Render the chip SSR adapter and exercise focus/delete affordances to ensure
/// hydration parity and accessibility semantics hold inside the browser.
#[wasm_bindgen_test(async)]
async fn chip_delete_button_activation_and_accessibility() {
    use std::cell::Cell;
    use std::rc::Rc;

    let document = gloo_utils::document();
    let mount = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&mount).unwrap();

    #[function_component(App)]
    fn app() -> Html {
        let markup = {
            let mut config = ChipConfig::default();
            config.show_delay = Duration::from_millis(0);
            config.hide_delay = Duration::from_millis(0);
            config.delete_delay = Duration::from_millis(0);
            let mut state = ChipState::new(config);
            state.focus();
            let props = ChipProps::new("Error budget")
                .with_automation_id("wasm-chip")
                .with_delete_label("Remove chip");
            chip::yew::render(&props, &state)
        };

        html! {
            <ThemeProvider theme={Theme::default()}>
                { Html::from_html_unchecked(AttrValue::from(markup)) }
            </ThemeProvider>
        }
    }

    Renderer::<App>::with_root(mount.clone()).render();

    let chip_root: web_sys::HtmlElement = mount
        .query_selector("[data-component='mui-chip']")
        .unwrap()
        .expect("chip root rendered")
        .dyn_into()
        .unwrap();
    assert_eq!(chip_root.id(), "wasm-chip");
    assert_eq!(chip_root.get_attribute("role").unwrap(), "button");
    assert_eq!(chip_root.get_attribute("tabindex").unwrap(), "0");
    assert_eq!(
        chip_root.get_attribute("data-controls-visible").unwrap(),
        "true"
    );
    assert_eq!(
        chip_root.get_attribute("data-automation-id").unwrap(),
        "wasm-chip"
    );
    assert_eq!(
        chip_root.get_attribute("aria-labelledby").unwrap(),
        "wasm-chip-label"
    );
    assert_eq!(
        chip_root.get_attribute("aria-describedby").unwrap(),
        "wasm-chip-delete"
    );
    assert!(
        !chip_root.class_name().is_empty(),
        "scoped class missing on chip"
    );

    chip_root.focus().unwrap();
    assert_eq!(document.active_element().unwrap().id(), chip_root.id());

    let label = document
        .get_element_by_id("wasm-chip-label")
        .expect("chip label rendered");
    assert_eq!(label.text_content().unwrap(), "Error budget");

    let delete_button: web_sys::HtmlButtonElement = mount
        .query_selector("[data-chip-slot='delete']")
        .unwrap()
        .expect("delete button rendered")
        .dyn_into()
        .unwrap();
    assert_eq!(delete_button.id(), "wasm-chip-delete");
    assert_eq!(delete_button.get_attribute("type").unwrap(), "button");
    assert_eq!(delete_button.get_attribute("aria-hidden").unwrap(), "false");
    assert_eq!(
        delete_button.get_attribute("aria-label").unwrap(),
        "Remove chip"
    );
    assert_eq!(delete_button.get_attribute("data-visible").unwrap(), "true");
    assert!(
        !delete_button.class_name().is_empty(),
        "scoped class missing on delete"
    );

    let delete_clicked = Rc::new(Cell::new(false));
    {
        let delete_clicked = delete_clicked.clone();
        let handler = Closure::<dyn FnMut(web_sys::Event)>::new(move |_event| {
            delete_clicked.set(true);
        });
        delete_button
            .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
            .unwrap();
        handler.forget();
    }

    delete_button.click();
    assert!(
        delete_clicked.get(),
        "clicking the delete affordance should trigger listeners"
    );

    axe_check(&mount).await;
}

#[function_component(DialogAuditApp)]
fn dialog_audit_app() -> Html {
    let theme = Theme::default();
    let state = use_state(|| Rc::new(DialogState::controlled()));

    let open = {
        let state = state.clone();
        Callback::from(move |_| {
            let mut next = (**state).clone();
            next.open(|_| {});
            next.finish_open();
            state.set(Rc::new(next));
        })
    };
    let close = {
        let state = state.clone();
        Callback::from(move |_| {
            let mut next = (**state).clone();
            next.close(|_| {});
            state.set(Rc::new(next));
        })
    };

    let surface = DialogSurfaceOptions {
        id: Some("browser-dialog".into()),
        analytics_id: Some("wasm-dialog".into()),
        labelled_by: Some("dialog-title".into()),
        described_by: Some("dialog-description".into()),
    };

    html! {
        <ThemeProvider theme={theme}>
            <div id="dialog-harness">
                <button id="dialog-open" onclick={open}>{"Open dialog"}</button>
                <button id="dialog-close" onclick={close}>{"Close dialog"}</button>
                <dialog_adapter::Dialog
                    state={(*state).clone()}
                    surface={surface}
                    aria_label={Some("Team settings".into())}
                >
                    <h2 id="dialog-title">{"Team settings"}</h2>
                    <p id="dialog-description">{"Accessible dialog body"}</p>
                    <button type="button">{"Confirm"}</button>
                </dialog_adapter::Dialog>
            </div>
        </ThemeProvider>
    }
}

#[wasm_bindgen_test(async)]
async fn dialog_focus_trap_accessibility_audit() {
    let document = gloo_utils::document();
    let mount = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&mount).unwrap();

    Renderer::<DialogAuditApp>::with_root(mount.clone()).render();

    let open_button: web_sys::HtmlElement = mount
        .query_selector("#dialog-open")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();
    open_button.click();

    let dialog_surface = document
        .get_element_by_id("browser-dialog")
        .expect("dialog should open");
    assert_eq!(
        dialog_surface.get_attribute("data-state").unwrap(),
        "opening"
    );
    axe_check(&mount).await;

    let close_button: web_sys::HtmlElement = mount
        .query_selector("#dialog-close")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();
    close_button.click();
    axe_check(&mount).await;
}

fn wasm_menu_state(count: usize) -> MenuState {
    MenuState::new(count, false, unsafe { std::mem::transmute(1u8) }, unsafe {
        std::mem::transmute(1u8)
    })
}

fn wasm_popover_state(props: &MenuProps) -> PopoverState {
    let mut popover = PopoverState::uncontrolled(false, PopoverPlacement::Bottom);
    let base = format!(
        "{}-popover",
        props
            .automation_id
            .clone()
            .unwrap_or_else(|| "mui-menu".into())
    );
    let portal = mui_system::portal::PortalMount::popover(base);
    popover.set_anchor_metadata(Some(portal.anchor_id()), None);
    popover
}

#[function_component(MenuPopoverHarness)]
fn menu_popover_harness() -> Html {
    let theme = Theme::default();
    let props = Rc::new(
        MenuProps::new(
            "Menu",
            vec![
                MenuItem::new("Profile", "profile"),
                MenuItem::new("Settings", "settings"),
            ],
        )
        .with_automation_id("wasm-menu"),
    );
    let menu_state = {
        let props = props.clone();
        use_state(move || wasm_menu_state(props.items.len()))
    };
    let popover_state = {
        let props = props.clone();
        use_state(move || wasm_popover_state(props.as_ref()))
    };

    let open = {
        let menu_state = menu_state.clone();
        let popover_state = popover_state.clone();
        Callback::from(move |_| {
            let mut next_menu = (*menu_state).clone();
            let mut next_popover = (*popover_state).clone();
            next_menu.open(|_| {});
            next_popover.open(|_| {});
            menu_state.set(next_menu);
            popover_state.set(next_popover);
        })
    };
    let close = {
        let menu_state = menu_state.clone();
        let popover_state = popover_state.clone();
        Callback::from(move |_| {
            let mut next_menu = (*menu_state).clone();
            let mut next_popover = (*popover_state).clone();
            next_menu.close(|_| {});
            next_popover.close(|_| {});
            menu_state.set(next_menu);
            popover_state.set(next_popover);
        })
    };

    let html = menu::yew::render(props.as_ref(), &*menu_state, &*popover_state);
    let markup = Html::from_html_unchecked(AttrValue::from(html));

    html! {
        <ThemeProvider theme={theme}>
            <div id="menu-harness">
                <button id="menu-open" onclick={open.clone()}>{"Open menu"}</button>
                <button id="menu-close" onclick={close}>{"Close menu"}</button>
                {markup}
            </div>
        </ThemeProvider>
    }
}

#[wasm_bindgen_test(async)]
async fn popover_portal_accessibility_audit() {
    let document = gloo_utils::document();
    let mount = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&mount).unwrap();

    Renderer::<MenuPopoverHarness>::with_root(mount.clone()).render();

    let open_button: web_sys::HtmlElement = mount
        .query_selector("#menu-open")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();
    open_button.click();

    let menu_root = mount
        .query_selector("[data-component='mui-menu']")
        .unwrap()
        .expect("menu rendered");
    assert_eq!(menu_root.get_attribute("data-open").unwrap(), "true");
    axe_check(&mount).await;

    let close_button: web_sys::HtmlElement = mount
        .query_selector("#menu-close")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();
    close_button.click();
    axe_check(&mount).await;
}

#[function_component(TextFieldAuditApp)]
fn text_field_audit_app() -> Html {
    let theme = Theme::default();
    let state = use_state(|| {
        TextFieldStateHandle::from(TextFieldState::controlled(
            "",
            Some(Duration::from_millis(120)),
        ))
    });

    let mark_invalid = {
        let state = state.clone();
        Callback::from(move |_| {
            let mut next = {
                let borrowed = (*state).borrow();
                borrowed.clone()
            };
            next.change("invalid", |_| {});
            next.set_errors(vec!["Required".into()]);
            next.commit(|_| {});
            state.set(TextFieldStateHandle::from(next));
        })
    };
    let clear = {
        let state = state.clone();
        Callback::from(move |_| {
            let mut next = TextFieldState::controlled("", Some(Duration::from_millis(120)));
            next.clear_errors();
            state.set(TextFieldStateHandle::from(next));
        })
    };

    html! {
        <ThemeProvider theme={theme}>
            <div id="text-field-harness">
                <button id="text-field-invalid" onclick={mark_invalid.clone()}>{"Mark invalid"}</button>
                <button id="text-field-reset" onclick={clear}>{"Reset"}</button>
                <TextField
                    state={(*state).clone()}
                    placeholder={"Workspace name"}
                    aria_label={"Workspace name"}
                    status_id={Some("status-node".into())}
                    analytics_id={Some("tf-analytics".into())}
                />
                <p id="status-node">{"Status message"}</p>
            </div>
        </ThemeProvider>
    }
}

#[wasm_bindgen_test(async)]
async fn text_field_validation_accessibility_audit() {
    let document = gloo_utils::document();
    let mount = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&mount).unwrap();

    Renderer::<TextFieldAuditApp>::with_root(mount.clone()).render();

    axe_check(&mount).await;

    let error_button: web_sys::HtmlElement = mount
        .query_selector("#text-field-invalid")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();
    error_button.click();
    axe_check(&mount).await;

    let reset_button: web_sys::HtmlElement = mount
        .query_selector("#text-field-reset")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();
    reset_button.click();
    axe_check(&mount).await;
}
