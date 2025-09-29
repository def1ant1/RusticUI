//! Yew adapter showcasing the focus trap sentinel utilities.
//!
//! The component mirrors the SSR snapshot emitted by
//! `utils_trap_focus_core::enterprise_story` so automation suites can diff the
//! server markup against the hydrated DOM without special casing attributes. We
//! intentionally keep the JSX-style tree verbose—enterprise teams often copy
//! this harness into monitoring dashboards where explicit IDs and automation
//! hooks are easier to audit.

use std::rc::Rc;

use rustic_ui_material::focus_trap::{
    FocusTrapSentinel, FocusTrapSentinelKind, FocusTrapSentinelProps,
};
use rustic_ui_styled_engine::ThemeProvider;
use utils_trap_focus_core::enterprise_story;
use yew::prelude::*;

/// Focus trap harness rendered by the Yew runtime.
#[function_component(TrapFocusHarness)]
pub fn trap_focus_harness() -> Html {
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
    let state = Rc::new(story.cloned_state());

    let start_props = FocusTrapSentinelProps {
        state: state.clone(),
        kind: FocusTrapSentinelKind::Start,
        options: sentinel_options.clone(),
        fallback_prefix: AttrValue::from(fallback_prefix.clone()),
    };
    let end_props = FocusTrapSentinelProps {
        state: state.clone(),
        kind: FocusTrapSentinelKind::End,
        options: sentinel_options,
        fallback_prefix: AttrValue::from(fallback_prefix),
    };

    html! {
        <ThemeProvider theme={theme}>
            <>
                <FocusTrapSentinel ..start_props />
                <section
                    id={container_id.clone()}
                    role="dialog"
                    aria-modal="true"
                    aria-labelledby={title_id.clone()}
                    aria-describedby={description_id.clone()}
                    data-automation-id={format!("{}::surface", automation_prefix)}
                    data-focus-trap="active"
                >
                    <header data-automation-id={format!("{}::header", automation_prefix)}>
                        <h2 id={title_id.clone()}>"Incident response"</h2>
                    </header>
                    <p
                        id={description_id.clone()}
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
                            id={dismiss_id.clone()}
                            type="button"
                            data-automation-id={format!("{}::action-dismiss", automation_prefix)}
                        >
                            {"Close incident"}
                        </button>
                        <button
                            id={primary_id.clone()}
                            type="button"
                            data-automation-id={format!("{}::action-escalate", automation_prefix)}
                        >
                            {"Escalate to secondary"}
                        </button>
                    </div>
                </section>
                <FocusTrapSentinel ..end_props />
            </>
        </ThemeProvider>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_focus_trap_surface() {
        let html = yew::ServerRenderer::<TrapFocusHarness>::new()
            .render()
            .unwrap();
        assert!(html.contains("data-automation-id=\"support-dialog::surface\""));
        assert!(html.contains("data-rustic-focus-trap=\"sentinel-start\""));
        assert!(html.contains("data-rustic-focus-trap=\"sentinel-end\""));
    }
}
