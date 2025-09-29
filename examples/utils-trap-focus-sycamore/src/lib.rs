//! Sycamore harness for the shared rustic-ui focus trap utilities.

use rustic_ui_material::focus_trap::sycamore as focus_trap_sycamore;
use rustic_ui_material::focus_trap::{FocusTrapSentinelKind, FocusTrapSentinelOptions};
use sycamore::prelude::*;
use utils_trap_focus_core::enterprise_story;

/// Assemble the focus trap markup using the shared story metadata.
pub fn trap_focus_markup() -> String {
    let story = enterprise_story();
    let options: FocusTrapSentinelOptions = story.sentinel_options.clone();
    let fallback_prefix = story.fallback_prefix.clone();
    let start = focus_trap_sycamore::render(&focus_trap_sycamore::FocusTrapSentinelProps {
        state: story.cloned_state(),
        kind: FocusTrapSentinelKind::Start,
        options: options.clone(),
        fallback_prefix: fallback_prefix.clone(),
    });
    let end = focus_trap_sycamore::render(&focus_trap_sycamore::FocusTrapSentinelProps {
        state: story.cloned_state(),
        kind: FocusTrapSentinelKind::End,
        options,
        fallback_prefix,
    });
    let body = format!(
        concat!(
            "<section id=\"{container}\" role=\"dialog\" aria-modal=\"true\" ",
            "aria-labelledby=\"{title}\" aria-describedby=\"{description}\" ",
            "data-automation-id=\"{prefix}::surface\" data-focus-trap=\"active\">\n",
            "  <header data-automation-id=\"{prefix}::header\">\n",
            "    <h2 id=\"{title}\">Incident response</h2>\n",
            "  </header>\n",
            "  <p id=\"{description}\" data-automation-id=\"{prefix}::body-copy\">",
            "Keyboard focus remains inside this container until operators resolve or dismiss the incident.",
            "</p>\n",
            "  <div role=\"group\" aria-label=\"Incident actions\" data-automation-id=\"{prefix}::actions\">\n",
            "    <button id=\"{dismiss}\" data-automation-id=\"{prefix}::action-dismiss\" type=\"button\">Close incident</button>\n",
            "    <button id=\"{primary}\" data-automation-id=\"{prefix}::action-escalate\" type=\"button\">Escalate to secondary</button>\n",
            "  </div>\n",
            "</section>\n"
        ),
        container = story.container_id,
        title = story.title_id,
        description = story.description_id,
        prefix = story.automation_prefix,
        dismiss = story.dismiss_button_id,
        primary = story.primary_button_id,
    );

    format!("{start}{body}{end}")
}

/// Hydration harness rendered by Sycamore.
#[component]
pub fn TrapFocusHarness<G: Html>(cx: Scope) -> View<G> {
    let markup = trap_focus_markup();
    view! { cx,
        div(dangerously_set_inner_html = markup)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markup_contains_sentinels() {
        let html = trap_focus_markup();
        assert!(html.contains("data-automation-id=\"support-dialog::surface\""));
        assert!(html.contains("data-rustic-focus-trap=\"sentinel-start\""));
        assert!(html.contains("data-rustic-focus-trap=\"sentinel-end\""));
    }
}
