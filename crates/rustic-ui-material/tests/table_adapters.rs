#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore",
))]
use rustic_ui_headless::list::{ListState, SelectionMode};

#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore",
))]
use rustic_ui_material::table::{self, TableColumn, TableProps, TableRow};

#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore",
))]
fn sample_props() -> TableProps {
    TableProps::new(
        vec![
            TableColumn::new("Name"),
            TableColumn::new("Active Users").numeric(),
        ],
        vec![
            TableRow::new(vec!["Regions".into(), "128".into()]),
            TableRow::new(vec!["Tenants".into(), "32".into()]),
        ],
    )
    .with_selection_mode(SelectionMode::Single)
    .with_automation_id("adapter-table")
}

#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore",
))]
fn build_state(len: usize) -> ListState {
    ListState::uncontrolled(len, &[], SelectionMode::Single)
}

#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore",
))]
fn assert_markup(html: &str) {
    assert!(html.contains("data-component=\"rustic-table\""));
    assert!(html.contains("data-rustic-table-id=\"rustic-table-adapter-table\"",));
    assert!(html.contains("data-rustic-table-cell=\"rustic-table-adapter-table-cell-0-0\"",));
    assert!(html.contains("role=\"grid\""));
}

#[cfg(feature = "yew")]
mod yew_tests {
    use super::*;

    #[test]
    fn yew_render_emits_expected_markup() {
        let props = sample_props();
        let state = build_state(props.rows.len());
        let html = table::yew::render(&props, &state);
        assert_markup(&html);
    }
}

#[cfg(feature = "leptos")]
mod leptos_tests {
    use super::*;

    #[test]
    fn leptos_render_emits_expected_markup() {
        let props = sample_props();
        let state = build_state(props.rows.len());
        let html = table::leptos::render(&props, &state);
        assert_markup(&html);
    }
}

#[cfg(feature = "dioxus")]
mod dioxus_tests {
    use super::*;

    #[test]
    fn dioxus_render_emits_expected_markup() {
        let props = sample_props();
        let state = build_state(props.rows.len());
        let html = table::dioxus::render(&props, &state);
        assert_markup(&html);
    }
}

#[cfg(feature = "sycamore")]
mod sycamore_tests {
    use super::*;

    #[test]
    fn sycamore_render_emits_expected_markup() {
        let props = sample_props();
        let state = build_state(props.rows.len());
        let html = table::sycamore::render(&props, &state);
        assert_markup(&html);
    }
}
