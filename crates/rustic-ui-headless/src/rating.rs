#![deny(missing_docs)]
//! Headless state machine powering Material style rating controls.
//!
//! The implementation centralises controlled/uncontrolled value management,
//! hover previews, half-step precision and analytics signalling so framework
//! adapters can remain declarative.  Rendering layers are responsible for
//! translating the returned descriptors into icons while this module focuses on
//! deterministic state transitions and WCAG aligned keyboard interaction.  The
//! state machine intentionally mirrors the semantics of MUI's `Rating`
//! component to ease migrations for enterprise teams.

use crate::{aria, interaction::ControlKey, selection::ControlStrategy};

/// Analytics payload emitted when the rating mutates.
#[derive(Debug, Clone, PartialEq)]
pub struct RatingAnalyticsEvent {
    /// Logical channel that downstream telemetry systems should attribute the
    /// event to.  The string is carried verbatim into Material renderers via a
    /// `data-rustic-analytics-channel` attribute.
    pub channel: Option<String>,
    /// Kind of transition that occurred.
    pub kind: RatingAnalyticsKind,
}

/// Describes the source interaction behind an analytics payload.
#[derive(Debug, Clone, PartialEq)]
pub enum RatingAnalyticsKind {
    /// Selection changed from `previous` to `next`.
    SelectionChanged {
        /// Value prior to the change.
        previous: f32,
        /// Value requested after the change.
        next: f32,
    },
    /// Hover preview changed to the provided value. `None` indicates the preview
    /// cleared.
    HoverPreview {
        /// Hovered preview value. `None` indicates the preview cleared.
        value: Option<f32>,
    },
    /// User explicitly confirmed the current selection (typically via Enter or
    /// Space).
    Commit {
        /// Value confirmed by the user.
        value: f32,
    },
}

/// Declarative configuration driving [`RatingState`].
#[derive(Debug, Clone)]
pub struct RatingConfig {
    /// Total number of icons rendered by the rating control.
    pub max: u32,
    /// Minimum increment applied when selecting or previewing values.
    pub precision: f32,
    /// Initial value applied for uncontrolled instances.
    pub default_value: f32,
    /// Whether the rating starts disabled.
    pub disabled: bool,
    /// Whether the rating is read-only (visual only, no interaction).
    pub read_only: bool,
    /// When set, selecting the same value twice clears the rating back to zero.
    pub clear_on_repeat: bool,
    /// Optional analytics channel propagated through telemetry attributes.
    pub analytics_channel: Option<String>,
}

impl RatingConfig {
    /// Enterprise focused defaults mirroring Material design guidance.
    pub fn enterprise_defaults() -> Self {
        Self {
            max: 5,
            precision: 0.5,
            default_value: 0.0,
            disabled: false,
            read_only: false,
            clear_on_repeat: true,
            analytics_channel: None,
        }
    }
}

impl Default for RatingConfig {
    fn default() -> Self {
        Self::enterprise_defaults()
    }
}

/// Snapshot describing a rendered icon.
#[derive(Debug, Clone, PartialEq)]
pub struct RatingItemDescriptor {
    /// Zero-based index of the icon.
    pub index: usize,
    /// Value represented by the icon (for example `1.0`, `2.0`, ...).
    pub value: f32,
    /// Fill ratio between `0.0` (empty) and `1.0` (fully filled).  Half steps
    /// map to `0.5`.
    pub fill: f32,
    /// Whether the icon represents the actively selected value.
    pub active: bool,
    /// Whether the icon is currently hovered.
    pub hovered: bool,
}

impl RatingItemDescriptor {
    fn empty(index: usize, value: f32) -> Self {
        Self {
            index,
            value,
            fill: 0.0,
            active: false,
            hovered: false,
        }
    }
}

/// Outcome produced after mutating the rating value.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RatingChangeOutcome {
    /// Updated value when the state machine mutated. `None` indicates the
    /// request was ignored (for example because the rating is disabled or the
    /// new value matched the previous one in controlled mode).
    pub value: Option<f32>,
    /// Analytics payload describing the change.
    pub analytics: Option<RatingAnalyticsEvent>,
}

/// Headless rating controller.
#[derive(Debug, Clone)]
pub struct RatingState {
    config: RatingConfig,
    control: ControlStrategy,
    value: f32,
    hover: Option<f32>,
    focus_visible: bool,
}

impl RatingState {
    /// Construct a controlled rating. The caller is responsible for updating the
    /// value by invoking [`RatingState::sync_value`] when [`RatingChangeOutcome`]
    /// emits a new value.
    pub fn controlled(config: RatingConfig, value: f32) -> Self {
        Self::new(ControlStrategy::Controlled, config, value)
    }

    /// Construct an uncontrolled rating that owns its internal value.
    pub fn uncontrolled(config: RatingConfig) -> Self {
        let initial = config.default_value;
        Self::new(ControlStrategy::Uncontrolled, config, initial)
    }

    fn new(control: ControlStrategy, mut config: RatingConfig, value: f32) -> Self {
        if config.max == 0 {
            config.max = 1;
        }
        let precision = config.precision.max(f32::EPSILON);
        config.precision = precision;
        let mut state = Self {
            control,
            value: 0.0,
            hover: None,
            focus_visible: false,
            config,
        };
        state.value = state.snap(value);
        state
    }

    /// Returns the configured maximum icon count.
    pub fn max(&self) -> u32 {
        self.config.max
    }

    /// Returns the currently selected value (ignoring hover previews).
    pub fn value(&self) -> f32 {
        self.value
    }

    /// Returns the value currently presented to the user. Hover previews take
    /// precedence over the committed selection.
    pub fn display_value(&self) -> f32 {
        self.hover.unwrap_or(self.value)
    }

    /// Returns the configured precision.
    pub fn precision(&self) -> f32 {
        self.config.precision
    }

    /// Returns whether the rating is disabled.
    pub fn is_disabled(&self) -> bool {
        self.config.disabled
    }

    /// Returns whether the rating is read-only.
    pub fn is_read_only(&self) -> bool {
        self.config.read_only
    }

    /// Update the disabled flag.
    pub fn set_disabled(&mut self, disabled: bool) {
        self.config.disabled = disabled;
    }

    /// Update the read-only flag.
    pub fn set_read_only(&mut self, read_only: bool) {
        self.config.read_only = read_only;
    }

    /// Synchronise the stored value (primarily used by controlled parents).
    pub fn sync_value(&mut self, value: f32) {
        self.value = self.snap(value);
    }

    /// Mark the rating as focused via keyboard navigation.
    pub fn focus(&mut self) {
        self.focus_visible = true;
    }

    /// Clear focus-visible styling.
    pub fn blur(&mut self) {
        self.focus_visible = false;
    }

    /// Returns whether focus-visible styling should apply.
    pub fn focus_visible(&self) -> bool {
        self.focus_visible
    }

    /// Returns the configured analytics channel if any.
    pub fn analytics_channel(&self) -> Option<&str> {
        self.config.analytics_channel.as_deref()
    }

    /// Update the hover preview. `None` clears the preview.
    pub fn set_hover(&mut self, value: Option<f32>) -> Option<RatingAnalyticsEvent> {
        if self.is_disabled() || self.is_read_only() {
            return None;
        }
        let next = value.map(|raw| self.snap(raw));
        if next == self.hover {
            return None;
        }
        self.hover = next;
        Some(RatingAnalyticsEvent {
            channel: self.config.analytics_channel.clone(),
            kind: RatingAnalyticsKind::HoverPreview { value: self.hover },
        })
    }

    /// Clear the hover preview if one is present.
    pub fn clear_hover(&mut self) -> Option<RatingAnalyticsEvent> {
        self.set_hover(None)
    }

    /// Select a new value.
    pub fn select(&mut self, value: f32) -> RatingChangeOutcome {
        if self.is_disabled() || self.is_read_only() {
            return RatingChangeOutcome::default();
        }
        let snapped = self.snap(value);
        let cleared = self.config.clear_on_repeat
            && (snapped - self.value).abs() < f32::EPSILON
            && snapped > 0.0;
        let next = if cleared { 0.0 } else { snapped };
        if self.control.is_controlled() {
            if (next - self.value).abs() < f32::EPSILON {
                return RatingChangeOutcome::default();
            }
            RatingChangeOutcome {
                value: Some(next),
                analytics: Some(RatingAnalyticsEvent {
                    channel: self.config.analytics_channel.clone(),
                    kind: RatingAnalyticsKind::SelectionChanged {
                        previous: self.value,
                        next,
                    },
                }),
            }
        } else {
            if (next - self.value).abs() < f32::EPSILON {
                return RatingChangeOutcome::default();
            }
            let previous = self.value;
            self.value = next;
            RatingChangeOutcome {
                value: Some(next),
                analytics: Some(RatingAnalyticsEvent {
                    channel: self.config.analytics_channel.clone(),
                    kind: RatingAnalyticsKind::SelectionChanged { previous, next },
                }),
            }
        }
    }

    /// Handle keyboard interaction. Returns the resulting change outcome.
    pub fn on_key(&mut self, key: ControlKey) -> RatingChangeOutcome {
        if self.is_disabled() || self.is_read_only() {
            return RatingChangeOutcome::default();
        }
        let delta = match key {
            ControlKey::ArrowRight | ControlKey::ArrowUp => self.config.precision,
            ControlKey::ArrowLeft | ControlKey::ArrowDown => -self.config.precision,
            ControlKey::Home => return self.select(0.0),
            ControlKey::End => return self.select(self.config.max as f32),
            ControlKey::Space | ControlKey::Enter => {
                return RatingChangeOutcome {
                    value: None,
                    analytics: Some(RatingAnalyticsEvent {
                        channel: self.config.analytics_channel.clone(),
                        kind: RatingAnalyticsKind::Commit { value: self.value },
                    }),
                }
            }
        };
        let candidate = (self.value + delta).clamp(0.0, self.config.max as f32);
        self.select(candidate)
    }

    /// Build ARIA/data attributes describing the root element.
    pub fn root_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::with_capacity(6);
        attrs.push(("role", aria::role_radiogroup().into()));
        attrs.push(("aria-disabled", self.is_disabled().to_string()));
        if self.is_read_only() {
            attrs.push(("data-read-only", "true".into()));
        }
        attrs.push(("data-value", format!("{:.2}", self.value())));
        attrs.push(("data-display-value", format!("{:.2}", self.display_value())));
        attrs.push((
            "data-focus-visible",
            if self.focus_visible() {
                "true"
            } else {
                "false"
            }
            .into(),
        ));
        if let Some(channel) = &self.config.analytics_channel {
            attrs.push(("data-rustic-analytics-channel", channel.clone()));
        }
        attrs
    }

    /// Build ARIA/data attributes for a specific icon.
    pub fn item_attributes(&self, index: usize) -> Vec<(&'static str, String)> {
        let descriptor = self.item_descriptor(index);
        let mut attrs = Vec::with_capacity(8);
        attrs.push(("role", aria::role_radio().into()));
        attrs.push(("aria-checked", descriptor.active.to_string()));
        attrs.push(("data-index", descriptor.index.to_string()));
        attrs.push(("data-value", format!("{:.2}", descriptor.value)));
        attrs.push(("data-fill", format!("{:.2}", descriptor.fill)));
        attrs.push(("data-hovered", descriptor.hovered.to_string()));
        attrs.push((
            "tabindex",
            if descriptor.active { "0" } else { "-1" }.into(),
        ));
        if self.is_disabled() || self.is_read_only() {
            attrs.push(("aria-disabled", "true".into()));
            attrs.push(("data-disabled", "true".into()));
        }
        attrs
    }

    /// Returns a descriptor representing the visual state of the icon at
    /// `index`.
    pub fn item_descriptor(&self, index: usize) -> RatingItemDescriptor {
        if index >= self.config.max as usize {
            return RatingItemDescriptor::empty(index, 0.0);
        }
        let value = (index + 1) as f32;
        let active_index = if self.value <= 0.0 {
            None
        } else {
            Some(
                (self.value.ceil() as usize)
                    .saturating_sub(1)
                    .min(self.config.max as usize - 1),
            )
        };
        let active = active_index == Some(index);
        let display = self.display_value();
        let hovered = self
            .hover
            .map(|hover| {
                let start = index as f32;
                let end = (index + 1) as f32;
                hover > start && hover <= end
            })
            .unwrap_or(false);
        let start = index as f32;
        let fill = (display - start).clamp(0.0, 1.0);
        RatingItemDescriptor {
            index,
            value,
            fill,
            active,
            hovered,
        }
    }

    fn snap(&self, raw: f32) -> f32 {
        let precision = self.config.precision;
        let steps = (raw / precision).round();
        let value = steps * precision;
        value.clamp(0.0, self.config.max as f32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snap_respects_precision() {
        let config = RatingConfig {
            precision: 0.5,
            ..RatingConfig::enterprise_defaults()
        };
        let state = RatingState::uncontrolled(config);
        assert!((state.snap(3.24) - 3.0).abs() < f32::EPSILON);
        assert!((state.snap(3.26) - 3.5).abs() < f32::EPSILON);
    }

    #[test]
    fn uncontrolled_select_updates_value() {
        let mut state = RatingState::uncontrolled(RatingConfig::enterprise_defaults());
        let outcome = state.select(3.0);
        assert_eq!(state.value(), 3.0);
        assert_eq!(outcome.value, Some(3.0));
    }

    #[test]
    fn controlled_select_returns_change_without_mutating() {
        let mut state = RatingState::controlled(RatingConfig::enterprise_defaults(), 2.0);
        let outcome = state.select(4.0);
        assert_eq!(state.value(), 2.0);
        assert_eq!(outcome.value, Some(4.0));
    }

    #[test]
    fn hover_emits_preview_event() {
        let mut state = RatingState::uncontrolled(RatingConfig::enterprise_defaults());
        let event = state.set_hover(Some(2.5)).unwrap();
        assert!(matches!(
            event.kind,
            RatingAnalyticsKind::HoverPreview { .. }
        ));
    }

    #[test]
    fn keyboard_arrow_increments() {
        let mut state = RatingState::uncontrolled(RatingConfig::enterprise_defaults());
        state.select(2.0);
        let outcome = state.on_key(ControlKey::ArrowRight);
        assert_eq!(outcome.value, Some(2.5));
        assert_eq!(state.value(), 2.5);
    }

    #[test]
    fn commit_event_emitted_on_enter() {
        let mut state = RatingState::uncontrolled(RatingConfig::enterprise_defaults());
        state.select(3.0);
        let outcome = state.on_key(ControlKey::Enter);
        assert!(matches!(
            outcome.analytics,
            Some(RatingAnalyticsEvent {
                kind: RatingAnalyticsKind::Commit { value },
                ..
            }) if (value - 3.0).abs() < f32::EPSILON
        ));
    }

    #[test]
    fn item_descriptor_reflects_half_fill() {
        let mut state = RatingState::uncontrolled(RatingConfig::enterprise_defaults());
        state.select(2.5);
        let descriptor = state.item_descriptor(2);
        assert!((descriptor.fill - 0.5).abs() < f32::EPSILON);
    }
}
