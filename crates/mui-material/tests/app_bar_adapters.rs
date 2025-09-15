#![cfg(all(feature = "dioxus", feature = "sycamore"))]

use mui_material::app_bar::{dioxus, sycamore};

#[test]
fn dioxus_and_sycamore_render_header() {
    let props_dx = dioxus::AppBarProps {
        title: "Dashboard".into(),
        aria_label: "Application header".into(),
        color: mui_material::app_bar::AppBarColor::Primary,
        size: mui_material::app_bar::AppBarSize::Medium,
    };
    let dx = dioxus::render(&props_dx);
    assert!(dx.starts_with("<header"));
    assert!(dx.contains("aria-label=\"Application header\""));

    let props_sy = sycamore::AppBarProps {
        title: "Dashboard".into(),
        aria_label: "Application header".into(),
        color: mui_material::app_bar::AppBarColor::Primary,
        size: mui_material::app_bar::AppBarSize::Medium,
    };
    let sy = sycamore::render(&props_sy);
    assert!(sy.starts_with("<header"));
    assert!(sy.contains("aria-label=\"Application header\""));
}
