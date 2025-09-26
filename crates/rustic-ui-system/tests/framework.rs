use rustic_ui_system::themed_element::{ThemedProps, Variant};

#[cfg(feature = "leptos")]
#[test]
fn leptos_adapter_renders() {
    let props = ThemedProps {
        value: "hi".into(),
        placeholder: Some("type".into()),
        variant: Variant::Plain,
        aria_label: Some("note".into()),
        ..Default::default()
    };
    let html = rustic_ui_system::themed_element::leptos::render(&props);
    assert!(html.contains("<style>"), "expected inlined stylesheet");
    assert!(html.contains("rustic_ui_themed_input__plain"));
    assert!(html.contains("aria-label=\"note\""));
    assert!(html.contains("<input"));
    assert!(html.contains("placeholder=\"type\""));
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
        value: "hi".into(),
        variant: Variant::Outlined,
        ..Default::default()
    };
    props.aria_label = Some("greet".into());
    props.debounce_ms = Some(200);
    let html = rustic_ui_system::themed_element::dioxus::render(&props);
    assert!(html.contains("rustic_ui_themed_input__outlined"));
    assert!(html.contains("<input"));
    assert!(html.contains("<style>"));
    assert!(html.contains("class=\""));
    assert!(
        !html.contains("style=\""),
        "inline styles should be replaced with generated classes"
    );
    assert!(html.contains("aria-label=\"greet\""));
    assert!(html.contains("data-debounce-ms=\"200\""));
}

#[cfg(feature = "sycamore")]
#[test]
fn sycamore_adapter_renders() {
    let props = ThemedProps {
        value: "hi".into(),
        placeholder: Some("search".into()),
        variant: Variant::Plain,
        aria_label: Some("global search".into()),
        ..Default::default()
    };
    let html = rustic_ui_system::themed_element::sycamore::render(&props);
    assert!(html.contains("aria-label=\"global search\""));
    assert!(html.contains("rustic_ui_themed_input"));
    assert!(html.contains("<input"));
    assert!(html.contains("placeholder=\"search\""));
    assert!(html.contains("<style>"));
    assert!(
        !html.contains("style=\""),
        "inline styles should be replaced with generated classes"
    );
}
