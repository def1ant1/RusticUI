#![deny(missing_docs)]
//! Focus-aware collapsible regions with deterministic concurrency semantics.
//!
//! Enterprise surfaces routinely coordinate multiple animations (height, opacity,
//! focus transitions) whenever a disclosure region expands or collapses.  This
//! state machine keeps those transitions serialized by issuing caller-managed
//! tokens whenever a transition begins.  Because the active tokens are tracked
//! with a [`BTreeSet`], integrations that spawn concurrent async tasks observe a
//! consistent ordering which keeps logging, telemetry, and snapshot tests
//! repeatable across platforms.
//!
//! Accessibility hooks mirror the [`aria::disclosure`] pattern so assistive
//! technology knows which region is controlled by which trigger.  The attribute
//! builders emit automation-first `data-rustic-*` markers to keep analytics and
//! QA pipelines centralized instead of wiring per-component selectors.

use std::collections::BTreeSet;

use crate::selection::ControlStrategy;

/// Describes the observable change after mutating the region.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegionTransition {
    /// No state change occurred.
    NoChange,
    /// The region is now expanded.
    Expanded,
    /// The region is now collapsed.
    Collapsed,
}

/// State machine backing disclosure regions such as accordions and FAQs.
#[derive(Debug, Clone)]
pub struct CollapsibleRegionState {
    control_strategy: ControlStrategy,
    expanded: bool,
    focus_return: Option<String>,
    active_tokens: BTreeSet<u64>,
}

impl CollapsibleRegionState {
    /// Construct an uncontrolled region with a default expanded state.
    pub fn uncontrolled(default_expanded: bool) -> Self {
        Self {
            control_strategy: ControlStrategy::Uncontrolled,
            expanded: default_expanded,
            focus_return: None,
            active_tokens: BTreeSet::new(),
        }
    }

    /// Construct a controlled region.  The caller must invoke [`sync`] after
    /// receiving notifications from [`expand`], [`collapse`], or [`toggle`].
    pub fn controlled() -> Self {
        Self {
            control_strategy: ControlStrategy::Controlled,
            expanded: false,
            focus_return: None,
            active_tokens: BTreeSet::new(),
        }
    }

    /// Returns whether the region is currently expanded.
    #[inline]
    pub const fn is_expanded(&self) -> bool {
        self.expanded
    }

    /// Update the focus return target.  When the region collapses adapters
    /// typically focus this identifier to keep keyboard users oriented.
    pub fn set_focus_return(&mut self, id: Option<impl Into<String>>) {
        self.focus_return = id.map(Into::into);
    }

    /// Returns the configured focus return target.
    #[inline]
    pub fn focus_return_target(&self) -> Option<&str> {
        self.focus_return.as_deref()
    }

    /// Attempt to reserve a transition token.  Returns `true` when the token was
    /// inserted and `false` when the token already existed.
    pub fn begin_transition(&mut self, token: u64) -> bool {
        self.active_tokens.insert(token)
    }

    /// Mark the transition associated with `token` as finished.
    pub fn finish_transition(&mut self, token: u64) {
        self.active_tokens.remove(&token);
    }

    /// Returns whether there are active transitions.
    #[inline]
    pub fn is_transitioning(&self) -> bool {
        !self.active_tokens.is_empty()
    }

    /// Expand the region.
    pub fn expand<F: FnOnce(bool)>(&mut self, notify: F) -> RegionTransition {
        if self.expanded {
            return RegionTransition::NoChange;
        }
        if !self.control_strategy.is_controlled() {
            self.expanded = true;
        }
        notify(true);
        RegionTransition::Expanded
    }

    /// Collapse the region.
    pub fn collapse<F: FnOnce(bool)>(&mut self, notify: F) -> RegionTransition {
        if !self.expanded {
            return RegionTransition::NoChange;
        }
        if !self.control_strategy.is_controlled() {
            self.expanded = false;
        }
        notify(false);
        RegionTransition::Collapsed
    }

    /// Toggle the region.
    pub fn toggle<F: FnOnce(bool)>(&mut self, notify: F) -> RegionTransition {
        if self.expanded {
            self.collapse(notify)
        } else {
            self.expand(notify)
        }
    }

    /// Synchronize the expanded state for controlled integrations.
    pub fn sync(&mut self, expanded: bool) {
        self.expanded = expanded;
    }

    /// Returns attributes for the trigger element.
    pub fn trigger_attributes(&self) -> CollapsibleTriggerAttributes<'_> {
        CollapsibleTriggerAttributes {
            expanded: self.expanded,
            controls: None,
            analytics_tag: None,
        }
    }

    /// Returns attributes for the region element.
    pub fn region_attributes(&self) -> CollapsibleContentAttributes<'_> {
        CollapsibleContentAttributes {
            expanded: self.expanded,
            id: None,
            analytics_tag: None,
        }
    }
}

/// Builder for trigger attributes (usually the button controlling the region).
#[derive(Debug, Clone)]
pub struct CollapsibleTriggerAttributes<'a> {
    expanded: bool,
    controls: Option<&'a str>,
    analytics_tag: Option<&'a str>,
}

impl<'a> CollapsibleTriggerAttributes<'a> {
    /// Attach an `aria-controls` relationship.
    pub fn controls(mut self, id: &'a str) -> Self {
        self.controls = Some(id);
        self
    }

    /// Attach an analytics identifier for QA pipelines.
    pub fn analytics_id(mut self, id: &'a str) -> Self {
        self.analytics_tag = Some(id);
        self
    }

    /// Returns the `aria-expanded` tuple.
    #[inline]
    pub fn aria_expanded(&self) -> (&'static str, &'static str) {
        (
            "aria-expanded",
            if self.expanded { "true" } else { "false" },
        )
    }

    /// Returns the optional `aria-controls` tuple.
    #[inline]
    pub fn aria_controls(&self) -> Option<(&'static str, &'a str)> {
        self.controls.map(|id| ("aria-controls", id))
    }

    /// Returns the analytics tuple when configured.
    #[inline]
    pub fn analytics_attribute(&self) -> Option<(&'static str, &'a str)> {
        self.analytics_tag
            .map(|value| ("data-rustic-analytics-id", value))
    }

    /// Collect the configured attributes into a vector for JSX/Sycamore adapters.
    pub fn as_pairs(&self) -> Vec<(&'static str, String)> {
        let mut pairs = Vec::with_capacity(3);
        let (key, value) = self.aria_expanded();
        pairs.push((key, value.to_string()));
        if let Some((key, value)) = self.aria_controls() {
            pairs.push((key, value.to_string()));
        }
        if let Some((key, value)) = self.analytics_attribute() {
            pairs.push((key, value.to_string()));
        }
        pairs.push(("data-rustic-collapsible", "trigger".to_string()));
        pairs
    }
}

/// Builder for the collapsible region attributes.
#[derive(Debug, Clone)]
pub struct CollapsibleContentAttributes<'a> {
    expanded: bool,
    id: Option<&'a str>,
    analytics_tag: Option<&'a str>,
}

impl<'a> CollapsibleContentAttributes<'a> {
    /// Attach an identifier that matches the trigger's `aria-controls` value.
    pub fn id(mut self, id: &'a str) -> Self {
        self.id = Some(id);
        self
    }

    /// Attach an analytics identifier.
    pub fn analytics_id(mut self, id: &'a str) -> Self {
        self.analytics_tag = Some(id);
        self
    }

    /// Returns whether the region should be hidden.
    #[inline]
    pub fn hidden(&self) -> bool {
        !self.expanded
    }

    /// Returns the optional id tuple.
    #[inline]
    pub fn id_attribute(&self) -> Option<(&'static str, &'a str)> {
        self.id.map(|value| ("id", value))
    }

    /// Returns the analytics tuple when configured.
    #[inline]
    pub fn analytics_attribute(&self) -> Option<(&'static str, &'a str)> {
        self.analytics_tag
            .map(|value| ("data-rustic-analytics-id", value))
    }

    /// Collects the configured pairs for use in adapters.
    pub fn as_pairs(&self) -> Vec<(&'static str, String)> {
        let mut pairs = Vec::with_capacity(4);
        if let Some((key, value)) = self.id_attribute() {
            pairs.push((key, value.to_string()));
        }
        if self.hidden() {
            pairs.push(("data-hidden", "true".to_string()));
        }
        if let Some((key, value)) = self.analytics_attribute() {
            pairs.push((key, value.to_string()));
        }
        pairs.push(("data-rustic-collapsible", "region".to_string()));
        pairs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uncontrolled_region_updates_immediately() {
        let mut state = CollapsibleRegionState::uncontrolled(false);
        let mut observed = false;
        let transition = state.expand(|expanded| observed = expanded);
        assert!(matches!(transition, RegionTransition::Expanded));
        assert!(state.is_expanded());
        assert!(observed);
    }

    #[test]
    fn controlled_region_requires_sync() {
        let mut state = CollapsibleRegionState::controlled();
        let transition = state.expand(|_| {});
        assert!(matches!(transition, RegionTransition::Expanded));
        assert!(!state.is_expanded());
        state.sync(true);
        assert!(state.is_expanded());
    }

    #[test]
    fn transition_tokens_are_unique() {
        let mut state = CollapsibleRegionState::uncontrolled(false);
        assert!(state.begin_transition(1));
        assert!(!state.begin_transition(1));
        state.finish_transition(1);
        assert!(!state.is_transitioning());
    }

    #[test]
    fn attribute_builders_emit_expected_pairs() {
        let state = CollapsibleRegionState::uncontrolled(true);
        let trigger = state
            .trigger_attributes()
            .controls("region-1")
            .analytics_id("trigger")
            .as_pairs();
        assert_eq!(trigger.len(), 4);
        let region = state
            .region_attributes()
            .id("region-1")
            .analytics_id("region")
            .as_pairs();
        assert_eq!(region.len(), 3);
    }
}
