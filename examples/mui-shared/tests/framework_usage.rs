use mui_shared::{
    layout::{automation_for_framework, hydration_marker, AppShell, Framework},
    routes::{ABOUT, HOME},
    theme::material_example_theme,
};

#[test]
fn leptos_hydration_marker_is_deterministic() {
    let marker = hydration_marker(&HOME, Framework::Leptos);
    assert_eq!(
        marker,
        "data-rustic-app-hydration-root=\"app-home-framework-leptos-hydration-root\""
    );
}

#[test]
fn yew_hydration_marker_is_deterministic() {
    let marker = hydration_marker(&ABOUT, Framework::Yew);
    assert_eq!(
        marker,
        "data-rustic-app-hydration-root=\"app-about-framework-yew-hydration-root\""
    );
}

#[test]
fn dioxus_hydration_marker_is_deterministic() {
    let builder = automation_for_framework(&HOME, Framework::Dioxus);
    assert_eq!(builder.value(), "app-home-framework-dioxus");
}

#[test]
fn sycamore_ssr_shell_contains_theme_metadata() {
    let shell = AppShell::for_route(&ABOUT);
    let theme = material_example_theme();
    let html = shell.render_ssr_document(|content| content, &theme);
    assert!(html.contains("data-theme=\"light\""));
    assert!(html.contains("data-pro-tip-lead=\"Need more patterns?\""));
}
