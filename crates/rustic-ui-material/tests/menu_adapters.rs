use rustic_ui_headless::menu::MenuState;
use rustic_ui_material::menu::{self, MenuItem, MenuProps};

fn sample_props() -> MenuProps {
    MenuProps::new(
        "Menu",
        vec![
            MenuItem::new("Profile", "profile"),
            MenuItem::new("Logout", "logout"),
        ],
    )
    .with_automation_id("adapter-menu")
}

fn build_state(count: usize) -> MenuState {
    MenuState::new(count, true, unsafe { std::mem::transmute(1u8) }, unsafe {
        std::mem::transmute(1u8)
    })
}

fn assert_portal_markup(html: &str) {
    assert!(html.contains("data-portal-root=\"adapter-menu-popover\""));
    assert!(html.contains("adapter-menu-popover-anchor"));
    assert_eq!(
        html.matches("<ul").count(),
        1,
        "menu surface should only render once"
    );
    assert!(
        !html.contains("aria-disabled=\"false\""),
        "enabled menu items should omit aria-disabled"
    );
    assert!(
        !html.contains("data-disabled=\"false\""),
        "enabled menu items should omit data-disabled"
    );
}

#[cfg(feature = "yew")]
mod yew_tests {
    use super::*;

    #[test]
    fn yew_render_emits_portal_metadata() {
        let props = sample_props();
        let state = build_state(props.items.len());
        let html = menu::yew::render(&props, &state);
        assert!(html.contains("data-automation-id=\"adapter-menu\""));
        assert_portal_markup(&html);
    }

    #[test]
    fn yew_render_marks_disabled_items() {
        let props = sample_props();
        let mut state = build_state(props.items.len());
        state.set_item_disabled(1, true);
        let html = menu::yew::render(&props, &state);
        assert!(html.contains("aria-disabled=\"true\""));
        assert!(html.contains("data-disabled=\"true\""));
    }
}

#[cfg(feature = "leptos")]
mod leptos_tests {
    use super::*;

    #[test]
    fn leptos_render_emits_portal_metadata() {
        let props = sample_props();
        let state = build_state(props.items.len());
        let html = menu::leptos::render(&props, &state);
        assert!(html.contains("data-automation-id=\"adapter-menu\""));
        assert_portal_markup(&html);
    }

    #[test]
    fn leptos_render_marks_disabled_items() {
        let props = sample_props();
        let mut state = build_state(props.items.len());
        state.set_item_disabled(1, true);
        let html = menu::leptos::render(&props, &state);
        assert!(html.contains("aria-disabled=\"true\""));
        assert!(html.contains("data-disabled=\"true\""));
    }
}

#[cfg(feature = "dioxus")]
mod dioxus_tests {
    use super::*;

    #[test]
    fn dioxus_render_emits_portal_metadata() {
        let props = sample_props();
        let state = build_state(props.items.len());
        let html = menu::dioxus::render(&props, &state);
        assert!(html.contains("data-automation-id=\"adapter-menu\""));
        assert_portal_markup(&html);
    }

    #[test]
    fn dioxus_render_marks_disabled_items() {
        let props = sample_props();
        let mut state = build_state(props.items.len());
        state.set_item_disabled(1, true);
        let html = menu::dioxus::render(&props, &state);
        assert!(html.contains("aria-disabled=\"true\""));
        assert!(html.contains("data-disabled=\"true\""));
    }
}

#[cfg(feature = "sycamore")]
mod sycamore_tests {
    use super::*;

    #[test]
    fn sycamore_render_emits_portal_metadata() {
        let props = sample_props();
        let state = build_state(props.items.len());
        let html = menu::sycamore::render(&props, &state);
        assert!(html.contains("data-component=\"rustic_ui_menu\""));
        assert_portal_markup(&html);
    }

    #[test]
    fn sycamore_render_marks_disabled_items() {
        let props = sample_props();
        let mut state = build_state(props.items.len());
        state.set_item_disabled(1, true);
        let html = menu::sycamore::render(&props, &state);
        assert!(html.contains("aria-disabled=\"true\""));
        assert!(html.contains("data-disabled=\"true\""));
    }
}
