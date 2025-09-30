use rustic_ui_headless::input_base::{
    InputAnalyticsEventKind, InputControlBuilder, InputSelection, InputState,
};

fn extract_attr(attrs: &[(&str, String)], key: &str) -> Option<String> {
    attrs
        .iter()
        .find(|(k, _)| *k == key)
        .map(|(_, v)| v.clone())
}

#[test]
fn uncontrolled_state_tracks_selection_and_analytics() {
    let mut state = InputState::uncontrolled("hello", Some(InputSelection::new(0, 5)));
    let change = state.change("world", Some(InputSelection::collapsed(5)));
    assert_eq!(change.value, "world");
    assert!(change.dirty);
    assert_eq!(change.selection.unwrap().start, 5);
    assert_eq!(change.analytics.len(), 1);
    assert_eq!(
        change.analytics[0].kind,
        InputAnalyticsEventKind::ValueChange
    );
    assert_eq!(change.analytics[0].detail.as_deref(), Some("world"));
    drop(change);
    assert!(state.dirty());
}

#[test]
fn controlled_state_requires_sync() {
    let mut state = InputState::controlled("seed", None);
    let first_commit = state.commit();
    assert!(!first_commit.previously_visited);
    assert_eq!(
        first_commit.analytics[0].kind,
        InputAnalyticsEventKind::Commit
    );
    let change = state.change("next", None);
    assert!(change.dirty);
    assert_eq!(state.value(), "next");
    state.sync_controlled_value("next");
    assert!(state.dirty());
    let commit = state.commit();
    assert!(commit.previously_visited);
    assert_eq!(commit.value, "next");
}

#[test]
fn validation_and_focus_analytics_are_drained() {
    let mut state = InputState::uncontrolled("value", None);
    state.set_errors(["required"]);
    let validation = state.drain_analytics();
    assert_eq!(validation.len(), 1);
    assert_eq!(validation[0].kind, InputAnalyticsEventKind::Validation);
    assert_eq!(validation[0].detail.as_deref(), Some("1"));
    let gained = state.set_focused(true);
    assert_eq!(gained[0].kind, InputAnalyticsEventKind::FocusGained);
    let lost = state.set_focused(false);
    assert_eq!(lost[0].kind, InputAnalyticsEventKind::FocusLost);
}

#[test]
fn reset_clears_errors_and_logs() {
    let mut state = InputState::uncontrolled("baseline", None);
    state.set_errors(vec!["boom".to_string()]);
    state.change("mutated", None);
    let reset = state.reset();
    assert!(reset.cleared_errors);
    assert_eq!(reset.analytics[0].kind, InputAnalyticsEventKind::Reset);
    assert_eq!(reset.value, "baseline");
    assert!(!state.dirty());
    assert!(state.errors().is_empty());
}

#[test]
fn builder_produces_form_control_bundle() {
    let mut bundle = InputControlBuilder::new("seed")
        .controlled()
        .selection(Some(InputSelection::new(0, 4)))
        .described_by(["hint", "hint", "error"])
        .automation_id("input.email")
        .id("email")
        .required(true)
        .build();

    assert_eq!(
        bundle.input.control_strategy(),
        rustic_ui_headless::ControlStrategy::Controlled
    );
    assert_eq!(bundle.form_control.controlled_value(), Some("seed"));
    let attrs = bundle.form_control.aria_attributes();
    let described = extract_attr(&attrs, "aria-describedby").unwrap();
    // Sorting places "error" before "hint" and we ensure duplicates are removed.
    assert_eq!(described, "error hint");
    let change = bundle.form_control.set_uncontrolled_value("noop");
    assert!(!change.dirty);
    let sync = bundle.form_control.sync_controlled_value("updated");
    assert!(sync.dirty);
    assert_eq!(bundle.input.selection().unwrap().end, 4);
}

#[test]
fn builder_respects_mode_switching() {
    let mut bundle = InputControlBuilder::new("value").uncontrolled().build();
    assert_eq!(bundle.form_control.controlled_value(), None);
    // Set value via form control to ensure uncontrolled path mutates directly.
    let change = bundle.form_control.set_uncontrolled_value("changed");
    assert!(change.dirty);
    assert_eq!(bundle.form_control.value(), "changed");
    assert_eq!(
        bundle.input.control_strategy(),
        rustic_ui_headless::ControlStrategy::Uncontrolled
    );
}

#[test]
fn silent_mutations_keep_analytics_clean() {
    let mut state = InputState::controlled("baseline", None);
    state.set_value_silently("override");
    state.set_initial_value("override");
    state.set_visited(true);
    state.clear_errors();
    assert_eq!(state.value(), "override");
    assert!(!state.dirty());
    assert!(state.drain_analytics().is_empty());
    state.set_visited(false);
    assert!(!state.visited());
}
