use mui_system::themed_element::{ThemedProps, Variant};

#[cfg(feature = "leptos")]
#[test]
fn leptos_adapter_renders() {
    let props = ThemedProps {
        child: "hi".into(),
        variant: Variant::Plain,
        role: Some("note".into()),
        ..Default::default()
    };
    let html = mui_system::themed_element::leptos::render(&props);
    assert!(html.contains("<style>"), "expected inlined stylesheet");
    assert!(html.contains("mui-themed-header--plain"));
    assert!(html.contains("role=\"note\""));
    assert!(html.contains("<div"));
    assert!(html.contains("class=\""));
    assert!(
        !html.contains("style=\""),
        "inline styles should be replaced with generated classes"
    );
}

#[cfg(feature = "dioxus")]
#[test]
fn dioxus_adapter_renders() {
    let mut props = ThemedProps {
        child: "hi".into(),
        variant: Variant::Outlined,
        ..Default::default()
    };
    props.role = Some("button".into());
    props.aria_label = Some("greet".into());
    let html = mui_system::themed_element::dioxus::render(&props);
    assert!(html.contains("mui-themed-header--outlined"));
    assert!(html.contains("<style>"));
    assert!(html.contains("class=\""));
    assert!(
        !html.contains("style=\""),
        "inline styles should be replaced with generated classes"
    );
    assert!(html.contains("aria-label=\"greet\""));
}

#[cfg(feature = "sycamore")]
#[test]
fn sycamore_adapter_renders() {
    let props = ThemedProps {
        child: "hi".into(),
        variant: Variant::Plain,
        role: Some("note".into()),
        ..Default::default()
    };
    let html = mui_system::themed_element::sycamore::render(&props);
    assert!(html.contains("role=\"note\""));
    assert!(html.contains("mui-themed-header"));
    assert!(html.contains("<style>"));
    assert!(
        !html.contains("style=\""),
        "inline styles should be replaced with generated classes"
    );
}
