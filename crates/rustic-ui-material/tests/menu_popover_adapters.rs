#![cfg(any(
    feature = "dioxus",
    feature = "leptos",
    feature = "sycamore",
    feature = "yew"
))]

use rustic_ui_headless::menu::MenuState;
use rustic_ui_headless::popover::{AnchorGeometry, PopoverPlacement, PopoverState};
use rustic_ui_material::menu::{self, MenuItem, MenuProps};
use rustic_ui_system::portal::PortalMount;

fn sample_props() -> MenuProps {
    MenuProps::new(
        "Actions",
        vec![
            MenuItem::new("Profile", "profile"),
            MenuItem::new("Logout", "logout"),
        ],
    )
    .with_automation_id("adapter-menu")
}

fn build_state(count: usize) -> MenuState {
    MenuState::new(count, false, unsafe { std::mem::transmute(1u8) }, unsafe {
        std::mem::transmute(1u8)
    })
}

fn build_popover(props: &MenuProps) -> PopoverState {
    let mut popover = PopoverState::uncontrolled(false, PopoverPlacement::Bottom);
    let base = props
        .automation_id
        .as_ref()
        .map(|id| format!("{}-popover", id))
        .unwrap_or_else(|| "mui-menu-popover".into());
    let portal = PortalMount::popover(base);
    popover.set_anchor_metadata(Some(portal.anchor_id()), None);
    popover
}

fn assert_popover_contract(html: &str, resolved: PopoverPlacement, outcome: &str) {
    assert!(html.contains("data-preferred-placement=\"bottom\""));
    assert!(html.contains(&format!(
        "data-resolved-placement=\"{}\"",
        resolved.as_str()
    )));
    assert!(html.contains(&format!("data-placement-outcome=\"{}\"", outcome)));
    assert!(html.contains("data-portal-layer=\"popover\""));
    assert!(html.contains("data-portal-root"));
    assert!(html.contains("data-portal-anchor"));
}

#[cfg(feature = "yew")]
#[test]
fn yew_menu_renders_popover_metadata() {
    let props = sample_props();
    let mut menu_state = build_state(props.items.len());
    menu_state.open(|_| {});
    let mut popover_state = build_popover(&props);
    popover_state.open(|_| {});
    let html = menu::yew::render(&props, &menu_state, &popover_state);
    assert!(html.contains("data-open=\"true\""));
    assert_popover_contract(&html, PopoverPlacement::Bottom, "preferred");
}

fn repositioned_popover_html<F>(render: F) -> String
where
    F: Fn(&MenuProps, &MenuState, &PopoverState) -> String,
{
    let props = sample_props();
    let menu_state = build_state(props.items.len());
    let mut popover_state = build_popover(&props);
    popover_state.set_anchor_metadata(
        popover_state.anchor_id().map(str::to_string),
        Some(AnchorGeometry {
            x: 0.0,
            y: 0.0,
            width: 120.0,
            height: 40.0,
        }),
    );
    popover_state.resolve_with(|_, _| PopoverPlacement::Top);
    render(&props, &menu_state, &popover_state)
}

#[cfg(feature = "leptos")]
#[test]
fn leptos_menu_repositions_surface_metadata() {
    let html = repositioned_popover_html(menu::leptos::render);
    assert_popover_contract(&html, PopoverPlacement::Top, "repositioned");
}

#[cfg(feature = "dioxus")]
#[test]
fn dioxus_menu_aligns_with_popover_state() {
    let html = repositioned_popover_html(menu::dioxus::render);
    assert_popover_contract(&html, PopoverPlacement::Top, "repositioned");
}

#[cfg(feature = "sycamore")]
#[test]
fn sycamore_menu_emits_resolved_attributes() {
    let html = repositioned_popover_html(menu::sycamore::render);
    assert_popover_contract(&html, PopoverPlacement::Top, "repositioned");
}

#[cfg(all(feature = "leptos", feature = "dioxus", feature = "sycamore"))]
#[test]
fn cross_framework_menu_markup_matches() {
    let props = sample_props();
    let menu_state = build_state(props.items.len());
    let popover_state = build_popover(&props);
    let leptos_html = menu::leptos::render(&props, &menu_state, &popover_state);
    let dioxus_html = menu::dioxus::render(&props, &menu_state, &popover_state);
    let sycamore_html = menu::sycamore::render(&props, &menu_state, &popover_state);
    assert_eq!(leptos_html, dioxus_html);
    assert_eq!(leptos_html, sycamore_html);
}
