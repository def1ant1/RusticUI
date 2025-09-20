//! State management for drawers and side sheets.
//!
//! Drawers share behaviour with disclosure widgets such as menus but introduce
//! additional metadata (anchor and variant) that adapters often need when
//! orchestrating layout.  Centralizing the logic keeps behaviour consistent
//! across frameworks and unlocks automation for future variants.

use crate::aria;
use crate::selection::ControlStrategy;

/// Describes whether the drawer behaves like a modal surface or a persistent
/// side sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawerVariant {
    /// Modal drawers block interaction with background content and require
    /// focus management.
    Modal,
    /// Persistent drawers remain visible without blocking background
    /// interaction.  They typically anchor navigation affordances.
    Persistent,
}

impl DrawerVariant {
    #[inline]
    fn is_modal(self) -> bool {
        matches!(self, Self::Modal)
    }
}

/// Represents where the drawer originates from on screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawerAnchor {
    /// Drawer slides from the leading edge (left in LTR, right in RTL).
    Start,
    /// Drawer slides from the trailing edge (right in LTR, left in RTL).
    End,
    /// Drawer drops from the top edge.
    Top,
    /// Drawer raises from the bottom edge.
    Bottom,
}

impl DrawerAnchor {
    /// Returns a string representation that adapters can map to CSS classes.
    #[inline]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::End => "end",
            Self::Top => "top",
            Self::Bottom => "bottom",
        }
    }
}

/// High level drawer state machine.
#[derive(Debug, Clone)]
pub struct DrawerState {
    open: bool,
    control_mode: ControlStrategy,
    variant: DrawerVariant,
    anchor: DrawerAnchor,
}

impl DrawerState {
    /// Create a new drawer state machine.
    pub fn new(
        default_open: bool,
        control_mode: ControlStrategy,
        variant: DrawerVariant,
        anchor: DrawerAnchor,
    ) -> Self {
        Self {
            open: if control_mode.is_controlled() {
                false
            } else {
                default_open
            },
            control_mode,
            variant,
            anchor,
        }
    }

    /// Returns whether the drawer is currently visible.
    #[inline]
    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Returns the configured variant.
    #[inline]
    pub fn variant(&self) -> DrawerVariant {
        self.variant
    }

    /// Returns the configured anchor.
    #[inline]
    pub fn anchor(&self) -> DrawerAnchor {
        self.anchor
    }

    /// Request the drawer to open.
    pub fn open<F: FnOnce(bool)>(&mut self, notify: F) {
        self.set_open(true, notify);
    }

    /// Request the drawer to close.
    pub fn close<F: FnOnce(bool)>(&mut self, notify: F) {
        self.set_open(false, notify);
    }

    /// Toggle the drawer state.
    pub fn toggle<F: FnOnce(bool)>(&mut self, notify: F) {
        self.set_open(!self.open, notify);
    }

    /// Synchronize the open flag when controlled externally.
    pub fn sync_open(&mut self, open: bool) {
        self.open = open;
    }

    /// Returns a builder for drawer surface attributes.
    #[inline]
    pub fn surface_attributes(&self) -> DrawerSurfaceAttributes<'_> {
        DrawerSurfaceAttributes::new(self)
    }

    /// Returns a builder for drawer backdrop attributes.
    #[inline]
    pub fn backdrop_attributes(&self) -> DrawerBackdropAttributes<'_> {
        DrawerBackdropAttributes::new(self)
    }

    fn set_open<F: FnOnce(bool)>(&mut self, next: bool, notify: F) {
        if !self.control_mode.is_controlled() {
            self.open = next;
        }
        notify(next);
    }
}

/// Builder for drawer surface attributes.  The builder exposes ARIA metadata so
/// adapters can keep markup declarative and ergonomic.
#[derive(Debug, Clone)]
pub struct DrawerSurfaceAttributes<'a> {
    state: &'a DrawerState,
    id: Option<&'a str>,
    labelled_by: Option<&'a str>,
    described_by: Option<&'a str>,
}

impl<'a> DrawerSurfaceAttributes<'a> {
    #[inline]
    fn new(state: &'a DrawerState) -> Self {
        Self {
            state,
            id: None,
            labelled_by: None,
            described_by: None,
        }
    }

    /// Attach an `id` attribute to the drawer element.
    #[inline]
    pub fn id(mut self, value: &'a str) -> Self {
        self.id = Some(value);
        self
    }

    /// Configure the labelling element for the drawer surface.
    #[inline]
    pub fn labelled_by(mut self, value: &'a str) -> Self {
        self.labelled_by = Some(value);
        self
    }

    /// Configure an element that describes the drawer contents.
    #[inline]
    pub fn described_by(mut self, value: &'a str) -> Self {
        self.described_by = Some(value);
        self
    }

    /// Returns the `role` for the drawer surface.  Modal drawers use the dialog
    /// role to communicate modality while persistent drawers expose the same
    /// role for consistent semantics.
    #[inline]
    pub fn role(&self) -> &'static str {
        aria::role_dialog()
    }

    /// Returns the `aria-modal` tuple describing whether focus should remain
    /// trapped inside the drawer.
    #[inline]
    pub fn aria_modal(&self) -> (&'static str, &'static str) {
        aria::aria_modal(self.state.variant.is_modal())
    }

    /// Returns the `aria-labelledby` tuple when configured.
    #[inline]
    pub fn aria_labelledby(&self) -> Option<(&'static str, &str)> {
        self.labelled_by.map(aria::aria_labelledby)
    }

    /// Returns the `aria-describedby` tuple when configured.
    #[inline]
    pub fn aria_describedby(&self) -> Option<(&'static str, &str)> {
        self.described_by.map(aria::aria_describedby)
    }

    /// Returns the `id` tuple when configured.
    #[inline]
    pub fn id_attr(&self) -> Option<(&'static str, &str)> {
        self.id.map(|value| ("id", value))
    }

    /// Returns a helper attribute capturing the configured anchor.  This uses a
    /// data attribute so adapters can style without reinventing naming
    /// conventions.
    #[inline]
    pub fn data_anchor(&self) -> (&'static str, &'static str) {
        ("data-anchor", self.state.anchor.as_str())
    }
}

/// Builder for drawer backdrop attributes.
#[derive(Debug, Clone)]
pub struct DrawerBackdropAttributes<'a> {
    state: &'a DrawerState,
}

impl<'a> DrawerBackdropAttributes<'a> {
    #[inline]
    fn new(state: &'a DrawerState) -> Self {
        Self { state }
    }

    /// Backdrops are hidden from assistive technology while still signalling
    /// modality to layout engines.
    #[inline]
    pub fn aria_hidden(&self) -> (&'static str, &'static str) {
        ("aria-hidden", "true")
    }

    /// Returns whether the backdrop should be rendered.  Persistent drawers do
    /// not require a backdrop since they keep the page interactive.
    #[inline]
    pub fn is_visible(&self) -> bool {
        self.state.variant.is_modal()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uncontrolled_drawer_mutates_internal_state() {
        let mut state = DrawerState::new(
            false,
            ControlStrategy::Uncontrolled,
            DrawerVariant::Modal,
            DrawerAnchor::Start,
        );
        state.open(|_| {});
        assert!(state.is_open());
        state.close(|_| {});
        assert!(!state.is_open());
    }

    #[test]
    fn controlled_drawer_emits_intents_without_mutating_state() {
        let mut state = DrawerState::new(
            false,
            ControlStrategy::Controlled,
            DrawerVariant::Modal,
            DrawerAnchor::End,
        );
        let mut last = None;
        state.toggle(|open| last = Some(open));
        assert_eq!(state.is_open(), false);
        assert_eq!(last, Some(true));
        state.sync_open(true);
        assert!(state.is_open());
    }

    #[test]
    fn surface_builder_emits_expected_attributes() {
        let state = DrawerState::new(
            true,
            ControlStrategy::Uncontrolled,
            DrawerVariant::Modal,
            DrawerAnchor::Bottom,
        );
        let attrs = state
            .surface_attributes()
            .id("drawer")
            .labelled_by("drawer-title")
            .described_by("drawer-description");
        assert_eq!(attrs.role(), "dialog");
        assert_eq!(attrs.aria_modal(), ("aria-modal", "true"));
        assert_eq!(attrs.id_attr(), Some(("id", "drawer")));
        assert_eq!(
            attrs.aria_labelledby(),
            Some(("aria-labelledby", "drawer-title"))
        );
        assert_eq!(
            attrs.aria_describedby(),
            Some(("aria-describedby", "drawer-description"))
        );
        assert_eq!(attrs.data_anchor(), ("data-anchor", "bottom"));
    }

    #[test]
    fn backdrop_visibility_tracks_variant() {
        let modal = DrawerState::new(
            true,
            ControlStrategy::Uncontrolled,
            DrawerVariant::Modal,
            DrawerAnchor::Start,
        );
        assert!(modal.backdrop_attributes().is_visible());

        let persistent = DrawerState::new(
            true,
            ControlStrategy::Uncontrolled,
            DrawerVariant::Persistent,
            DrawerAnchor::End,
        );
        assert!(!persistent.backdrop_attributes().is_visible());
    }
}
