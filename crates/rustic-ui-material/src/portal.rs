//! Material themed wrappers around [`rustic_ui_headless::portal`].
//!
//! Enterprise applications often need to render overlay surfaces across SSR and
//! CSR pipelines.  The headless [`PortalState`] already provides hydration and
//! transition bookkeeping; this module adds the Material theme specific CSS and
//! automation hooks so framework adapters can remain thin.  Each helper funnels
//! styling through [`crate::style_helpers`] to guarantee consistent class name
//! generation across WebAssembly and server rendered paths.
//!
//! The module intentionally exposes simple functions instead of bespoke types so
//! Yew, Leptos, Sycamore and Dioxus integrations can share the same building
//! blocks without carrying generic bounds.  Tests register the helpers through
//! integration harnesses which keeps selector contracts fully automated.

use rustic_ui_headless::portal::PortalState;
use rustic_ui_system::portal::PortalMount;

#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]
use rustic_ui_styled_engine::{css_with_theme, Style};

/// Creates a new [`PortalState`] and ensures the automation id follows the
/// Material naming contract.
#[must_use]
pub fn create_state(automation_id: Option<&str>, trap_focus: bool) -> PortalState {
    PortalState::new(automation_id.map(|id| id.to_string()), trap_focus)
}

/// Generates a [`PortalMount`] configured for popover style overlays.
#[must_use]
pub fn popover_mount(component: &str, automation_id: Option<&str>) -> PortalMount {
    PortalMount::popover(crate::style_helpers::automation_id(
        component,
        automation_id,
        crate::style_helpers::EMPTY_SEGMENTS,
    ))
}

/// Style applied to the hidden anchor element injected next to the trigger.
#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]
fn anchor_style() -> Style {
    css_with_theme!(
        r#"
        position: absolute;
        width: 0;
        height: 0;
        overflow: hidden;
        "#
    )
}

/// Returns attributes for the hidden anchor element.  The anchor keeps
/// automation hooks colocated with the trigger so QA pipelines can traverse the
/// overlay stack deterministically.
#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]
#[must_use]
pub fn themed_anchor_attributes(mount: &PortalMount) -> Vec<(String, String)> {
    let attrs = mount.anchor_attributes();
    crate::style_helpers::themed_attributes(anchor_style(), attrs)
}

/// Serialises the anchor attributes for SSR renderers.
#[cfg(any(
    feature = "yew",
    feature = "leptos",
    feature = "dioxus",
    feature = "sycamore"
))]
#[must_use]
pub fn themed_anchor_html(mount: &PortalMount) -> String {
    let attrs = mount.anchor_attributes();
    crate::style_helpers::themed_attributes_html(anchor_style(), attrs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn automation_id_is_namespaced() {
        let mount = popover_mount("menu", Some("primary"));
        assert!(mount
            .container_id()
            .starts_with(crate::style_helpers::COMPONENT_PREFIX));
    }

    #[test]
    fn themed_anchor_includes_class() {
        #[cfg(any(
            feature = "yew",
            feature = "leptos",
            feature = "dioxus",
            feature = "sycamore"
        ))]
        {
            let mount = popover_mount("tooltip", None);
            let attrs = themed_anchor_attributes(&mount);
            assert!(attrs.iter().any(|(k, _)| k == "class"));
        }
    }
}
