//! Leptos harness demonstrating the reusable focus trap sentinels.
//!
//! The component mirrors the markup emitted by `utils_trap_focus_core` so SSR
//! pipelines and hydrated DOM trees expose identical automation hooks.

use std::sync::Arc;

use leptos::prelude::*;
use rustic_ui_material::focus_trap::{FocusTrapSentinel, FocusTrapSentinelKind};
use rustic_ui_styled_engine::ThemeProvider;
use utils_trap_focus_core::enterprise_story;

/// Hydration-ready focus trap surface rendered with Leptos.
#[component]
pub fn TrapFocusHarness() -> impl IntoView {
    let story = enterprise_story();
    let theme = story.theme.clone();
    let sentinel_options = story.sentinel_options.clone();
    let fallback_prefix = story.fallback_prefix.clone();
    let automation_prefix = story.automation_prefix.clone();
    let container_id = story.container_id.clone();
    let title_id = story.title_id.clone();
    let description_id = story.description_id.clone();
    let dismiss_id = story.dismiss_button_id.clone();
    let primary_id = story.primary_button_id.clone();
    let state = Arc::new(story.cloned_state());

    view! {
        <ThemeProvider theme=theme>
            <FocusTrapSentinel
                state=state.clone()
                kind=FocusTrapSentinelKind::Start
                options=sentinel_options.clone()
                fallback_prefix=fallback_prefix.clone()
            />
            <section
                id=container_id.clone()
                role="dialog"
                aria-modal="true"
                aria-labelledby=title_id.clone()
                aria-describedby=description_id.clone()
                data-automation-id={format!("{}::surface", automation_prefix)}
                data-focus-trap="active"
            >
                <header data-automation-id={format!("{}::header", automation_prefix)}>
                    <h2 id=title_id.clone()>{"Incident response"}</h2>
                </header>
                <p
                    id=description_id.clone()
                    data-automation-id={format!("{}::body-copy", automation_prefix)}
                >
                    {"Keyboard focus remains inside this container until operators resolve or dismiss the incident."}
                </p>
                <div
                    role="group"
                    aria-label="Incident actions"
                    data-automation-id={format!("{}::actions", automation_prefix)}
                >
                    <button
                        id=dismiss_id.clone()
                        r#type="button"
                        data-automation-id={format!("{}::action-dismiss", automation_prefix)}
                    >
                        {"Close incident"}
                    </button>
                    <button
                        id=primary_id.clone()
                        r#type="button"
                        data-automation-id={format!("{}::action-escalate", automation_prefix)}
                    >
                        {"Escalate to secondary"}
                    </button>
                </div>
            </section>
            <FocusTrapSentinel
                state=state
                kind=FocusTrapSentinelKind::End
                options=sentinel_options
                fallback_prefix=fallback_prefix
            />
        </ThemeProvider>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ssr_snapshot_contains_sentinels() {
        use leptos::ssr::render_to_string;

        let html = render_to_string(|| view! { <TrapFocusHarness /> }).into_owned();
        assert!(html.contains("data-automation-id=\"support-dialog::surface\""));
        assert!(html.contains("data-rustic-focus-trap=\"sentinel-start\""));
        assert!(html.contains("data-rustic-focus-trap=\"sentinel-end\""));
    }
}
