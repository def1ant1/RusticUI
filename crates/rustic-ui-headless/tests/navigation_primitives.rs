use rustic_ui_headless::bottom_navigation::{
    BottomNavigationActivationMode, BottomNavigationState,
};
use rustic_ui_headless::breadcrumbs::BreadcrumbsState;
use rustic_ui_headless::interaction::ControlKey;
use rustic_ui_headless::link::LinkState;
use rustic_ui_headless::pagination::{PaginationItemKind, PaginationState};
use rustic_ui_headless::speed_dial::SpeedDialState;
use rustic_ui_headless::ControlStrategy;

#[test]
fn bottom_navigation_emits_selection_analytics() {
    let mut state = BottomNavigationState::new(
        3,
        Some(0),
        BottomNavigationActivationMode::Automatic,
        ControlStrategy::Uncontrolled,
        ControlStrategy::Uncontrolled,
    );
    state.set_analytics_channel(Some("nav"));
    state.set_item_analytics_tag(1, Some("files"));

    let mut seen = Vec::new();
    let outcome = state.on_key(ControlKey::ArrowRight, |selection| {
        seen.push(selection.index)
    });

    assert_eq!(outcome.selected, Some(1));
    assert_eq!(seen, vec![1]);
    assert_eq!(
        outcome
            .analytics
            .as_ref()
            .and_then(|event| event.item_tag.as_deref()),
        Some("files")
    );
}

#[test]
fn breadcrumbs_activation_reports_analytics() {
    let mut state = BreadcrumbsState::new(
        3,
        Some(0),
        ControlStrategy::Uncontrolled,
        ControlStrategy::Uncontrolled,
    );
    state.set_analytics_channel(Some("breadcrumbs"));
    state.set_item_analytics_tag(2, Some("current"));

    let mut activated = None;
    let outcome = state.on_key(ControlKey::End, |activation| {
        activated = Some(activation.index)
    });
    assert_eq!(outcome.focused, Some(2));
    assert_eq!(activated, None);

    let mut analytics_index = None;
    state.activate(2, |activation| {
        analytics_index = activation.analytics.as_ref().map(|event| event.index);
    });
    assert_eq!(analytics_index, Some(2));
}

#[test]
fn link_state_handles_keyboard_activation() {
    let mut state = LinkState::new(true);
    state.set_analytics_channel(Some("links"));
    state.set_analytics_tag(Some("primary"));

    let outcome = state.on_key(ControlKey::Space);
    assert!(outcome.activate);
    assert_eq!(
        outcome
            .analytics
            .as_ref()
            .and_then(|event| event.link_tag.as_deref()),
        Some("primary")
    );
}

#[test]
fn pagination_activation_tracks_selected_page() {
    let mut state = PaginationState::new(
        5,
        Some(1),
        ControlStrategy::Uncontrolled,
        ControlStrategy::Uncontrolled,
    );
    state.set_analytics_channel(Some("pagination"));
    state.set_page_analytics_tag(3, Some("reports"));

    let mut selected = None;
    let result = state.activate(PaginationItemKind::Page(3), |selection| {
        selected = Some(selection.page_index);
    });

    assert_eq!(selected, Some(3));
    assert_eq!(result.selected_page, Some(3));
    assert_eq!(
        result
            .analytics
            .as_ref()
            .and_then(|event| event.page_tag.as_deref()),
        Some("reports")
    );
}

#[test]
fn speed_dial_toggle_and_activate_emit_events() {
    let mut state = SpeedDialState::new(
        2,
        false,
        ControlStrategy::Uncontrolled,
        ControlStrategy::Uncontrolled,
    );
    state.set_analytics_channel(Some("speed-dial"));
    state.set_action_analytics_tag(0, Some("new"));

    let opened = state.toggle(|open| assert!(open));
    assert!(opened.is_some());

    let mut activated = None;
    let outcome = state.activate(0, |selection| activated = selection.analytics.clone());
    assert_eq!(outcome.activated, Some(0));
    assert_eq!(
        activated.as_ref().map(|event| match &event.kind {
            rustic_ui_headless::speed_dial::SpeedDialAnalyticsKind::Action { tag, .. } => {
                tag.clone()
            }
            _ => None,
        }),
        Some(Some("new".into()))
    );
}
