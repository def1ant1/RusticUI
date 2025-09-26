use mui_headless::popover::{AnchorGeometry, CollisionOutcome, PopoverPlacement, PopoverState};
use proptest::prelude::*;

fn placement_strategy() -> impl Strategy<Value = PopoverPlacement> {
    prop_oneof![
        Just(PopoverPlacement::Top),
        Just(PopoverPlacement::Bottom),
        Just(PopoverPlacement::Start),
        Just(PopoverPlacement::End),
        Just(PopoverPlacement::Center),
    ]
}

fn anchor_strategy() -> impl Strategy<Value = AnchorGeometry> {
    (
        -500.0f64..500.0,
        -500.0f64..500.0,
        1.0f64..300.0,
        1.0f64..300.0,
    )
        .prop_map(|(x, y, width, height)| AnchorGeometry {
            x,
            y,
            width,
            height,
        })
}

proptest! {
    /// Validates that the resolver receives the precise geometry snapshot and
    /// that any deviation from the preferred placement updates the collision
    /// outcome metadata consumed by analytics dashboards and styling hooks.
    #[test]
    fn collision_resolution_tracks_outcome(
        preferred in placement_strategy(),
        resolved in placement_strategy(),
        geometry in anchor_strategy(),
    ) {
        let mut state = PopoverState::uncontrolled(false, preferred);
        state.set_anchor_metadata(Some("anchor"), Some(geometry));
        let captured_geometry = std::cell::Cell::new(None);
        let captured_preference = std::cell::Cell::new(None);

        let observed = state.resolve_with(|incoming_geometry, incoming_preference| {
            captured_geometry.set(Some(incoming_geometry));
            captured_preference.set(Some(incoming_preference));
            resolved
        });

        prop_assert_eq!(captured_geometry.get(), Some(geometry));
        prop_assert_eq!(captured_preference.get(), Some(preferred));

        prop_assert_eq!(observed, resolved);
        prop_assert_eq!(state.resolved_placement(), resolved);
        let expected_outcome = if preferred == resolved {
            CollisionOutcome::Preferred
        } else {
            CollisionOutcome::Repositioned
        };
        prop_assert_eq!(state.last_outcome(), expected_outcome);

        let anchor_attrs = state.anchor_attributes();
        prop_assert_eq!(
            anchor_attrs.data_placement(),
            ("data-popover-placement", state.resolved_placement().as_str())
        );
    }
}

proptest! {
    /// When no geometry is provided the popover should retain the preferred
    /// placement and avoid calling the resolver.  This keeps SSR payloads
    /// deterministic even when anchors hydrate later on the client.
    #[test]
    fn resolver_short_circuits_without_geometry(preferred in placement_strategy()) {
        let mut state = PopoverState::uncontrolled(true, preferred);
        let invoked = std::cell::Cell::new(false);
        let resolved = state.resolve_with(|_, _| {
            invoked.set(true);
            preferred
        });

        prop_assert_eq!(resolved, preferred);
        prop_assert_eq!(state.last_outcome(), CollisionOutcome::Preferred);
        prop_assert_eq!(state.resolved_placement(), preferred);
        prop_assert!(!invoked.get(), "resolver should not run without geometry");
    }
}

proptest! {
    /// Both control strategies ultimately publish identical automation metadata.
    /// This property exercises random visibility toggles to guarantee open/close
    /// notifications stay in sync with the rendered attributes for analytics and
    /// hydration parity.
    #[test]
    fn control_modes_emit_consistent_open_states(
        default_open in any::<bool>(),
        toggles in prop::collection::vec(any::<bool>(), 1..32),
    ) {
        let mut uncontrolled = PopoverState::uncontrolled(default_open, PopoverPlacement::Bottom);
        let mut controlled = PopoverState::controlled(PopoverPlacement::Bottom);

        for desired in toggles {
            let was_controlled_open = controlled.is_open();
            if desired {
                uncontrolled.open(|_| {});
            } else {
                uncontrolled.close(|_| {});
            }
            prop_assert_eq!(uncontrolled.is_open(), desired);

            let mut observed = None;
            if desired {
                controlled.open(|flag| observed = Some(flag));
            } else {
                controlled.close(|flag| observed = Some(flag));
            }
            if desired != was_controlled_open {
                prop_assert_eq!(observed, Some(desired));
            } else {
                prop_assert_eq!(observed, None);
            }
            controlled.sync_open(desired);
            prop_assert_eq!(controlled.is_open(), desired);

            let surface = uncontrolled.surface_attributes();
            prop_assert_eq!(surface.data_open(), ("data-open", if desired { "true" } else { "false" }));
        }
    }
}
